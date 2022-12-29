use crate::cache::UniCache;
use lecar::controller::Controller;

pub struct LecarUniCache {
    lecar_cache: Controller,
}

impl UniCache<String> for LecarUniCache {
    fn new(capacity: usize) -> Self {
        Self {
            lecar_cache: Controller::new(capacity, capacity, capacity),
        }
    }

    fn put(&mut self, item: String) {
        self.lecar_cache.insert(&item, 0);
    }

    fn get_encoded_index(&mut self, item: &String) -> Option<usize> {
        self.lecar_cache.get_index_of(item)
    }

    fn get_with_encoded_index(&mut self, index: usize) -> String {
        self.lecar_cache.get_index(index).unwrap().to_string()
    }
}
