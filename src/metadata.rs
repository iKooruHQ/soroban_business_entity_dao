use soroban_sdk::{Bytes};


// Assuming we have a DAOMetadata structure in our SDK
#[derive(Clone, Debug)]
pub struct DAOMetadata {
    pub name: Bytes,         // Name of the DAO
    pub description: Bytes,  // Short description or mission statement
    pub version: Bytes,      // Version or iteration of this DAO
}
