use chrono::{DateTime, Duration, Utc};
use crate::common::node_info::ApplicationId;
use hdk::prelude::*;

// This is the interval within which a filter by time will match an entry
const MS_TIME_RANGE: i64 = 60_000;

pub trait QueryFilter {
    type FilterObject;
    fn apply_filter(&self, filter: &Self::FilterObject) -> bool;

    fn filter_by_usize(actual_value: usize, filter_value: Option<usize>) -> bool {
        if let Some(filter) = filter_value {
            if actual_value != filter {
                return false;
            }
        }
        return true;
    }

    fn filter_by_string(actual_value: &String, filter_value: &Option<String>) -> bool {
        if let Some(filter) = filter_value {
            if !actual_value.eq(filter) {
                return false;
            }
        }
        return true;
    }

    fn filter_by_pub_key(
        actual_value: &AgentPubKey,
        filter_value: &Option<AgentPubKey>,
    ) -> bool {
        if let Some(filter) = filter_value {
            if !actual_value.eq(filter) {
                return false;
            }
        }
        return true;
    }
    fn filter_by_application_id(
        actual_value: &ApplicationId,
        filter_value: &Option<ApplicationId>,
    ) -> bool {
        return Self::filter_by_string(&actual_value.0, &filter_value.clone().map(|id| id.0));
    }

    fn filter_by_start_date(
        actual_date: &DateTime<Utc>,
        filter_start_date: &Option<DateTime<Utc>>,
    ) -> bool {
        if let Some(start_date) = filter_start_date {
            if actual_date.lt(start_date) {
                return false;
            }
        }
        return true;
    }

    fn filter_by_end_date(
        actual_date: &DateTime<Utc>,
        filter_end_date: &Option<DateTime<Utc>>,
    ) -> bool {
        if let Some(end_date) = filter_end_date {
            if actual_date.gt(end_date) {
                return false;
            }
        }
        return true;
    }

    fn filter_by_time_range(
        actual_date: &DateTime<Utc>,
        maybe_filter_date: &Option<DateTime<Utc>>,
    ) -> bool {
        let time_range = Duration::milliseconds(MS_TIME_RANGE);
        if let Some(filter_date) = maybe_filter_date {
            let max_date = filter_date.checked_add_signed(time_range);
            let min_date = filter_date.checked_sub_signed(time_range);

            warn!("adsf {} {:?} {:?}", actual_date, max_date, min_date);

            return match (max_date, min_date) {
                (Some(max), Some(min)) => actual_date.lt(&max) && actual_date.gt(&min),
                _ => false,
            };
        }
        return true;
    }
}
