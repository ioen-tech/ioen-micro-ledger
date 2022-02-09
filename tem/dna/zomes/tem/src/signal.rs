use chrono::{TimeZone, Utc};
use hdk::prelude::*;

use crate::{
    common::{
        encrypted_data::query_number_of_committed_encrypted_objects,
        node_info::{query_my_node_info, NodeId},
        objects::{contract::Contract, receipt::Receipt},
    },
    edge::create_receipt_for_cloud,
};

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(tag = "signal_name", content = "signal_payload")]
pub enum SignalPayload {
    ContractCreated {
        creator_cloud: NodeId,
        contract: Contract,
    },
}

#[hdk_extern]
pub fn recv_remote_signal(signal: ExternIO) -> ExternResult<()> {
    let payload: SignalPayload = signal.decode()?;

    match payload {
        SignalPayload::ContractCreated { contract, .. } => {
            let now = sys_time()?;

            let event_time = Utc.timestamp(now.as_micros(), 0);

            let receipt_id = query_number_of_committed_encrypted_objects::<Receipt>()?;

            let receipt = Receipt {
                active: contract.active,
                contract_id: contract.revision_id,
                event_time,
                payload: ().try_into()?,
                receipt_id,
                receipt_type: contract.contract_type,
            };

            let my_node_info = query_my_node_info()?;

            let mut success = false;

            while !success {
                if let Ok(_) = create_receipt_for_cloud(&my_node_info, receipt.clone()) {
                    success = true;
                }
            }
        }
    }
    Ok(())
}
