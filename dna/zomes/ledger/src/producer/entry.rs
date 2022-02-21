use hdk::prelude::*;

#[hdk_entry(id = "producer")]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct Producer {
    pub address: String,
    pub postcode: i64,
    pub method: String,
}
