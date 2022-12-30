use crate::{UniCache};
use crate::medium::{EncodedMediumRecord, MaybeEncoded, MediumRecord, Record};

pub fn encode<U: UniCache<String>>(
    record: &mut Record,
    cache: &mut U,
) -> (bool, usize) {
    // split sql into template and parameters
    let raw_size = record.get_size() as f32;
    match &record {
        Record::Decoded(me) => {
            let post_publication = match cache.get_encoded_index(&me.post_publication) {
                Some(i) => MaybeEncoded::Encoded(i),
                None => {
                    cache.put(me.post_publication.to_string());
                    MaybeEncoded::Decoded(me.post_publication.to_string())
                },
            };
            let post_tag_1 =
                if &me.post_tag_1.len() > &2usize {
                    MaybeEncoded::Decoded(me.post_tag_1.to_string())
                } else {
                    match cache.get_encoded_index(&me.post_tag_1) {
                        Some(i) => MaybeEncoded::Encoded(i),
                        None => {
                            cache.put(me.post_tag_1.to_string());
                            MaybeEncoded::Decoded(me.post_tag_1.to_string())
                        },
                    }
                };
            let post_tag_2 =
                if &me.post_tag_2.len() > &2usize {
                    MaybeEncoded::Decoded(me.post_tag_2.to_string())
                } else {
                    match cache.get_encoded_index(&me.post_tag_2) {
                        Some(i) => MaybeEncoded::Encoded(i),
                        None => {
                            cache.put(me.post_tag_2.to_string());
                            MaybeEncoded::Decoded(me.post_tag_2.to_string())
                        },
                    }
                };
            let post_tag_3 =
                if &me.post_tag_3.len() > &2usize {
                    MaybeEncoded::Decoded(me.post_tag_3.to_string())
                } else {
                    match cache.get_encoded_index(&me.post_tag_3) {
                        Some(i) => MaybeEncoded::Encoded(i),
                        None => {
                            cache.put(me.post_tag_3.to_string());
                            MaybeEncoded::Decoded(me.post_tag_3.to_string())
                        },
                    }
                };
            let post_tag_4 =
                if &me.post_tag_4.len() > &2usize {
                    MaybeEncoded::Decoded(me.post_tag_4.to_string())
                } else {
                    match cache.get_encoded_index(&me.post_tag_4) {
                        Some(i) => MaybeEncoded::Encoded(i),
                        None => {
                            cache.put(me.post_tag_4.to_string());
                            MaybeEncoded::Decoded(me.post_tag_4.to_string())
                        },
                    }
                };
            let post_author: Vec<MaybeEncoded> = {
                me.post_author.split_whitespace().map( |x| {
                    let x_string = x.to_string(); // TODO remove this to_string
                    match cache.get_encoded_index(&x_string) {
                        Some(i) => MaybeEncoded::Encoded(i),
                        None => {
                            cache.put(x_string.clone());
                            MaybeEncoded::Decoded(x_string)
                        },
                    }
                }).collect()
            };
            let post_name: Vec<MaybeEncoded> = {
                me.post_name.split_whitespace().enumerate().map( |(id, x)| {
                    let x_string = x.to_string(); // TODO remove this to_string
                    match cache.get_encoded_index(&x_string) {
                        Some(i) => {
                            MaybeEncoded::Encoded(i)
                        },
                        None => {
                            cache.put(x_string.clone());
                            MaybeEncoded::Decoded(x_string)
                        },
                    }
                }).collect()
            };
            let encoded = EncodedMediumRecord {
                post_time: me.post_time.to_string(),
                post_name,
                post_author,
                post_publication,
                post_tag_1,
                post_tag_2,
                post_tag_3,
                post_tag_4
            };
            println!("ENCODED: {:?}", encoded);
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

pub fn decode<U: UniCache<String>>(record: &mut Record, cache: &mut U) {
    let mut rec = Record::None;
    std::mem::swap(record, &mut rec);
    let decoded = match rec {
        Record::Encoded(e) => {
            let post_publication = match e.post_publication {
                MaybeEncoded::Encoded(i) => cache.get_with_encoded_index(i),
                MaybeEncoded::Decoded(s) => {
                    cache.put(s.clone());
                    s
                },
            };
            let post_tag_1 = match e.post_tag_1 {
                MaybeEncoded::Encoded(i) => cache.get_with_encoded_index(i),
                MaybeEncoded::Decoded(s) => {
                    if s.len() > 2 {
                        s
                    } else {
                        cache.put(s.clone());
                        s
                    }
                },
            };
            let post_tag_2 = match e.post_tag_2 {
                MaybeEncoded::Encoded(i) => cache.get_with_encoded_index(i),
                MaybeEncoded::Decoded(s) => {
                    if s.len() > 2 {
                        s
                    } else {
                        cache.put(s.clone());
                        s
                    }
                },
            };
            let post_tag_3 = match e.post_tag_3 {
                MaybeEncoded::Encoded(i) => cache.get_with_encoded_index(i),
                MaybeEncoded::Decoded(s) => {
                    if s.len() > 2 {
                        s
                    } else {
                        cache.put(s.clone());
                        s
                    }
                },
            };
            let post_tag_4 = match e.post_tag_4{
                MaybeEncoded::Encoded(i) => cache.get_with_encoded_index(i),
                MaybeEncoded::Decoded(s) => {
                    if s.len() > 2 {
                        s
                    } else {
                        cache.put(s.clone());
                        s
                    }
                },
            };
            let pauthor: Vec<String> = e.post_author
                .into_iter()
                .map(|x| {
                    let s = match x {
                        MaybeEncoded::Encoded(i) => cache.get_with_encoded_index(i),
                        MaybeEncoded::Decoded(s) => {
                            cache.put(s.clone());
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
                        MaybeEncoded::Encoded(i) => cache.get_with_encoded_index(i),
                        MaybeEncoded::Decoded(s) => {
                            cache.put(s.clone());
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
                post_tag_1,
                post_tag_2,
                post_tag_3,
                post_tag_4
            };
            Record::Decoded(m)
        },
        _ => unimplemented!(),
    };
    *record = decoded;
}
