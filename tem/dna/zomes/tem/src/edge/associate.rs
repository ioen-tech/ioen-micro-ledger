use crate::{
    cloud::HANDLE_EDGE_ASSOCIATON_FN_NAME,
    common::{
        node_info::{ApplicationId, NodeId},
        objects::association::query_all_my_associations,
    },
    err,
};
use hdk::prelude::*;

use crate::common::{cloud_node::get_cloud_node, objects::association::CloudEdgeAssociation};

/**
 * As a bidder, get the cloud registered in the network and save it in our source chain
 */
pub fn associate_with_cloud(my_application_id: ApplicationId) -> ExternResult<()> {
    let associations = query_all_my_associations()?;

    if associations.len() > 0 {
        // We have already associated with the cloud node: do nothing
        return Ok(());
    }

    match get_cloud_node()? {
        None => Err(crate::err("There is no registered cloud in this network")),
        Some(cloud_node) => {
            let association = request_associate_to_cloud(cloud_node.clone(), my_application_id)?;

            let mut functions: GrantedFunctions = HashSet::new();
            functions.insert((zome_info()?.zome_name, "recv_remote_signal".into()));

            create_cap_grant(CapGrantEntry {
                tag: "".into(),
                // empty access converts to unrestricted
                access: ().try_into()?,
                functions,
            })?;

            create_entry(&association)?;

            Ok(())
        }
    }
}

/** Helper functions */

fn request_associate_to_cloud(
    cloud_node: AgentPubKey,
    my_application_id: ApplicationId,
) -> ExternResult<CloudEdgeAssociation> {
    let agent_info = agent_info()?;

    let zome_info = zome_info()?;

    let response: ZomeCallResponse = call_remote(
        cloud_node,
        zome_info.zome_name,
        FunctionName(HANDLE_EDGE_ASSOCIATON_FN_NAME.into()),
        None,
        &NodeId(my_application_id, agent_info.agent_initial_pubkey),
    )?;

    match response {
        ZomeCallResponse::Ok(result) => {
            let association: CloudEdgeAssociation = result.decode()?;
            Ok(association)
        }
        ZomeCallResponse::NetworkError(error) => {
            Err(err(format!("Network Error: {}", error).as_str()))
        }
        ZomeCallResponse::Unauthorized(_, _, _, _) => Err(err("Unauthorized call remote")),
    }
}
