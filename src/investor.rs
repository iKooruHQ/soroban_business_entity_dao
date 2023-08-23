use soroban_sdk::{Env, Address};
use super::storage_types::DataKey;

fn add_investor(env: &Env, investor_address: Address) {
    if !crate::admin::check_admin(env) {
        panic!("Only the admin can add an investor.");
    }
    env.storage().instance().set(&DataKey::Investor(investor_address.clone()), &investor_address);
}
