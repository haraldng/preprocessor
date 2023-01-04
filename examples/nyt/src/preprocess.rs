use crate::util::{Article, EncodedArticle, RawArticle};
use lazy_static::lazy_static;
use preprocessor::cache::unicache::{OmniCache, UniCache};
use regex::Regex;
use serde::{Serialize, Deserialize};
use preprocessor::util::{MaybeEncoded, MaybeProcessed};
use std::fmt::{Formatter, Debug};

const MAX_THRESHOLD: usize = 40;
const MIN_THRESHOLD: usize = 3;

const URL_BASE: &str = "https://www.nytimes.com/";
const DATE_SPLIT: usize = 11;
const NAME_SEP: &str = "-";
const PATH_SEP: &str = "/";

lazy_static! {
    static ref RE: Regex = Regex::new(r"^\d{4}/\d{2}/\d{2}/").unwrap();
}

pub struct NytUniCache<U: UniCache> {
    url_date_cache: U,
    url_path_cache: U,
    url_name_cache: U,
    pub_date_cache: U,
    doc_type_cache: U,
    news_desk_cache: U,
    section_cache: U,
    material_cache: U,
    main_headline_cache: U,
    print_headline_cache: U,
    by_cache: U,
}

impl<U: UniCache> OmniCache<Article, U> for NytUniCache<U> {
    fn new(capacity: usize) -> Self {
        Self {
            url_date_cache: U::new(capacity),
            url_path_cache: U::new(capacity),
            url_name_cache: U::new(capacity),
            pub_date_cache: U::new(capacity),
            doc_type_cache: U::new(capacity),
            news_desk_cache: U::new(capacity),
            section_cache: U::new(capacity),
            material_cache: U::new(capacity),
            main_headline_cache: U::new(capacity),
            print_headline_cache: U::new(capacity),
            by_cache: U::new(capacity)
        }
    }

    fn encode(&mut self, data: &mut Article) {
        let rec = std::mem::take(data);
        match rec {
            Article::Decoded(me) => {
                let web_url = Self::try_encode_url(me.web_url, &mut self.url_date_cache, &mut self.url_path_cache, &mut self.url_name_cache);
                let pub_date = {
                    let (date, time) = me
                        .pub_date
                        .split_once('T')
                        .unwrap_or_else(|| panic!("No T: {}", me.pub_date));
                    let (time, zone) = time.split_once('+').unwrap();
                    let enc_date = Self::try_encode(date, &mut self.pub_date_cache);
                    let enc_zone = Self::try_encode(zone, &mut self.pub_date_cache);
                    (enc_date, time.to_string(), enc_zone)
                };
                let document_type = Self::try_encode(&me.document_type, &mut self.doc_type_cache);
                let news_desk = Self::try_encode(&me.news_desk, &mut self.news_desk_cache);
                let section_name = Self::try_encode(&me.section_name, &mut self.section_cache);
                let type_of_material = Self::try_encode(&me.type_of_material, &mut self.material_cache);
                let main_headline = Self::try_encode_vec(me.main_headline, &mut self.main_headline_cache);
                let print_headline = Self::try_encode_vec(me.print_headline, &mut self.print_headline_cache);
                let by = Self::try_encode_vec(me.by, &mut self.by_cache);
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
                *data = Article::Encoded(encoded);
            }
            _ => unimplemented!(),
        }
    }

    fn decode(&mut self, data: &mut Article) {
        let rec = std::mem::take(data);
        let decoded = match rec {
            Article::Encoded(me) => {
                let web_url = match me.web_url {
                    MaybeEncodedURL::Encoded(e) => Self::decode_url(e, &mut self.url_date_cache, &mut self.url_path_cache, &mut self.url_name_cache),
                    MaybeEncodedURL::Decoded(s) => s,
                };
                let pub_date = {
                    let (enc_date, time, enc_zone) = me.pub_date;
                    let date = Self::try_decode(enc_date, &mut self.pub_date_cache);
                    let zone = Self::try_decode(enc_zone, &mut self.pub_date_cache);
                    format!("{}T{}+{}", date, time, zone)
                };
                let document_type = Self::try_decode(me.document_type, &mut self.doc_type_cache);
                let news_desk = Self::try_decode(me.news_desk, &mut self.news_desk_cache);
                let section_name = Self::try_decode(me.section_name, &mut self.section_cache);
                let type_of_material = Self::try_decode(me.type_of_material, &mut self.material_cache);
                let main_headline = Self::try_decode_vec(me.main_headline, &mut self.main_headline_cache, " ");
                let print_headline = Self::try_decode_vec(me.print_headline, &mut self.print_headline_cache, " ");
                let by = Self::try_decode_vec(me.by, &mut self.by_cache, " ");

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
                Article::Decoded(m)
            }
            _ => unimplemented!(),
        };
        *data = decoded;
    }
}

