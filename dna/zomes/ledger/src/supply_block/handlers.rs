use hdk::prelude::*;
use hdk::prelude::holo_hash::*;
use super::SupplyBlock;

#[hdk_extern]
pub fn get_supply_block(entry_hash: EntryHashB64) -> ExternResult<Option<SupplyBlock>> {
  let maybe_element = get(EntryHash::from(entry_hash), GetOptions::default())?;

  match maybe_element {
    None => Ok(None),
    Some(element) => {
      let supply_block: SupplyBlock = element.entry()
        .to_app_option()?
        .ok_or(WasmError::Guest("Could not deserialize element to SupplyBlock.".into()))?;
    
      Ok(Some(supply_block))
    }
  }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct NewSupplyBlockOutput {
  header_hash: HeaderHashB64,
  entry_hash: EntryHashB64,
}

#[hdk_extern]
pub fn create_supply_block(supply_block: SupplyBlock) -> ExternResult<NewSupplyBlockOutput> {
  let header_hash = create_entry(&supply_block)?;

  let entry_hash = hash_entry(&supply_block)?;

  let output = NewSupplyBlockOutput {
    header_hash: HeaderHashB64::from(header_hash),
    entry_hash: EntryHashB64::from(entry_hash)
  };

  Ok(output)
}



