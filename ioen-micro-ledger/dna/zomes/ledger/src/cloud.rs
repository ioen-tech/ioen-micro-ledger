use crate::{
    common::{
        node_info::{ApplicationId, NodeId, NodeInfo},
        objects::{
            association::CloudEdgeAssociation,
            contract::{Contract, ContractFilter},
            filter::QueryFilter,
            message::{Message, MessageFilter},
            receipt::{find_contract_for_receipt, Receipt, ReceiptFilter},
            utils::find_node_id_from_application_id,
        },
    },
    err,
    signal::SignalPayload,
};
use hdk::prelude::*;

use self::encrypt_for_edge::{
    create_encrypted_data_for_edge, get_and_decrypt_data_for_all_edges,
    get_and_decrypt_data_for_edge,
};

mod encrypt_for_edge;
pub mod register;

/** Functions executed only by the cloud */

#[hdk_extern]
pub fn register_as_cloud(my_application_id: ApplicationId) -> ExternResult<()> {
    register::register_as_cloud(my_application_id)
}

pub const HANDLE_EDGE_ASSOCIATON_FN_NAME: &str = "handle_edge_association";
#[hdk_extern]
pub fn handle_edge_association(edge_id: NodeId) -> ExternResult<CloudEdgeAssociation> {
    register::handle_edge_association(edge_id)
}

/**
 */
pub fn create_contract_for_edge(
    my_node_info: &NodeInfo,
    contract: Contract,
) -> ExternResult<EntryHash> {
    if !contract.source.eq(&my_node_info.node_id.0) {
        return Err(err("Contract source is not us"));
    }

    let counterparty = contract.get_counterparty(&my_node_info)?;

    let entry_hash = encrypt_for_edge::create_encrypted_data_for_edge(
        my_node_info,
        counterparty.1.clone(),
        contract.clone(),
    )?;

    let signal_payload = SignalPayload::ContractCreated {
        contract,
        creator_cloud: my_node_info.node_id.clone(),
    };
    let payload = ExternIO::encode(signal_payload)?;
    remote_signal(payload, vec![counterparty.1])?;

    Ok(entry_hash)
}

pub fn get_and_filter_contracts(
    my_node_info: &NodeInfo,
    contract_filter: ContractFilter,
) -> ExternResult<Vec<(Contract, Timestamp)>> {
    let contracts = get_contracts_to_filter(&my_node_info, &contract_filter)?;

    Ok(contracts
        .into_iter()
        .filter(|(c, _)| c.apply_filter(&contract_filter))
        .collect())
}

/**
 * Query all the edge nodes we have associated with,
 * query all the contracts to find the contract for the given receipt,
 * then create the receipt for the edge specified as the source of the Contract
 */
pub fn create_receipt_for_edge(
    my_node_info: &NodeInfo,
    receipt: Receipt,
) -> ExternResult<EntryHash> {
    let all_contracts: Vec<Contract> = get_and_decrypt_data_for_all_edges(my_node_info)?
        .into_iter()
        .map(|(c, _)| c)
        .collect();

    let contract = find_contract_for_receipt(all_contracts, &receipt)?;

    let counterparty = contract.get_counterparty(&my_node_info)?;

    create_encrypted_data_for_edge(my_node_info, counterparty.1, receipt)
}

pub fn get_and_filter_receipts(
    my_node_info: &NodeInfo,
    receipt_filter: ReceiptFilter,
) -> ExternResult<Vec<(Receipt, Timestamp)>> {
    let receipts: Vec<(Receipt, Timestamp)> = get_and_decrypt_data_for_all_edges(my_node_info)?;

    Ok(receipts
        .into_iter()
        .filter(|(c, _)| c.apply_filter(&receipt_filter))
        .collect())
}

pub fn create_message_for_edge(
    my_node_info: &NodeInfo,
    message: Message,
) -> ExternResult<EntryHash> {
    if !message.source.eq(&my_node_info.node_id.0) {
        return Err(err("Contract source is not us"));
    }

    let counterparty = message.get_counterparty(&my_node_info)?;

    return encrypt_for_edge::create_encrypted_data_for_edge(my_node_info, counterparty.1, message);
}

pub fn get_and_filter_messages(
    my_node_info: &NodeInfo,
    message_filter: MessageFilter,
) -> ExternResult<Vec<(Message, Timestamp)>> {
    let messages = get_messages_to_filter(&my_node_info, &message_filter)?;

    Ok(messages
        .into_iter()
        .filter(|(c, _)| c.apply_filter(&message_filter))
        .collect())
}

/** Helpers */

fn get_contracts_to_filter(
    my_node_info: &NodeInfo,
    contract_filter: &ContractFilter,
) -> ExternResult<Vec<(Contract, Timestamp)>> {
    match contract_filter.destination.clone() {
        Some(destination) => {
            let node_id = find_node_id_from_application_id(my_node_info, &destination)?;
            get_and_decrypt_data_for_edge(my_node_info, &node_id.1)
        }
        None => get_and_decrypt_data_for_all_edges(my_node_info),
    }
}

fn get_messages_to_filter(
    my_node_info: &NodeInfo,
    message_filter: &MessageFilter,
) -> ExternResult<Vec<(Message, Timestamp)>> {
    if let Some(destination) = message_filter.destination.clone() {
        if !destination.eq(&my_node_info.node_id.0) {
            let node_id = find_node_id_from_application_id(my_node_info, &destination)?;
            return get_and_decrypt_data_for_edge(my_node_info, &node_id.1);
        }
    }
    if let Some(source) = message_filter.source.clone() {
        if !source.eq(&my_node_info.node_id.0) {
            let node_id = find_node_id_from_application_id(my_node_info, &source)?;
            return get_and_decrypt_data_for_edge(my_node_info, &node_id.1);
        }
    }

    return get_and_decrypt_data_for_all_edges(my_node_info);
}
