use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use histogram::Histogram;
use preprocessor::cache::lfu_cache::LfuUniCache;
use preprocessor::cache::lru_cache::LruUniCache;
use preprocessor::cache::lecar_cache::LecarUniCache;
use preprocessor::cache::unicache::UniCache;
use preprocessor::util::*;
use strum::IntoEnumIterator;

mod preprocess;

use crate::preprocess::*;

const CACHE_CAPACITY: usize = 500;
const FILE: &str = "../../datasets/bustracker/queries-sample.txt";   // change this to run sample or full dataset.
const NUM_QUERIES: i64 = 100000; // -1 to run the whole benchmark


fn main() {
    let total_start = Instant::now();
    let mut query_len_histo = Histogram::new();

    let mut lfu_cache = LfuUniCache::new(CACHE_CAPACITY);
    let mut lru_cache = LruUniCache::new(CACHE_CAPACITY);
    let mut lecar_cache = LecarUniCache::new(CACHE_CAPACITY);

    let mut lfu_res = Results::new(CachePolicy::LFU);
    let mut lru_res = Results::new(CachePolicy::LRU);
    let mut lecar_res = Results::new(CachePolicy::LECAR);

    let file = File::open(FILE).unwrap();
    let reader = BufReader::new(file);
    for (idx, line) in reader.lines().enumerate() {
        if idx as i64 == NUM_QUERIES {
            break;
        }
        let command = StoreCommand {
            id: 0,
            sql: line.unwrap(),
        };
        for cache_type in CachePolicy::iter() {
            let mut raw_command = command.clone();
            match cache_type {
                CachePolicy::LFU => {
                    let start = Instant::now();
                    let (hit, compression_rate) = encode(&mut raw_command, &mut lfu_cache);
                    let encode_end = Instant::now();
                    decode(&mut raw_command, &mut lfu_cache);
                    let end = Instant::now();
                    lfu_res.update(start, encode_end, end, hit, compression_rate);
                }
                CachePolicy::LRU => {
                    let start = Instant::now();
                    let (hit, compression_rate) = encode(&mut raw_command, &mut lru_cache);
                    let encode_end = Instant::now();
                    decode(&mut raw_command, &mut lru_cache);
                    let end = Instant::now();
                    lru_res.update(start, encode_end, end, hit, compression_rate);
                }
                CachePolicy::LECAR => {
                    let start = Instant::now();
                    let (hit, compression_rate) = encode(&mut raw_command, &mut lecar_cache);
                    let encode_end = Instant::now();
                    decode(&mut raw_command, &mut lecar_cache);
                    let end = Instant::now();
                    lecar_res.update(start, encode_end, end, hit, compression_rate);
                }
            }
            assert_eq!(raw_command.sql, command.sql);
        }
        query_len_histo.increment(command.sql.len() as u64).unwrap();
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
            CachePolicy::LECAR => println!("{}", lecar_res),
        }
    }

}