use crate::CacheType;
use histogram::Histogram;
use std::fmt;
use std::fmt::Formatter;
use std::time::Instant;

pub(crate) struct Results {
    cache_type: CacheType,
    encode_histo: Histogram,
    decode_histo: Histogram,
    compression_histo: Histogram,
    hit_count: usize,
    total: usize,
}

impl Results {
    pub(crate) fn new(cache_type: CacheType) -> Self {
        Self {
            cache_type,
            encode_histo: Default::default(),
            decode_histo: Default::default(),
            compression_histo: Default::default(),
            hit_count: 0,
            total: 0,
        }
    }

    pub(crate) fn update(
        &mut self,
        start: Instant,
        encode_end: Instant,
        end: Instant,
        cache_hit: bool,
        compression_ratio: usize,
    ) {
        let encode_time = encode_end.duration_since(start).as_nanos() as u64;
        let decode_time = end.duration_since(encode_end).as_nanos() as u64;
        self.encode_histo.increment(encode_time).unwrap();
        self.decode_histo.increment(decode_time).unwrap();
        self.compression_histo
            .increment(compression_ratio as u64)
            .unwrap();
        self.total += 1;
        if cache_hit {
            self.hit_count += 1;
        }
    }
}

impl fmt::Display for Results {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let hit_rate = self.hit_count as f32 / self.total as f32;
        write!(
            f,
            "--------------------------------
            \nCache type: {:?}\n\
            Encoding (ns): Avg: {}, p50: {}, p95: {}, Min: {}, Max: {}, StdDev: {}\n\
            Decoding (ns): Avg: {}, p50: {}, p95: {}, Min: {}, Max: {}, StdDev: {}\n\
            Compression Rate (%): Avg: {}, p50: {}, p95: {}, Min: {}, Max: {}, StdDev: {}\n\
            Hit rate: {}\n",
            self.cache_type,
            self.encode_histo.mean().unwrap(),
            self.encode_histo.percentile(50f64).unwrap(),
            self.encode_histo.percentile(95f64).unwrap(),
            self.encode_histo.minimum().unwrap(),
            self.encode_histo.maximum().unwrap(),
            self.encode_histo.stddev().unwrap(),
            self.decode_histo.mean().unwrap(),
            self.decode_histo.percentile(50f64).unwrap(),
            self.decode_histo.percentile(95f64).unwrap(),
            self.decode_histo.minimum().unwrap(),
            self.decode_histo.maximum().unwrap(),
            self.decode_histo.stddev().unwrap(),
            self.compression_histo.mean().unwrap(),
            self.compression_histo.percentile(50f64).unwrap(),
            self.compression_histo.percentile(95f64).unwrap(),
            self.compression_histo.minimum().unwrap(),
            self.compression_histo.maximum().unwrap(),
            self.compression_histo.stddev().unwrap(),
            hit_rate,
        )
    }
}
