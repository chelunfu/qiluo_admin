use futures::future::join_all;
use rand::{
    distr::{weighted::WeightedIndex, Distribution},
    rng,
    seq::IndexedRandom,
};
use sea_orm::{
    entity::prelude::DatabaseConnection, ConnectOptions, ConnectionTrait, Database,
    DatabaseBackend, DbErr,
};
use std::{
    collections::{HashMap, HashSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, Instant},
};
use parking_lot::Mutex;
use tokio::sync::OnceCell;

use crate::config::{
    appconfig::{DbCfg, ReadStrategy},
    APPCOFIG,
};

// 只在 db 模块内可见，供 router.rs 使用
pub(super) static DB_REGISTRY: OnceCell<DbRegistry> = OnceCell::const_new();

// 只在 db 模块内可见
pub(super) struct DbRegistry {
    pub(super) conns: Vec<DatabaseConnection>,
    pub(super) by_name: HashMap<String, usize>,
    pub(super) default_idx: usize,
    pub(super) rw: Option<RwRule>,
    pub(super) rr: AtomicUsize,
    pub(super) health: Mutex<HealthState>,
    pub(super) backend: DatabaseBackend,
}

pub(super) struct RwRule {
    pub(super) write_idx: usize,
    pub(super) reads: Vec<ReadReplica>,
    pub(super) strategy: ReadStrategy,
    pub(super) retry_attempts: usize,
    pub(super) circuit_break: Duration,
    pub(super) fallback_to_write: bool,
}

#[derive(Clone)]
pub(super) struct ReadReplica {
    pub(super) idx: usize,
    pub(super) weight: u32,
}

#[derive(Default)]
pub(super) struct HealthState {
    pub(super) down_until: Vec<Option<Instant>>,
}

fn build_options(cfg: &DbCfg) -> ConnectOptions {
    let mut opt = ConnectOptions::new(&cfg.uri);
    opt.max_connections(cfg.max_connections)
        .min_connections(cfg.min_connections)
        .connect_timeout(Duration::from_millis(cfg.connect_timeout))
        .idle_timeout(Duration::from_millis(cfg.idle_timeout))
        .sqlx_logging(cfg.enable_logging);
    opt
}

async fn init_registry() -> DbRegistry {
    let app_cfg = APPCOFIG.clone();
    if app_cfg.databases.is_empty() {
        panic!("No databases configured");
    }

    // 并发连接（失败不 panic，跳过）
    let tasks = app_cfg.databases.iter().map(|cfg| {
        let cfg = cfg.clone();
        async move {
            match Database::connect(build_options(&cfg)).await {
                Ok(db) => {
                    tracing::info!("Database [{}] connected", cfg.name);
                    Ok((cfg.name, db))
                }
                Err(e) => {
                    tracing::error!("Database [{}] connect failed: {}", cfg.name, e);
                    Err((cfg.name, e))
                }
            }
        }
    });
    let results = join_all(tasks).await;

    let mut conns: Vec<DatabaseConnection> = Vec::new();
    let mut by_name: HashMap<String, usize> = HashMap::new();
    for r in results {
        match r {
            Ok((name, db)) => {
                if by_name.contains_key(&name) {
                    tracing::warn!(
                        "Duplicated database name '{}', keep the first, ignore later",
                        name
                    );
                    continue;
                }
                let idx = conns.len();
                by_name.insert(name, idx);
                conns.push(db);
            }
            Err((name, _)) => {
                tracing::error!("Skip database '{}' due to connection failure", name);
            }
        }
    }

    if conns.is_empty() {
        panic!("All database connections failed, abort start");
    }

    // 路由与后端
    let routing = app_cfg.db_routing.clone().unwrap_or_default();
    let resolve = |name: &str| -> Option<usize> { by_name.get(name).copied() };

    let default_idx = match routing.default.as_deref().and_then(resolve) {
        Some(i) => i,
        None => {
            if let Some(n) = routing.default.as_deref() {
                tracing::warn!("db_routing.default unknown '{}', fallback to index 0", n);
            }
            0
        }
    };
    let backend = conns[default_idx].get_database_backend();

    let write_idx = match routing.write.as_deref().and_then(resolve) {
        Some(i) => i,
        None => {
            if let Some(n) = routing.write.as_deref() {
                tracing::warn!("db_routing.write unknown '{}', fallback to default", n);
            }
            default_idx
        }
    };

    // 构建读库（过滤后端不一致）
    let mut reads: Vec<ReadReplica> = Vec::new();
    let weights = routing.read_weights.clone().unwrap_or_default();
    if let Some(read_names) = routing.reads.clone() {
        for n in read_names {
            match resolve(&n) {
                Some(i) => {
                    if conns[i].get_database_backend() != backend {
                        tracing::warn!("read '{}' backend differs from writer, skip", n);
                        continue;
                    }
                    let w = weights.get(&n).copied().unwrap_or(1).max(1);
                    reads.push(ReadReplica { idx: i, weight: w });
                }
                None => tracing::warn!("db_routing.reads unknown '{}', skip", n),
            }
        }
    }

    let circuit_break = Duration::from_millis(routing.circuit_break_ms);

    let health = HealthState {
        down_until: vec![None; conns.len()],
    };

    DbRegistry {
        conns,
        by_name,
        default_idx,
        rw: Some(RwRule {
            write_idx,
            reads,
            strategy: routing.read_strategy,
            retry_attempts: routing.retry_attempts,
            circuit_break,
            fallback_to_write: routing.fallback_to_write,
        }),
        rr: AtomicUsize::new(0),
        health: Mutex::new(health),
        backend,
    }
}

