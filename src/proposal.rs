use soroban_sdk::{Env};
use super::storage_types::{DataKey, Proposal};

fn create_proposal(env: Env, proposal: Proposal) -> u32 {
    assert!(proposal.tot_votes == 0);
    let next_id = get_and_inc_prop_id(&env);
    env.data().set(DataKey::Proposal(next_id), proposal.clone());
    next_id
}

fn get_and_inc_prop_id(env: &Env) -> u32 {
    let prev = env
        .data()
        .get(DataKey::ProposalId)
        .unwrap_or(Ok(0u32))
        .unwrap();
    env.data().set(DataKey::ProposalId, prev + 1);
    prev
}
