mod load;

mod preprocess;
mod util;
mod cache;
mod medium;


use crate::cache::unicache::*;
/*
use crate::load::StoreCommand;
use crate::preprocess::{decode, encode};
use std::io::{prelude::*, BufReader};
 */
use crate::util::Results;
use histogram::Histogram;
use std::fs::File;
use std::time::Instant;
use strum::IntoEnumIterator;
use crate::cache::lecar_cache::LecarUniCache;
use crate::cache::lfu_cache::LfuUniCache;
use crate::cache::lru_cache::LruUniCache;
use crate::medium::{MediumRecord, RawMediumRecord, Record};

type CacheKey = String;
const CACHE_CAPACITY: usize = u8::MAX as usize;
// const CACHE_CAPACITY: usize = 500000;
const NUM_QUERIES: i64 = 100000; // -1 to run the whole benchmark
const FILE: &str = "datasets/medium/Train.csv";   // change this to run sample or full dataset.

fn main() {
    let total_start = Instant::now();
    let mut query_len_histo = Histogram::new();

    let mut lfu_caches = [false; 4].map(|_| LfuUniCache::new(CACHE_CAPACITY));
    let mut lfu_decoders = [false; 4].map(|_| LfuUniCache::new(CACHE_CAPACITY));

    let mut lru_caches = [false; 4].map(|_| LruUniCache::new(CACHE_CAPACITY));
    let mut lru_decoders = [false; 4].map(|_| LruUniCache::new(CACHE_CAPACITY));

    let mut lecar_caches = [false; 4].map(|_| LecarUniCache::new(CACHE_CAPACITY));
    let mut lecar_decoders = [false; 4].map(|_| LecarUniCache::new(CACHE_CAPACITY));


    // let y: [&mut LfuUniCache<String>; 4] = {
    //     todo!()
    // };
/*
    let mut lfu_cache = LfuUniCache::new(CACHE_CAPACITY);
    let mut lru_cache = LruUniCache::new(CACHE_CAPACITY);
    let mut lecar_cache = LecarUniCache::new(CACHE_CAPACITY);

    let mut lfu_decoder = LfuUniCache::new(CACHE_CAPACITY);
    let mut lru_decoder = LruUniCache::new(CACHE_CAPACITY);
    let mut lecar_decoder = LecarUniCache::new(CACHE_CAPACITY);
    */

    let mut lfu_res = Results::new(CachePolicy::LFU);
    let mut lru_res = Results::new(CachePolicy::LRU);
    let mut lecar_res = Results::new(CachePolicy::LECAR);

    let file = File::open("datasets/medium/medium_articles.csv").unwrap();
    let mut reader = csv::Reader::from_reader(file);

    for (idx, record) in reader.deserialize().enumerate() {
        if idx as i64 == NUM_QUERIES {
            break;
        }

        let raw_record: RawMediumRecord = record.unwrap();
        let record: MediumRecord = raw_record.into();
        // println!("\n{:?}", record);
        let command = Record::Decoded(record);
        // println!("size: {}", command.get_size());

        for cache_type in CachePolicy::iter() {
            let mut compressed_command = command.clone();
            match cache_type {
                CachePolicy::LFU => {
                    let start = Instant::now();
                    let (hit, compression_rate) = medium::preprocess::encode(&mut compressed_command, &mut lfu_caches);
                    let encode_end = Instant::now();
                    // println!("Compressed size: {}", compressed_command.get_size());
                    medium::preprocess::decode(&mut compressed_command, &mut lfu_decoders);
                    let end = Instant::now();
                    lfu_res.update(start, encode_end, end, hit, compression_rate);
                }
                CachePolicy::LRU => {
                    let start = Instant::now();
                    let (hit, compression_rate) = medium::preprocess::encode(&mut compressed_command, &mut lru_caches);
                    let encode_end = Instant::now();
                    // println!("Compressed size: {}", compressed_command.get_size());
                    medium::preprocess::decode(&mut compressed_command, &mut lru_decoders);
                    let end = Instant::now();
                    lru_res.update(start, encode_end, end, hit, compression_rate);
                }
                CachePolicy::LECAR => {
                    let start = Instant::now();
                    let (hit, compression_rate) = medium::preprocess::encode(&mut compressed_command, &mut lecar_caches);
                    let encode_end = Instant::now();
                    // println!("Compressed size: {}", compressed_command.get_size());
                    medium::preprocess::decode(&mut compressed_command, &mut lecar_decoders);
                    let end = Instant::now();
                    lecar_res.update(start, encode_end, end, hit, compression_rate);
                }
            }
            assert_eq!(compressed_command, command);
        }
        query_len_histo.increment(command.get_size() as u64).unwrap();
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


    /*
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
     */
}