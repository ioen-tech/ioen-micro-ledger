use hdk::prelude::*;

use crate::{
    common::node_info::{ApplicationId, NodeId, NodeInfo},
    err,
};

pub fn get_counterparty(
    my_node_info: &NodeInfo,
    source: &ApplicationId,
    destination: &ApplicationId,
) -> ExternResult<NodeId> {
    let counterparty_application_id =
        get_counterparty_application_id(my_node_info, source, destination)?;
    find_node_id_from_application_id(my_node_info, &counterparty_application_id)
}

pub fn find_node_id_from_application_id(
    my_node_info: &NodeInfo,
    application_id: &ApplicationId,
) -> ExternResult<NodeId> {
    my_node_info
        .associations
        .iter()
        .find_map(|association| {
            if association.cloud.0.eq(&application_id) {
                return Some(association.cloud.clone());
            } else if association.edge.0.eq(&application_id) {
                return Some(association.edge.clone());
            }
            return None;
        })
        .ok_or(err(
            "Couldn't find counterparty because we haven't associated with them",
        ))
}

fn get_counterparty_application_id(
    my_node_info: &NodeInfo,
    source: &ApplicationId,
    destination: &ApplicationId,
) -> ExternResult<ApplicationId> {
    if my_node_info.node_id.0.eq(source) {
        Ok(destination.clone())
    } else if my_node_info.node_id.0.eq(destination) {
        Ok(source.clone())
    } else {
        Err(err(
            "This contract does not have my application ID as Source or Destination",
        ))
    }
}
