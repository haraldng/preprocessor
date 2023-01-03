use crate::util::{MaybeEncoded, Record};
use crate::{EncodedArticle, RawArticle};
use lazy_static::lazy_static;
use preprocessor::cache::unicache::UniCache;
use regex::Regex;
use serde::Deserialize;

pub const URL_DATE: usize = 0;
pub const URL_PATH: usize = 1;
pub const URL_NAME: usize = 2;
pub const PUB_DATE: usize = 3;
pub const DOCUMENT_TYPE: usize = 4;
pub const NEWS_DESK: usize = 5;
pub const SECTION_NAME: usize = 6;
pub const TYPE_OF_MATERIAL: usize = 7;
pub const MAIN_HEADLINE: usize = 8;
pub const PRINT_HEADLINE: usize = 9;
pub const BY: usize = 10;

pub const NUM_CACHES: usize = BY + 1;
const THRESHOLD: usize = 3;

const URL_BASE: &str = "https://www.nytimes.com/";
const DATE_SPLIT: usize = 11;
const NAME_SEP: &str = "-";
const PATH_SEP: &str = "/";

pub fn encode<U: UniCache<String>>(
    record: &mut Record,
    cache: &mut [U; NUM_CACHES],
) -> (bool, usize) {
    // split sql into template and parameters
    let rec = std::mem::take(record);
    let raw_size = rec.get_size() as f32;
    match rec {
        Record::Decoded(me) => {
            let web_url = MaybeEncodedURL::try_encode(me.web_url, cache);
            let pub_date = {
                let (date, time) = me
                    .pub_date
                    .split_once('T')
                    .unwrap_or_else(|| panic!("No T: {}", me.pub_date));
                let (time, zone) = time.split_once('+').unwrap();
                let enc_date = try_encode(date.to_string(), &mut cache[PUB_DATE]);
                let enc_zone = try_encode(zone.to_string(), &mut cache[PUB_DATE]);
                (enc_date, time.to_string(), enc_zone)
            };
            let document_type = try_encode(me.document_type, &mut cache[DOCUMENT_TYPE]);
            let news_desk = try_encode(me.news_desk, &mut cache[NEWS_DESK]);
            let section_name = try_encode(me.section_name, &mut cache[SECTION_NAME]);
            let type_of_material = try_encode(me.type_of_material, &mut cache[TYPE_OF_MATERIAL]);
            let main_headline = try_encode_vec(me.main_headline, &mut cache[MAIN_HEADLINE]);
            let print_headline = try_encode_vec(me.print_headline, &mut cache[PRINT_HEADLINE]);
            let by = try_encode_vec(me.by, &mut cache[BY]);
            let encoded = EncodedArticle {
                web_url,
                // keywords: me.keywords,
                pub_date,
                document_type,
                news_desk,
                section_name,
                type_of_material,
                main_headline,
                print_headline,
                by,
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
    if s.len() > THRESHOLD {
        match cache.get_encoded_index(&s) {
            Some(i) => MaybeEncoded::Encoded(i),
            None => {
                cache.put(s.clone());
                MaybeEncoded::Decoded(s)
            }
        }
    } else {
        MaybeEncoded::Decoded(s)
    }
}

fn try_encode_vec<U: UniCache<String>>(s: String, cache: &mut U) -> Vec<MaybeEncoded> {
    s.split_whitespace()
        .map(|x| try_encode(x.to_string(), cache))
        .collect()
}

fn try_encode_vec_with<U: UniCache<String>>(
    s: String,
    cache: &mut U,
    sep: &str,
) -> Vec<MaybeEncoded> {
    s.split(sep)
        .map(|x| try_encode(x.to_string(), cache))
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
        Record::Encoded(me) => {
            let web_url = match me.web_url {
                MaybeEncodedURL::Encoded(e) => decode_url(e, cache),
                MaybeEncodedURL::Decoded(s) => s,
            };
            let pub_date = {
                let (enc_date, time, enc_zone) = me.pub_date;
                let date = try_decode(enc_date, &mut cache[PUB_DATE]);
                let zone = try_decode(enc_zone, &mut cache[PUB_DATE]);
                format!("{}T{}+{}", date, time, zone)
            };
            let document_type = try_decode(me.document_type, &mut cache[DOCUMENT_TYPE]);
            let news_desk = try_decode(me.news_desk, &mut cache[NEWS_DESK]);
            let section_name = try_decode(me.section_name, &mut cache[SECTION_NAME]);
            let type_of_material = try_decode(me.type_of_material, &mut cache[TYPE_OF_MATERIAL]);
            let main_headline = try_decode_vec(me.main_headline, &mut cache[MAIN_HEADLINE], " ");
            let print_headline = try_decode_vec(me.print_headline, &mut cache[PRINT_HEADLINE], " ");
            let by = try_decode_vec(me.by, &mut cache[BY], " ");

            let m = RawArticle {
                web_url,
                // keywords: me.keywords,
                pub_date,
                document_type,
                news_desk,
                section_name,
                type_of_material,
                main_headline,
                print_headline,
                by,
            };
            Record::Decoded(m)
        }
        _ => unimplemented!(),
    };
    *record = decoded;
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub enum MaybeEncodedURL {
    Encoded(EncodedURL),
    Decoded(String),
}

impl MaybeEncodedURL {
    fn try_encode<U: UniCache<String>>(s: String, cache: &mut [U; NUM_CACHES]) -> Self {
        match s.strip_prefix(URL_BASE) {
            Some(rest) => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"^\d{4}/\d{2}/\d{2}/").unwrap();
                }
                if RE.is_match(rest) {
                    let (date, path_name) = rest.split_at(DATE_SPLIT);
                    let enc_date = try_encode(date.to_string(), &mut cache[URL_DATE]);
                    let (enc_path, enc_name) = match path_name.rsplit_once('/') {
                        Some((path, name)) => {
                            let enc_path = try_encode_vec_with(
                                path.to_string(),
                                &mut cache[URL_PATH],
                                PATH_SEP,
                            );
                            let enc_name = try_encode_vec_with(
                                name.to_string(),
                                &mut cache[URL_NAME],
                                NAME_SEP,
                            );
                            (enc_path, enc_name)
                        }
                        None => {
                            let enc_name = try_encode_vec_with(
                                path_name.to_string(),
                                &mut cache[URL_NAME],
                                NAME_SEP,
                            );
                            (vec![], enc_name)
                        }
                    };

                    let e = EncodedURL {
                        date: enc_date,
                        path: enc_path,
                        name: enc_name,
                    };
                    Self::Encoded(e)
                } else {
                    Self::Decoded(s)
                }
            }
            None => Self::Decoded(s),
        }
    }

    pub(crate) fn get_size(&self) -> usize {
        match self {
            MaybeEncodedURL::Encoded(e) => {
                let mut size = 0;
                size += e.date.get_size();
                e.path.iter().for_each(|x| size += x.get_size());
                e.name.iter().for_each(|x| size += x.get_size());
                size
            }
            MaybeEncodedURL::Decoded(s) => s.len(),
        }
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct EncodedURL {
    date: MaybeEncoded,
    path: Vec<MaybeEncoded>,
    name: Vec<MaybeEncoded>,
}

fn decode_url<U: UniCache<String>>(enc_url: EncodedURL, cache: &mut [U; NUM_CACHES]) -> String {
    let year = try_decode(enc_url.date, &mut cache[URL_DATE]);
    let name = try_decode_vec(enc_url.name, &mut cache[URL_NAME], NAME_SEP);
    if !enc_url.path.is_empty() {
        let path = try_decode_vec(enc_url.path, &mut cache[URL_PATH], PATH_SEP);
        format!("{}{}{}/{}", URL_BASE, year, path, name)
    } else {
        format!("{}{}{}", URL_BASE, year, name)
    }
    // format!("https://www.nytimes.com/2019/12/31/us/texas-church-shooting-white-settlement.html")
}

#[test]
fn regex_test() {
    let s = "https://www.nytimes.com/2019/12/31/us/texas-church-shooting-white-settlement.html";
    let rest = s.strip_prefix(URL_BASE).unwrap();
    let r = Regex::new(r"^\d{4}/\d{2}/\d{2}/").unwrap();

    assert!(r.is_match(rest));
    println!("{:?}", rest.split_at(11));
}
