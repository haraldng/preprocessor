use indexmap::IndexMap;
use lfu_cache::LfuCache;
use lru::LruCache;
use std::marker::PhantomData;
use std::num::NonZeroUsize;

pub type CacheKey = String;

/// The cache we use in paxos
pub struct CacheModel {
    use_lfu: bool,
    lfu_cache: LfuCache<CacheKey, PhantomData<u32>>,
    lru_cache: LruCache<CacheKey, PhantomData<u32>>,
    index_cache: IndexMap<CacheKey, PhantomData<u32>>,
}

impl CacheModel {
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
    pub fn put(&mut self, key: CacheKey) {
        if self.use_lfu {
            self.lfu_cache.insert(key.clone(), PhantomData);
        } else {
            self.lru_cache.put(key.clone(), PhantomData);
        }

        self.index_cache.insert(key, PhantomData);
    }

    /// update the frequency or recency of the template.
    pub fn update_cache(&mut self, key: &CacheKey) {
        if self.use_lfu {
            let _ = self
                .lfu_cache
                .get(key)
                .expect("Tried to update frequency of cache item that doesn't exist.");
        } else {
            self.lru_cache.promote(key);
        }
    }

    /// get index from cache
    pub fn get_index_of(&mut self, key: &CacheKey) -> Option<usize> {
        self.index_cache.get_index_of(key)
    }

    /// get value from cache
    pub fn get_with_index(&mut self, index: usize) -> Option<(&CacheKey, &PhantomData<u32>)> {
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
