use crate::common::error::{Error, Result};
use crate::config::APPCOFIG;
use crate::model::prelude::ListData;
use crate::model::sys::args::acache::CacheItem;
pub mod memory;
pub mod redis;
pub mod traits;

use memory::MemoryCache;
use once_cell::sync::OnceCell;
use redis::RedisCache;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Cache {
    Redis(RedisCache),
    Memory(MemoryCache),
}

impl Cache {
    pub async fn new() -> Result<Self> {
        let config = APPCOFIG.clone().cache;
        let namespace = config.namespace.unwrap_or_else(|| "qiluo".to_string());
        match config.cache_type.as_str() {
            "redis" => {
                if config.url.is_none() {
                    return Err(Error::Message("Redis URL cannot be empty".to_string()));
                }
                let url = config.url.unwrap();

                let redis_cache = RedisCache::new(&url, namespace).await?;
                Ok(Cache::Redis(redis_cache))
            }
            "memory" => {
                if config.pool_size.is_none() {
                    return Err(Error::Message(
                        "Memory cache max size must be specified".to_string(),
                    ));
                }
                let memory_cache = MemoryCache::new(namespace);
                Ok(Cache::Memory(memory_cache))
            }
            _ => Err(Error::Message(format!(
                "Unsupported cache type: {}. Supported types are 'redis' and 'memory'",
                config.cache_type
            ))),
        }
    }

    pub async fn recycling(&self) {
        match self {
            Cache::Redis(cache) => cache.recycling().await,
            Cache::Memory(cache) => cache.recycling().await,
        }
    }

    pub async fn set_string(&self, k: &str, v: &str) -> Result<String> {
        match self {
            Cache::Redis(cache) => cache.set_string(k, v).await,
            Cache::Memory(cache) => cache.set_string(k, v).await,
        }
    }

    pub async fn get_string(&self, k: &str) -> Result<String> {
        match self {
            Cache::Redis(cache) => cache.get_string(k).await,
            Cache::Memory(cache) => cache.get_string(k).await,
        }
    }

    pub async fn set_string_ex(&self, k: &str, v: &str, t: i32) -> Result<bool> {
        match self {
            Cache::Redis(cache) => cache.set_string_ex(k, v, t).await,
            Cache::Memory(cache) => cache.set_string_ex(k, v, t).await,
        }
    }

    pub async fn remove(&self, k: &str) -> Result<usize> {
        match self {
            Cache::Redis(cache) => cache.remove(k).await,
            Cache::Memory(cache) => cache.remove(k).await,
        }
    }

    pub async fn contains_key(&self, k: &str) -> bool {
        match self {
            Cache::Redis(cache) => cache.contains_key(k).await,
            Cache::Memory(cache) => cache.contains_key(k).await,
        }
    }

    pub async fn ttl(&self, k: &str) -> Result<i64> {
        match self {
            Cache::Redis(cache) => cache.ttl(k).await,
            Cache::Memory(cache) => cache.ttl(k).await,
        }
    }

    pub async fn get_one_use(&self, k: &str) -> Result<String> {
        match self {
            Cache::Redis(cache) => cache.get_one_use(k).await,
            Cache::Memory(cache) => cache.get_one_use(k).await,
        }
    }

    pub async fn get_oneuse_value<T>(&self, k: &str) -> Result<T>
    where
        T: Serialize + for<'de> Deserialize<'de> + Clone,
    {
        match self {
            Cache::Redis(cache) => cache.get_oneuse_value(k).await,
            Cache::Memory(cache) => cache.get_oneuse_value(k).await,
        }
    }

    pub async fn get_all(&self) -> Result<Vec<(String, String)>> {
        match self {
            Cache::Redis(cache) => cache.get_all().await,
            Cache::Memory(cache) => cache.get_all().await,
        }
    }

    pub async fn get_all_paginated(
        &self,
        page_num: u64,
        page_size: u64,
        search_key: Option<String>,
    ) -> Result<ListData<CacheItem>> {
        match self {
            Cache::Redis(cache) => {
                cache
                    .get_all_paginated(page_num, page_size, search_key)
                    .await
            }
            Cache::Memory(cache) => {
                cache
                    .get_all_paginated(page_num, page_size, search_key)
                    .await
            }
        }
    }

    pub async fn get_value<T>(&self, k: &str) -> Result<T>
    where
        T: Serialize + for<'de> Deserialize<'de> + Clone,
    {
        match self {
            Cache::Redis(cache) => cache.get_value(k).await,
            Cache::Memory(cache) => cache.get_value(k).await,
        }
    }

    pub async fn set_value<T>(&self, k: &str, value: &T) -> Result<String>
    where
        T: Serialize + Sync,
    {
        match self {
            Cache::Redis(cache) => cache.set_value(k, value).await,
            Cache::Memory(cache) => cache.set_value(k, value).await,
        }
    }

    pub async fn set_value_ex<T>(&self, k: &str, value: &T, t: i32) -> Result<bool>
    where
        T: Serialize + Sync,
    {
        match self {
            Cache::Redis(cache) => cache.set_value_ex(k, value, t).await,
            Cache::Memory(cache) => cache.set_value_ex(k, value, t).await,
        }
    }

