fn init_as_admin_if_not_set(env: &Env) {
    // If there's no admin set, make the invoker the admin
    if get_admin(env).is_none() {
        set_admin(env, get_invoker_from_env(env));
    }
}

use super::storage_types::DataKey;
use soroban_sdk::{Address, Env};

pub fn check_admin(env: &Env) -> bool {
    let key = DataKey::Admin;
    match env.storage().instance().get::<DataKey, Address>(&key) {
        Some(stored_admin) => &stored_admin == &get_invoker_from_env(env),

        None => false,
    }
}

fn get_admin(env: &Env) -> Option<Address> {
    let key = DataKey::Admin;
    env.storage().instance().get::<DataKey, Address>(&key)
}

fn set_admin(env: &Env, admin: Address) {
    let key = DataKey::Admin;
    env.storage().instance().set(&key, &admin);
}

// Use this function to get the invoker's address, or update it according to Soroban SDK
fn get_invoker_from_env(env: &Env) -> Address {
    // Replace the following with the actual method to get the invoker's address from Soroban SDK
    // Example: Address::from_env(env)
    panic!("Replace this with the actual method to get the invoker's address from Soroban SDK");
}
