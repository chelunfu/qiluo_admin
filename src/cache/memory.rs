// src/cache/memory.rs
use crate::common::error::Result;
use crate::model::prelude::ListData;
use crate::model::sys::args::acache::CacheItem;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tokio::time::interval;

#[derive(Debug, Clone)]
struct MemoryItem {
    value: String,
    expire_time: Option<u64>,
    #[allow(dead_code)]
    created_at: u64,
}

impl MemoryItem {
    fn new(value: String, ttl_seconds: Option<i32>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let expire_time = ttl_seconds.map(|ttl| now + ttl as u64);

        Self {
            value,
            expire_time,
            created_at: now,
        }
    }

    fn is_expired(&self) -> bool {
        if let Some(expire_time) = self.expire_time {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            now >= expire_time
        } else {
            false
        }
    }

    fn ttl(&self) -> i64 {
        if let Some(expire_time) = self.expire_time {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            if now >= expire_time {
                -2 // 已过期
            } else {
                (expire_time - now) as i64
            }
        } else {
            -1 // 永不过期
        }
    }
}

#[derive(Debug)]
pub struct MemoryCache {
    storage: Arc<DashMap<String, MemoryItem>>,
    namespace: Arc<RwLock<String>>,
    lists: Arc<DashMap<String, Vec<String>>>,
    sets: Arc<DashMap<String, DashMap<String, bool>>>,
    sorted_sets: Arc<DashMap<String, Vec<(String, f64)>>>,
}

impl MemoryCache {
    pub fn new(namespace: String) -> Self {
        let cache = Self {
            storage: Arc::new(DashMap::new()),
            namespace: Arc::new(RwLock::new(namespace)),
            lists: Arc::new(DashMap::new()),
            sets: Arc::new(DashMap::new()),
            sorted_sets: Arc::new(DashMap::new()),
        };

        // 启动清理任务
        cache.start_cleanup_task();
        cache
    }

