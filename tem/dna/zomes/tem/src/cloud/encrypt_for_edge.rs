use hdk::prelude::*;
use std::convert::TryInto;

use crate::common::{
    encrypted_data::{self, EdgeSecret, EncryptableObject, EncryptedDataWrapper},
    node_info::NodeInfo,
    objects::association::CloudEdgeAssociation,
};

/**
 * Encrypt and publish the given data so that only you or the given bidder can read it
 * This can only be called if the given bidder has first associated with us
 */
pub fn create_encrypted_data_for_edge<T: EncryptableObject>(
    my_node_info: &NodeInfo,
    edge: AgentPubKey,
    data: T,
) -> ExternResult<EntryHash> {
    let agent_info = agent_info()?;

    let secret = find_secret_for_edge(my_node_info, edge.clone())?
        .ok_or(crate::err("Cannot create encrypted data for the given edge because we don't have any association with them"))?;

    encrypted_data::create_encrypted_data::<T>(agent_info.agent_initial_pubkey, edge, secret, data)
}

/**
 * Get and decrypt all entries of the given type and in the given time range that have been published
 * with the secret of the given bidder
 */
pub fn get_and_decrypt_data_for_edge<T: EncryptableObject>(
    my_node_info: &NodeInfo,
    edge_address: &AgentPubKey,
) -> ExternResult<Vec<(T, Timestamp)>> {
    let encrypted_datas =
        encrypted_data::get_encrypted_entries_for_edge(edge_address, T::entry_type())?;

    let entries: Vec<(T, Timestamp)> = encrypted_datas
        .into_iter()
        .map(|encrypted_data| {
            let entry: T = decrypt_data_from_edge(my_node_info, encrypted_data.0)?;
            Ok((entry, encrypted_data.1))
        })
        .collect::<ExternResult<Vec<(T, Timestamp)>>>()?;

    Ok(entries)
}

/**
 * Get and decrypt all entries of the given type for all the edges that this cloud node has associated with
 */
pub fn get_and_decrypt_data_for_all_edges<T: EncryptableObject>(
    my_node_info: &NodeInfo,
) -> ExternResult<Vec<(T, Timestamp)>> {
    let edges: Vec<AgentPubKey> = my_node_info
        .associations
        .iter()
        .map(|a| a.edge.1.clone())
        .collect();

    let all_data = edges
        .into_iter()
        .map(|edge| get_and_decrypt_data_for_edge(my_node_info, &edge))
        .collect::<ExternResult<Vec<Vec<(T, Timestamp)>>>>()?
        .into_iter()
        .flatten()
        .collect();

    Ok(all_data)
}

/** Helper functions */

fn decrypt_data_from_edge<T: EncryptableObject>(
    my_node_info: &NodeInfo,
    encrypted_data_wrapper: EncryptedDataWrapper,
) -> ExternResult<T> {
    let secret = find_secret_for_edge(my_node_info, encrypted_data_wrapper.edge.clone())?.ok_or(
        crate::err(
            "Cannot decrypt data from this edge because we don't have any association with it",
        ),
    )?;

    let data = encrypted_data::decrypt_data(encrypted_data_wrapper, &secret)?;

    let sb = SerializedBytes::from(UnsafeBytes::from(data.as_ref().to_vec()));

    let result: T = sb.try_into().or(Err(crate::err("Cannot deserialize")))?;

    Ok(result)
}

fn find_secret_for_edge(
    my_node_info: &NodeInfo,
    edge: AgentPubKey,
) -> ExternResult<Option<EdgeSecret>> {
    let association = find_association_with_edge(my_node_info, edge)?;

    Ok(association.map(|a| a.secret))
}

fn find_association_with_edge(
    my_node_info: &NodeInfo,
    edge: AgentPubKey,
) -> ExternResult<Option<CloudEdgeAssociation>> {
    let association_with_edge = my_node_info
        .associations
        .iter()
        .find(|association| association.edge.clone().1.eq(&edge));

    Ok(association_with_edge.map(|a| a.clone()))
}
