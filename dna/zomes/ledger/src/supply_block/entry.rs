use hdk::prelude::*;

#[hdk_entry(id = "supply_block")]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct SupplyBlock {
    pub quality: i64,
    pub from: i64,
    pub to: i64,
    pub price: f64,
}