impl<U: UniCache> NytUniCache<U> {

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

    fn try_encode_vec_with(
        s: String,
        cache: &mut U,
        sep: &str,
    ) -> MaybeProcessed {
        if s.len() > MAX_THRESHOLD {
            MaybeProcessed::NotProcessed(s)
        } else {
            let p = s.split(sep)
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

    fn try_encode_url(s: String, url_date_cache: &mut U, url_path_cache: &mut U, url_name_cache: &mut U) -> MaybeEncodedURL {
        match s.strip_prefix(URL_BASE) {
            Some(rest) => {
                if RE.is_match(rest) {
                    let (date, path_name) = rest.split_at(DATE_SPLIT);
                    let enc_date = Self::try_encode(date, url_date_cache);
                    let (enc_path, enc_name) = match path_name.rsplit_once('/') {
                        Some((path, name)) => {
                            let enc_path = Self::try_encode_vec_with(
                                path.to_string(),
                                url_path_cache,
                                PATH_SEP,
                            );
                            let enc_name = Self::try_encode_vec_with(
                                name.to_string(),
                                url_name_cache,
                                NAME_SEP,
                            );
                            (Some(enc_path), enc_name)
                        }
                        None => {
                            let enc_name = Self::try_encode_vec_with(
                                path_name.to_string(),
                                url_name_cache,
                                NAME_SEP,
                            );
                            (None, enc_name)
                        }
                    };

                    let e = EncodedURL {
                        date: enc_date,
                        path: enc_path,
                        name: enc_name,
                    };
                    MaybeEncodedURL::Encoded(e)
                } else {
                    MaybeEncodedURL::Decoded(s)
                }
            }
            None => MaybeEncodedURL::Decoded(s),
        }
    }

    fn decode_url(enc_url: EncodedURL, url_date_cache: &mut U, url_path_cache: &mut U, url_name_cache: &mut U) -> String {
        let year = Self::try_decode(enc_url.date, url_date_cache);
        let name = Self::try_decode_vec(enc_url.name, url_name_cache, NAME_SEP);
        match enc_url.path {
            Some(mp) => {
                let path = Self::try_decode_vec(mp, url_path_cache, PATH_SEP);
                format!("{}{}{}/{}", URL_BASE, year, path, name)
            },
            None => format!("{}{}{}", URL_BASE, year, name),
        }
        // format!("https://www.nytimes.com/2019/12/31/us/texas-church-shooting-white-settlement.html")
    }

}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum MaybeEncodedURL {
    Encoded(EncodedURL),
    Decoded(String),
}

impl MaybeEncodedURL {

    pub(crate) fn get_size(&self) -> usize {
        match self {
            MaybeEncodedURL::Encoded(e) => {
                let mut size = 0;
                size += e.date.get_size();
                size += e.name.get_size();
                if let Some(mp) = e.path.as_ref() {
                    size += mp.get_size();
                }
                size
            }
            MaybeEncodedURL::Decoded(s) => s.len(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct EncodedURL {
    date: MaybeEncoded,
    path: Option<MaybeProcessed>,
    name: MaybeProcessed,
}

impl<U: UniCache> Debug for NytUniCache<U> {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        Result::Ok(())
    }
}

#[test]
fn regex_test() {
    let s = "https://www.nytimes.com/2019/12/31/us/texas-church-shooting-white-settlement.html";
    let rest = s.strip_prefix(URL_BASE).unwrap();
    let r = Regex::new(r"^\d{4}/\d{2}/\d{2}/").unwrap();

    assert!(r.is_match(rest));
    println!("{:?}", rest.split_at(11));
}
