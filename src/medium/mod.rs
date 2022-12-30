pub(crate) mod preprocess;

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct MediumRecord {
    post_time: String,
    post_name: String,
    post_author: String,
    post_publication: String,
    post_tag_1: String,
    post_tag_2: String,
    post_tag_3: String,
    post_tag_4: String,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct EncodedMediumRecord {
    post_time: String,
    post_name: Vec<MaybeEncoded>,
    post_author: Vec<MaybeEncoded>,
    post_publication: MaybeEncoded,
    post_tag_1: MaybeEncoded,
    post_tag_2: MaybeEncoded,
    post_tag_3: MaybeEncoded,
    post_tag_4: MaybeEncoded,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub enum Record {
    Decoded(MediumRecord),
    Encoded(EncodedMediumRecord),
    None,
}

impl Record {
    pub(crate) fn get_size(&self) -> usize {
        let mut size = 0;
        match self {
            Record::Encoded(e) => {
                size += e.post_time.len();
                size += e.post_publication.get_size();
                size += e.post_tag_1.get_size();
                size += e.post_tag_2.get_size();
                size += e.post_tag_3.get_size();
                size += e.post_tag_4.get_size();
                e.post_name.iter().for_each(|x| size += x.get_size());
                e.post_author.iter().for_each(|x|  size += x.get_size());
            },
            Record::Decoded(d) => {
                size += d.post_time.len();
                size += d.post_publication.len();
                size += d.post_tag_1.len();
                size += d.post_tag_2.len();
                size += d.post_tag_3.len();
                size += d.post_tag_4.len();
                size += d.post_name.len();
                size += d.post_author.len();
            },
            _ => {},
        }
        size
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub enum MaybeEncoded {
    Encoded(usize),
    Decoded(String)
}

impl MaybeEncoded {
    fn get_size(&self) -> usize {
        match self {
            // MaybeEncoded::Encoded(i) => std::mem::size_of_val(i),
            MaybeEncoded::Encoded(_) => 1,
            MaybeEncoded::Decoded(s) => s.len(),
        }

    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use super::*;
    use crate::medium::MediumRecord;

    #[test]
    fn test_medium() {
        /*
        let file = File::open("datasets/medium/Train.csv").unwrap();
        let mut reader = csv::Reader::from_reader(file);


        for record in reader.deserialize() {
            let record: MediumRecord = record.unwrap();
            println!("{:?}\n", record);
        }
        */

        let e = MaybeEncoded::Encoded(5);
        let de = MaybeEncoded::Decoded("Welcome".to_string());
        println!("e: {}, de: {}", e.get_size(), de.get_size());
        assert!(de.get_size() > e.get_size())
    }
}
