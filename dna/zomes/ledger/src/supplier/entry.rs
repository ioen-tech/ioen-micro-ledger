use hdk::prelude::*;

#[hdk_entry(id = "supplier")]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct Supplier {
    pub address: String,
    pub postcode: String,
    pub method: String,
}
