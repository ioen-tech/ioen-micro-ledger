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

#[hdk_extern]
pub fn create_supply_agreement(supply_agreement: SupplyAgreement) -> ExternResult<NewSupplyAgreementOutput> {
  let header_hash = create_entry(&supply_agreement)?;

  let entry_hash = hash_entry(&supply_agreement)?;

  let output = NewSupplyAgreementOutput {
    header_hash: HeaderHashB64::from(header_hash),
    entry_hash: EntryHashB64::from(entry_hash)
  };

  Ok(output)
}



