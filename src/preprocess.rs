use crate::{cache, load, preprocess};
use lazy_static::lazy_static;
use regex::Regex;

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

pub fn encode(command: &mut load::StoreCommand, cache: &mut cache::CacheModel) {
    // split sql into template and parameters
    let (template, parameters) = preprocess::split_query(&command.sql);

    if let Some(index) = cache.get_index_of(&template) {
        // exists in cache
        // send index and parameters
        let compressed = format!("1*|*{}*|*{}", index, parameters);
        command.sql = compressed;
        cache.update_cache(&template);
    } else {
        // send template and parameters
        let uncompressed = format!("0*|*{}*|*{}", template.clone(), parameters);
        command.sql = uncompressed;
        // update cache for leader
        cache.put(template);
    }
}

pub fn decode(command: &mut load::StoreCommand, cache: &mut cache::CacheModel) {
    let parts: Vec<&str> = command.sql.split("*|*").collect();
    if parts.len() != 3 {
        panic!("Unexpected query: {:?}", command.sql);
    }

    let (compressed, index_or_template, parameters) = (parts[0], parts[1], parts[2]);

    let template = if compressed == "1" {
        // compressed messsage
        let index = index_or_template.parse::<usize>().unwrap();
        let template = cache
            .get_with_index(index)
            .unwrap_or_else(|| {
                panic!(
                    "Could not template from index: {}, query: {:?}, cache size: {}",
                    index,
                    command.sql,
                    cache.len()
                )
            })
            .0
            .clone();
        cache.update_cache(&template);
        template
    } else {
        let template: cache::CacheKey = index_or_template.to_string();
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
        template = re.replace_all(&template, "@").to_string();
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

    let parts = template.split('@').collect::<Vec<_>>();
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
