use soroban_sdk::Env;
use super::storage_types::{DataKey, Investor};

fn add_investor(env: Env, investor: Investor) {
    if !check_admin(&env) {
        panic!("Only the admin can add an investor.");
    }
    env.data().set(DataKey::Investor(investor.address.clone()), investor);
}

