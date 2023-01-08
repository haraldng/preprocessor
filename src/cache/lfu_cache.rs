use crate::cache::unicache::UniCache;
use lfu_cache::LfuCache;
use std::fmt::{Debug, Formatter};

enum Role {
    Encoder,
    Decoder,
}

pub struct LfuUniCache {
    index: u8,
    role: Role,
    encoder_cache: LfuCache<String, u8>,
    decoder_cache: LfuCache<u8, String>,
}

impl Debug for LfuUniCache {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        Result::Ok(())
    }
}

impl UniCache for LfuUniCache {
    fn new(capacity: usize) -> Self {
        Self {
            index: 0,
            encoder_cache: LfuCache::with_capacity(0),
            decoder_cache: LfuCache::with_capacity(capacity),
            role: Role::Decoder,
        }
    }

    fn put(&mut self, item: String) {
        if self.index == u8::MAX {
            match self.role {
                Role::Encoder => {
                    let (_, evicted_index) = self.encoder_cache.pop_lfu_key_value().unwrap();
                    let _evicted_after_insert = self.encoder_cache.insert(item, evicted_index);
                    // assert!(evicted_after_insert.is_none());
                }
                Role::Decoder => {
                    let (evicted_index, _) = self.decoder_cache.pop_lfu_key_value().unwrap();
                    let _evicted_after_insert = self.decoder_cache.insert(evicted_index, item);
                    // assert!(evicted_after_insert.is_none());
                }
            };
        } else {
            match self.role {
                Role::Encoder => {
                    let _ = self.encoder_cache.insert(item, self.index);
                }
                Role::Decoder => {
                    let _ = self.decoder_cache.insert(self.index, item);
                }
            };
            self.index += 1;
        }
    }

    fn get_encoded_index(&mut self, item: &str) -> Option<u8> {
        match self.role {
            Role::Encoder => self.encoder_cache.get(&item.to_string()).copied(),
            Role::Decoder => {
                // assert!(self.decoder_cache.is_empty());
                let capacity = self.decoder_cache.capacity().unwrap();
                self.encoder_cache = LfuCache::with_capacity(capacity.into());
                self.role = Role::Encoder;
                None
            }
        }
    }

    fn get_with_encoded_index(&mut self, index: u8) -> String {
        match self.role {
            Role::Encoder => {
                unimplemented!()
            }
            Role::Decoder => self.decoder_cache.get(&index).unwrap().clone(),
        }
    }
}
