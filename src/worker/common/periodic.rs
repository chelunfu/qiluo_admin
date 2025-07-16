use super::{Job, Processor, Result, Worker};
use crate::cache::CacheManager;
use crate::GID;
pub use cron_clock::{Schedule as Cron, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::str::FromStr;

// Redis 键常量
const PERIODIC_KEY: &str = "periodic";
const DEFAULT_QUEUE: &str = "default";
const DEFAULT_JOB_NAME: &str = "Scheduled PeriodicJob";

/// 解析 cron 表达式
pub fn parse(cron: &str) -> Result<Cron> {
    Cron::from_str(cron).map_err(Into::into)
}

/// 销毁所有定期任务
pub async fn destroy_all() -> Result<()> {
    let cache = CacheManager::instance().await;
    let _ = cache.remove(PERIODIC_KEY).await;
    Ok(())
}

/// 任务构建器
#[derive(Debug, Clone)]
pub struct Builder {
    pub name: Option<String>,
    pub queue: Option<String>,
    pub args: Option<JsonValue>,
    pub retry: Option<bool>,
    pub cron: Cron,
}

/// 创建任务构建器
pub fn builder(cron_str: &str) -> Result<Builder> {
    Ok(Builder {
        name: None,
        queue: None,
        args: None,
        retry: None,
        cron: Cron::from_str(cron_str)?,
    })
}

impl Builder {
    /// 设置任务名称
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置队列名称
    pub fn queue<S: Into<String>>(mut self, queue: S) -> Self {
        self.queue = Some(queue.into());
        self
    }

    /// 设置任务参数
    pub fn args<Args>(mut self, args: Args) -> Result<Self>
    where
        Args: Sync + Send + for<'de> Deserialize<'de> + Serialize + 'static,
    {
        let args_value = serde_json::to_value(args)?;

        // 确保参数始终是数组格式
        self.args = Some(match args_value {
            JsonValue::Array(_) => args_value,
            other => JsonValue::Array(vec![other]),
        });

        Ok(self)
    }

    /// 设置重试选项
    #[must_use]
    pub fn retry(mut self, retry: bool) -> Self {
        self.retry = Some(retry);
        self
    }

    /// 注册任务处理器
    pub async fn register<W, Args>(self, processor: &mut Processor, worker: W) -> Result<()>
    where
        Args: Sync + Send + for<'de> Deserialize<'de> + 'static,
        W: Worker<Args> + 'static,
    {
        processor.register(worker);
        let periodic_job = self.into_periodic_job(W::class_name())?;
        processor.register_periodic(periodic_job).await?;
        Ok(())
    }

    /// 转换为定期任务
    pub fn into_periodic_job(self, class_name: String) -> Result<PeriodicJob> {
        let name = self.name.unwrap_or_else(|| DEFAULT_JOB_NAME.to_string());

        PeriodicJob::new(
            name, class_name, self.cron, self.queue, self.args, self.retry,
        )
    }
}

/// 定期任务结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PeriodicJob {
    pub name: String,
    pub class: String,
    pub cron: String,
    pub queue: Option<String>,
    pub args: Option<String>,
    retry: Option<bool>,

    #[serde(skip)]
    cron_schedule: Option<Cron>,

    #[serde(skip)]
    json_args: Option<JsonValue>,
}

impl Default for PeriodicJob {
    fn default() -> Self {
        Self {
            name: DEFAULT_JOB_NAME.to_string(),
            class: String::new(),
            cron: String::new(),
            queue: None,
            args: None,
            retry: None,
            cron_schedule: None,
            json_args: Some(JsonValue::Null),
        }
    }
}

impl PeriodicJob {
    /// 创建新的定期任务
    pub fn new(
        name: String,
        class: String,
        cron: Cron,
        queue: Option<String>,
        args: Option<JsonValue>,
        retry: Option<bool>,
    ) -> Result<Self> {
        let mut job = Self {
            name,
            class,
            cron: cron.to_string(),
            queue,
            args: args.as_ref().map(|a| a.to_string()),
            retry,
            cron_schedule: Some(cron),
            json_args: args.or(Some(JsonValue::Null)),
        };

        // 如果 cron_schedule 为空，则重新解析
        if job.cron_schedule.is_none() {
            job.hydrate_cron_schedule()?;
        }

        Ok(job)
    }

    /// 从字符串创建定期任务
    pub fn from_json_string(periodic_job_str: &str) -> Result<Self> {
        let mut job: Self = serde_json::from_str(periodic_job_str)?;
        job.hydrate_attributes()?;
        Ok(job)
    }

    /// 补充运行时属性
    fn hydrate_attributes(&mut self) -> Result<()> {
        self.hydrate_cron_schedule()?;
        self.hydrate_json_args()?;
        Ok(())
    }

    /// 补充 cron 调度器
    fn hydrate_cron_schedule(&mut self) -> Result<()> {
        self.cron_schedule = Some(Cron::from_str(&self.cron)?);
        Ok(())
    }

    /// 补充 JSON 参数
    fn hydrate_json_args(&mut self) -> Result<()> {
        self.json_args = match &self.args {
            Some(args_str) => Some(serde_json::from_str(args_str)?),
            None => Some(JsonValue::Null),
        };
        Ok(())
    }

    pub async fn insert(&self) -> Result<i64> {
        let payload = serde_json::to_string(self)?;
        self.update(&payload).await
    }
    pub async fn update(&self, periodic_job_str: &str) -> Result<i64> {
        let next_time = self.next_scheduled_time().ok_or_else(|| {
            format!(
                "failed: class: {}, name: {}",
                self.class, self.name
            )
        })?;
        let cache = CacheManager::instance().await;

        cache
            .zadd_ch(PERIODIC_KEY, periodic_job_str, next_time)
            .await
    }

    /// 获取下次调度时间
    #[must_use]
    pub fn next_scheduled_time(&self) -> Option<f64> {
        self.cron_schedule
            .as_ref()?
            .upcoming(Utc)
            .next()
            .map(|dt| dt.timestamp() as f64)
    }

    /// 转换为可执行的任务
    #[must_use]
    pub async fn into_job(&self) -> Job {
        let args = self.json_args.clone().unwrap_or(JsonValue::Null);

        Job {
            queue: self
                .queue
                .clone()
                .unwrap_or_else(|| DEFAULT_QUEUE.to_string()),
            class: self.class.clone(),
            jid: GID().await,
            created_at: chrono::Utc::now().timestamp() as f64,
            enqueued_at: None,
            retry: self.retry.unwrap_or(false),
            args,
            error_message: None,
            failed_at: None,
            retry_count: None,
            retried_at: None,
            unique_for: None,
        }
    }

    // Getter 方法
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn class(&self) -> &str {
        &self.class
    }

    pub fn cron(&self) -> &str {
        &self.cron
    }

    pub fn queue(&self) -> Option<&str> {
        self.queue.as_deref()
    }

    pub fn retry(&self) -> Option<bool> {
        self.retry
    }
}
