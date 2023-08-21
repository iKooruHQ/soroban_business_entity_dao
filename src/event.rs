use soroban_sdk::{symbol_short, Address, Env, Symbol};

// Proposal events
pub(crate) fn proposal_created(e: &Env, creator: Address, proposal_id: i128) {
    let topics = (symbol_short!("proposal_created"), creator);
    e.events().publish(topics, proposal_id);
}

pub(crate) fn proposal_voted(e: &Env, voter: Address, proposal_id: i128, vote: bool) {
    let topics = (symbol_short!("proposal_voted"), voter, proposal_id);
    e.events().publish(topics, vote);
}

pub(crate) fn proposal_executed(e: &Env, executor: Address, proposal_id: i128) {
    let topics = (symbol_short!("proposal_executed"), executor);
    e.events().publish(topics, proposal_id);
}

// Member events
pub(crate) fn member_added(e: &Env, admin: Address, new_member: Address) {
    let topics = (Symbol::new(e, "member_added"), admin, new_member);
    e.events().publish(topics, ());
}

pub(crate) fn member_left(e: &Env, member: Address) {
    let topics = (symbol_short!("member_left"), member);
    e.events().publish(topics, ());
}

// Investment events
pub(crate) fn investor_joined(e: &Env, investor: Address, amount_invested: i128) {
    let topics = (symbol_short!("investor_joined"), investor);
    e.events().publish(topics, amount_invested);
}

// Milestone events
pub(crate) fn milestone_achieved(e: &Env, milestone_id: i128) {
    let topics = (symbol_short!("milestone_achieved"));
    e.events().publish(topics, milestone_id);
}
