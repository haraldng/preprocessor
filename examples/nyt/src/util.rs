use crate::preprocess::MaybeEncodedURL;
use lecar::cache::Cache;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct RawArticle {
    pub web_url: String,
    // pub keywords: String,
    pub pub_date: String,
    pub document_type: String,
    pub news_desk: String,
    pub section_name: String,
    pub type_of_material: String,
    pub main_headline: String,
    pub print_headline: String,
    pub by: String,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct EncodedArticle {
    pub(crate) web_url: MaybeEncodedURL,
    // pub(crate) keywords: String,
    pub(crate) pub_date: (MaybeEncoded, String, MaybeEncoded),
    pub(crate) document_type: MaybeEncoded,
    pub(crate) news_desk: MaybeEncoded,
    pub(crate) section_name: MaybeEncoded,
    pub(crate) type_of_material: MaybeEncoded,
    pub(crate) main_headline: Vec<MaybeEncoded>,
    pub(crate) print_headline: Vec<MaybeEncoded>,
    pub(crate) by: Vec<MaybeEncoded>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub enum Record {
    Decoded(RawArticle),
    Encoded(EncodedArticle),
    None,
}

impl Record {
    pub(crate) fn get_size(&self) -> usize {
        let mut size = 0;
        match self {
            Record::Encoded(e) => {
                size += e.web_url.get_size();
                size += e.pub_date.0.get_size();
                size += e.pub_date.1.len();
                size += e.pub_date.2.get_size();
                size += e.document_type.get_size();
                size += e.news_desk.get_size();
                size += e.section_name.get_size();
                size += e.type_of_material.get_size();
                e.main_headline.iter().for_each(|x| size += x.get_size());
                e.print_headline.iter().for_each(|x| size += x.get_size());
                e.by.iter().for_each(|x| size += x.get_size());
            }
            Record::Decoded(d) => {
                size += d.web_url.len();
                // size += d.keywords.len();
                size += d.pub_date.len();
                size += d.document_type.len();
                size += d.news_desk.len();
                size += d.section_name.len();
                size += d.type_of_material.len();
                size += d.main_headline.len();
                size += d.print_headline.len();
                size += d.by.len();
            }
            Record::None => {
                unimplemented!()
            }
        }
        size
    }
}

impl Default for Record {
    fn default() -> Self {
        Record::None
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub enum MaybeEncoded {
    Encoded(usize),
    Decoded(String),
}

impl MaybeEncoded {
    pub(crate) fn get_size(&self) -> usize {
        match self {
            // MaybeEncoded::Encoded(i) => std::mem::size_of_val(i),
            MaybeEncoded::Encoded(_) => 1,
            MaybeEncoded::Decoded(s) => s.len(),
        }
    }
}
