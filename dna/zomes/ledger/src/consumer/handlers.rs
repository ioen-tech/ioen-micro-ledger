use hdk::prelude::*;
use hdk::prelude::holo_hash::*;
use super::Consumer;

#[hdk_extern]
pub fn get_consumer(entry_hash: EntryHashB64) -> ExternResult<Option<Consumer>> {
  let maybe_element = get(EntryHash::from(entry_hash), GetOptions::default())?;

  match maybe_element {
    None => Ok(None),
    Some(element) => {
      let consumer: Consumer = element.entry()
        .to_app_option()?
        .ok_or(WasmError::Guest("Could not deserialize element to Consumer.".into()))?;
    
      Ok(Some(consumer))
    }
  }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct NewConsumerOutput {
  header_hash: HeaderHashB64,
  entry_hash: EntryHashB64,
}

#[hdk_extern]
pub fn create_consumer(consumer: Consumer) -> ExternResult<NewConsumerOutput> {
  let agent_info = agent_info()?;
  let consumer_path = format!("Consumer.{}", consumer.postcode);
  let path = Path::from(consumer_path);
  path.ensure()?;

  let header_hash = create_entry(&consumer)?;

  let entry_hash = hash_entry(&consumer)?;

  create_link(path.path_entry_hash()?, entry_hash.clone(), ())?;
  let link_tag: LinkTag = LinkTag::new(String::from("Consumer"));
  create_link(EntryHash::from(agent_info.agent_initial_pubkey), entry_hash.clone(), link_tag)?;

  let output = NewConsumerOutput {
    header_hash: HeaderHashB64::from(header_hash),
    entry_hash: EntryHashB64::from(entry_hash)
  };

  Ok(output)
}

#[hdk_extern]
pub fn agent_info_consumer(_: ()) -> ExternResult<Vec<Consumer>> {
  let agent_info = agent_info()?;
  let link_tag: LinkTag = LinkTag::new(String::from("Consumer"));
  let links = get_links(EntryHash::from(agent_info.agent_initial_pubkey), Some(link_tag))?;

  let suppliers: Vec<Consumer> = links
      .into_iter()
      .filter_map(|link| get_consumer(EntryHashB64::from(link.target) ).transpose())
      .collect::<ExternResult<Vec<Consumer>>>()?;

  Ok(suppliers)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateConsumerInput {
  original_header_hash: HeaderHashB64,
  updated_consumer: Consumer
}

#[hdk_extern]
pub fn update_consumer(input: UpdateConsumerInput) -> ExternResult<NewConsumerOutput> {
  let header_hash = update_entry(HeaderHash::from(input.original_header_hash), &input.updated_consumer)?;

  let entry_hash = hash_entry(&input.updated_consumer)?;

  let output = NewConsumerOutput {
    header_hash: HeaderHashB64::from(header_hash),
    entry_hash: EntryHashB64::from(entry_hash)
  };

  Ok(output)
}


#[hdk_extern]
pub fn delete_consumer(header_hash: HeaderHashB64) -> ExternResult<HeaderHash> {
  delete_entry(HeaderHash::from(header_hash))
}

