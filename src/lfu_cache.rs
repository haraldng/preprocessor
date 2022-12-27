use crate::cache::UniCache;
use indexmap::IndexMap;
use lfu_cache::LfuCache;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

pub struct LfuUniCache<T: Clone + Debug + Hash + Eq> {
    lfu_cache: LfuCache<T, PhantomData<bool>>,
    index_cache: IndexMap<T, PhantomData<bool>>,
}

impl<T: Clone + Debug + Hash + Eq> UniCache<T> for LfuUniCache<T> {
    fn new(capacity: usize) -> Self {
        Self {
            lfu_cache: LfuCache::with_capacity(capacity),
            index_cache: IndexMap::with_capacity(capacity),
        }
    }

    fn put(&mut self, item: T) {
        let _maybe_evicted = self.lfu_cache.insert(item.clone(), PhantomData);
        // TODO remove maybe_evicted item from index_cache
        self.index_cache.insert(item, PhantomData);
    }

    fn get_encoded_index(&mut self, item: &T) -> Option<usize> {
        if self.lfu_cache.get(item).is_some() {
            self.index_cache.get_index_of(item)
        } else {
            None
        }
    }

    fn get_with_encoded_index(&mut self, index: usize) -> T {
        let item = self.index_cache.get_index(index).unwrap().0.clone();
        self.lfu_cache.get(&item).unwrap_or_else(|| {
            panic!(
                "Tried to update frequency of cache item that doesn't exist: {:?}",
                &item
            )
        });
        item
    }
}
