/*
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
*/

#[derive(Debug, Clone)]
pub struct StoreCommand {
    pub id: usize,
    pub sql: String,
}
/*
impl StoreCommand {
    pub fn new(id: usize, sql: String) -> StoreCommand {
        StoreCommand { id: id, sql }
    }
}

pub fn read_from_file(filepath: &str) -> Vec<StoreCommand> {
    if let Ok(lines) = read_lines(filepath) {
        lines
            .into_iter()
            .filter_map(|line| line.ok())
            .enumerate()
            .map(|(id, sql)| StoreCommand::new(id, sql))
            .collect()
    } else {
        println!("File not found: {}", filepath);
        vec![]
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
*/
