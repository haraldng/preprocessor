use histogram::Histogram;
use preprocess::{decode, encode, NUM_CACHES};
use preprocessor::cache::lecar_cache::LecarUniCache;
use preprocessor::cache::lfu_cache::LfuUniCache;
use preprocessor::cache::lru_cache::LruUniCache;
use preprocessor::cache::unicache::*;
use preprocessor::util::*;
use std::fs::File;
use std::time::Instant;
use strum::IntoEnumIterator;

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

    let mut lfu_caches = [false; NUM_CACHES].map(|_| LfuUniCache::new(CACHE_CAPACITY));
    let mut lfu_decoders = [false; NUM_CACHES].map(|_| LfuUniCache::new(CACHE_CAPACITY));
    let mut lfu_res = Results::new(CachePolicy::LFU);

    let mut lru_caches = [false; NUM_CACHES].map(|_| LruUniCache::new(CACHE_CAPACITY));
    let mut lru_decoders = [false; NUM_CACHES].map(|_| LruUniCache::new(CACHE_CAPACITY));
    let mut lru_res = Results::new(CachePolicy::LRU);

    let mut lecar_caches = [false; NUM_CACHES].map(|_| LecarUniCache::new(CACHE_CAPACITY));
    let mut lecar_decoders = [false; NUM_CACHES].map(|_| LecarUniCache::new(CACHE_CAPACITY));
    let mut lecar_res = Results::new(CachePolicy::LECAR);

    let file = File::open(FILE).unwrap();
    let mut reader = csv::Reader::from_reader(file);

    for (idx, record) in reader.deserialize().enumerate() {
        if idx as i64 == NUM_QUERIES {
            break;
        }

        let raw_record: RawHeader = record.unwrap();
        // println!("\n{:?}", raw_record);
        let raw = Record::Decoded(raw_record);
        // println!("size: {}", raw.get_size());

        for cache_type in CachePolicy::iter() {
            let mut processed = raw.clone();
            match cache_type {
                CachePolicy::LFU => {
                    let start = Instant::now();
                    let (hit, compression_rate) = encode(&mut processed, &mut lfu_caches);
                    let encode_end = Instant::now();
                    // println!("Compressed size: {}", compressed_command.get_size());
                    decode(&mut processed, &mut lfu_decoders);
                    let end = Instant::now();
                    lfu_res.update(start, encode_end, end, hit, compression_rate);
                }
                CachePolicy::LRU => {
                    let start = Instant::now();
                    let (hit, compression_rate) = encode(&mut processed, &mut lru_caches);
                    let encode_end = Instant::now();
                    // println!("Compressed rate: {}, size: {}, {:?}", compression_rate, processed.get_size(), processed);
                    decode(&mut processed, &mut lru_decoders);
                    let end = Instant::now();
                    lru_res.update(start, encode_end, end, hit, compression_rate);
                }
                CachePolicy::LECAR => {
                    // continue; // TODO
                      let start = Instant::now();
                      let (hit, compression_rate) = encode(&mut processed, &mut lecar_caches);
                      let encode_end = Instant::now();
                      // println!("Compressed size: {}", compressed_command.get_size());
                      decode(&mut processed, &mut lecar_decoders);
                      let end = Instant::now();
                      lecar_res.update(start, encode_end, end, hit, compression_rate);

                }
            }
            assert_eq!(
                processed, raw,
                "{}: Incorrect encode/decode with {:?}",
                idx,
                cache_type,
            );
        }

        query_len_histo.increment(raw.get_size() as u64).unwrap();
        // approx_query_len_histo.increment(record.get_approximate_encoded_size() as u64).unwrap();
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
            CachePolicy::LFU => println!("{}", lfu_res),
            CachePolicy::LRU => println!("{}", lru_res),
            CachePolicy::LECAR => {
                println!("{}", lecar_res)
            }
        }
    }
}
