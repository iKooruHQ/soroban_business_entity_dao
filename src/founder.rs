use soroban_sdk::Env;
use super::storage_types::{DataKey, Founder};

fn add_founder(env: Env, founder: Founder) {
    if !check_admin(&env) {
        panic!("Only the admin can add a founder.");
    }
    env.data().set(DataKey::Founder(founder.address.clone()), founder);
}

