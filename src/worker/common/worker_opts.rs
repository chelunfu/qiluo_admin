use crate::common::error::Result;
use serde::Serialize;
use std::marker::PhantomData;

use super::{enqueue_opts::EnqueueOpts, Worker};

pub struct WorkerOpts<Args, W: Worker<Args> + ?Sized> {
    queue: String,
    retry: bool,
    args: PhantomData<Args>,
    worker: PhantomData<W>,
    unique_for: Option<std::time::Duration>,
}

impl<Args, W> WorkerOpts<Args, W>
where
    W: Worker<Args>,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            queue: "default".into(),
            retry: true,
            args: PhantomData,
            worker: PhantomData,
            unique_for: None,
        }
    }

    #[must_use]
    pub fn retry(self, retry: bool) -> Self {
        Self { retry, ..self }
    }

    #[must_use]
    pub fn queue<S: Into<String>>(self, queue: S) -> Self {
        Self {
            queue: queue.into(),
            ..self
        }
    }

    #[must_use]
    pub fn unique_for(self, unique_for: std::time::Duration) -> Self {
        Self {
            unique_for: Some(unique_for),
            ..self
        }
    }

    #[allow(clippy::wrong_self_convention)]
    fn into_opts(&self) -> EnqueueOpts {
        self.into()
    }

    pub async fn perform_async(&self, args: impl Serialize + Send + 'static) -> Result<()> {
        self.into_opts().perform_async(W::class_name(), args).await
    }

    pub async fn perform_in(
        &self,
        duration: std::time::Duration,
        args: impl Serialize + Send + 'static,
    ) -> Result<()> {
        self.into_opts()
            .perform_in(W::class_name(), duration, args)
            .await
    }
}

impl<Args, W: Worker<Args>> From<&WorkerOpts<Args, W>> for EnqueueOpts {
    fn from(opts: &WorkerOpts<Args, W>) -> Self {
        Self {
            retry: opts.retry,
            queue: opts.queue.clone(),
            unique_for: opts.unique_for,
        }
    }
}

impl<Args, W: Worker<Args>> Default for WorkerOpts<Args, W> {
    fn default() -> Self {
        Self::new()
    }
}
