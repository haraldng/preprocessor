use preprocessor::cache::unicache::UniCache;
use crate::util::{EncodedMediumRecord, MaybeEncoded, MediumRecord, Record};

pub const PUBLICATION: usize = 0;
pub const TAGS: usize = 1;
pub const NAME: usize = 2;
pub const AUTHOR: usize = 3;

const THRESHOLD: usize = 0;

pub fn encode<U: UniCache<String>>(
    record: &mut Record,
    cache: &mut [U; 4],
) -> (bool, usize) {
    // split sql into template and parameters
    let raw_size = record.get_size() as f32;
    match &record {
        Record::Decoded(me) => {
            let post_publication = match cache[PUBLICATION].get_encoded_index(&me.post_publication) {
                Some(i) => MaybeEncoded::Encoded(i),
                None => {
                    cache[PUBLICATION].put(me.post_publication.to_string());
                    MaybeEncoded::Decoded(me.post_publication.to_string())
                },
            };
            let post_tags: [MaybeEncoded; 4] = me.clone().post_tags.map(|x| {
                if x.len() > THRESHOLD {
                    match cache[TAGS].get_encoded_index(&x) {
                        Some(i) => MaybeEncoded::Encoded(i),
                        None => {
                            cache[TAGS].put(x.clone());
                            MaybeEncoded::Decoded(x)
                        },
                    }
                } else {
                    MaybeEncoded::Decoded(x)
                }
            });
            let post_author: Vec<MaybeEncoded> = {
                me.post_author.split_whitespace().map( |x| {
                    let x = x.to_string(); // TODO remove this to_string
                    if x.len() > THRESHOLD {
                        match cache[AUTHOR].get_encoded_index(&x) {
                            Some(i) => MaybeEncoded::Encoded(i),
                            None => {
                                cache[AUTHOR].put(x.clone());
                                MaybeEncoded::Decoded(x)
                            },
                        }
                    } else {
                        MaybeEncoded::Decoded(x)
                    }
                }).collect()
            };
            let post_name: Vec<MaybeEncoded> = {
                me.post_name.split_whitespace().map( |x| {
                    let x = x.to_string(); // TODO remove this to_string
                    if x.len() > THRESHOLD {
                        match cache[NAME].get_encoded_index(&x) {
                            Some(i) => MaybeEncoded::Encoded(i),
                            None => {
                                cache[NAME].put(x.clone());
                                MaybeEncoded::Decoded(x)
                            },
                        }
                    } else {
                        MaybeEncoded::Decoded(x)
                    }
                }).collect()
            };
            let encoded = EncodedMediumRecord {
                post_time: me.post_time.to_string(),
                post_name,
                post_author,
                post_publication,
                post_tags
            };
            // println!("ENCODED: {:?}", encoded);
            *record = Record::Encoded(encoded);
        },
        _ => unimplemented!()
    }
    let compressed_size = record.get_size() as f32;
    (
        false,
        (100f32 * (1f32 - compressed_size / raw_size)) as usize,
    )
}

pub fn decode<U: UniCache<String>>(record: &mut Record, cache: &mut [U; 4]) {
    let mut rec = Record::None;
    std::mem::swap(record, &mut rec);
    let decoded = match rec {
        Record::Encoded(e) => {
            let post_publication = match e.post_publication {
                MaybeEncoded::Encoded(i) => cache[PUBLICATION].get_with_encoded_index(i),
                MaybeEncoded::Decoded(s) => {
                    cache[PUBLICATION].put(s.clone());
                    s
                },
            };
            let post_tags: [String; 4] = e.post_tags.map(|x| {
                let s = match x {
                    MaybeEncoded::Encoded(i) => cache[TAGS].get_with_encoded_index(i),
                    MaybeEncoded::Decoded(s) => {
                        if s.len() > THRESHOLD {
                            cache[TAGS].put(s.clone());
                        }
                        s
                    },
                };
                s
            });
            let pauthor: Vec<String> = e.post_author
                .into_iter()
                .map(|x| {
                    let s = match x {
                        MaybeEncoded::Encoded(i) => cache[AUTHOR].get_with_encoded_index(i),
                        MaybeEncoded::Decoded(s) => {
                            if s.len() > THRESHOLD {
                                cache[AUTHOR].put(s.clone());
                            }
                            s
                        },
                    };
                    s
                })
                .collect();
            let post_author = pauthor.join(" ");
            let pname: Vec<String> = e.post_name
                .into_iter()
                .map(|x| {
                    let s = match x {
                        MaybeEncoded::Encoded(i) => cache[NAME].get_with_encoded_index(i),
                        MaybeEncoded::Decoded(s) => {
                            if s.len() > THRESHOLD {
                                cache[NAME].put(s.clone());
                            }
                            s
                        },
                    };
                    s
                })
                .collect();
            let post_name = pname.join(" ");
            let m = MediumRecord {
                post_time: e.post_time,
                post_name,
                post_author,
                post_publication,
                post_tags
            };
            Record::Decoded(m)
        },
        _ => unimplemented!(),
    };
    *record = decoded;
}