    pub async fn with_namespace(&self, namespace: String) -> Self {
        match self {
            Cache::Redis(cache) => Cache::Redis(cache.with_namespace(namespace).await),
            Cache::Memory(cache) => Cache::Memory(cache.with_namespace(namespace).await),
        }
    }

    pub async fn set_namespace(&self, namespace: String) {
        match self {
            Cache::Redis(cache) => cache.set_namespace(namespace).await,
            Cache::Memory(cache) => cache.set_namespace(namespace).await,
        }
    }

    pub async fn namespaced_key(&self, key: &str) -> String {
        match self {
            Cache::Redis(cache) => cache.namespaced_key(key).await,
            Cache::Memory(cache) => cache.namespaced_key(key).await,
        }
    }

    pub async fn namespaced_keys(&self, keys: Vec<String>) -> Vec<String> {
        match self {
            Cache::Redis(cache) => cache.namespaced_keys(keys).await,
            Cache::Memory(cache) => cache.namespaced_keys(keys).await,
        }
    }

    pub async fn brpop(
        &self,
        keys: Vec<String>,
        timeout: usize,
    ) -> Result<Option<(String, String)>> {
        match self {
            Cache::Redis(cache) => cache.brpop(keys, timeout).await,
            Cache::Memory(cache) => cache.brpop(keys, timeout).await,
        }
    }

    pub async fn sadd(&self, key: &str, members: &[&str]) -> Result<i64> {
        match self {
            Cache::Redis(cache) => cache.sadd(key, members).await,
            Cache::Memory(cache) => cache.sadd(key, members).await,
        }
    }

    pub async fn set_nx_ex<V>(&self, key: &str, value: V, ttl_in_seconds: usize) -> Result<bool>
    where
        V: ToString + Send + Sync,
    {
        match self {
            Cache::Redis(cache) => cache.set_nx_ex(key, value, ttl_in_seconds).await,
            Cache::Memory(cache) => cache.set_nx_ex(key, value, ttl_in_seconds).await,
        }
    }

    pub async fn zrange(&self, key: &str, start: i64, stop: i64) -> Result<Vec<String>> {
        match self {
            Cache::Redis(cache) => cache.zrange(key, start, stop).await,
            Cache::Memory(cache) => cache.zrange(key, start, stop).await,
        }
    }

    pub async fn zrangebyscore_limit(
        &self,
        key: &str,
        min_score: f64,
        max_score: f64,
        offset: isize,
        count: isize,
    ) -> Result<Vec<String>> {
        match self {
            Cache::Redis(cache) => {
                cache
                    .zrangebyscore_limit(key, min_score, max_score, offset, count)
                    .await
            }
            Cache::Memory(cache) => {
                cache
                    .zrangebyscore_limit(key, min_score, max_score, offset, count)
                    .await
            }
        }
    }

    pub async fn zadd<V, S>(&self, key: &str, value: V, score: S) -> Result<i64>
    where
        V: ToString + Send + Sync,
        S: Into<f64> + Send + Sync,
    {
        match self {
            Cache::Redis(cache) => cache.zadd(key, value, score).await,
            Cache::Memory(cache) => cache.zadd(key, value, score).await,
        }
    }

    pub async fn lpush<V>(&self, key: &str, value: V) -> Result<i64>
    where
        V: ToString + Send + Sync,
    {
        match self {
            Cache::Redis(cache) => cache.lpush(key, value).await,
            Cache::Memory(cache) => cache.lpush(key, value).await,
        }
    }

    pub async fn zadd_ch<V, S>(&self, key: &str, value: V, score: S) -> Result<i64>
    where
        V: ToString + Send + Sync,
        S: Into<f64> + Send + Sync,
    {
        match self {
            Cache::Redis(cache) => cache.zadd_ch(key, value, score).await,
            Cache::Memory(cache) => cache.zadd_ch(key, value, score).await,
        }
    }

    pub async fn zrem<V>(&self, key: &str, value: V) -> Result<bool>
    where
        V: ToString + Send + Sync,
    {
        match self {
            Cache::Redis(cache) => cache.zrem(key, value).await,
            Cache::Memory(cache) => cache.zrem(key, value).await,
        }
    }
}

// 全局缓存实例
static GLOBAL_CACHE: OnceCell<Arc<Cache>> = OnceCell::new();
pub struct CacheManager;

impl CacheManager {
    /// 初始化全局缓存
    pub async fn init() -> Result<()> {
        let cache = Cache::new().await?;
        GLOBAL_CACHE
            .set(Arc::new(cache))
            .map_err(|_| "Failed to initialize global cache".to_string())?;
        Ok(())
    }

    /// 获取全局缓存实例
    pub async fn instance() -> Arc<Cache> {
        GLOBAL_CACHE.get().cloned().expect(
            "Cache not initialized. This should not happen if CacheManager::init() was called during application startup."
        )
    }
}
