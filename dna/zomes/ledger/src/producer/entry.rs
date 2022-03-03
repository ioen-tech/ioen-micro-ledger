use hdk::prelude::*;

#[hdk_entry(id = "producer")]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct Producer {
    pub address: String,
    pub postcode: String,
    pub method: String,
}
