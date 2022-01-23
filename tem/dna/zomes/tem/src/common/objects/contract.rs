use chrono::{
    serde::{ts_milliseconds, ts_milliseconds_option},
    DateTime, Utc,
};
use hdk::prelude::*;

use crate::common::{
    encrypted_data::EncryptableObject,
    node_info::{ApplicationId, NodeId, NodeInfo},
};

use super::{filter::QueryFilter, utils::get_counterparty};

#[derive(Debug, Clone, Serialize, Deserialize, SerializedBytes)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub revision_id: usize,
    pub contract_number: String,
    pub source: ApplicationId,
    pub destination: ApplicationId,
    #[serde(rename = "type")]
    pub contract_type: String,
    pub active: String,
    #[serde(with = "ts_milliseconds")]
    pub valid_from: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub valid_to: DateTime<Utc>,
    pub payload: SerializedBytes,
    pub status: String,
    pub revision_number: String,
}

impl Contract {
    pub fn get_counterparty(&self, my_node_info: &NodeInfo) -> ExternResult<NodeId> {
        get_counterparty(my_node_info, &self.source, &self.destination)
    }
}

impl EncryptableObject for Contract {
    fn entry_type() -> String {
        "contract".into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, SerializedBytes)]
#[serde(rename_all = "camelCase")]
pub struct ContractFilter {
    pub contract_number: Option<String>,
    pub source: Option<ApplicationId>,
    pub destination: Option<ApplicationId>,
    #[serde(rename = "type")]
    pub contract_type: Option<String>,
    pub active: Option<String>,
    #[serde(default)]
    #[serde(with = "ts_milliseconds_option")]
    pub valid_from: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(with = "ts_milliseconds_option")]
    pub valid_to: Option<DateTime<Utc>>,

    pub status: Option<String>,
    pub revision_number: Option<String>,
}

impl QueryFilter for Contract {
    type FilterObject = ContractFilter;
    fn apply_filter(&self, filter: &ContractFilter) -> bool {
        Self::filter_by_string(&self.contract_number, &filter.contract_number)
            && Self::filter_by_application_id(&self.source, &filter.source)
            && Self::filter_by_application_id(&self.destination, &filter.destination)
            && Self::filter_by_start_date(&self.valid_from, &filter.valid_from)
            && Self::filter_by_end_date(&self.valid_to, &filter.valid_to)
            && Self::filter_by_string(&self.status, &filter.status)
            && Self::filter_by_string(&self.revision_number, &filter.revision_number)
    }
}
