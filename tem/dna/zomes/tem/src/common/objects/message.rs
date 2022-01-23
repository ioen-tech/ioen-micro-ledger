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
pub struct Message {
    pub message_id: usize,
    pub contract_id: usize,

    pub source: ApplicationId,
    pub destination: ApplicationId,

    #[serde(rename = "type")]
    pub message_type: String,

    #[serde(with = "ts_milliseconds")]
    pub message_time: DateTime<Utc>,

    pub payload: SerializedBytes,
}

impl Message {
    pub fn get_counterparty(&self, my_node_info: &NodeInfo) -> ExternResult<NodeId> {
        get_counterparty(my_node_info, &self.source, &self.destination)
    }
}

impl EncryptableObject for Message {
    fn entry_type() -> String {
        "message".into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, SerializedBytes)]
#[serde(rename_all = "camelCase")]
pub struct MessageFilter {
    pub message_id: Option<usize>,
    pub contract_id: Option<usize>,
    pub source: Option<ApplicationId>,
    pub destination: Option<ApplicationId>,

    #[serde(rename = "type")]
    pub message_type: Option<String>,
    #[serde(with = "ts_milliseconds_option")]
    #[serde(default)]
    pub message_time: Option<DateTime<Utc>>,
}

impl QueryFilter for Message {
    type FilterObject = MessageFilter;
    fn apply_filter(&self, filter: &MessageFilter) -> bool {
        Self::filter_by_usize(self.message_id, filter.message_id)
            && Self::filter_by_usize(self.contract_id, filter.contract_id)
            && Self::filter_by_application_id(&self.source, &filter.source)
            && Self::filter_by_application_id(&self.destination, &filter.destination)
            && Self::filter_by_string(&self.message_type, &filter.message_type)
            && Self::filter_by_time_range(&self.message_time, &filter.message_time)
    }
}
