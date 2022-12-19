mod cache;
mod load;
mod preprocess;

use crate::preprocess::{decode, encode};
use histogram::Histogram;
use std::time::Instant;

fn main() {
    let commands = load::read_from_file("queries.txt");
    let mut cache = cache::CacheModel::with(500, true);
    let num_commands = commands.len();
    println!("Total number of commands: {}", num_commands);

    let mut encode_histo = Histogram::new();
    let mut decode_histo = Histogram::new();
    let mut query_len_histo = Histogram::new();

    // run with checks
    for command in &commands[0..] {
        let mut raw_command = command.clone();
        let start = Instant::now();
        encode(&mut raw_command, &mut cache);
        let encode_end = Instant::now();
        decode(&mut raw_command, &mut cache);
        let decode_end = Instant::now();

        assert_eq!(raw_command.sql, command.sql);

        let encode_time = encode_end.duration_since(start).as_nanos();
        let decode_time = decode_end.duration_since(encode_end).as_nanos();
        encode_histo.increment(encode_time as u64).unwrap();
        decode_histo.increment(decode_time as u64).unwrap();
        query_len_histo.increment(command.sql.len() as u64).unwrap();
    }
    println!("Number of commands: {}", num_commands);
    println!(
        "Encoding (ns): Avg: {}, p50: {}, p95: {}, Min: {}, Max: {}, StdDev: {}",
        encode_histo.mean().unwrap(),
        encode_histo.percentile(50f64).unwrap(),
        encode_histo.percentile(95f64).unwrap(),
        encode_histo.minimum().unwrap(),
        encode_histo.maximum().unwrap(),
        encode_histo.stddev().unwrap(),
    );
    println!(
        "Decoding (ns): Avg: {}, p50: {}, p95: {}, Min: {}, Max: {}, StdDev: {}",
        decode_histo.mean().unwrap(),
        decode_histo.percentile(50f64).unwrap(),
        decode_histo.percentile(95f64).unwrap(),
        decode_histo.minimum().unwrap(),
        decode_histo.maximum().unwrap(),
        decode_histo.stddev().unwrap(),
    );
    println!(
        "Query length: Avg: {}, p50: {}, p95: {}, Min: {}, Max: {}, StdDev: {}",
        query_len_histo.mean().unwrap(),
        query_len_histo.percentile(50f64).unwrap(),
        query_len_histo.percentile(95f64).unwrap(),
        query_len_histo.minimum().unwrap(),
        query_len_histo.maximum().unwrap(),
        query_len_histo.stddev().unwrap(),
    );
}
