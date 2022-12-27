use crate::{load, preprocess, CacheKey, UniCache};
use lazy_static::lazy_static;
use regex::Regex;

const SEPARATOR: char = '#';

const RULES: [&str; 2] = [
    r#"('\d+\\.*?')"#, // hash values
    //r#"'((')|(.*?([^\\])'))"#,        // string
    //r#""((")|(.*?([^\\])"))"#,        // double-quoted string
    r#"([^a-zA-Z'(,\*])\d+(\.\d+)?"#, // integers(prevent us from capturing table name like "a1")
];

lazy_static! {
    static ref REGEX_SETS: [Regex; 2] =
        [Regex::new(RULES[0]).unwrap(), Regex::new(RULES[1]).unwrap(),];
}

pub fn encode<U: UniCache<CacheKey>>(
    command: &mut load::StoreCommand,
    cache: &mut U,
) -> (bool, usize) {
    // split sql into template and parameters
    let (template, parameters) = preprocess::split_query(&command.sql);
    let raw_length = command.sql.len() as f32;

    if let Some(index) = cache.get_encoded_index(&template) {
        // exists in cache
        // send index and parameters
        let compressed = format!("1*|*{}*|*{}", index, parameters);
        let compressed_length = compressed.len() as f32;
        command.sql = compressed;
        // cache.update_cache(&template);

        (
            true,
            (100f32 * (1f32 - compressed_length / raw_length)) as usize,
        )
    } else {
        // send template and parameters
        let uncompressed = format!("0*|*{}*|*{}", template.clone(), parameters);
        // let uncompressed_length = uncompressed.len();
        command.sql = uncompressed;
        // update cache for leader
        cache.put(template);

        (false, 0)
    }
}

pub fn decode<U: UniCache<CacheKey>>(command: &mut load::StoreCommand, cache: &mut U) {
    let parts: Vec<&str> = command.sql.split("*|*").collect();
    if parts.len() != 3 {
        panic!("Unexpected query: {:?}", command.sql);
    }

    let (compressed, index_or_template, parameters) = (parts[0], parts[1], parts[2]);

    let template = if compressed == "1" {
        // compressed messsage
        let index = index_or_template.parse::<usize>().unwrap();
        cache.get_with_encoded_index(index)
    } else {
        let template: CacheKey = index_or_template.to_string();
        cache.put(template.clone());
        template
    };
    command.sql = preprocess::merge_query(&template, parameters);
}

// Split a raw sql query into a template and parameters
// A query template contains only the operations but no values
// All parameters are connected as a string by comma
pub fn split_query(query: &str) -> (String, String) {
    let mut bitmap: Vec<bool> = vec![false; query.len()];
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
    let mut template = query.to_string();
    for re in REGEX_SETS.iter() {
        template = re.replace_all(&template, SEPARATOR.to_string()).to_string();
    }

    indice_pairs.sort_by_key(|p| p.0);
    // println!("indice_pairs: {:?}", indice_pairs);

    // println!("template: {:?}", template);
    let parameters = indice_pairs
        .iter()
        .map(|p| p.1)
        .collect::<Vec<_>>()
        .join(",");
    // println!("parameters: {:?}", parameters);

    (template, parameters)
}

// Merge template string with parameters
// There should be the exact number of parameters to fill in
pub fn merge_query(template: &str, parameters: &str) -> String {
    if parameters.is_empty() {
        return template.to_string();
    }

    let parameter_list = parameters.split(',').collect::<Vec<_>>();
    let num_parameters = parameter_list.len();

    let parts = template.split(SEPARATOR).collect::<Vec<_>>();
    assert_eq!(
        parts.len(),
        num_parameters + 1,
        "Unmatched templates {} \n and parameters {}",
        template,
        parameters
    );

    let mut query = String::with_capacity(template.len() + parameters.len());
    for i in 0..num_parameters {
        query.push_str(parts[i]);
        query.push_str(parameter_list[i]);
    }
    query.push_str(parts[num_parameters]);

    query
}
