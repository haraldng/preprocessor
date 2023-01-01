use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct RawArticle {
    pub web_url: String,
    pub keywords: String,
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
pub struct EncodedMediumRecord {
    pub(crate) post_time: String,
    pub(crate) post_name: Vec<MaybeEncoded>,
    pub(crate) post_author: Vec<MaybeEncoded>,
    pub(crate) post_publication: MaybeEncoded,
    pub(crate) post_tags: [MaybeEncoded; 4]
}


#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub enum Record {
    Decoded(RawArticle),
    // Encoded(EncodedMediumRecord),
    None,
}

impl Record {
    pub(crate) fn get_size(&self) -> usize {
        let mut size = 0;
        match self {
            /*
            Record::Encoded(e) => {
                size += e.post_time.len();
                size += e.post_publication.get_size();
                e.post_tags.iter().for_each(|x| size += x.get_size());
                e.post_name.iter().for_each(|x| size += x.get_size());
                e.post_author.iter().for_each(|x|  size += x.get_size());
            },
            */
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
            },
            Record::None => {
                let url = 5;
            },
        }
        size
    }

    pub(crate) fn get_approximate_encoded_size(&self) -> usize {
        let mut size = 0;
        match self {
            Record::Decoded(d) => {
                size += d.web_url.len()/2;
                // size += d.keywords.len();
                size += d.pub_date.len();
                size += 1;
                size += 1;
                size += 1;
                size += 1;
                size += d.main_headline.len();
                size += d.print_headline.len();
                size += 3;
            },
            Record::None => {
            },
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