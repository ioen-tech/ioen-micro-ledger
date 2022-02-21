use hdk::prelude::*;
use hdk::prelude::holo_hash::*;
use super::Producer;

#[hdk_extern]
pub fn get_producer(entry_hash: EntryHashB64) -> ExternResult<Option<Producer>> {
  let maybe_element = get(EntryHash::from(entry_hash), GetOptions::default())?;

  match maybe_element {
    None => Ok(None),
    Some(element) => {
      let producer: Producer = element.entry()
        .to_app_option()?
        .ok_or(WasmError::Guest("Could not deserialize element to Producer.".into()))?;
    
      Ok(Some(producer))
    }
  }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct NewProducerOutput {
  header_hash: HeaderHashB64,
  entry_hash: EntryHashB64,
}

#[hdk_extern]
pub fn create_producer(producer: Producer) -> ExternResult<NewProducerOutput> {
  let header_hash = create_entry(&producer)?;

  let entry_hash = hash_entry(&producer)?;

  let output = NewProducerOutput {
    header_hash: HeaderHashB64::from(header_hash),
    entry_hash: EntryHashB64::from(entry_hash)
  };

  Ok(output)
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateProducerInput {
  original_header_hash: HeaderHashB64,
  updated_producer: Producer
}

#[hdk_extern]
pub fn update_producer(input: UpdateProducerInput) -> ExternResult<NewProducerOutput> {
  let header_hash = update_entry(HeaderHash::from(input.original_header_hash), &input.updated_producer)?;

  let entry_hash = hash_entry(&input.updated_producer)?;

  let output = NewProducerOutput {
    header_hash: HeaderHashB64::from(header_hash),
    entry_hash: EntryHashB64::from(entry_hash)
  };

  Ok(output)
}


#[hdk_extern]
pub fn delete_producer(header_hash: HeaderHashB64) -> ExternResult<HeaderHash> {
  delete_entry(HeaderHash::from(header_hash))
}

