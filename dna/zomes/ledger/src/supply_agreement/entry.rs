use hdk::prelude::*;
use hdk::prelude::holo_hash::*;

#[hdk_entry(id = "supply_agreement")]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct SupplyAgreement {
    pub from: i64,
    pub to: i64,
    pub rate: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewSupplyAgreementHashes {
    pub header_hash: HeaderHashB64,
    pub entry_hash: EntryHashB64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewSupplyAgreement {
    pub supplier_entry_hash: EntryHashB64,
    pub supply_agreement: SupplyAgreement
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExistingSupplyAgreement {
    pub supplier_entry_hash: EntryHashB64,
    pub supply_agreement_entry_hash: EntryHashB64
}