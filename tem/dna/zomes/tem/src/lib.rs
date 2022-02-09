#![allow(dead_code)]
use hdk::prelude::*;

use common::{
    encrypted_data::{self, EncryptedDataWrapper},
    node_info::{query_my_node_info, NodeType},
    objects::{
        association::CloudEdgeAssociation,
        contract::{Contract, ContractFilter},
        message::{Message, MessageFilter},
        receipt::{Receipt, ReceiptFilter},
    },
};

mod cloud;
mod common;
mod edge;
mod signal;

/** Global definitions */

pub fn err(reason: &str) -> WasmError {
    WasmError::Guest(String::from(reason))
}

#[hdk_extern]
fn who_am_i(_: ()) -> ExternResult<AgentPubKey> {
    let agent_info = agent_info()?;

    Ok(AgentPubKey::from(agent_info.agent_initial_pubkey))
}

entry_defs![
    CloudEdgeAssociation::entry_def(),
    EncryptedDataWrapper::entry_def(),
    Path::entry_def()
];

#[hdk_extern]
pub fn validate(data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    match data.element.entry().clone().into_option() {
        None => Ok(ValidateCallbackResult::Valid),
        Some(entry) => match EncryptedDataWrapper::try_from(entry) {
            Ok(encryped_data_wrapper) => encrypted_data::validate_encrypted_data_wrapper(
                &data.element,
                encryped_data_wrapper,
            ),
            Err(_) => Ok(ValidateCallbackResult::Valid),
        },
    }
}

/** Common functions executed by any node: querying objects */

/**
 * Create an encrypted Contract between the given Source and Destination so that only they can read it
 */
#[hdk_extern]
pub fn create_contract(contract: Contract) -> ExternResult<EntryHash> {
    let my_node_info = query_my_node_info()?;

    match my_node_info.node_type {
        NodeType::Cloud => cloud::create_contract_for_edge(&my_node_info, contract),
        NodeType::Edge => edge::create_contract_for_cloud(&my_node_info, contract),
    }
}

#[hdk_extern]
pub fn get_contracts(contract_filter: ContractFilter) -> ExternResult<Vec<(Contract, Timestamp)>> {
    let my_node_info = query_my_node_info()?;

    match my_node_info.node_type {
        NodeType::Cloud => cloud::get_and_filter_contracts(&my_node_info, contract_filter),
        NodeType::Edge => edge::get_and_filter_contracts(&my_node_info, contract_filter),
    }
}

#[hdk_extern]
pub fn get_receipts(receipt_filter: ReceiptFilter) -> ExternResult<Vec<(Receipt, Timestamp)>> {
    let my_node_info = query_my_node_info()?;

    match my_node_info.node_type {
        NodeType::Cloud => cloud::get_and_filter_receipts(&my_node_info, receipt_filter),
        NodeType::Edge => edge::get_and_filter_receipts(&my_node_info, receipt_filter),
    }
}

#[hdk_extern]
pub fn get_messages(message_filter: MessageFilter) -> ExternResult<Vec<(Message, Timestamp)>> {
    let my_node_info = query_my_node_info()?;

    match my_node_info.node_type {
        NodeType::Cloud => cloud::get_and_filter_messages(&my_node_info, message_filter),
        NodeType::Edge => edge::get_and_filter_messages(&my_node_info, message_filter),
    }
}

/**
 * Create an encrypted receipt for the given contract id
 * so that only the Source and Destination of the Contract can read it
 */
#[hdk_extern]
pub fn create_receipt(receipt: Receipt) -> ExternResult<EntryHash> {
    let my_node_info = query_my_node_info()?;

    match my_node_info.node_type {
        NodeType::Cloud => cloud::create_receipt_for_edge(&my_node_info, receipt),
        NodeType::Edge => edge::create_receipt_for_cloud(&my_node_info, receipt),
    }
}

/**
 * Create an encrypted Message between the given Source and Destination so that only they can read it
 */
#[hdk_extern]
pub fn create_message(message: Message) -> ExternResult<EntryHash> {
    let my_node_info = query_my_node_info()?;

    match my_node_info.node_type {
        NodeType::Cloud => cloud::create_message_for_edge(&my_node_info, message),
        NodeType::Edge => edge::create_message_for_cloud(&my_node_info, message),
    }
}
