mod cache;
mod lecar_cache;
mod lfu_cache;
mod load;
mod lru_cache;
mod preprocess;
mod util;

use crate::cache::{CacheType, UniCache};
use crate::lecar_cache::LecarUniCache;
use crate::lfu_cache::LfuUniCache;
use crate::load::StoreCommand;
use crate::lru_cache::LruUniCache;
use crate::preprocess::{decode, encode};
use crate::util::Results;
use histogram::Histogram;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;
use strum::IntoEnumIterator;

type CacheKey = String;
const CACHE_CAPACITY: usize = 500;
const NUM_QUERIES: i64 = -1; // -1 to run the whole benchmark
const FILE: &str = "queries-sample.txt";   // change this to run sample or full dataset.

fn main() {
    let total_start = Instant::now();
    let mut query_len_histo = Histogram::new();

    let mut lfu_cache = LfuUniCache::new(CACHE_CAPACITY);
    let mut lfu_decoder = LfuUniCache::new(CACHE_CAPACITY);

    let mut lru_cache = LruUniCache::new(CACHE_CAPACITY);
    let mut lru_decoder = LruUniCache::new(CACHE_CAPACITY);

    let mut lecar_cache = LecarUniCache::new(CACHE_CAPACITY);
    let mut lecar_decoder = LecarUniCache::new(CACHE_CAPACITY);

    let mut lfu_res = Results::new(CacheType::LFU);
    let mut lru_res = Results::new(CacheType::LRU);
    let mut lecar_res = Results::new(CacheType::LECAR);

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
        for cache_type in CacheType::iter() {
            let mut raw_command = command.clone();
            match cache_type {
                CacheType::LFU => {
                    let start = Instant::now();
                    let (hit, compression_rate) = encode(&mut raw_command, &mut lfu_cache);
                    let encode_end = Instant::now();
                    decode(&mut raw_command, &mut lfu_decoder);
                    let end = Instant::now();
                    lfu_res.update(start, encode_end, end, hit, compression_rate);
                }
                CacheType::LRU => {
                    let start = Instant::now();
                    let (hit, compression_rate) = encode(&mut raw_command, &mut lru_cache);
                    let encode_end = Instant::now();
                    decode(&mut raw_command, &mut lru_decoder);
                    let end = Instant::now();
                    lru_res.update(start, encode_end, end, hit, compression_rate);
                }
                CacheType::LECAR => {
                    let start = Instant::now();
                    let (hit, compression_rate) = encode(&mut raw_command, &mut lecar_cache);
                    let encode_end = Instant::now();
                    decode(&mut raw_command, &mut lecar_decoder);
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
    for cache_type in CacheType::iter() {
        match cache_type {
            CacheType::LFU => println!("{}", lfu_res),
            CacheType::LRU => println!("{}", lru_res),
            CacheType::LECAR => println!("{}", lecar_res),
        }
    }
}
