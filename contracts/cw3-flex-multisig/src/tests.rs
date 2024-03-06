use cosmwasm_std::{
    testing::{mock_dependencies, mock_env},
    Addr,
};
use cw3::{Ballot, Status, Votes};
use cw_utils::{Expiration, Threshold};

use crate::{
    migrate::{migrate_ballots, migrate_proposal, OldProposal, OLD_BALLOTS, OLD_PROPOSALS},
    state::{BALLOTS, PROPOSALS},
};

#[test]
fn test_proposal_migration() {
    let mut deps = mock_dependencies();
    OLD_PROPOSALS
        .save(
            deps.as_mut().storage,
            1,
            &OldProposal {
                title: "foobar".to_string(),
                description: "foobar".to_string(),
                start_height: 1,
                expires: Expiration::Never {},
                msgs: vec![],
                status: Status::Passed,
                threshold: Threshold::AbsoluteCount { weight: 1 },
                total_weight: 1,
                votes: Votes {
                    yes: 1,
                    no: 0,
                    abstain: 0,
                    veto: 0,
                },
            },
        )
        .unwrap();

    migrate_proposal(deps.as_mut().storage, mock_env().contract.address).unwrap();

    let proposal = PROPOSALS.load(&deps.storage, 1u64).unwrap();
    assert_eq!(proposal.proposer, mock_env().contract.address);
}

#[test]
fn test_ballots_migration() {
    let mut deps = mock_dependencies();
    let voter = deps.as_ref().api.addr_canonicalize("foobar").unwrap();
    let deps_mut = deps.as_mut();
    let api = deps_mut.api;
    OLD_BALLOTS
        .save(
            deps_mut.storage,
            (1, &voter),
            &Ballot {
                weight: 1,
                vote: cw3::Vote::Yes,
            },
        )
        .unwrap();

    migrate_ballots(deps_mut.storage, api).unwrap();

    let proposal = BALLOTS
        .load(&deps.storage, (1u64, &Addr::unchecked("foobar")))
        .unwrap();
    assert_eq!(proposal.weight, 1);
}