// 给 db 模块内部（包括 router.rs）用
pub(super) async fn registry() -> &'static DbRegistry {
    DB_REGISTRY.get_or_init(init_registry).await
}

// ---------- 对外 API：DB / DB_WRITE / DB_READ / DB_BY_NAME / DB_BY_INDEX ----------

pub async fn db() -> &'static DatabaseConnection {
    let reg = registry().await;
    &reg.conns[reg.default_idx]
}

pub async fn db_write() -> &'static DatabaseConnection {
    let reg = registry().await;
    if let Some(rw) = &reg.rw {
        &reg.conns[rw.write_idx]
    } else {
        &reg.conns[reg.default_idx]
    }
}

pub async fn db_read() -> &'static DatabaseConnection {
    let reg = registry().await;
    let idx = reg
        .pick_next_read_idx(&HashSet::new())
        .unwrap_or(reg.default_idx);
    &reg.conns[idx]
}

pub async fn db_by_name(name: &str) -> &'static DatabaseConnection {
    let reg = registry().await;
    match reg.by_name.get(name) {
        Some(idx) => &reg.conns[*idx],
        None => {
            tracing::error!(
                "db_by_name: unknown '{}', fallback to default index={}",
                name,
                reg.default_idx
            );
            &reg.conns[reg.default_idx]
        }
    }
}

pub async fn db_by_index(index: usize) -> &'static DatabaseConnection {
    let reg = registry().await;
    match reg.conns.get(index) {
        Some(_) => &reg.conns[index],
        None => {
            tracing::error!(
                "db_by_index: index {} out of range (total={}), fallback to default index={}",
                index,
                reg.conns.len(),
                reg.default_idx
            );
            &reg.conns[reg.default_idx]
        }
    }
}

// ---------- with_read 封装 ----------
/*
SeaORM 实体查询：
let post = with_read(|db| async move {
entity::post::Entity::find_by_id(id).one(db).await
}).await?;

原生 SQL：
use sea_orm::{Statement, DatabaseBackend};
let row = with_read(|db| async move {
let stmt = Statement::from_string(DatabaseBackend::Postgres, "SELECT 1".into());
db.query_one(stmt).await
}).await?;
 */
// 包装一次“读操作”：连接类错误自动切换读库并重试；必要时回退写库
pub async fn with_read<T, F, Fut>(op: F) -> Result<T, DbErr>
where
    F: Fn(&DatabaseConnection) -> Fut + Copy,
    Fut: std::future::Future<Output = Result<T, DbErr>>,
{
    let reg = registry().await;
    let Some(rw) = &reg.rw else {
        return op(&reg.conns[reg.default_idx]).await;
    };

    let mut tried: HashSet<usize> = HashSet::new();
    let mut last_err: Option<DbErr> = None;
    let base_tries = rw.reads.len() + if rw.fallback_to_write { 1 } else { 0 };
    let total_tries = base_tries + rw.retry_attempts;

    for _ in 0..total_tries.max(1) {
        let Some(idx) = reg.pick_next_read_idx(&tried) else {
            break;
        };
        tried.insert(idx);

        match op(&reg.conns[idx]).await {
            Ok(v) => return Ok(v),
            Err(e) => {
                if is_connection_like_error(&e) && reg.is_read_idx(idx) {
                    reg.mark_down(idx);
                    tracing::warn!("with_read failed on idx={}, marked down: {}", idx, e);
                    last_err = Some(e);
                    continue;
                }
                return Err(e);
            }
        }
    }

    if let Some(rw) = &reg.rw {
        if rw.fallback_to_write {
            tracing::warn!("with_read: all readers failed, fallback to writer");
            return op(&reg.conns[rw.write_idx]).await;
        }
    }
    Err(last_err.unwrap_or_else(|| DbErr::Custom("all read attempts failed".into())))
}
// ---------- DbRegistry 的内部方法，给 router.rs 使用 ----------

