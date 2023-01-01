use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct RawHeader {
    pub message_id: String,
    pub date: String,
    pub from: String,
    pub to: String,
    pub subject: String,
    pub x_from: String,
    pub x_to: String,
    pub x_cc: String,
    pub x_bcc: String,
    pub x_folder: String,
    pub x_origin: String,
    pub x_filename: String,
}


#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct EncodedHeader {
    pub message_id: String,
    pub date: String,
    pub from: MaybeEncoded,
    pub to: Vec<MaybeEncoded>,
    pub subject: Vec<MaybeEncoded>,
    pub x_from: MaybeEncoded,
    pub x_to: Vec<MaybeEncoded>,
    pub x_cc: Vec<MaybeEncoded>,
    pub x_bcc: Vec<MaybeEncoded>,
    pub x_folder: Vec<MaybeEncoded>,
    pub x_origin: MaybeEncoded,
    pub x_filename: MaybeEncoded,
}


#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub enum Record {
    Decoded(RawHeader),
    Encoded(EncodedHeader),
    None,
}

impl Record {
    pub(crate) fn get_size(&self) -> usize {
        let mut size = 0;
        match self {
            Record::Encoded(e) => {
                // size += e.message_id.len();
                size += e.date.len();
                size += e.from.get_size();
                e.to.iter().for_each(|x| size += x.get_size());
                e.subject.iter().for_each(|x| size += x.get_size());
                size += e.x_from.get_size();
                e.x_to.iter().for_each(|x| size += x.get_size());
                e.x_cc.iter().for_each(|x| size += x.get_size());
                e.x_bcc.iter().for_each(|x| size += x.get_size());
                e.x_folder.iter().for_each(|x| size += x.get_size());
                size += e.x_origin.get_size();
                size += e.x_filename.get_size();

            },
            Record::Decoded(d) => {
                // size += d.message_id.len();
                // size += d.keywords.len();
                size += d.date.len();
                size += d.from.len();
                size += d.to.len();
                size += d.subject.len();
                size += d.x_from.len();
                size += d.x_to.len();
                size += d.x_cc.len();
                size += d.x_bcc.len();
                size += d.x_folder.len();
                size += d.x_origin.len();
                size += d.x_filename.len();
            },
            Record::None => {
                unimplemented!()
            },
        }
        size
    }
/*
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
*/
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