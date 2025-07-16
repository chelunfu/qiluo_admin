use crate::common::error::Result;
use async_trait::async_trait;

use super::common::Worker;

#[async_trait]
#[allow(clippy::module_name_repetitions)]
pub trait AppWorker<T>: Worker<T>
where
    Self: Sized,
    T: Send + Sync + serde::Serialize + 'static,
{
    fn new() -> Self;

    //同步加入队列
    async fn enqueue_sync(args: T) -> Result<()> {
        Self::perform_async(args).await
    }
    // 异步加入队列
    async fn enqueue_async(args: T) -> Result<()> {
        tokio::spawn(async move { Self::perform_async(args).await });
        Ok(())
    }
    //异步执行
    async fn execute_async(args: T) -> Result<()> {
        tokio::spawn(async move { Self::new().perform(args).await });
        Ok(())
    }
    //同步执行
    async fn execute_sync(args: T) -> Result<()> {
        Self::new().perform(args).await
    }
}
