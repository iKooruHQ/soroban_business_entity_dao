use soroban_sdk::{Bytes, Env};
use soroban_token_sdk::{TokenMetadata, TokenUtils};

// Assuming we have a DAOMetadata structure in our SDK
#[derive(Clone, Debug)]
pub struct DAOMetadata {
    pub name: Bytes,         // Name of the DAO
    pub description: Bytes,  // Short description or mission statement
    pub version: Bytes,      // Version or iteration of this DAO
}

pub fn read_name(e: &Env) -> Bytes {
    let util = DAOUtils::new(e); // Notice that this changed from TokenUtils
    util.get_metadata_unchecked().unwrap().name
}

pub fn read_description(e: &Env) -> Bytes {
    let util = DAOUtils::new(e);
    util.get_metadata_unchecked().unwrap().description
}

pub fn read_version(e: &Env) -> Bytes {
    let util = DAOUtils::new(e);
    util.get_metadata_unchecked().unwrap().version
}

pub fn write_metadata(e: &Env, metadata: DAOMetadata) {
    let util = DAOUtils::new(e); // Again, assuming such utility exists for DAOs in your SDK
    util.set_metadata(&metadata);
}
