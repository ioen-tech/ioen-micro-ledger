use chrono::{
    serde::{ts_milliseconds, ts_milliseconds_option},
    DateTime, Utc,
};
use hdk::prelude::*;

use crate::{common::encrypted_data::EncryptableObject, err};

use super::{contract::Contract, filter::QueryFilter};

#[derive(Debug, Clone, Serialize, Deserialize, SerializedBytes)]
#[serde(rename_all = "camelCase")]
pub struct Receipt {
    pub receipt_id: usize,
    pub contract_id: usize,
    #[serde(rename = "type")]
    pub receipt_type: String,
    pub active: String,
    #[serde(with = "ts_milliseconds")]
    pub event_time: DateTime<Utc>,
    pub payload: SerializedBytes,
}

impl EncryptableObject for Receipt {
    fn entry_type() -> String {
        "receipt".into()
    }
}

pub fn find_contract_for_receipt(
    contracts: Vec<Contract>,
    receipt: &Receipt,
) -> ExternResult<Contract> {
    let maybe_contract = contracts
        .into_iter()
        .find(|contract| contract.revision_id == receipt.contract_id);

    match maybe_contract {
        None => Err(err(
            "There is no contract identified with the given contract ID",
        )),
        Some(contract) => Ok(contract),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, SerializedBytes)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptFilter {
    #[serde(rename = "type")]
    pub receipt_type: Option<String>,
    #[serde(with = "ts_milliseconds_option")]
    #[serde(default)]
    pub event_time: Option<DateTime<Utc>>,
}

impl QueryFilter for Receipt {
    type FilterObject = ReceiptFilter;
    fn apply_filter(&self, filter: &ReceiptFilter) -> bool {
        Self::filter_by_string(&self.receipt_type, &filter.receipt_type)
            && Self::filter_by_time_range(&self.event_time, &filter.event_time)
    }
}
