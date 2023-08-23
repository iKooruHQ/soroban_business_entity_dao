use soroban_sdk::{Env, Address}; 
use super::storage_types::DataKey;  // Import only DataKey

fn add_founder(env: &Env, founder_address: Address) { // Change the function signature
    if !crate::admin::check_admin(env) { 
        panic!("Only the admin can add a founder.");
    }
    env.storage().instance().set(&DataKey::Founder(founder_address.clone()), &founder_address);
}