    fn start_cleanup_task(&self) {
        let storage = Arc::clone(&self.storage);
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(60)); // 每分钟清理一次
            loop {
                interval.tick().await;
                storage.retain(|_, item| !item.is_expired());
            }
        });
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
        self.storage.retain(|_, item| !item.is_expired());
    }

    pub async fn set_string(&self, k: &str, v: &str) -> Result<String> {
        let key = self.get_namespaced_key(k).await;
        let item = MemoryItem::new(v.to_string(), None);
        self.storage.insert(key, item);
        Ok("OK".to_string())
    }

    pub async fn get_string(&self, k: &str) -> Result<String> {
        let key = self.get_namespaced_key(k).await;
        if let Some(item) = self.storage.get(&key) {
            if item.is_expired() {
                self.storage.remove(&key);
                Err("Key not found or expired".into())
            } else {
                Ok(item.value.clone())
            }
        } else {
            Err("Key not found or expired".into())
        }
    }

    pub async fn set_string_ex(&self, k: &str, v: &str, t: i32) -> Result<bool> {
        let key = self.get_namespaced_key(k).await;
        let item = MemoryItem::new(v.to_string(), Some(t));
        self.storage.insert(key, item);
        Ok(true)
    }

    pub async fn remove(&self, k: &str) -> Result<usize> {
        let key = self.get_namespaced_key(k).await;
        let mut removed = 0;

        if self.storage.remove(&key).is_some() {
            removed += 1;
        }
        if self.lists.remove(&key).is_some() {
            removed += 1;
        }
        if self.sets.remove(&key).is_some() {
            removed += 1;
        }
        if self.sorted_sets.remove(&key).is_some() {
            removed += 1;
        }

        Ok(removed)
    }

    pub async fn contains_key(&self, k: &str) -> bool {
        let key = self.get_namespaced_key(k).await;

        // 检查字符串存储
        if let Some(item) = self.storage.get(&key) {
            if item.is_expired() {
                self.storage.remove(&key);
                return false;
            } else {
                return true;
            }
        }

        // 检查其他数据类型
        self.lists.contains_key(&key)
            || self.sets.contains_key(&key)
            || self.sorted_sets.contains_key(&key)
    }

    pub async fn ttl(&self, k: &str) -> Result<i64> {
        let key = self.get_namespaced_key(k).await;
        if let Some(item) = self.storage.get(&key) {
            Ok(item.ttl())
        } else {
            Ok(-2) // key 不存在
        }
    }

    pub async fn get_one_use(&self, k: &str) -> Result<String> {
        let result = self.get_string(k).await;
        if result.is_ok() {
            let _ = self.remove(k).await;
        }
        result
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
        let mut result = Vec::new();
        for entry in self.storage.iter() {
            if !entry.value().is_expired() {
                result.push((entry.key().clone(), entry.value().value.clone()));
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
        let mut items = Vec::new();

        // 1. 处理字符串类型数据
        for entry in self.storage.iter() {
            if entry.value().is_expired() {
                continue;
            }

            let key = entry.key();
            if let Some(ref search) = search_key {
                if !key.contains(search) {
                    continue;
                }
            }

            // 去除命名空间前缀显示
            let display_key = key.clone();

            items.push(CacheItem {
                key: display_key,
                value: entry.value().value.clone(),
                ttl: Some(entry.value().ttl()),
            });
        }

        // 2. 处理有序集合数据
        for entry in self.sorted_sets.iter() {
            let key = entry.key();
            if let Some(ref search) = search_key {
                if !key.contains(search) {
                    continue;
                }
            }

            let display_key = key.clone();

            let count = entry.value().len();
            items.push(CacheItem {
                key: display_key,
                value: format!("ZSET ({} members)", count),
                ttl: Some(-1), // 集合类型没有TTL
            });
        }

        // 3. 处理普通集合数据
        for entry in self.sets.iter() {
            let key = entry.key();
            if let Some(ref search) = search_key {
                if !key.contains(search) {
                    continue;
                }
            }

            let display_key = key.clone();

            let count = entry.value().len();
            items.push(CacheItem {
                key: display_key,
                value: format!("SET ({} members)", count),
                ttl: Some(-1),
            });
        }

        // 4. 处理列表数据
        for entry in self.lists.iter() {
            let key = entry.key();
            if let Some(ref search) = search_key {
                if !key.contains(search) {
                    continue;
                }
            }

            let display_key = key.clone();

            let count = entry.value().len();
            items.push(CacheItem {
                key: display_key,
                value: format!("LIST ({} items)", count),
                ttl: Some(-1),
            });
        }

        // 5. 排序和分页
        items.sort_by(|a, b| a.key.cmp(&b.key)); // 按键名排序

        let total = items.len() as u64;
        let start = ((page_num - 1) * page_size) as usize;
        let end = (start + page_size as usize).min(items.len());

        if start < items.len() {
            items = items[start..end].to_vec();
        } else {
            items.clear();
        }

        // 6. 修复分页信息
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
        let new_cache = self.clone();
        *new_cache.namespace.write().await = namespace;
        new_cache
    }

    pub async fn set_namespace(&self, namespace: String) {
        *self.namespace.write().await = namespace;
    }

    pub async fn namespaced_key(&self, key: &str) -> String {
        self.get_namespaced_key(key).await
    }

    pub async fn namespaced_keys(&self, keys: Vec<String>) -> Vec<String> {
        let namespace = self.namespace.read().await;
        if namespace.is_empty() {
            keys
        } else {
            keys.into_iter()
                .map(|key| format!("{}:{}", namespace, key))
                .collect()
        }
    }

    pub async fn brpop(
        &self,
        keys: Vec<String>,
        _timeout: usize,
    ) -> Result<Option<(String, String)>> {
        
        for key in keys {
            let namespaced_key = self.get_namespaced_key(&key).await;
            if let Some(mut list) = self.lists.get_mut(&namespaced_key) {
                if let Some(value) = list.pop() {
                    return Ok(Some((key, value)));
                }
            }
        }
        Ok(None)
    }

    pub async fn sadd(&self, key: &str, members: &[&str]) -> Result<i64> {
        let namespaced_key = self.get_namespaced_key(key).await;
        let set = self.sets.entry(namespaced_key).or_default();
        let mut added = 0;
        for member in members {
            if set.insert(member.to_string(), true).is_none() {
                added += 1;
            }
        }
        Ok(added)
    }

    pub async fn set_nx_ex<V>(&self, key: &str, value: V, ttl_in_seconds: usize) -> Result<bool>
    where
        V: ToString + Send + Sync,
    {
        let namespaced_key = self.get_namespaced_key(key).await;

        // 应该检查所有存储结构
        if self.storage.contains_key(&namespaced_key)
            || self.lists.contains_key(&namespaced_key)
            || self.sets.contains_key(&namespaced_key)
            || self.sorted_sets.contains_key(&namespaced_key)
        {
            Ok(false)
        } else {
            let item = MemoryItem::new(value.to_string(), Some(ttl_in_seconds as i32));
            self.storage.insert(namespaced_key, item);
            Ok(true)
        }
    }

    pub async fn zrange(&self, key: &str, start: i64, stop: i64) -> Result<Vec<String>> {
        let namespaced_key = self.get_namespaced_key(key).await;
        if let Some(sorted_set) = self.sorted_sets.get(&namespaced_key) {
            let len = sorted_set.len() as i64;
            let start = if start < 0 {
                (len + start).max(0)
            } else {
                start.min(len - 1)
            };
            let stop = if stop < 0 {
                (len + stop).max(0)
            } else {
                stop.min(len - 1)
            };

            if start <= stop {
                let result = sorted_set
                    .iter()
                    .skip(start as usize)
                    .take((stop - start + 1) as usize)
                    .map(|(value, _)| value.clone())
                    .collect();
                Ok(result)
            } else {
                Ok(Vec::new())
            }
        } else {
            Ok(Vec::new())
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
        let namespaced_key = self.get_namespaced_key(key).await;
        if let Some(sorted_set) = self.sorted_sets.get(&namespaced_key) {
            // 1. 首先创建一个排序的副本
            let mut sorted_items: Vec<_> = sorted_set.iter().cloned().collect();
            sorted_items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

            // 2. 然后按分数范围过滤
            let filtered: Vec<String> = sorted_items
                .iter()
                .filter(|(_, score)| *score >= min_score && *score <= max_score)
                .skip(offset as usize)
                .take(count as usize)
                .map(|(value, _)| value.clone())
                .collect();

            Ok(filtered)
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn zadd<V, S>(&self, key: &str, value: V, score: S) -> Result<i64>
    where
        V: ToString + Send + Sync,
        S: Into<f64> + Send + Sync,
    {
        let namespaced_key = self.get_namespaced_key(key).await;
        let mut sorted_set = self.sorted_sets.entry(namespaced_key).or_default();

        let value_str = value.to_string();
        let score_val = score.into();

        // 检查是否已存在，如果存在则更新分数
        if let Some(pos) = sorted_set.iter().position(|(v, _)| v == &value_str) {
            let old_score = sorted_set[pos].1;
            sorted_set[pos].1 = score_val;

            // 只有当分数改变时才需要重新排序
            if old_score != score_val {
                sorted_set
                    .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            }

            Ok(1) // 更新现有元素
        } else {
            // 新元素：使用二分查找找到正确的插入位置
            let insert_pos = sorted_set
                .binary_search_by(|(_, s)| {
                    s.partial_cmp(&score_val)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap_or_else(|pos| pos);

            sorted_set.insert(insert_pos, (value_str, score_val));
            Ok(1) // 添加新元素
        }
    }

    pub async fn lpush<V>(&self, key: &str, value: V) -> Result<i64>
    where
        V: ToString + Send + Sync,
    {
        let namespaced_key = self.get_namespaced_key(key).await;
        let mut list = self.lists.entry(namespaced_key).or_default();
        list.insert(0, value.to_string());
        Ok(list.len() as i64)
    }

    pub async fn zadd_ch<V, S>(&self, key: &str, value: V, score: S) -> Result<i64>
    where
        V: ToString + Send + Sync,
        S: Into<f64> + Send + Sync,
    {
        // 在 Memory 实现中，zadd_ch 和 zadd 行为相同
        self.zadd(key, value, score).await
    }

    pub async fn zrem<V>(&self, key: &str, value: V) -> Result<bool>
    where
        V: ToString + Send + Sync,
    {
        let namespaced_key = self.get_namespaced_key(key).await;
        if let Some(mut sorted_set) = self.sorted_sets.get_mut(&namespaced_key) {
            let value_str = value.to_string();
            if let Some(pos) = sorted_set.iter().position(|(v, _)| v == &value_str) {
                sorted_set.remove(pos);
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

impl Clone for MemoryCache {
    fn clone(&self) -> Self {
        Self {
            storage: Arc::clone(&self.storage),
            namespace: Arc::clone(&self.namespace),
            lists: Arc::clone(&self.lists),
            sets: Arc::clone(&self.sets),
            sorted_sets: Arc::clone(&self.sorted_sets),
        }
    }
}
