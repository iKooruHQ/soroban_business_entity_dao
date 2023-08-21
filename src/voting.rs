use soroban_sdk::{Env, Address};
use super::storage_types::{DataKey, ProposalVote};

fn vote(env: Env, prop_id: u32) {
    assert!(!voted(&env, env.invoker(), prop_id));
    let mut prop = env
        .data()
        .get::<_, Proposal>(DataKey::Proposal(prop_id))
        .unwrap()
        .unwrap();
    assert!(prop.end_time > env.ledger().timestamp());
    let member_shares = get_shares(&env, env.invoker());
    prop.tot_votes = prop.tot_votes + member_shares;
    env.data().set(DataKey::Proposal(prop_id), prop);
    set_voted(&env, env.invoker(), prop_id);
}

fn voted(env: &Env, voter: Address, prop_id: u32) -> bool {
    env.data().get(DataKey::Voted(ProposalVote {
        voter,
        prop_id,
    })).unwrap_or(Ok(false)).unwrap()
}

fn set_voted(env: &Env, voter: Address, prop_id: u32) {
    env.data().set(DataKey::Voted(ProposalVote {
        voter,
        prop_id,
    }), true);
}
