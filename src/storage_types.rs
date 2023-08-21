use soroban_sdk::{contracttype, Address};

// Represents a proposal key 
#[derive(Clone)]
#[contracttype]
pub struct ProposalDataKey {
    pub proposer: Address,
    pub proposal_id: u32,
}

// Represents the voting details of an individual towards a specific proposal
#[derive(Clone)]
#[contracttype]
pub struct VotingDataKey {
    pub voter: Address,
    pub proposal_id: u32,
}

// Represents the milestone details of a specific proposal
#[derive(Clone)]
#[contracttype]
pub struct MilestoneDataKey {
    pub proposal_id: u32,
    pub milestone_number: u32,
}

// Represents the voting state (Yes/No) of an individual towards a specific proposal
#[contracttype]
pub struct VotingValue {
    pub vote: bool,
    pub vote_weight: i128,
}

// Represents the milestone details for a proposal
#[contracttype]
pub struct MilestoneValue {
    pub description: String,
    pub completion_date: u32,
}

// Enum representing the different types of DAO data we can store
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Proposal(ProposalDataKey),
    Voting(VotingDataKey),
    Milestone(MilestoneDataKey),
    InvestorFunds(Address),
    FounderShares(Address),
    Admin,
}
