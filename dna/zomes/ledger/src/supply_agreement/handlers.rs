use hdk::prelude::*;
use hdk::prelude::holo_hash::*;
use super::SupplyAgreement;

#[hdk_extern]
pub fn get_supply_agreement(entry_hash: EntryHashB64) -> ExternResult<Option<SupplyAgreement>> {
  let maybe_element = get(EntryHash::from(entry_hash), GetOptions::default())?;

  match maybe_element {
    None => Ok(None),
    Some(element) => {
      let supply_agreement: SupplyAgreement = element.entry()
        .to_app_option()?
        .ok_or(WasmError::Guest("Could not deserialize element to SupplyAgreement.".into()))?;
    
      Ok(Some(supply_agreement))
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewSupplyAgreementOutput {
  header_hash: HeaderHashB64,
  entry_hash: EntryHashB64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewSupplyAgreement {
  supplier_entry_hash: EntryHashB64,
  supply_agreement: SupplyAgreement
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExistingSupplyAgreement {
  supplier_entry_hash: EntryHashB64,
  supply_agreement_entry_hash: EntryHashB64
}

#[hdk_extern]
pub fn list_suppliers_agreements(supplier_entry_hash: EntryHashB64) -> ExternResult<Vec<SupplyAgreement>> {
  let links = get_links(EntryHash::from(supplier_entry_hash.clone()), None)?;

  let supply_agreements: Vec<SupplyAgreement> = links
      .into_iter()
      .filter_map(|link| get_supply_agreement(EntryHashB64::from(link.target) ).transpose())
      .collect::<ExternResult<Vec<SupplyAgreement>>>()?;

  Ok(supply_agreements)
}

#[hdk_extern]
pub fn list_all_supply_agreements(_: ()) -> ExternResult<Vec<SupplyAgreement>> {
  let path = Path::from(format!("SupplyAgreements"));

  let links = get_links(path.path_entry_hash()?, None)?;

  let supply_agreements: Vec<SupplyAgreement> = links
      .into_iter()
      .filter_map(|link| get_supply_agreement(EntryHashB64::from(link.target) ).transpose())
      .collect::<ExternResult<Vec<SupplyAgreement>>>()?;

  Ok(supply_agreements)
}

#[hdk_extern]
pub fn create_supply_agreement(new_supply_agreement: NewSupplyAgreement) -> ExternResult<NewSupplyAgreementOutput> {

  let supply_agreement_path = format!("SupplyAgreements");
  let path = Path::from(supply_agreement_path);
  path.ensure()?;

  let header_hash = create_entry(&new_supply_agreement.supply_agreement)?;

  let entry_hash = hash_entry(&new_supply_agreement.supply_agreement)?;
  // Link to path
  create_link(path.path_entry_hash()?, entry_hash.clone(), ())?;
  // Link to supplier
  create_link(EntryHash::from(new_supply_agreement.supplier_entry_hash.clone()), entry_hash.clone(), ())?;

  let output = NewSupplyAgreementOutput {
    header_hash: HeaderHashB64::from(header_hash),
    entry_hash: EntryHashB64::from(entry_hash)
  };

  Ok(output)
}

#[hdk_extern]
pub fn existing_supply_agreement(existing_supply_agreement: ExistingSupplyAgreement) -> ExternResult<HeaderHash> {
  // Link to supplier
  create_link(EntryHash::from(existing_supply_agreement.supplier_entry_hash), EntryHash::from(existing_supply_agreement.supply_agreement_entry_hash), ())
}