use hdk::prelude::*;


#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
pub struct TimeRange {
    pub start_time: Option<Timestamp>,
    pub end_time: Option<Timestamp>,
}

pub fn is_timestamp_in_range(timestamp: &Timestamp, time_range: &TimeRange) -> bool {
    if let Some(start_timestamp) = time_range.start_time {
        if timestamp.lt(&start_timestamp) {
            return false;
        }
    }

    if let Some(end_timestamp) = time_range.end_time {
        if timestamp.gt(&end_timestamp) {
            return false;
        }
    }

    true
}
