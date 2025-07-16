// src/cache/redis.rs
use crate::common::error::Result;
use crate::model::prelude::ListData;
use crate::model::sys::args::acache::CacheItem;
use bb8::Pool;
use bb8_redis::{bb8, redis, RedisConnectionManager};
use redis::{AsyncCommands, RedisResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct RedisCache {
    pool: Pool<RedisConnectionManager>,
    namespace: Arc<RwLock<String>>,
}

impl RedisCache {
    pub async fn new(redis_url: &str, namespace: String) -> Result<Self> {
        let manager = RedisConnectionManager::new(redis_url)?;
        let pool = Pool::builder().build(manager).await?;

        Ok(Self {
            pool,
            namespace: Arc::new(RwLock::new(namespace)),
        })
    }

    async fn get_namespaced_key(&self, key: &str) -> String {
        let namespace = self.namespace.read().await;
        if namespace.is_empty() {
            key.to_string()
        } else {
            format!("{}:{}", namespace, key)
        }
    }

    pub async fn recycling(&self) {
        // Redis 自动处理过期键，这里可以为空或执行一些维护操作
    }

    pub async fn set_string(&self, k: &str, v: &str) -> Result<String> {
        let key = self.get_namespaced_key(k).await;
        let mut conn = self.pool.get().await?;
        let result: String = redis::cmd("SET")
            .arg(&key)
            .arg(v)
            .query_async(&mut *conn)
            .await?;
        Ok(result)
    }

    pub async fn get_string(&self, k: &str) -> Result<String> {
        let key = self.get_namespaced_key(k).await;
        let mut conn = self.pool.get().await?;
        let result: Option<String> = redis::cmd("GET").arg(&key).query_async(&mut *conn).await?;
        result.ok_or_else(|| format!("Key not found: {}", key).into())
    }

    pub async fn set_string_ex(&self, k: &str, v: &str, t: i32) -> Result<bool> {
        let key = self.get_namespaced_key(k).await;
        let mut conn = self.pool.get().await?;
        let result: RedisResult<()> = conn.set_ex(&key, v, t as u64).await;
        Ok(result.is_ok())
    }

    pub async fn remove(&self, k: &str) -> Result<usize> {
        let key = self.get_namespaced_key(k).await;
        let mut conn = self.pool.get().await?;
        let result: usize = conn.del(&key).await?;
        Ok(result)
    }

    pub async fn contains_key(&self, k: &str) -> bool {
        let key = self.get_namespaced_key(k).await;
        let mut conn = self
            .pool
            .get()
            .await
            .unwrap_or_else(|_| panic!("Failed to get connection"));
        let result: RedisResult<bool> = conn.exists(&key).await;
        result.unwrap_or(false)
    }

    pub async fn ttl(&self, k: &str) -> Result<i64> {
        let key = self.get_namespaced_key(k).await;
        let mut conn = self.pool.get().await?;
        let result: i64 = conn.ttl(&key).await?;
        Ok(result)
    }

    pub async fn get_one_use(&self, k: &str) -> Result<String> {
        let key = self.get_namespaced_key(k).await;
        let mut conn = self.pool.get().await?;
        let result: Option<String> = conn.get_del(&key).await?;
        result.ok_or_else(|| format!("Key not found: {}", key).into())
    }
    pub async fn get_oneuse_value<T>(&self, k: &str) -> Result<T>
    where
        T: Serialize + for<'de> Deserialize<'de> + Clone,
    {
        let result = self.get_value(k).await;
        if result.is_ok() {
            let _ = self.remove(k).await;
        }
        result
    }
    pub async fn get_all(&self) -> Result<Vec<(String, String)>> {
        let mut conn = self.pool.get().await?;
        let keys: Vec<String> = conn.keys("*").await?;
        let mut result = Vec::new();

        for key in keys {
            if let Ok(Some(value)) = conn.get::<_, Option<String>>(&key).await {
                result.push((key, value));
            }
        }

        Ok(result)
    }

    pub async fn get_all_paginated(
        &self,
        page_num: u64,
        page_size: u64,
        search_key: Option<String>,
    ) -> Result<ListData<CacheItem>> {
        let mut conn = self.pool.get().await?;
        let namespace = self.namespace.read().await;

        // 构建包含命名空间的搜索模式
        let pattern = if let Some(search) = &search_key {
            if namespace.is_empty() {
                format!("*{}*", search)
            } else {
                format!("{}:*{}*", namespace, search)
            }
        } else if namespace.is_empty() {
            "*".to_string()
        } else {
            format!("{}:*", namespace)
        };

        let keys: Vec<String> = conn.keys(&pattern).await?;
        let total = keys.len() as u64;

        let start = ((page_num - 1) * page_size) as usize;
        let end = (start + page_size as usize).min(keys.len());

        let mut items = Vec::new();
        if start < keys.len() {
            for key in &keys[start..end] {
                // 获取键的数据类型
                let key_type: String = redis::cmd("TYPE")
                    .arg(key)
                    .query_async(&mut *conn)
                    .await
                    .unwrap_or_else(|_| "unknown".to_string());

                // 根据数据类型获取相应的值表示
                let value = match key_type.as_str() {
                    "string" => conn
                        .get::<_, Option<String>>(key)
                        .await
                        .unwrap_or(None)
                        .unwrap_or_else(|| "".to_string()),
                    "zset" => {
                        let count: i64 = redis::cmd("ZCARD")
                            .arg(key)
                            .query_async(&mut *conn)
                            .await
                            .unwrap_or(0);
                        format!("ZSET ({} members)", count)
                    }
                    "set" => {
                        let count: i64 = redis::cmd("SCARD")
                            .arg(key)
                            .query_async(&mut *conn)
                            .await
                            .unwrap_or(0);
                        format!("SET ({} members)", count)
                    }
                    "list" => {
                        let count: i64 = redis::cmd("LLEN")
                            .arg(key)
                            .query_async(&mut *conn)
                            .await
                            .unwrap_or(0);
                        format!("LIST ({} items)", count)
                    }
                    "hash" => {
                        let count: i64 = redis::cmd("HLEN")
                            .arg(key)
                            .query_async(&mut *conn)
                            .await
                            .unwrap_or(0);
                        format!("HASH ({} fields)", count)
                    }
                    _ => format!("UNKNOWN TYPE: {}", key_type),
                };

                let ttl: i64 = conn.ttl(key).await.unwrap_or(-1);
                let display_key = key.clone();
                items.push(CacheItem {
                    key: display_key,
                    value,
                    ttl: Some(ttl),
                });
            }
        }

        let total_pages = total.div_ceil(page_size);

        Ok(ListData {
            list: items,
            total,
            total_pages,
            page_num,
        })
    }

    pub async fn get_value<T>(&self, k: &str) -> Result<T>
    where
        T: Serialize + for<'de> Deserialize<'de> + Clone,
    {
        let value_str = self.get_string(k).await?;
        Ok(serde_json::from_str(&value_str)?)
    }

    pub async fn set_value<T>(&self, k: &str, value: &T) -> Result<String>
    where
        T: Serialize + Sync,
    {
        let value_str = serde_json::to_string(value)?;
        self.set_string(k, &value_str).await
    }

    pub async fn set_value_ex<T>(&self, k: &str, value: &T, t: i32) -> Result<bool>
    where
        T: Serialize + Sync,
    {
        let value_str = serde_json::to_string(value)?;
        self.set_string_ex(k, &value_str, t).await
    }

    pub async fn with_namespace(&self, namespace: String) -> Self {
        Self {
            pool: self.pool.clone(),
            namespace: Arc::new(RwLock::new(namespace)),
        }
    }

    pub async fn set_namespace(&self, namespace: String) {
        *self.namespace.write().await = namespace;
    }

    pub async fn namespaced_key(&self, key: &str) -> String {
        self.get_namespaced_key(key).await
    }

    pub async fn namespaced_keys(&self, keys: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        for key in keys {
            result.push(self.get_namespaced_key(&key).await);
        }
        result
    }

    pub async fn brpop(
        &self,
        keys: Vec<String>,
        timeout: usize,
    ) -> Result<Option<(String, String)>> {
        let namespaced_keys = self.namespaced_keys(keys.clone()).await;
        let mut conn = self.pool.get().await?;

        let result: Option<(String, String)> = conn.brpop(&namespaced_keys, timeout as f64).await?;

        if let Some((key, value)) = result {
            // 返回原始键名（去除命名空间）
            let original_key = keys
                .into_iter()
                .zip(namespaced_keys.iter())
                .find(|(_, namespaced)| *namespaced == &key)
                .map(|(original, _)| original)
                .unwrap_or(key);
            Ok(Some((original_key, value)))
        } else {
            Ok(None)
        }
    }

    pub async fn sadd(&self, key: &str, members: &[&str]) -> Result<i64> {
        let namespaced_key = self.get_namespaced_key(key).await;
        let mut conn = self.pool.get().await?;
        let result: i64 = conn.sadd(&namespaced_key, members).await?;
        Ok(result)
    }

    pub async fn set_nx_ex<V>(&self, key: &str, value: V, ttl_in_seconds: usize) -> Result<bool>
    where
        V: ToString + Send + Sync,
    {
        let namespaced_key = self.get_namespaced_key(key).await;
        let mut conn = self.pool.get().await?;
        let result: Option<String> = redis::cmd("SET")
            .arg(&namespaced_key)
            .arg(value.to_string())
            .arg("NX")
            .arg("EX")
            .arg(ttl_in_seconds)
            .query_async(&mut *conn)
            .await?;
        Ok(result.is_some())
    }

    pub async fn zrange(&self, key: &str, start: i64, stop: i64) -> Result<Vec<String>> {
        let namespaced_key = self.get_namespaced_key(key).await;
        let mut conn = self.pool.get().await?;
        let result: Vec<String> = redis::cmd("ZRANGE")
            .arg(&namespaced_key)
            .arg(start)
            .arg(stop)
            .query_async(&mut *conn)
            .await?;
        Ok(result)
    }

    pub async fn zrangebyscore_limit(
        &self,
        key: &str,
        min_score: f64,
        max_score: f64,
        offset: isize,
        count: isize,
    ) -> Result<Vec<String>> {
        let namespaced_key = self.get_namespaced_key(key).await;
        let mut conn = self.pool.get().await?;
        let result: Vec<String> = conn
            .zrangebyscore_limit(&namespaced_key, min_score, max_score, offset, count)
            .await?;
        Ok(result)
    }

    pub async fn zadd<V, S>(&self, key: &str, value: V, score: S) -> Result<i64>
    where
        V: ToString + Send + Sync,
        S: Into<f64> + Send + Sync,
    {
        let namespaced_key = self.get_namespaced_key(key).await;
        let mut conn = self.pool.get().await?;
        let result: i64 = conn
            .zadd(&namespaced_key, value.to_string(), score.into())
            .await?;
        Ok(result)
    }

    pub async fn lpush<V>(&self, key: &str, value: V) -> Result<i64>
    where
        V: ToString + Send + Sync,
    {
        let namespaced_key = self.get_namespaced_key(key).await;
        let mut conn = self.pool.get().await?;
        let result: i64 = conn.lpush(&namespaced_key, value.to_string()).await?;
        Ok(result)
    }

    pub async fn zadd_ch<V, S>(&self, key: &str, value: V, score: S) -> Result<i64>
    where
        V: ToString + Send + Sync,
        S: Into<f64> + Send + Sync,
    {
        let namespaced_key = self.get_namespaced_key(key).await;
        let mut conn = self.pool.get().await?;
        // 使用 CH 选项，返回改变的元素数量
        let result: i64 = redis::cmd("ZADD")
            .arg(&namespaced_key)
            .arg("CH")
            .arg(score.into())
            .arg(value.to_string())
            .query_async(&mut *conn)
            .await?;
        Ok(result)
    }

    pub async fn zrem<V>(&self, key: &str, value: V) -> Result<bool>
    where
        V: ToString + Send + Sync,
    {
        let namespaced_key = self.get_namespaced_key(key).await;
        let mut conn = self.pool.get().await?;
        let result: i64 = conn.zrem(&namespaced_key, value.to_string()).await?;
        Ok(result > 0)
    }
}

impl Clone for RedisCache {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
            namespace: Arc::clone(&self.namespace),
        }
    }
}
