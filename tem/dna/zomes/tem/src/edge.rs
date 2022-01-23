use hdk::prelude::*;

use crate::common::{
    node_info::{ApplicationId, NodeInfo},
    objects::{
        contract::{Contract, ContractFilter},
        filter::QueryFilter,
        message::{Message, MessageFilter},
        receipt::{find_contract_for_receipt, Receipt, ReceiptFilter},
    },
};

use self::encrypt_for_cloud::get_and_decrypt_entries_for_cloud;

pub mod associate;
mod encrypt_for_cloud;

/** Functions executed only by the edge */

#[hdk_extern]
pub fn associate_with_cloud(my_application_id: ApplicationId) -> ExternResult<()> {
    associate::associate_with_cloud(my_application_id)
}

pub fn create_contract_for_cloud(
    my_node_info: &NodeInfo,
    contract: Contract,
) -> ExternResult<EntryHash> {
    let counterparty = contract.get_counterparty(&my_node_info)?;
    encrypt_for_cloud::create_encrypted_data_for_cloud(my_node_info, counterparty.1, contract)
}

pub fn get_and_filter_contracts(
    my_node_info: &NodeInfo,
    contract_filter: ContractFilter,
) -> ExternResult<Vec<(Contract, Timestamp)>> {
    let contracts: Vec<(Contract, Timestamp)> = get_and_decrypt_entries_for_cloud(my_node_info)?;

    Ok(contracts
        .into_iter()
        .filter(|(c, _)| c.apply_filter(&contract_filter))
        .collect())
}

pub fn create_receipt_for_cloud(
    my_node_info: &NodeInfo,
    receipt: Receipt,
) -> ExternResult<EntryHash> {
    let contracts: Vec<(Contract, Timestamp)> = get_and_decrypt_entries_for_cloud(my_node_info)?;

    let contracts = contracts.into_iter().map(|(c, _)| c).collect();

    let contract = find_contract_for_receipt(contracts, &receipt)?;

    let counterparty = contract.get_counterparty(&my_node_info)?;
    encrypt_for_cloud::create_encrypted_data_for_cloud(my_node_info, counterparty.1, receipt)
}

pub fn get_and_filter_receipts(
    my_node_info: &NodeInfo,
    receipt_filter: ReceiptFilter,
) -> ExternResult<Vec<(Receipt, Timestamp)>> {
    let receipts: Vec<(Receipt, Timestamp)> = get_and_decrypt_entries_for_cloud(my_node_info)?;

    Ok(receipts
        .into_iter()
        .filter(|(c, _)| c.apply_filter(&receipt_filter))
        .collect())
}

pub fn create_message_for_cloud(
    my_node_info: &NodeInfo,
    message: Message,
) -> ExternResult<EntryHash> {
    let counterparty = message.get_counterparty(my_node_info)?;
    encrypt_for_cloud::create_encrypted_data_for_cloud(my_node_info, counterparty.1, message)
}

pub fn get_and_filter_messages(
    my_node_info: &NodeInfo,
    message_filter: MessageFilter,
) -> ExternResult<Vec<(Message, Timestamp)>> {
    let messages: Vec<(Message, Timestamp)> = get_and_decrypt_entries_for_cloud(my_node_info)?;

    Ok(messages
        .into_iter()
        .filter(|(c, _)| c.apply_filter(&message_filter))
        .collect())
}

