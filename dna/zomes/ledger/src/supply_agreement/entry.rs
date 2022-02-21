use hdk::prelude::*;

#[hdk_entry(id = "supply_agreement")]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct SupplyAgreement {
    pub from: i64,
    pub to: i64,
    pub price: f64,
}
