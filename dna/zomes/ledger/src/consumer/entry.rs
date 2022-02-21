use hdk::prelude::*;

#[hdk_entry(id = "consumer")]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct Consumer {
    pub address: String,
    pub postcode: i64,
}
