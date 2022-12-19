use indexmap::IndexMap;
use lfu_cache::LfuCache;
use lru::LruCache;
use std::marker::PhantomData;
use std::num::NonZeroUsize;
use lecar::controller::Controller;
use strum_macros::EnumIter; 

pub type CacheKey = String;

#[derive(Debug, Eq, PartialEq, EnumIter)]
pub enum CacheType {
    LFU,
    LRU,
    LECAR
}

/// The cache we use in paxos
pub struct CacheModel {
    cache_type: CacheType,
    lfu_cache: LfuCache<CacheKey, PhantomData<u32>>,
    lru_cache: LruCache<CacheKey, PhantomData<u32>>,
    lecar_cache: Controller,
    index_cache: IndexMap<CacheKey, PhantomData<u32>>,
}

impl CacheModel {
    /// create a new cache model
    pub fn with(capacity: usize, cache_type: CacheType) -> Self {
        CacheModel {
            cache_type,
            lfu_cache: LfuCache::with_capacity(capacity),
            lru_cache: LruCache::new(NonZeroUsize::new(capacity).unwrap()),
            lecar_cache: Controller::new(capacity, capacity, capacity),
            index_cache: IndexMap::with_capacity(capacity),
        }
    }

    /// save (key, value) pair into cache
    /// Optimization: The value could be skipped if it always equals to the key
    pub fn put(&mut self, key: CacheKey) {
        match self.cache_type {
            CacheType::LFU => { self.lfu_cache.insert(key.clone(), PhantomData); },
            CacheType::LRU => { self.lru_cache.put(key.clone(), PhantomData); },
            CacheType::LECAR => { self.lecar_cache.insert(&key, 0); },
        }

        self.index_cache.insert(key, PhantomData);
    }

    /// update the frequency or recency of the template.
    pub fn update_cache(&mut self, key: &CacheKey) {
        match self.cache_type {
            CacheType::LFU => { self.lfu_cache.get(key).expect("Tried to update frequency of cache item that doesn't exist."); },
            CacheType::LRU => { self.lru_cache.promote(key); },
            CacheType::LECAR => { self.lecar_cache.get(key).expect("Tried to update frequency of cache item that doesn't exist.");  },
        }
    }

    /// get index from cache
    pub fn get_index_of(&self, key: &CacheKey) -> Option<usize> {
        self.index_cache.get_index_of(key)
    }

    /// get value from cache
    pub fn get_with_index(&self, index: usize) -> Option<(&CacheKey, &PhantomData<u32>)> {
        self.index_cache.get_index(index)
    }

    /// return cache length
    pub fn len(&self) -> usize {
        match self.cache_type {
            CacheType::LFU => self.lfu_cache.len(),
            CacheType::LRU => self.lru_cache.len(),
            CacheType::LECAR => self.lecar_cache.len().0,
        }
    }
}
