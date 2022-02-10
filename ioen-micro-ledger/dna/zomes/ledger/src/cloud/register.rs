use hdk::prelude::*;

use crate::common::{
    cloud_node::{all_clouds_path, get_cloud_node},
    encrypted_data::generate_edge_secret,
    node_info::{query_my_node_info, query_register_cloud_node_links, ApplicationId, NodeId},
    objects::association::CloudEdgeAssociation,
};

use super::HANDLE_EDGE_ASSOCIATON_FN_NAME;

// Registers this agent as a cloud node
pub fn register_as_cloud(my_application_id: ApplicationId) -> ExternResult<()> {
    // No need to register ourselves if we have already been registered
    if query_register_cloud_node_links()?.len() > 0 {
        return Ok(());
    }

    // If there is some other node registered we can't register
    // TODO: discuss and design a secure solution to replace this
    if let Some(_) = get_cloud_node()? {
        return Err(crate::err(
            "Can't register as a cloud: there is one cloud node registered in this network already",
        ));
    }

    let path = all_clouds_path();
    path.ensure()?;

    // Convert our public key to an EntryHash to be able to link to it
    let agent_pub_key = agent_info()?.agent_initial_pubkey;
    let any_dht_hash: AnyDhtHash = agent_pub_key.clone().into();

    // Create capability grant for the bidders to be able to commit BidCommitments
    create_unrestricted_cap_grants()?;

    let sb: SerializedBytes = my_application_id.try_into()?;

    create_link(path.path_entry_hash()?, any_dht_hash.into(), sb.bytes().clone())?;

    Ok(())
}

/**
 * Generates a secret, commits it to the source chain and returns it to the edge
 * This secret is the one with which all encrypted data in the DHT shared between this cloud
 * and the edge will be encrypted
 */
pub fn handle_edge_association(edge_id: NodeId) -> ExternResult<CloudEdgeAssociation> {
    let secret = generate_edge_secret()?;

    let my_application_id = query_my_node_info()?;

    let association = CloudEdgeAssociation {
        cloud: my_application_id.node_id,
        edge: edge_id,
        secret: secret.clone(),
    };

    create_entry(&association.clone())?;

    Ok(association)
}

/** Helper functions */

fn create_unrestricted_cap_grants() -> ExternResult<()> {
    let mut functions: GrantedFunctions = BTreeSet::new();
    functions.insert((
        zome_info()?.name,
        HANDLE_EDGE_ASSOCIATON_FN_NAME.into(),
    ));

    create_cap_grant(CapGrantEntry {
        tag: "".into(),
        // empty access converts to unrestricted
        access: ().into(),
        functions,
    })?;

    Ok(())
}
