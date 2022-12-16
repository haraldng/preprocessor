use lru::LruCache;
use std::num::NonZeroUsize;
use lfu_cache::LfuCache;
use indexmap::IndexMap;

pub type CacheKey = String;
pub type CacheValue = String;

/// The cache we use in paxos
pub struct CacheModel
{
    use_lfu: bool,
    lfu_cache: LfuCache<CacheKey, CacheValue>,
    lru_cache: LruCache<CacheKey, CacheValue>,
    index_cache: IndexMap<CacheKey, CacheValue>,
}

impl CacheModel
{
    /// create a new cache model
    pub fn with(capacity: usize, use_lfu: bool) -> Self {
        let lfu_cache_capacity = if use_lfu { capacity } else { 1 };
        let lru_cache_capacity = if use_lfu { 1 } else { capacity };

        CacheModel {
            use_lfu,
            lfu_cache: LfuCache::with_capacity(lfu_cache_capacity),
            lru_cache: LruCache::new(NonZeroUsize::new(lru_cache_capacity).unwrap()),
            index_cache: IndexMap::with_capacity(capacity),
        }
    }

    /// save (key, value) pair into cache
    /// Optimization: The value could be skipped if it always equals to the key
    pub fn put(&mut self, key: CacheKey, value: CacheValue) {
        if self.use_lfu {
            self.lfu_cache.insert(key.clone(), value.clone());
        } else {
            self.lru_cache.put(key.clone(), value.clone());
        }

        self.index_cache.insert(key, value);
    }

    /// get index from cache
    pub fn get_index_of(&mut self, key: CacheKey) -> Option<usize> {
        if self.use_lfu {
            if let Some(_value) = self.lfu_cache.get(&key) {
                return self.index_cache.get_index_of(&key)
            }
        } else if let Some(_value) = self.lru_cache.get(&key) {
            return self.index_cache.get_index_of(&key)
        }

        None
    }

    /// get value from cache
    pub fn get_with_index(&mut self, index: usize) -> Option<(&CacheKey, &CacheValue)> {
        self.index_cache.get_index(index)
    }

    /// return cache length
    pub fn len(&self) -> usize {
        if self.use_lfu {
            self.lfu_cache.len()
        } else {
            self.lru_cache.len()
        }
    }
}