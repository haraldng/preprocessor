use crate::cache::UniCache;
use indexmap::IndexMap;
use lru::LruCache;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::num::NonZeroUsize;

pub struct LruUniCache<T: Clone + Debug + Hash + Eq> {
    lru_cache: LruCache<T, PhantomData<bool>>,
    index_cache: IndexMap<T, PhantomData<bool>>,
}

impl<T: Clone + Debug + Hash + Eq> UniCache<T> for LruUniCache<T> {
    fn new(capacity: usize) -> Self {
        Self {
            lru_cache: LruCache::new(NonZeroUsize::new(capacity).unwrap()),
            index_cache: IndexMap::with_capacity(capacity),
        }
    }

    fn put(&mut self, item: T) {
        let maybe_evicted = self.lru_cache.push(item.clone(), PhantomData);
        if let Some((evicted, _)) = maybe_evicted {
            let _ = self.index_cache.remove(&evicted);
        }
        self.index_cache.insert(item, PhantomData);
    }

    fn get_encoded_index(&mut self, item: &T) -> Option<usize> {
        if self.lru_cache.get(item).is_some() {
            self.index_cache.get_index_of(item)
        } else {
            None
        }
    }

    fn get_with_encoded_index(&mut self, index: usize) -> T {
        let item = self.index_cache.get_index(index).unwrap().0.clone();
        self.lru_cache.promote(&item);
        item
    }
}
