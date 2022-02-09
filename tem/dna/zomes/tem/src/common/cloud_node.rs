use hdk::prelude::*;

/**
 * Gets the market node from the global path
 */
pub fn get_cloud_node() -> ExternResult<Option<AgentPubKey>> {
    let path = all_clouds_path();

    let links = get_links(path.path_entry_hash()?, None)?;

    match links.len() {
        0 => Ok(None),
        1 => Ok(Some(links[0].target.clone().into())),
        _ => Err(crate::err(
            "There is more than one node declared as a market",
        )),
    }
}

pub fn all_clouds_path() -> Path {
    Path::from("all_clouds")
}
