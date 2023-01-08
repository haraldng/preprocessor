use histogram::Histogram;
use preprocess::{EmailUniCache};
// use preprocessor::cache::lecar_cache::LecarUniCache;
// use preprocessor::cache::lfu_cache::LfuUniCache;
use preprocessor::cache::lru_cache::LruUniCache;
use preprocessor::cache::unicache::*;
use preprocessor::util::*;
use std::fs::File;
use std::time::Instant;
use strum::IntoEnumIterator;
use preprocessor::cache::lecar_cache::LecarUniCache;
use preprocessor::cache::lfu_cache::LfuUniCache;

mod preprocess;
mod util;
// use preprocess::{encode, decode};
use util::*;

const CACHE_CAPACITY: usize = u8::MAX as usize;
// const CACHE_CAPACITY: usize = 500000;
const NUM_QUERIES: i64 = -1; // -1 to run the whole benchmark
const FILE: &str = "../../datasets/email/clean-sorted.csv"; // change this to run sample or full dataset.

fn main() {
    let total_start = Instant::now();
    let mut query_len_histo = Histogram::new();

    let mut lru_cache: EmailUniCache<LruUniCache> = EmailUniCache::new(CACHE_CAPACITY);
    let mut lru_decoder: EmailUniCache<LruUniCache> = EmailUniCache::new(CACHE_CAPACITY);
    let mut lru_res = Results::new(CachePolicy::LRU);

    let mut lfu_cache: EmailUniCache<LfuUniCache> = EmailUniCache::new(CACHE_CAPACITY);
    let mut lfu_decoder: EmailUniCache<LfuUniCache> = EmailUniCache::new(CACHE_CAPACITY);
    let mut lfu_res = Results::new(CachePolicy::LFU);

    let mut lecar_cache: EmailUniCache<LecarUniCache> = EmailUniCache::new(CACHE_CAPACITY);
    let mut lecar_decoder: EmailUniCache<LecarUniCache> = EmailUniCache::new(CACHE_CAPACITY);
    let mut lecar_res = Results::new(CachePolicy::LECAR);

    let file = File::open(FILE).unwrap();
    let mut reader = csv::Reader::from_reader(file);

    for (idx, record) in reader.deserialize().enumerate() {
        if idx as i64 == NUM_QUERIES {
            break;
        }

        let raw_record: RawHeader = record.unwrap();
        // println!("\n{:?}", raw_record);
        let raw = Header::Decoded(raw_record);
        // println!("size: {}", raw.get_size());
        let raw_size = raw.get_size() as f32;
        for cache_type in CachePolicy::iter() {
            let mut processed = raw.clone();
            match cache_type {
                CachePolicy::LFU => {
                    let start = Instant::now();
                    lfu_cache.encode(&mut processed);
                    let encode_end = Instant::now();
                    // println!("Compressed rate: {}, size: {}, {:?}", compression_rate, processed.get_size(), processed);
                    let processed_size = processed.get_size() as f32;
                    lfu_decoder.decode(&mut processed);
                    let end = Instant::now();
                    let compression_rate = 100f32 * (1f32 - processed_size / raw_size);
                    lfu_res.update(start, encode_end, end, false, compression_rate as usize);
                }
                CachePolicy::LRU => {
                    let start = Instant::now();
                    lru_cache.encode(&mut processed);
                    let encode_end = Instant::now();
                    // println!("Compressed rate: {}, size: {}, {:?}", compression_rate, processed.get_size(), processed);
                    let processed_size = processed.get_size() as f32;
                    lru_decoder.decode(&mut processed);
                    let end = Instant::now();
                    let compression_rate = 100f32 * (1f32 - processed_size / raw_size);
                    lru_res.update(start, encode_end, end, false, compression_rate as usize);
                }
                CachePolicy::LECAR => {
                    let start = Instant::now();
                    lecar_cache.encode(&mut processed);
                    let encode_end = Instant::now();
                    // println!("Compressed rate: {}, size: {}, {:?}", compression_rate, processed.get_size(), processed);
                    let processed_size = processed.get_size() as f32;
                    lecar_decoder.decode(&mut processed);
                    let end = Instant::now();
                    let compression_rate = 100f32 * (1f32 - processed_size / raw_size);
                    lecar_res.update(start, encode_end, end, false, compression_rate as usize);
                }
            }
            assert_eq!(
                processed, raw,
                "{}: Incorrect encode/decode with {:?}",
                idx, cache_type,
            );
        }

        query_len_histo.increment(raw.get_size() as u64).unwrap();
    }
    let total_end = Instant::now();
    /*** Print Results ***/
    println!("Total time: {:?}", total_end.duration_since(total_start));

    println!(
        "Number of Queries: {}. Query length: Avg: {}, p50: {}, p95: {}, Min: {}, Max: {}, StdDev: {}",
        query_len_histo.entries(),
        query_len_histo.mean().unwrap(),
        query_len_histo.percentile(50f64).unwrap(),
        query_len_histo.percentile(95f64).unwrap(),
        query_len_histo.minimum().unwrap(),
        query_len_histo.maximum().unwrap(),
        query_len_histo.stddev().unwrap(),
    );

    for cache_type in CachePolicy::iter() {
        match cache_type {
            CachePolicy::LFU => {
                println!("{}", lfu_res)
            },
            CachePolicy::LRU => println!("{}", lru_res),
            CachePolicy::LECAR => {
                println!("{}", lecar_res)
            }
        }
    }
}
