//! This contract demonstrates a sample implementation of the DAO interface.
use crate::admin::{has_administrator, read_administrator, write_administrator};
use crate::founder::{add_founder, remove_founder, is_founder};
use crate::investor::{add_investor, remove_investor, is_investor};
use crate::proposal::{create_proposal, read_proposal, update_proposal_status};
use crate::voting::{start_voting, tally_votes};
use crate::execution::{execute_action};
use crate::milestone::{create_milestone, update_milestone};
use crate::event;
use crate::metadata::{read_description, write_description};
use soroban_sdk::{contractimpl, Address, String, Env};

pub trait DAOInterface {
    fn initialize(e: Env, admin: Address, description: String);

    fn add_founder(e: Env, admin: Address, founder: Address);

    fn remove_founder(e: Env, admin: Address, founder: Address);

    fn add_investor(e: Env, admin: Address, investor: Address);

    fn remove_investor(e: Env, admin: Address, investor: Address);

    fn create_proposal(e: Env, creator: Address, title: String, description: String, action: String);

    fn vote_on_proposal(e: Env, member: Address, proposal_id: u64, vote: bool);

    fn execute_proposal(e: Env, executor: Address, proposal_id: u64);

    fn create_milestone(e: Env, creator: Address, description: String);

    fn update_milestone_status(e: Env, milestone_id: u64, status: String);

    fn set_admin(e: Env, new_admin: Address);

    fn get_description(e: Env) -> String;
}

#[contract]
pub struct DAO;

#[contractimpl]
impl DAOInterface for DAO {
    fn initialize(e: Env, admin: Address, description: String) {
        if has_administrator(&e) {
            panic!("DAO already initialized")
        }
        write_administrator(&e, &admin);
        write_description(&e, description);
    }

    fn add_founder(e: Env, admin: Address, founder: Address) {
        admin.require_auth();
        add_founder(&e, founder);
    }

    fn remove_founder(e: Env, admin: Address, founder: Address) {
        admin.require_auth();
        remove_founder(&e, founder);
    }

    fn add_investor(e: Env, admin: Address, investor: Address) {
        admin.require_auth();
        add_investor(&e, investor);
    }

    fn remove_investor(e: Env, admin: Address, investor: Address) {
        admin.require_auth();
        remove_investor(&e, investor);
    }

    fn create_proposal(e: Env, creator: Address, title: String, description: String, action: String) {
        creator.require_auth();
        if !(is_founder(&e, &creator) || is_investor(&e, &creator)) {
            panic!("Only founders or investors can create a proposal");
        }
        create_proposal(&e, creator, title, description, action);
    }

    fn vote_on_proposal(e: Env, member: Address, proposal_id: u64, vote: bool) {
        member.require_auth();
        if !(is_founder(&e, &member) || is_investor(&e, &member)) {
            panic!("Only founders or investors can vote on a proposal");
        }
        start_voting(&e, member, proposal_id, vote);
    }

    fn execute_proposal(e: Env, executor: Address, proposal_id: u64) {
        executor.require_auth();
        if !(is_founder(&e, &executor) || is_investor(&e, &executor)) {
            panic!("Only founders or investors can execute a proposal");
        }
        execute_action(&e, executor, proposal_id);
    }

    fn create_milestone(e: Env, creator: Address, description: String) {
        creator.require_auth();
        if !(is_founder(&e, &creator) || is_investor(&e, &creator)) {
            panic!("Only founders or investors can create a milestone");
        }
        create_milestone(&e, description);
    }

    fn update_milestone_status(e: Env, milestone_id: u64, status: String) {
        if !is_founder(&e, &executor) {
            panic!("Only founders can update the status of a milestone");
        }
        update_milestone(&e, milestone_id, status);
    }

    fn set_admin(e: Env, new_admin: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();
        write_administrator(&e, &new_admin);
        event::set_admin(&e, admin, new_admin);
    }

    fn get_description(e: Env) -> String {
        read_description(&e)
    }
}
