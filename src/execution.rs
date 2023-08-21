
use soroban_sdk::{Env};
use super::storage_types::{DataKey};

fn execute(env: Env, prop_id: u32) {
    assert!(!get_executed(&env, prop_id));
    let prop = env
        .data()
        .get::<_, Proposal>(DataKey::Proposal(prop_id))
        .unwrap()
        .unwrap();
    assert!(prop.end_time > env.ledger().timestamp());
    assert!(prop.tot_votes > tot_shares(&env) / 2);
    for instr in prop.instr.iter() {
        if env.current_contract() == instr.c_id {
            // Handle internal contract calls
        } else {
            env.invoke_contract(&instr.c_id, &instr.fun_name, &instr.args);
        }
    }
    set_executed(&env, prop_id);
}

fn set_executed(env: &Env, prop_id: u32) {
    env.data().set(DataKey::Executed(prop_id), true)
}

fn get_executed(env: &Env, prop_id: u32) -> bool {
    env.data().get(DataKey::Executed(prop_id)).unwrap_or(Ok(false)).unwrap()
}
