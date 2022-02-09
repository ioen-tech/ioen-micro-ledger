use hdk::prelude::*;

use crate::err;

use super::{time_range, time_range::TimeRange};

pub fn query_and_convert<T: EntryDefRegistration + TryFrom<Entry> + TryFrom<SerializedBytes, Error = SerializedBytesError>>(
    time_range: TimeRange,
) -> ExternResult<Vec<(T, Timestamp)>> {
    let query_result = query_elements::<T>(time_range)?;

    let result = query_result
        .into_iter()
        .map(|element| {
            let entry: T = element
                .entry()
                .to_app_option()?
                .ok_or(err("Could not deserialize entry"))?;

            Ok((entry, element.header().timestamp()))
        })
        .collect::<ExternResult<Vec<(T, Timestamp)>>>()?;

    Ok(result)
}

pub fn query_elements<T: EntryDefRegistration>(
    time_range: TimeRange,
) -> ExternResult<Vec<Element>> {
    let filter = ChainQueryFilter::new()
        .entry_type(EntryType::App(AppEntryType::new(
            entry_def_index!(T)?,
            zome_info()?.id,
            EntryVisibility::Private,
        )))
        .header_type(HeaderType::Create)
        .include_entries(true);

    let query_result: Vec<Element> = query(filter)?;

    let filtered: Vec<Element> = query_result
        .into_iter()
        .filter(|element: &Element| {
            time_range::is_timestamp_in_range(&element.header().timestamp(), &time_range)
        })
        .map(|element| element.clone())
        .collect();

    Ok(filtered.clone())
}
