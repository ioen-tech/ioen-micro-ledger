use hdk::prelude::*;

mod consumer;
mod producer;
mod supply_agreement;
mod supply_block;
mod bill;

use consumer::Consumer;
use producer::Producer;
use supply_agreement::SupplyAgreement;
use supply_block::SupplyBlock;
use bill::Bill;

entry_defs![
  PathEntry::entry_def(),
  Consumer::entry_def(),
  Producer::entry_def(),
  SupplyAgreement::entry_def(),
  SupplyBlock::entry_def(),
  Bill::entry_def()
];

#[hdk_extern]
fn who_am_i(_: ()) -> ExternResult<AgentPubKey> {
    let agent_info = agent_info()?;

    Ok(AgentPubKey::from(agent_info.agent_initial_pubkey))
}