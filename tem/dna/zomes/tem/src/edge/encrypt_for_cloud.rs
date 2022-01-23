use crate::{
    common::{
        encrypted_data::{self, EdgeSecret, EncryptableObject, EncryptedDataWrapper},
        node_info::NodeInfo,
    },
    err,
};

use hdk::prelude::*;
use std::convert::{TryFrom, TryInto};

/**
 * Encrypt and publish the given data so that only you or the cloud you have associated can read it
 * This can only be called if we have a market associated
 */
pub fn create_encrypted_data_for_cloud<T: EncryptableObject>(
    my_node_info: &NodeInfo,
    expected_cloud: AgentPubKey,
    data: T,
) -> ExternResult<EntryHash> {
    let association = my_node_info
        .associations
        .first()
        .ok_or(err("This edge node has no cloud nodes associated with it"))?;

    if association.cloud.1 != expected_cloud {
        return Err(err(
            "Destination address is not the cloud node we have associated with",
        ));
    }

    encrypted_data::create_encrypted_data::<T>(
        association.cloud.1.clone(),
        association.edge.1.clone(),
        association.secret.clone(),
        data,
    )
}

/**
 * Get and decrypt all entries of the given type and in the given time range that have been published
 * with the secret of the market we have associated
 */
pub fn get_and_decrypt_entries_for_cloud<T: EncryptableObject>(
    my_node_info: &NodeInfo,
) -> ExternResult<Vec<(T, Timestamp)>> {
    let agent_info = agent_info()?;

    let encrypted_datas = encrypted_data::get_encrypted_entries_for_edge(
        &agent_info.agent_initial_pubkey,
        T::entry_type(),
    )?;

    let association = my_node_info
        .associations
        .first()
        .ok_or(err("This edge node has no cloud nodes associated with it"))?;

    let meters: Vec<(T, Timestamp)> = encrypted_datas
        .into_iter()
        .map(|encrypted_data| {
            let meter: T = decrypt_data_from_cloud(&association.secret, encrypted_data.0)?;
            Ok((meter, encrypted_data.1))
        })
        .collect::<ExternResult<Vec<(T, Timestamp)>>>()?;

    Ok(meters)
}

/** Helper functions */

fn decrypt_data_from_cloud<T: TryFrom<SerializedBytes>>(
    edge_secret: &EdgeSecret,
    encrypted_data_wrapper: EncryptedDataWrapper,
) -> ExternResult<T> {
    let data = encrypted_data::decrypt_data(encrypted_data_wrapper, edge_secret)?;

    let sb = SerializedBytes::from(UnsafeBytes::from(data.as_ref().to_vec()));

    let result: T = sb
        .try_into()
        .or(Err(crate::err("Cannot deserialize data")))?;

    Ok(result)
}
