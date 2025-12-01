use async_trait::async_trait;
use futures::future::BoxFuture;
use sea_orm::{
    AccessMode, ConnectionTrait, DatabaseBackend, DatabaseTransaction, DbErr, ExecResult,
    IsolationLevel, QueryResult, Statement, TransactionError, TransactionTrait,
};
use std::collections::HashSet;

use super::registry::{is_connection_like_error, registry, DB_REGISTRY};

pub struct AutoRouter;
static AUTO_ROUTER: AutoRouter = AutoRouter;

pub fn db_auto() -> &'static AutoRouter {
    &AUTO_ROUTER
}

fn is_locking_select(sql: &str) -> bool {
    let up = sql.to_ascii_uppercase();
    up.contains("FOR UPDATE")
        || up.contains("FOR SHARE")
        || up.contains("LOCK IN SHARE MODE")
        || up.contains("FOR NO KEY UPDATE")
        || up.contains("FOR KEY SHARE")
}

fn stmt_sql(stmt: &Statement) -> String {
    stmt.to_string()
}

#[async_trait]
impl ConnectionTrait for AutoRouter {
    fn get_database_backend(&self) -> DatabaseBackend {
        if let Some(reg) = DB_REGISTRY.get() {
            reg.backend
        } else {
            tracing::error!("AutoRouter used before warm_up, default to Postgres backend");
            DatabaseBackend::Postgres
        }
    }

    fn support_returning(&self) -> bool {
        if let Some(reg) = DB_REGISTRY.get() {
            reg.conns[reg.write_idx()].support_returning()
        } else {
            false
        }
    }

    async fn execute(&self, stmt: Statement) -> Result<ExecResult, DbErr> {
        let reg = registry().await;
        reg.conns[reg.write_idx()].execute(stmt).await
    }

    async fn execute_unprepared(&self, sql: &str) -> Result<ExecResult, DbErr> {
        let reg = registry().await;
        reg.conns[reg.write_idx()].execute_unprepared(sql).await
    }

    async fn query_one(&self, stmt: Statement) -> Result<Option<QueryResult>, DbErr> {
        let reg = registry().await;
        let sql = stmt_sql(&stmt);

        if is_locking_select(&sql) || reg.read_count() == 0 {
            return reg.conns[reg.write_idx()].query_one(stmt).await;
        }

        let Some(rw) = &reg.rw else {
            return reg.conns[reg.default_idx].query_one(stmt).await;
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

            match reg.conns[idx].query_one(stmt.clone()).await {
                Ok(v) => return Ok(v),
                Err(e) => {
                    if is_connection_like_error(&e) && reg.is_read_idx(idx) {
                        reg.mark_down(idx);
                        tracing::warn!("query_one failed on read idx={}, marked down: {}", idx, e);
                        last_err = Some(e);
                        continue;
                    }
                    return Err(e);
                }
            }
        }

        if rw.fallback_to_write {
            tracing::warn!("query_one: all readers failed, fallback to writer");
            return reg.conns[rw.write_idx].query_one(stmt).await;
        }

        Err(last_err.unwrap_or_else(|| DbErr::Custom("all read attempts failed".into())))
    }

    async fn query_all(&self, stmt: Statement) -> Result<Vec<QueryResult>, DbErr> {
        let reg = registry().await;
        let sql = stmt_sql(&stmt);

        if is_locking_select(&sql) || reg.read_count() == 0 {
            return reg.conns[reg.write_idx()].query_all(stmt).await;
        }

        let Some(rw) = &reg.rw else {
            return reg.conns[reg.default_idx].query_all(stmt).await;
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

            match reg.conns[idx].query_all(stmt.clone()).await {
                Ok(v) => return Ok(v),
                Err(e) => {
                    if is_connection_like_error(&e) && reg.is_read_idx(idx) {
                        reg.mark_down(idx);
                        tracing::warn!("query_all failed on read idx={}, marked down: {}", idx, e);
                        last_err = Some(e);
                        continue;
                    }
                    return Err(e);
                }
            }
        }

        if rw.fallback_to_write {
            tracing::warn!("query_all: all readers failed, fallback to writer");
            return reg.conns[rw.write_idx].query_all(stmt).await;
        }

        Err(last_err.unwrap_or_else(|| DbErr::Custom("all read attempts failed".into())))
    }

    fn is_mock_connection(&self) -> bool {
        false
    }
}

#[async_trait]
impl TransactionTrait for AutoRouter {
    async fn begin(&self) -> Result<DatabaseTransaction, DbErr> {
        let reg = registry().await;
        reg.conns[reg.write_idx()].begin().await
    }

    async fn begin_with_config(
        &self,
        isolation_level: Option<IsolationLevel>,
        access_mode: Option<AccessMode>,
    ) -> Result<DatabaseTransaction, DbErr> {
        let reg = registry().await;
        reg.conns[reg.write_idx()]
            .begin_with_config(isolation_level, access_mode)
            .await
    }

    async fn transaction<F, T, E>(&self, txn: F) -> Result<T, TransactionError<E>>
    where
        F: for<'c> FnOnce(&'c DatabaseTransaction) -> BoxFuture<'c, Result<T, E>> + Send,
        T: Send,
        E: std::fmt::Debug + std::fmt::Display + Send,
    {
        let reg = registry().await;
        reg.conns[reg.write_idx()].transaction(txn).await
    }

    async fn transaction_with_config<F, T, E>(
        &self,
        txn: F,
        isolation_level: Option<IsolationLevel>,
        access_mode: Option<AccessMode>,
    ) -> Result<T, TransactionError<E>>
    where
        F: for<'c> FnOnce(&'c DatabaseTransaction) -> BoxFuture<'c, Result<T, E>> + Send,
        T: Send,
        E: std::fmt::Debug + std::fmt::Display + Send,
    {
        let reg = registry().await;
        reg.conns[reg.write_idx()]
            .transaction_with_config(txn, isolation_level, access_mode)
            .await
    }
}
