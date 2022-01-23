use hdk::prelude::*;

use crate::{
    common::{node_info::NodeId, encrypted_data::EdgeSecret},
    err,
};

#[hdk_entry(id = "association", visibility = "private")]
#[derive(Clone)]
pub struct CloudEdgeAssociation {
    pub cloud: NodeId,
    pub edge: NodeId,
    pub secret: EdgeSecret,
}

pub fn query_all_my_associations() -> ExternResult<Vec<CloudEdgeAssociation>> {
    let filter = ChainQueryFilter::new()
        .entry_type(EntryType::App(AppEntryType::new(
            entry_def_index!(CloudEdgeAssociation)?,
            ZomeId::from(0),
            EntryVisibility::Private,
        )))
        .header_type(HeaderType::Create)
        .include_entries(true);

    let query_result: Vec<Element> = query(filter)?;

    let associations: Vec<CloudEdgeAssociation> = query_result
        .into_iter()
        .map(|element| {
            element
                .entry()
                .to_app_option()?
                .ok_or(err("Could not deserialize element"))
        })
        .collect::<ExternResult<Vec<CloudEdgeAssociation>>>()?;

    Ok(associations)
}
