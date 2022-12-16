mod load;
mod cache;
mod preprocess;

use std::time::Instant;

fn encode(command: &mut load::StoreCommand, cache: &mut cache::CacheModel) {
    // split sql into template and parameters
    let (template, parameters) = preprocess::split_query(&command.sql);
    let cache_key: cache::CacheKey = template.clone();
    let cache_value: cache::CacheValue = template.clone();

    if let Some(index) = cache.get_index_of(cache_key.clone()) {
        // exists in cache
        // send index and parameters
        let compressed = format!("1*|*{}*|*{}", index.to_string(), parameters);

        command.sql = compressed;
    } else {
        // send template and parameters
        let uncompressed = format!("0*|*{}*|*{}", template, parameters);

        command.sql = uncompressed;
    }

    // update cache for leader
    cache.put(cache_key, cache_value);
}

fn decode(command: &mut load::StoreCommand, cache: &mut cache::CacheModel) {
    let parts: Vec<&str> = command.sql.split("*|*").collect();
    if parts.len() != 3 { 
        panic!("Unexpected query: {:?}", command.sql);
    }

    let (compressed, index_or_template, parameters) = (parts[0], parts[1].to_string(), parts[2].to_string());
    let mut template = index_or_template.clone();

    if compressed == "1" {
        // compressed messsage
        let index = index_or_template.parse::<usize>().unwrap();
        if let Some((_key, value)) = cache.get_with_index(index) {
            template = value.clone();
        } else { 
            let index = index;
            let sql = command.sql.clone();
            let size = cache.len();

            panic!("Query:{} is out of index: {}/{:?}", sql, index, size);
        }
    }

    // update cache for followers
    let cache_key: cache::CacheKey = template.clone();
    let cache_value: cache::CacheValue = template.clone();
    cache.put(cache_key, cache_value);
    command.sql = preprocess::merge_query(template, parameters);
}

fn main() {
    let mut commands = load::read_from_file("queries.txt");
    let mut cache = cache::CacheModel::with(500, true);
    let command_num = commands.len() as u32;
    println!("find {} commands.", command_num);

    let now = Instant::now();
    /// run with checks
    for command in commands.into_iter() {
        let mut raw_command = command.clone();
        encode(&mut raw_command, &mut cache);
        decode(&mut raw_command, &mut cache);

        // this adds a small overhead for benchmark
        // but it allows us to double check the correctness of encode/decode procedure
        assert!(raw_command.sql == command.sql);
    }

    let elapsed = now.elapsed();
    println!("We handled {} commands in {:?}.", command_num, elapsed);
    println!("On average, one command takes {:?}.", elapsed/command_num);
    // println!("On average, one command takes {:.2} microsecs.", average_time);
}