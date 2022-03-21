use hdk::prelude::*;
use hdk::prelude::holo_hash::*;
#[hdk_entry(id = "supplier")]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct Supplier {
    pub address: String,
    pub postcode: String,
    pub method: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateSupplierInput {
  pub original_header_hash: HeaderHashB64,
  pub updated_supplier: Supplier
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SupplierHashes {
  pub header_hash: HeaderHashB64,
  pub entry_hash: EntryHashB64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SupplierOutput {
    pub supplier: Supplier,
    pub supplier_hashes: SupplierHashes,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SupplierFilter {
  pub postcode: String,
  pub method: String,
}