use soroban_sdk::{Env, Address};
use super::storage_types::{DataKey, Milestone, Investor};

fn check_investor(env: &Env, investor_address: &Address) -> bool {
    match env.data().get(DataKey::Investor(investor_address.clone())) {
        Some(_) => true, // If there's a record for this address, then it's a registered investor.
        None => false,
    }
}

fn add_investor_shares(env: &Env, shares_to_add: u32, investor_address: &Address) {
    // Placeholder implementation. Add shares for the investor.
    // Perhaps updating the investor's total shares in the data storage or any other logic.
}

fn unlock_milestone(env: &Env, milestone_id: u32, investor_address: &Address) {
    if !check_investor(env, &investor_address) {
        panic!("Only registered investors can unlock milestones.");
    }

    let mut investor: Investor = match env.data().get(DataKey::Investor(investor_address.clone())) {
        Some(i) => i.unwrap(),
        None => panic!("Investor not found!"), // This panic might be redundant since you've checked the investor existence before.
    };

    let milestone: &mut Milestone = investor.milestones.get_mut(milestone_id as usize).expect("Invalid milestone ID!");

    if !milestone.unlocked {
        milestone.unlocked = true;
        add_investor_shares(env, milestone.s_unlocked, &investor_address);
    }
}
