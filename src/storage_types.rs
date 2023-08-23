use soroban_sdk::{contracttype, Address,String, Vec};

// Represents a proposal key 
#[derive(Clone)]
#[contracttype]
pub struct ProposalDataKey {
    pub proposer: Address,
    pub proposal_id: u32,
}

#[derive(Clone)]
pub struct Proposal {
    pub tot_votes: u32,          // Total votes for the proposal
    pub end_time: u64,           // End time for the proposal (likely a timestamp)
    pub instr: Vec<Instruction>, // List of instructions associated with the proposal

}

#[derive(Clone)]
pub struct Founder {
    pub address: Address,
    // Add more attributes if needed, like name, shares, etc.
}

#[derive(Clone)]
pub struct Investor {
    pub address: Address,
    pub milestones: Vec<Milestone>,
}

#[derive(Clone)]
pub struct Instruction {
    pub c_id: String,      // Contract ID or address
    pub fun_name: String,  // Function name to be invoked
    pub args: Vec<String>, // Arguments for the function
 
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

#[contracttype]
#[derive(Clone, Debug)]
pub struct Milestone {
    pub s_unlocked: i32,
    pub unlocked: bool,
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
    // pub description: String,
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
    Investor(Address),
    FounderShares(Address),
    Founder(Address), 
    Admin,
}
