use crate::util::{EncodedHeader, MaybeEncoded, Record};
use crate::RawHeader;
use preprocessor::cache::unicache::UniCache;

pub const FROM: usize = 0;
pub const TO: usize = 1;
pub const SUBJECT: usize = 2;
pub const X_FROM: usize = 3;
pub const X_TO: usize = 4;
pub const X_CC: usize = 5;
pub const X_BCC: usize = 6;
pub const X_FOLDER: usize = 7;
pub const X_ORIGIN: usize = 8;
pub const X_FILENAME: usize = 9;
pub const MESSAGE_ID: usize = 10;
pub const DATE: usize = 11;

pub const NUM_CACHES: usize = DATE + 1;
const THRESHOLD: usize = 3;
const NUM_MESSAGE_ID_ELEMENTS: usize = 3;

pub fn encode<U: UniCache<String>>(
    record: &mut Record,
    cache: &mut [U; NUM_CACHES],
) -> (bool, usize) {
    // split sql into template and parameters
    let rec = std::mem::take(record);
    let raw_size = rec.get_size() as f32;
    match rec {
        Record::Decoded(me) => {
            let message_id = {
                let splitted = me.message_id.splitn(NUM_MESSAGE_ID_ELEMENTS, '.');
                splitted
                    .enumerate()
                    .map(|(i, x)| {
                        let x = x.to_string();
                        if i == NUM_MESSAGE_ID_ELEMENTS - 1 {
                            try_encode(x, &mut cache[MESSAGE_ID])
                        } else {
                            MaybeEncoded::Decoded(x)
                        }
                    })
                    .collect()
            };
            let date = try_encode(me.date, &mut cache[DATE]);
            let from = try_encode(me.from, &mut cache[FROM]);
            let to = try_encode_vec(me.to, &mut cache[TO]);
            let subject = try_encode_vec(me.subject, &mut cache[SUBJECT]);
            let x_from = try_encode(me.x_from, &mut cache[X_FROM]);
            let x_to = try_encode_vec(me.x_to, &mut cache[X_TO]);
            let x_cc = try_encode_vec(me.x_cc, &mut cache[X_CC]);
            let x_bcc = try_encode_vec(me.x_bcc, &mut cache[X_BCC]);
            let x_folder = try_encode(me.x_folder, &mut cache[X_FOLDER]);
            let x_origin = try_encode(me.x_origin, &mut cache[X_ORIGIN]);
            let x_filename = try_encode(me.x_filename, &mut cache[X_FILENAME]);
            let encoded = EncodedHeader {
                message_id,
                date,
                from,
                to,
                subject,
                x_from,
                x_to,
                x_cc,
                x_bcc,
                x_folder,
                x_origin,
                x_filename,
            };
            // println!("ENCODED: {:?}", encoded);
            *record = Record::Encoded(encoded);
        }
        _ => unimplemented!(),
    }
    let compressed_size = record.get_size() as f32;
    (
        false,
        (100f32 * (1f32 - compressed_size / raw_size)) as usize,
    )
}

fn try_encode<U: UniCache<String>>(s: String, cache: &mut U) -> MaybeEncoded {
    match cache.get_encoded_index(&s) {
        Some(i) => MaybeEncoded::Encoded(i),
        None => {
            if s.len() > THRESHOLD {
                cache.put(s.clone());
            }
            MaybeEncoded::Decoded(s)
        }
    }
}

fn try_encode_vec<U: UniCache<String>>(s: String, cache: &mut U) -> Vec<MaybeEncoded> {
    s.split_whitespace()
        .map(|x| try_encode(x.into(), cache))
        .collect()
}

fn try_decode<U: UniCache<String>>(x: MaybeEncoded, cache: &mut U) -> String {
    match x {
        MaybeEncoded::Encoded(i) => cache.get_with_encoded_index(i),
        MaybeEncoded::Decoded(s) => {
            if s.len() > THRESHOLD {
                cache.put(s.clone());
            }
            s
        }
    }
}

fn try_decode_vec<U: UniCache<String>>(
    v: Vec<MaybeEncoded>,
    cache: &mut U,
    join_on: &str,
) -> String {
    v.into_iter()
        .map(|x| try_decode(x, cache))
        .collect::<Vec<String>>()
        .join(join_on)
}

pub fn decode<U: UniCache<String>>(record: &mut Record, cache: &mut [U; NUM_CACHES]) {
    let rec = std::mem::take(record);
    let decoded = match rec {
        Record::Encoded(e) => {
            let message_id = {
                e.message_id
                    .into_iter()
                    .enumerate()
                    .map(|(i, x)| {
                        if i == NUM_MESSAGE_ID_ELEMENTS - 1 {
                            try_decode(x, &mut cache[MESSAGE_ID])
                        } else {
                            match x {
                                MaybeEncoded::Decoded(d) => d,
                                _ => unimplemented!(),
                            }
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(".")
            };
            let date = try_decode(e.date, &mut cache[DATE]);
            let from = try_decode(e.from, &mut cache[FROM]);
            let to = try_decode_vec(e.to, &mut cache[TO], " ");
            let subject = try_decode_vec(e.subject, &mut cache[SUBJECT], " ");
            let x_from = try_decode(e.x_from, &mut cache[X_FROM]);
            let x_to = try_decode_vec(e.x_to, &mut cache[X_TO], " ");
            let x_cc = try_decode_vec(e.x_cc, &mut cache[X_CC], " ");
            let x_bcc = try_decode_vec(e.x_bcc, &mut cache[X_BCC], " ");
            let x_folder = try_decode(e.x_folder, &mut cache[X_FOLDER]);
            let x_origin = try_decode(e.x_origin, &mut cache[X_ORIGIN]);
            let x_filename = try_decode(e.x_filename, &mut cache[X_FILENAME]);

            let m = RawHeader {
                message_id,
                date,
                from,
                to,
                subject,
                x_from,
                x_to,
                x_cc,
                x_bcc,
                x_folder,
                x_origin,
                x_filename,
            };
            Record::Decoded(m)
        }
        _ => unimplemented!(),
    };
    *record = decoded;
}
