use hdk::prelude::*;
use hdk::prelude::holo_hash::*;
use super::Bill;

#[hdk_extern]
pub fn get_bill(entry_hash: EntryHashB64) -> ExternResult<Option<Bill>> {
  let maybe_element = get(EntryHash::from(entry_hash), GetOptions::default())?;

  match maybe_element {
    None => Ok(None),
    Some(element) => {
      let bill: Bill = element.entry()
        .to_app_option()?
        .ok_or(WasmError::Guest("Could not deserialize element to Bill.".into()))?;
    
      Ok(Some(bill))
    }
  }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct NewBillOutput {
  header_hash: HeaderHashB64,
  entry_hash: EntryHashB64,
}

#[hdk_extern]
pub fn create_bill(bill: Bill) -> ExternResult<NewBillOutput> {
  let header_hash = create_entry(&bill)?;

  let entry_hash = hash_entry(&bill)?;

  let output = NewBillOutput {
    header_hash: HeaderHashB64::from(header_hash),
    entry_hash: EntryHashB64::from(entry_hash)
  };

  Ok(output)
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateBillInput {
  original_header_hash: HeaderHashB64,
  updated_bill: Bill
}

#[hdk_extern]
pub fn update_bill(input: UpdateBillInput) -> ExternResult<NewBillOutput> {
  let header_hash = update_entry(HeaderHash::from(input.original_header_hash), &input.updated_bill)?;

  let entry_hash = hash_entry(&input.updated_bill)?;

  let output = NewBillOutput {
    header_hash: HeaderHashB64::from(header_hash),
    entry_hash: EntryHashB64::from(entry_hash)
  };

  Ok(output)
}


#[hdk_extern]
pub fn delete_bill(header_hash: HeaderHashB64) -> ExternResult<HeaderHash> {
  delete_entry(HeaderHash::from(header_hash))
}

