use crate::common::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use super::WorkerOpts;

#[async_trait]
pub trait Worker<Args>: Send + Sync {
    fn disable_argument_coercion(&self) -> bool {
        false
    }

    #[must_use]
    fn opts() -> WorkerOpts<Args, Self>
    where
        Self: Sized,
    {
        WorkerOpts::new()
    }
    fn max_retries(&self) -> usize {
        25
    }
    #[must_use]
    fn class_name() -> String
    where
        Self: Sized,
    {
        use convert_case::{Case, Casing};

        let type_name = std::any::type_name::<Self>();
        let name = type_name.split("::").last().unwrap_or(type_name);
        name.to_case(Case::UpperCamel)
    }

    async fn perform_async(args: Args) -> Result<()>
    where
        Self: Sized,
        Args: Send + Sync + Serialize + 'static,
    {
        Self::opts().perform_async(args).await
    }

    async fn perform_in_seconds(seconds: u64, args: Args) -> Result<()>
    where
        Self: Sized,
        Args: Send + Sync + Serialize + 'static,
    {
        let duration = std::time::Duration::from_secs(seconds);
        Self::perform_in(duration, args).await
    }

    async fn perform_in_milliseconds(milliseconds: u64, args: Args) -> Result<()>
    where
        Self: Sized,
        Args: Send + Sync + Serialize + 'static,
    {
        let duration = std::time::Duration::from_millis(milliseconds);
        Self::perform_in(duration, args).await
    }

    async fn perform_in_minutes(minutes: u64, args: Args) -> Result<()>
    where
        Self: Sized,
        Args: Send + Sync + Serialize + 'static,
    {
        let duration = std::time::Duration::from_secs(minutes * 60);
        Self::perform_in(duration, args).await
    }

    async fn perform_in(duration: std::time::Duration, args: Args) -> Result<()>
    where
        Self: Sized,
        Args: Send + Sync + Serialize + 'static,
    {
        Self::opts().perform_in(duration, args).await
    }

    async fn perform(&self, args: Args) -> Result<()>;
}

#[derive(Clone)]
pub struct WorkerRef {
    #[allow(clippy::type_complexity)]
    work_fn: Arc<
        Box<dyn Fn(JsonValue) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send + Sync>,
    >,
    max_retries: usize,
}

async fn invoke_worker<Args, W>(args: JsonValue, worker: Arc<W>) -> Result<()>
where
    Args: Send + Sync + 'static,
    W: Worker<Args> + 'static,
    for<'de> Args: Deserialize<'de>,
{
    let args = if worker.disable_argument_coercion() {
        args
    } else if std::any::TypeId::of::<Args>() == std::any::TypeId::of::<()>() {
        JsonValue::Null
    } else {
        match args {
            JsonValue::Array(mut arr) if arr.len() == 1 => {
                arr.pop().expect("value change after size check")
            }
            _ => args,
        }
    };

    let args: Args = serde_json::from_value(args)?;
    worker.perform(args).await
}

impl WorkerRef {
    pub(crate) fn wrap<Args, W>(worker: Arc<W>) -> Self
    where
        Args: Send + Sync + 'static,
        W: Worker<Args> + 'static,
        for<'de> Args: Deserialize<'de>,
    {
        Self {
            work_fn: Arc::new(Box::new({
                let worker = worker.clone();
                move |args: JsonValue| {
                    let worker = worker.clone();
                    Box::pin(async move { invoke_worker(args, worker).await })
                }
            })),
            max_retries: worker.max_retries(),
        }
    }

    #[must_use]
    pub fn max_retries(&self) -> usize {
        self.max_retries
    }

    pub async fn call(&self, args: JsonValue) -> Result<()> {
        (Arc::clone(&self.work_fn))(args).await
    }
}
