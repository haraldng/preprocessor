use serde::{Serialize, Deserialize};
use preprocessor::cache::unicache::{OmniCache, UniCache};
use omnipaxos_core::storage::Entry;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::{Debug, Formatter};

const THRESHOLD: usize = 500;
const SEPARATOR: char = '#';
const RULES: [&str; 2] = [
    r#"('\d+\\.*?')"#, // hash values
    r#"([^a-zA-Z'(,\*])\d+(\.\d+)?"#, // integers(prevent us from capturing table name like "a1")
];
lazy_static! {
    static ref REGEX_SETS: [Regex; 2] =
        [Regex::new(RULES[0]).unwrap(), Regex::new(RULES[1]).unwrap(),];
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum Template {
    Encoded(u8),
    Decoded(String)
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum Query {
    Encoded(Template, Vec<String>),
    Decoded(String),
    None
}

impl Default for Query {
    fn default() -> Self {
        Self::None
    }
}

impl Entry for Query {}

impl Query {
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        match self {
            Query::Encoded(t, params) => {
                params.iter().for_each(|x| size += x.len());
                match t {
                    Template::Encoded(i) => size += std::mem::size_of_val(i),
                    Template::Decoded(s) => size += s.len(),
                }
            },
            Query::Decoded(s) => size += s.len(),
            _ => unimplemented!(),
        }
        size
    }
}

pub struct BustrackerUniCache<U: UniCache>  {
    cache: U,
}

impl<U: UniCache> Debug for BustrackerUniCache<U> {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        Result::Ok(())
    }
}

impl<U: UniCache> OmniCache<Query, U> for BustrackerUniCache<U> {
    fn new(capacity: usize) -> Self {
        Self {
            cache: U::new(capacity)
        }
    }

    fn encode(&mut self, data: &mut Query) {
        if data.get_size() > THRESHOLD {
            return;
        }
        // split sql into template and parameters
        let query = std::mem::take(data);
        match query {
            Query::Decoded(s) => {
                let (template, parameters) = split_query(s);
                if let Some(index) = self.cache.get_encoded_index(&template) {
                    // exists in lib.cache
                    // send index and parameters
                    // lib.cache.update_cache(&template);
                    *data = Query::Encoded(Template::Encoded(index), parameters)
                } else {
                    // update lib.cache for leader
                    self.cache.put(template.clone());
                    *data = Query::Encoded(Template::Decoded(template), parameters)
                }
            },
            _ => unimplemented!(),
        }
    }

    fn decode(&mut self, data: &mut Query) {
        let query = std::mem::take(data);
        match query {
            Query::Encoded(t, parameters) => {
                let template = match t {
                    Template::Encoded(index) => {
                        self.cache.get_with_encoded_index(index)
                    },
                    Template::Decoded(template) => {
                        self.cache.put(template.clone());
                        template
                    }
                };
                let decoded = merge_query(template, parameters);
                *data = Query::Decoded(decoded);
            }
            _ => *data = query,
        }
    }
}

// Split a raw sql query into a template and parameters
// A query template contains only the operations but no values
// All parameters are connected as a string by comma
pub fn split_query(query: String) -> (String, Vec<String>) {
    let mut bitmap = [false; THRESHOLD];
    let mut indice_pairs = Vec::with_capacity(50);
    for re in REGEX_SETS.iter() {
        for (index, mat) in query.match_indices(re) {
            if bitmap[index] {
                continue;
            } else {
                for bitmap_entry in bitmap.iter_mut().skip(index).take(mat.len()) {
                    *bitmap_entry = true;
                }
            }
            indice_pairs.push((index, mat));
        }
    }
    indice_pairs.sort_by_key(|p| p.0);
    // println!("indice_pairs: {:?}", indice_pairs);

    // println!("template: {:?}", template);
    let parameters = indice_pairs
        .into_iter()
        .map(|p| p.1.to_string())
        .collect();
    let mut template = query;
    for re in REGEX_SETS.iter() {
        template = re.replace_all(&template, SEPARATOR.to_string()).to_string();
    }
    // println!("parameters: {:?}", parameters);

    (template, parameters)
}

// Merge template string with parameters
// There should be the exact number of parameters to fill in
pub fn merge_query(template: String, parameters: Vec<String>) -> String {
    if parameters.is_empty() {
        return template;
    }
    let num_parameters = parameters.len();
    let parts = template.split(SEPARATOR).collect::<Vec<_>>();
    /*
    assert_eq!(
        parts.len(),
        num_parameters + 1,
        "Unmatched templates {} \n and parameters {}",
        template,
        parameters
    );
    */
    let mut query = String::with_capacity(THRESHOLD);
    // let query = concat_string!("", parts, parameters);

    for (i, param) in parameters.iter().enumerate() {
        query.push_str(parts[i]);
        query.push_str(param);
    }
    query.push_str(parts[parameters.len()]);
    query
}