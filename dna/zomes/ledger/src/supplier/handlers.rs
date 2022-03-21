use hdk::prelude::*;
use hdk::prelude::holo_hash::*;
use super::Supplier;
use super::UpdateSupplierInput;
use super::SupplierHashes;
use super::SupplierFilter;

#[hdk_extern]
pub fn get_supplier(entry_hash: EntryHashB64) -> ExternResult<Option<Supplier>> {
  let maybe_element = get(EntryHash::from(entry_hash), GetOptions::default())?;

  match maybe_element {
    None => Ok(None),
    Some(element) => {
      let supplier: Supplier = element.entry()
        .to_app_option()?
        .ok_or(WasmError::Guest("Could not deserialize element to Supplier.".into()))?;
      Ok(Some(supplier))
    }
  }
}

#[hdk_extern]
pub fn agent_info_supplier(_: ()) -> ExternResult<Vec<Supplier>> {
  let agent_info = agent_info()?;
  let link_tag: LinkTag = LinkTag::new(String::from("Supplier"));
  let links = get_links(EntryHash::from(agent_info.agent_initial_pubkey), Some(link_tag))?;

  let suppliers: Vec<Supplier> = links
      .into_iter()
      .filter_map(|link| get_supplier(EntryHashB64::from(link.target) ).transpose())
      .collect::<ExternResult<Vec<Supplier>>>()?;

  Ok(suppliers)
}

#[hdk_extern]
pub fn list_suppliers(supplier_filter: SupplierFilter) -> ExternResult<Vec<Supplier>> {
  let path = Path::from(format!("Suppliers.{}.{}", supplier_filter.method, supplier_filter.postcode));

  let links = get_links(path.path_entry_hash()?, None)?;

  let suppliers: Vec<Supplier> = links
      .into_iter()
      .filter_map(|link| get_supplier(EntryHashB64::from(link.target) ).transpose())
      .collect::<ExternResult<Vec<Supplier>>>()?;

  Ok(suppliers)
}

#[hdk_extern]
pub fn create_supplier(supplier: Supplier) -> ExternResult<SupplierHashes> {
  let agent_info = agent_info()?;
  let supplier_path = format!("Suppliers.{}.{}", supplier.method, supplier.postcode);
  let path = Path::from(supplier_path);
  path.ensure()?;

  let header_hash = create_entry(&supplier)?;

  let entry_hash = hash_entry(&supplier)?;

  create_link(path.path_entry_hash()?, entry_hash.clone(), ())?;
  let link_tag: LinkTag = LinkTag::new(String::from("Supplier"));
  create_link(EntryHash::from(agent_info.agent_initial_pubkey), entry_hash.clone(),  link_tag)?;

  let output = SupplierHashes {
    header_hash: HeaderHashB64::from(header_hash),
    entry_hash: EntryHashB64::from(entry_hash)
  };

  Ok(output)
}

#[hdk_extern]
pub fn update_supplier(input: UpdateSupplierInput) -> ExternResult<SupplierHashes> {
  let header_hash = update_entry(HeaderHash::from(input.original_header_hash), &input.updated_supplier)?;

  let entry_hash = hash_entry(&input.updated_supplier)?;

  let output = SupplierHashes {
    header_hash: HeaderHashB64::from(header_hash),
    entry_hash: EntryHashB64::from(entry_hash)
  };

  Ok(output)
}


#[hdk_extern]
pub fn delete_supplier(header_hash: HeaderHashB64) -> ExternResult<HeaderHash> {
  delete_entry(HeaderHash::from(header_hash))
}

