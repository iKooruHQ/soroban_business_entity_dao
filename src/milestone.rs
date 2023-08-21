use soroban_sdk::{Env, Address};
use super::storage_types::{DataKey, Milestone, Investor};

fn unlock_milestone(env: Env, milestone_id: u32, investor_address: Address) {
    if !check_investor(&env, investor_address.clone()) {
        panic!("Only registered investors can unlock milestones.");
    }
    let mut investor: Investor = env.data().get(DataKey::Investor(investor_address)).unwrap().unwrap();
    let milestone: &mut Milestone = investor.milestones.get_mut(milestone_id as usize).unwrap();
    if !milestone.unlocked {
        milestone.unlocked = true;
        add_shares(&env, milestone.s_unlocked, investor.address.clone());
    }
}
