use serde::{Serialize, Deserialize};
use omnipaxos_core::storage::Entry;
use preprocessor::util::{MaybeEncoded, MaybeProcessed};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct EncodedHeader {
    pub message_id: (String, String, MaybeEncoded),
    pub date: MaybeEncoded,
    pub from: MaybeEncoded,
    pub to: MaybeProcessed,
    pub subject: MaybeProcessed,
    pub x_from: MaybeEncoded,
    pub x_to: MaybeProcessed,
    pub x_cc: MaybeProcessed,
    pub x_bcc: MaybeProcessed,
    pub x_folder: MaybeEncoded,
    pub x_origin: MaybeEncoded,
    pub x_filename: MaybeEncoded,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Header {
    Decoded(RawHeader),
    Encoded(EncodedHeader),
    None,
}

impl Default for Header {
    fn default() -> Self {
        Header::None
    }
}

impl Header {
    pub fn get_size(&self) -> usize {
        let mut size = 0;
        match self {
            Header::Encoded(e) => {
                let (r1, r2, me) = &e.message_id;
                size += r1.len() + r2.len() + me.get_size();

                size += e.date.get_size();
                size += e.from.get_size();
                size += e.to.get_size();
                size += e.subject.get_size();
                size += e.x_from.get_size();
                size += e.x_to.get_size();
                size += e.x_cc.get_size();
                size += e.x_bcc.get_size();
                size += e.x_folder.get_size();
                size += e.x_origin.get_size();
                size += e.x_filename.get_size();
            }
            Header::Decoded(d) => {
                size += d.message_id.len();
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
            }
            Header::None => {
                unimplemented!()
            }
        }
        size
    }
}

impl Entry for Header {}