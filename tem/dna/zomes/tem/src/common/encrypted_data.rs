use crate::err;
use hdk::prelude::*;
use std::convert::TryInto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeSecret(pub XSalsa20Poly1305KeyRef);

#[hdk_entry(id = "encrypted_data_wrapper", visibility = "public")]
#[derive(Clone)]
pub struct EncryptedDataWrapper {
    pub cloud: AgentPubKey,
    pub edge: AgentPubKey,
    pub data: XSalsa20Poly1305EncryptedData,
}

pub trait EncryptableObject: TryInto<SerializedBytes> + TryFrom<SerializedBytes> {
    fn entry_type() -> String;
}

/**
 * Validates that a piece of encrypted data is only valid if it was authored by either its bidder or its market node
 */
pub fn validate_encrypted_data_wrapper(
    element: &Element,
    encrypted_data_wrapper: EncryptedDataWrapper,
) -> ExternResult<ValidateCallbackResult> {
    let author = element.header().author();

    if encrypted_data_wrapper.cloud.eq(author) || encrypted_data_wrapper.edge.eq(author) {
        return Ok(ValidateCallbackResult::Valid);
    } else {
        return Ok(ValidateCallbackResult::Invalid(
            "This encrypted data wrapper wasn't authored by either its cloud node or its edge"
                .into(),
        ));
    }
}

pub fn generate_edge_secret() -> ExternResult<EdgeSecret> {
    let key_ref = XSalsa20Poly1305KeyRef::try_from_random()?;
    Ok(EdgeSecret(key_ref))
}

pub fn create_encrypted_data<T: EncryptableObject>(
    cloud_node: AgentPubKey,
    edge: AgentPubKey,
    edge_secret: EdgeSecret,
    data: T,
) -> ExternResult<EntryHash> {
    let data_bytes: SerializedBytes = data
        .try_into()
        .or(Err(crate::err("Could not serialize data")))?;

    let encrypted_data = x_salsa20_poly1305_encrypt(
        edge_secret.0,
        XSalsa20Poly1305Data::from(data_bytes.bytes().clone()),
    )?;

    let encrypted_data_wrapper = EncryptedDataWrapper {
        data: encrypted_data,
        edge: edge.clone(),
        cloud: cloud_node.clone(),
    };

    create_entry(&encrypted_data_wrapper)?;

    let encrypted_data_wrapper_hash = hash_entry(&encrypted_data_wrapper)?;

    create_link(
        edge.into(),
        encrypted_data_wrapper_hash.clone(),
        entry_type_tag::<T>(),
    )?;

    Ok(encrypted_data_wrapper_hash)
}

pub fn decrypt_data(
    encrypted_data_wrapper: EncryptedDataWrapper,
    edge_secret: &EdgeSecret,
) -> ExternResult<XSalsa20Poly1305Data> {
    let data = x_salsa20_poly1305_decrypt(edge_secret.0, encrypted_data_wrapper.data)?;

    match data {
        Some(salsa_data) => Ok(salsa_data),
        None => Err(err("Error while decrypting the data")),
    }
}

pub fn get_encrypted_entries_for_edge(
    edge_address: &AgentPubKey,
    entry_type: String,
) -> ExternResult<Vec<(EncryptedDataWrapper, Timestamp)>> {
    let edge_hash: EntryHash = edge_address.clone().into();

    let links = get_links(edge_hash, Some(LinkTag::new(entry_type.as_str())))?;

    let elements: Vec<Element> = links
        .into_inner()
        .into_iter()
        .map(|link| {
            let maybe_element = get(link.target, GetOptions::default())?;
            maybe_element.ok_or(crate::err("Could not get targeted element"))
        })
        .collect::<ExternResult<Vec<Element>>>()?;

    let mapped: Vec<(EncryptedDataWrapper, Timestamp)> = elements
        .into_iter()
        .map(|element| {
            let encrypted_data: EncryptedDataWrapper = element
                .entry()
                .to_app_option()?
                .ok_or(err("Failed to deserialize encrypted data"))?;

            Ok((encrypted_data, element.header().timestamp()))
        })
        .collect::<ExternResult<Vec<(EncryptedDataWrapper, Timestamp)>>>()?;

    Ok(mapped)
}

pub fn query_number_of_committed_encrypted_objects<T: EncryptableObject>() -> ExternResult<usize> {
    let filter = ChainQueryFilter::new().header_type(HeaderType::CreateLink);

    let query_result: Vec<Element> = query(filter)?;
    let number = query_result
        .into_iter()
        .filter_map(|e| match e.header() {
            Header::CreateLink(create_link) => Some(create_link.clone()),
            _ => None,
        })
        .filter(|create_link| create_link.tag.eq(&entry_type_tag::<T>()))
        .count();

    Ok(number)
}

fn entry_type_tag<T: EncryptableObject>() -> LinkTag {
    LinkTag::new(T::entry_type())
}
