use omnipaxos_core::storage::Entry;
use crate::preprocess::MaybeEncodedURL;
use serde::{Serialize, Deserialize};
use preprocessor::util::{MaybeEncoded, MaybeProcessed};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct EncodedArticle {
    pub(crate) web_url: MaybeEncodedURL,
    // pub(crate) keywords: String,
    pub(crate) pub_date: (MaybeEncoded, String, MaybeEncoded),
    pub(crate) document_type: MaybeEncoded,
    pub(crate) news_desk: MaybeEncoded,
    pub(crate) section_name: MaybeEncoded,
    pub(crate) type_of_material: MaybeEncoded,
    pub(crate) main_headline: MaybeProcessed,
    pub(crate) print_headline: Option<MaybeProcessed>,
    pub(crate) by: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Article {
    Decoded(RawArticle),
    Encoded(EncodedArticle),
    None,
}

impl Article {
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        match self {
            Article::Encoded(e) => {
                size += e.web_url.get_size();
                size += e.pub_date.0.get_size();
                size += e.pub_date.1.len();
                size += e.pub_date.2.get_size();
                size += e.document_type.get_size();
                size += e.news_desk.get_size();
                size += e.section_name.get_size();
                size += e.type_of_material.get_size();
                size += e.main_headline.get_size();
                size += match &e.print_headline {
                    Some(p) => p.get_size(),
                    None => 0,
                };
                size += e.by.len();
            }
            Article::Decoded(d) => {
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
            Article::None => {
                unimplemented!()
            }
        }
        size
    }
}

impl Default for Article {
    fn default() -> Self {
        Article::None
    }
}

impl Entry for Article {}