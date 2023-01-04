use crate::util::{EncodedHeader, MaybeEncoded, RawHeader, Header, MaybeProcessed};
use preprocessor::cache::unicache::{OmniCache, UniCache};
use std::fmt::{Debug, Formatter};

const MAX_THRESHOLD: usize = 700;
const MIN_THRESHOLD: usize = 3;
const NUM_MESSAGE_ID_ELEMENTS: usize = 3;

pub struct EmailUniCache<U: UniCache> {
    from_cache: U,
    to_cache: U,
    subject_cache: U,
    x_from_cache: U,
    x_to_cache: U,
    x_cc_cache: U,
    x_bcc_cache: U,
    x_folder_cache: U,
    x_origin_cache: U,
    x_filename_cache: U,
    message_id_cache: U,
    date_cache: U,
}

impl<U: UniCache> OmniCache<Header, U> for EmailUniCache<U> {
    fn new(capacity: usize) -> Self {
        Self {
            from_cache: U::new(capacity),
            to_cache: U::new(capacity),
            subject_cache: U::new(capacity),
            x_from_cache: U::new(capacity),
            x_to_cache: U::new(capacity),
            x_cc_cache: U::new(capacity),
            x_bcc_cache: U::new(capacity),
            x_folder_cache: U::new(capacity),
            x_origin_cache: U::new(capacity),
            x_filename_cache: U::new(capacity),
            message_id_cache: U::new(capacity),
            date_cache: U::new(capacity)
        }
    }

    fn encode(&mut self, data: &mut Header) {
        // split sql into template and parameters
        let rec = std::mem::take(data);
        match rec {
            Header::Decoded(me) => {
                let message_id = {
                    let mut splitted = me.message_id.splitn(NUM_MESSAGE_ID_ELEMENTS, '.');
                    let r1 = splitted.next().unwrap();
                    let r2 = splitted.next().unwrap();
                    let me = splitted.next().unwrap();
                    let e = Self::try_encode(me, &mut self.message_id_cache);
                    (r1.to_string(), r2.to_string(), e)
                };
                let date = Self::try_encode(&me.date, &mut self.date_cache);
                let from = Self::try_encode(&me.from, &mut self.from_cache);
                let to = Self::try_encode_vec(me.to, &mut self.to_cache);
                let subject = Self::try_encode_vec(me.subject, &mut self.subject_cache);
                let x_from = Self::try_encode(&me.x_from, &mut self.x_from_cache);
                let x_to = Self::try_encode_vec(me.x_to, &mut self.x_to_cache);
                let x_cc = Self::try_encode_vec(me.x_cc, &mut self.x_cc_cache);
                let x_bcc = Self::try_encode_vec(me.x_bcc, &mut self.x_bcc_cache);
                let x_folder = Self::try_encode(&me.x_folder, &mut self.x_folder_cache);
                let x_origin = Self::try_encode(&me.x_origin, &mut self.x_origin_cache);
                let x_filename = Self::try_encode(&me.x_filename, &mut self.x_filename_cache);
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
                *data = Header::Encoded(encoded);
            }
            _ => unimplemented!(),
        }
    }

    fn decode(&mut self, data: &mut Header) {
        let rec = std::mem::take(data);
        let decoded = match rec {
            Header::Encoded(e) => {
                let message_id = {
                    let (r1, r2, me) = e.message_id;
                    let decoded = Self::try_decode(me, &mut self.message_id_cache);
                    [r1, r2, decoded].join(".")
                };
                let date = Self::try_decode(e.date, &mut self.date_cache);
                let from = Self::try_decode(e.from, &mut self.from_cache);
                let to = Self::try_decode_vec(e.to, &mut self.to_cache, " ");
                let subject = Self::try_decode_vec(e.subject, &mut self.subject_cache, " ");
                let x_from = Self::try_decode(e.x_from, &mut self.x_from_cache);
                let x_to = Self::try_decode_vec(e.x_to, &mut self.x_to_cache, " ");
                let x_cc = Self::try_decode_vec(e.x_cc, &mut self.x_cc_cache, " ");
                let x_bcc = Self::try_decode_vec(e.x_bcc, &mut self.x_bcc_cache, " ");
                let x_folder = Self::try_decode(e.x_folder, &mut self.x_folder_cache);
                let x_origin = Self::try_decode(e.x_origin, &mut self.x_origin_cache);
                let x_filename = Self::try_decode(e.x_filename, &mut self.x_filename_cache);

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
                Header::Decoded(m)
            }
            _ => unimplemented!(),
        };
        *data = decoded;
    }
}

impl<U: UniCache> EmailUniCache<U> {

    fn try_encode(s: &str, cache: &mut U) -> MaybeEncoded {
        if s.len() > MIN_THRESHOLD {
            match cache.get_encoded_index(s) {
                Some(i) => MaybeEncoded::Encoded(i),
                None => {
                    let s = s.to_string();
                    cache.put(s.clone());
                    MaybeEncoded::Decoded(s)
                }
            }
        } else {
            MaybeEncoded::Decoded(s.to_string())
        }
    }

    fn try_encode_vec(s: String, cache: &mut U) -> MaybeProcessed{
        if s.len() > MAX_THRESHOLD {
            MaybeProcessed::NotProcessed(s)
        } else {
            let p = s.split_whitespace()
                .map(|x| Self::try_encode(x, cache))
                .collect();
            MaybeProcessed::Processed(p)
        }

    }

    fn try_decode(x: MaybeEncoded, cache: &mut U) -> String {
        match x {
            MaybeEncoded::Encoded(i) => cache.get_with_encoded_index(i),
            MaybeEncoded::Decoded(s) => {
                if s.len() > MIN_THRESHOLD {
                    cache.put(s.clone());
                }
                s
            }
        }
    }

    fn try_decode_vec(
        v: MaybeProcessed,
        cache: &mut U,
        join_on: &str,
    ) -> String {
        match v {
            MaybeProcessed::Processed(p) => {
                p.into_iter()
                    .map(|x| Self::try_decode(x, cache))
                    .collect::<Vec<String>>()
                    .join(join_on)
            },
            MaybeProcessed::NotProcessed(s) => s
        }
    }
}

impl<U: UniCache> Debug for EmailUniCache<U> {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        Result::Ok(())
    }
}
