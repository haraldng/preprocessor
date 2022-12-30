use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct RawMediumRecord {
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
pub struct MediumRecord {
    pub(crate) post_time: String,
    pub(crate) post_name: String,
    pub(crate) post_author: String,
    pub(crate) post_publication: String,
    pub(crate) post_tags: [String; 4]
}

impl From<RawMediumRecord> for MediumRecord {
    fn from(r: RawMediumRecord) -> Self {
        Self {
            post_time: r.post_time,
            post_name: r.post_name,
            post_author: r.post_author,
            post_publication: r.post_publication,
            post_tags: [r.post_tag_1, r.post_tag_2, r.post_tag_3, r.post_tag_4]
        }
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct EncodedMediumRecord {
    pub(crate) post_time: String,
    pub(crate) post_name: Vec<MaybeEncoded>,
    pub(crate) post_author: Vec<MaybeEncoded>,
    pub(crate) post_publication: MaybeEncoded,
    pub(crate) post_tags: [MaybeEncoded; 4]
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
                /*
                size += e.post_time.len();
                size += 5;
                size += e.post_name.len();
                size += e.post_author.len();
                */
                size += e.post_time.len();
                size += e.post_publication.get_size();
                e.post_tags.iter().for_each(|x| size += x.get_size());
                e.post_name.iter().for_each(|x| size += x.get_size());
                e.post_author.iter().for_each(|x|  size += x.get_size());
            },
            Record::Decoded(d) => {
                size += d.post_time.len();
                size += d.post_publication.len();
                d.post_tags.iter().for_each(|x| size += x.len());
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