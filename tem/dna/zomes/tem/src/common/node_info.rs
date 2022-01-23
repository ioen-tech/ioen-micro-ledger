use hdk::prelude::*;

use crate::err;

use super::{
    cloud_node::all_clouds_path,
    objects::association::{query_all_my_associations, CloudEdgeAssociation},
};

#[derive(Debug, Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Eq)]
pub struct ApplicationId(pub String);

pub fn query_pub_key_from_application_id(
    application_id: &ApplicationId,
) -> ExternResult<AgentPubKey> {
    let associations = query_all_my_associations()?;

    let association = associations.into_iter().find_map(|association| {
        if association.cloud.0.eq(application_id) {
            return Some(association.cloud.1);
        }
        if association.edge.0.eq(application_id) {
            return Some(association.edge.1);
        }
        return None;
    }).ok_or(err("Could not find the agent address for the given application id because we have not associated with it"))?;

    Ok(association)
}

#[derive(Debug, Clone, Serialize, Deserialize, SerializedBytes)]
pub struct NodeId(pub ApplicationId, pub AgentPubKey);

#[derive(Debug, Clone, Serialize, Deserialize, SerializedBytes)]
pub struct NodeInfo {
    pub node_id: NodeId,
    pub associations: Vec<CloudEdgeAssociation>,
    pub node_type: NodeType,
}

#[derive(Debug, Clone, Serialize, Deserialize, SerializedBytes)]
pub enum NodeType {
    Cloud,
    Edge,
}

pub fn query_my_node_info() -> ExternResult<NodeInfo> {
    let associations = query_all_my_associations()?;
    let register_market_node_links = query_register_cloud_node_links()?;

    let node_type = find_my_node_type(&register_market_node_links)?;

    let node_id = match register_market_node_links.first() {
        Some(create_link) => {
            let s = SerializedBytes::from(UnsafeBytes::from(create_link.tag.0.clone()));
            let application_id: ApplicationId = s.try_into()?;
            Ok(NodeId(
                application_id,
                create_link.target_address.clone().into(),
            ))
        }
        None => match associations.first() {
            None => Err(err("Could not query my application ID")),
            Some(association) => Ok(association.edge.clone()),
        },
    }?;

    Ok(NodeInfo {
        node_id,
        associations,
        node_type,
    })
}

/**
 * Returns the CreateLinks headers in the local chain for registering a market node
 */
pub fn query_register_cloud_node_links() -> ExternResult<Vec<CreateLink>> {
    let filter = ChainQueryFilter::new().header_type(HeaderType::CreateLink);

    let elements = query(filter)?;

    let create_links = elements
        .into_iter()
        .map(|element| {
            CreateLink::try_from(element.clone())
                .or(Err(crate::err("Can't convert header to CreateLink")))
        })
        .collect::<ExternResult<Vec<CreateLink>>>()?;

    let my_pub_key = agent_info()?.agent_initial_pubkey;

    let register_cloud_node_links = create_links
        .into_iter()
        .filter(|create_link| create_link.target_address.eq(&my_pub_key.clone().into()))
        .collect::<Vec<CreateLink>>();

    Ok(register_cloud_node_links)
}

/** Helper functions */

fn find_my_node_type(register_market_node_links: &Vec<CreateLink>) -> ExternResult<NodeType> {
    match register_market_node_links.get(0) {
        None => Ok(NodeType::Edge),
        Some(create_link) => {
            let agent_info = agent_info()?;
            let agent_pub_key: EntryHash = agent_info.agent_initial_pubkey.into();

            let base_is_path = create_link.base_address == all_clouds_path().hash()?;
            let target_is_me = agent_pub_key == create_link.target_address;

            match base_is_path && target_is_me {
                true => Ok(NodeType::Cloud),
                false => Ok(NodeType::Edge),
            }
        }
    }
}
