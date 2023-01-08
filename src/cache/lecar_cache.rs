use crate::cache::unicache::UniCache;
use lecar::controller::Controller;
use std::fmt::{Debug, Formatter};

pub struct LecarUniCache {
    lecar_cache: Controller,
}

impl Debug for LecarUniCache {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        Result::Ok(())
    }
}

impl UniCache for LecarUniCache {
    fn new(capacity: usize) -> Self {
        Self {
            lecar_cache: Controller::new(capacity, capacity, capacity),
        }
    }

    fn put(&mut self, item: String) {
        self.lecar_cache.insert(&item, 0);
    }

    fn get_encoded_index(&mut self, item: &str) -> Option<u8> {
        self.lecar_cache.get_index_of(item).map(|x| x as u8)
    }

    fn get_with_encoded_index(&mut self, index: u8) -> String {
        self.lecar_cache.get_index(index as usize).unwrap().to_string()
    }
}


