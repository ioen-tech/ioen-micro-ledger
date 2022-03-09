use hdk::prelude::*;

#[hdk_entry(id = "bill")]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct Bill {
    pub address: String,
    pub due_date: i64,
    pub status: String,
    pub notes: String,
}
