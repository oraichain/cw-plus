use cosmwasm_std::{Addr, Api, CanonicalAddr, CosmosMsg, Empty, Order, StdResult, Storage};
use cw3::{Ballot, Proposal, Status, Votes};
use cw_storage_plus::Map;
use cw_utils::{Expiration, Threshold};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::{BALLOTS, PROPOSALS};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OldProposal {
    pub title: String,
    pub description: String,
    pub start_height: u64,
    pub expires: Expiration,
    pub msgs: Vec<CosmosMsg<Empty>>,
    pub status: Status,
    /// pass requirements
    pub threshold: Threshold,
    // the total weight when the proposal started (used to calculate percentages)
    pub total_weight: u64,
    // summary of existing votes
    pub votes: Votes,
}

pub const OLD_PROPOSALS: Map<u64, OldProposal> = Map::new("proposals");
pub const OLD_BALLOTS: Map<(u64, &[u8]), Ballot> = Map::new("votes");

pub fn migrate_proposal(storage: &mut dyn Storage, contract_address: Addr) -> StdResult<usize> {
    let mut old_proposals: Vec<(u64, OldProposal)> = vec![];
    for (_, key) in OLD_PROPOSALS
        .range(storage, None, None, Order::Ascending)
        .enumerate()
        .map(|key| (key.0, key.1.ok()))
    {
        if let Some(key) = key {
            old_proposals.push(key);
        }
    }
    let total_proposals_len = old_proposals.len();
    for (key, proposal) in old_proposals {
        PROPOSALS.save(
            storage,
            key,
            &Proposal {
                title: proposal.title,
                description: proposal.description,
                start_height: proposal.start_height,
                expires: proposal.expires,
                msgs: proposal.msgs,
                status: proposal.status,
                threshold: proposal.threshold,
                total_weight: proposal.total_weight,
                votes: proposal.votes,
                proposer: contract_address.clone(),
                deposit: None,
            },
        )?;
    }
    Ok(total_proposals_len)
}

pub fn migrate_ballots(storage: &mut dyn Storage, api: &dyn Api) -> StdResult<usize> {
    let mut old_ballots: Vec<((u64, Vec<u8>), Ballot)> = vec![];
    for (_, key) in OLD_BALLOTS
        .range(storage, None, None, Order::Ascending)
        .enumerate()
        .map(|key| (key.0, key.1.ok()))
    {
        if let Some(key) = key {
            old_ballots.push(key);
        }
    }
    let total_proposals_len = old_ballots.len();
    for (key, ballot) in old_ballots {
        let canon_addr = CanonicalAddr::from(key.1);
        let human_addr = api.addr_humanize(&canon_addr)?;
        BALLOTS.save(storage, (key.0, &human_addr), &ballot)?;
    }
    Ok(total_proposals_len)
}
