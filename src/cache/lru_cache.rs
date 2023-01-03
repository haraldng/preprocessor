use crate::cache::unicache::UniCache;
use indexmap::IndexMap;
use lru::LruCache;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::num::NonZeroUsize;

pub struct LruUniCache {
    lru_cache: LruCache<String, PhantomData<bool>>,
    index_cache: IndexMap<String, PhantomData<bool>>,
}

impl Debug for LruUniCache {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        Result::Ok(())
    }
}

impl UniCache for LruUniCache {
    fn new(capacity: usize) -> Self {
        Self {
            lru_cache: LruCache::new(NonZeroUsize::new(capacity).unwrap()),
            index_cache: IndexMap::with_capacity(capacity),
        }
    }

    fn put(&mut self, item: String) {
        let maybe_evicted = self.lru_cache.push(item.clone(), PhantomData);
        if let Some((evicted, _)) = maybe_evicted {
            let _ = self.index_cache.remove(&evicted);
        }
        self.index_cache.insert(item, PhantomData);
    }

    fn get_encoded_index(&mut self, item: &str) -> Option<u8> {
        if self.lru_cache.get(item).is_some() {
            self.index_cache.get_index_of(item).map(|x| x as u8)
        } else {
            None
        }
    }

    fn get_with_encoded_index(&mut self, index: u8) -> String {
        let item = self
            .index_cache
            .get_index(index as usize)
            .unwrap()
            .0
            .clone();
        self.lru_cache.promote(&item);
        item
    }
}
