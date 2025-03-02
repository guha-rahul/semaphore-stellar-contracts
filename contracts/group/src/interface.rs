use crate::{
    datatypes::{Error, Member},
    proof::Proof,
};
use soroban_sdk::{Address, Bytes, Env, Vec};

pub trait SemaphoreGroupInterface {
    // Group Management
    fn create_group(env: Env, group_id: u32, admin: Address) -> Result<(), Error>;
    fn update_group_admin(env: Env, group_id: u32, new_admin: Address) -> Result<(), Error>;
    fn accept_group_admin(env: Env, group_id: u32) -> Result<(), Error>;
    fn get_pending_admin(env: Env, group_id: u32) -> Result<Address, Error>;

    // Member Management
    fn add_member(env: Env, group_id: u32, identity_commitment: Bytes) -> Result<(), Error>;
    fn add_members(env: Env, group_id: u32, identity_commitments: Vec<Bytes>) -> Result<(), Error>;
    fn update_member(
        env: Env,
        group_id: u32,
        old_identity_commitment: Bytes,
        new_identity_commitment: Bytes,
    ) -> Result<(), Error>;
    fn remove_member(env: Env, group_id: u32, identity_commitment: Bytes) -> Result<(), Error>;

    // Verification methods
    fn verify_proof(
        env: Env,
        group_id: u32,
        identity_commitment: Bytes,
        proof: Proof,
    ) -> Result<bool, Error>;

    fn get_merkle_root(env: Env, group_id: u32) -> Result<Bytes, Error>;
    fn get_proof(env: Env, group_id: u32, leaf_index: u32) -> Result<Proof, Error>;

    // View Functions
    fn get_group_admin(env: Env, group_id: u32) -> Result<Address, Error>;
    fn get_member(env: Env, group_id: u32, identity_commitment: Bytes) -> Result<Member, Error>;
    fn get_member_count(env: Env, group_id: u32) -> Result<u32, Error>;
    fn is_member(env: Env, group_id: u32, identity_commitment: Bytes) -> Result<bool, Error>;
}
