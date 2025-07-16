// src/cache/traits.rs
use crate::common::error::Result;
use crate::model::prelude::ListData;
use crate::model::sys::args::acache::CacheItem;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait CacheProvider: Send + Sync + Clone {
    async fn recycling(&self);
    async fn set_string(&self, k: &str, v: &str) -> Result<String>;
    async fn get_string(&self, k: &str) -> Result<String>;
    async fn set_string_ex(&self, k: &str, v: &str, t: i32) -> Result<bool>;
    async fn remove(&self, k: &str) -> Result<usize>;
    async fn contains_key(&self, k: &str) -> bool;
    async fn ttl(&self, k: &str) -> Result<i64>;
    async fn get_one_use(&self, k: &str) -> Result<String>;
    async fn get_all(&self) -> Result<Vec<(String, String)>>;
    async fn get_all_paginated(
        &self,
        page_num: u64,
        page_size: u64,
        search_key: Option<String>,
    ) -> Result<ListData<CacheItem>>;

    async fn get_value<T>(&self, k: &str) -> Result<T>
    where
        T: Serialize + for<'de> Deserialize<'de> + Clone;

    async fn set_value<T>(&self, k: &str, value: &T) -> Result<String>
    where
        T: Serialize + Sync;

    async fn set_value_ex<T>(&self, k: &str, value: &T, t: i32) -> Result<bool>
    where
        T: Serialize + Sync;

    async fn with_namespace(&self, namespace: String) -> Self;
    async fn set_namespace(&self, namespace: String);
    async fn namespaced_key(&self, key: &str) -> String;
    async fn namespaced_keys(&self, keys: Vec<String>) -> Vec<String>;

    async fn brpop(&self, keys: Vec<String>, timeout: usize) -> Result<Option<(String, String)>>;

    async fn sadd(&self, key: &str, members: &[&str]) -> Result<i64>;

    async fn set_nx_ex<V>(&self, key: &str, value: V, ttl_in_seconds: usize) -> Result<bool>
    where
        V: ToString + Send + Sync;

    async fn zrange(&self, key: &str, start: i64, stop: i64) -> Result<Vec<String>>;

    async fn zrangebyscore_limit<S>(
        &self,
        key: &str,
        min_score: f64,
        max_score: f64,
        offset: isize,
        count: isize,
    ) -> Result<Vec<String>>
    where
        S: Into<f64> + Send + Sync;

    async fn zadd<V, S>(&self, key: &str, value: V, score: S) -> Result<i64>
    where
        V: ToString + Send + Sync,
        S: Into<f64> + Send + Sync;

    async fn lpush<V>(&self, key: &str, value: V) -> Result<i64>
    where
        V: ToString + Send + Sync;

    async fn zadd_ch<V, S>(&self, key: &str, value: V, score: S) -> Result<i64>
    where
        V: ToString + Send + Sync,
        S: Into<f64> + Send + Sync;

    async fn zrem<V>(&self, key: &str, value: V) -> Result<bool>
    where
        V: ToString + Send + Sync;
}