impl DbRegistry {
    pub(super) fn write_idx(&self) -> usize {
        self.rw
            .as_ref()
            .map(|rw| rw.write_idx)
            .unwrap_or(self.default_idx)
    }

    pub(super) fn read_count(&self) -> usize {
        self.rw.as_ref().map(|rw| rw.reads.len()).unwrap_or(0)
    }

    pub(super) fn is_read_idx(&self, idx: usize) -> bool {
        self.rw
            .as_ref()
            .map(|rw| rw.reads.iter().any(|r| r.idx == idx))
            .unwrap_or(false)
    }

    pub(super) fn mark_down(&self, idx: usize) {
        if let Some(rw) = &self.rw {
            let mut h = self.health.lock();
            let until = Instant::now() + rw.circuit_break;
            if let Some(slot) = h.down_until.get_mut(idx) {
                *slot = Some(until);
            }
        }
    }

    pub(super) fn is_healthy(&self, idx: usize) -> bool {
        let now = Instant::now();
        let mut h = self.health.lock();

        if let Some(opt) = h.down_until.get_mut(idx) {
            match opt {
                Some(until) if now >= *until => {
                    *opt = None;
                    true
                }
                Some(_) => false,
                None => true,
            }
        } else {
            true
        }
    }

    // 选择读库：优先健康读库；为空且允许回退 -> 写库；否则 None
    pub(super) fn pick_next_read_idx(&self, exclude: &HashSet<usize>) -> Option<usize> {
        let Some(rw) = &self.rw else {
            return Some(self.default_idx);
        };

        let now = Instant::now();
        let health_snapshot: Vec<bool> = {
            let mut h = self.health.lock();
            rw.reads
                .iter()
                .map(|r| {
                    if let Some(opt) = h.down_until.get_mut(r.idx) {
                        match opt {
                            Some(until) if now >= *until => {
                                *opt = None;
                                true
                            }
                            Some(_) => false,
                            None => true,
                        }
                    } else {
                        true
                    }
                })
                .collect()
        }; // 锁在这里释放

        let healthy: Vec<&ReadReplica> = rw
            .reads
            .iter()
            .enumerate()
            .filter(|(i, r)| !exclude.contains(&r.idx) && health_snapshot[*i])
            .map(|(_, r)| r)
            .collect();

        if healthy.is_empty() {
            if rw.fallback_to_write && !exclude.contains(&rw.write_idx) {
                return Some(rw.write_idx);
            }
            return None;
        }

        let chosen_idx = match rw.strategy {
            ReadStrategy::RoundRobin => {
                let i = self.rr.fetch_add(1, Ordering::Relaxed);
                healthy[i % healthy.len()].idx
            }
            ReadStrategy::Random => {
                let mut rng = rng();
                healthy.choose(&mut rng).unwrap().idx
            }
            ReadStrategy::Weighted => {
                let weights: Vec<u32> = healthy.iter().map(|r| r.weight.max(1)).collect();
                if let Ok(dist) = WeightedIndex::new(weights) {
                    let mut rng = rng();
                    let pos = dist.sample(&mut rng);
                    healthy[pos].idx
                } else {
                    let i = self.rr.fetch_add(1, Ordering::Relaxed);
                    healthy[i % healthy.len()].idx
                }
            }
        };

        Some(chosen_idx)
    }
}

// 给 with_read + router.rs 共用
pub(super) fn is_connection_like_error(e: &DbErr) -> bool {
    let s = e.to_string().to_ascii_lowercase();
    let hints = [
        "connection",
        "closed",
        "timeout",
        "timed out",
        "pool timed out",
        "io",
        "i/o",
        "network",
        "broken pipe",
        "reset",
        "eof",
        "transport",
    ];
    hints.iter().any(|h| s.contains(h))
}
