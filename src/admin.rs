use soroban_sdk::{Env, Address};
use super::storage_types::{DataKey};

fn check_admin(env: &Env) -> bool {
    env.invoker() == env.data().get(DataKey::Admin).unwrap().unwrap()
}

fn get_admin(env: &Env) -> Option<Result<Address, ConversionError>> {
    env.data().get(DataKey::Admin)
}

fn set_admin(env: &Env, admin: Address) {
    env.data().set(DataKey::Admin, admin)
}

