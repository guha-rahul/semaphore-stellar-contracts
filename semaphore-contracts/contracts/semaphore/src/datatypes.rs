use soroban_sdk::{contracttype, contracterror};

#[contracterror]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
    #[repr(u32)]
    pub enum Error {
        GroupDoesNotExist = 1,
        GroupAlreadyExists = 2,
        CallerIsNotTheGroupAdmin = 3,
        CallerIsNotThePendingGroupAdmin = 4,
        MemberAlreadyExists = 5,
        MemberDoesNotExist = 6,
        InvalidIdentityCommitment = 7,
    }

    #[derive(Clone)]
    #[contracttype]
    pub enum DataKey {
        Admin(u32),              // maps group_id -> admin address
        PendingAdmin(u32),       // maps group_id -> pending admin address
        Member(u32, u32),        // maps (group_id, identity_commitment) -> Member
        MemberCount(u32),        // maps group_id -> number of members
    }

    #[derive(Clone)]
    #[contracttype]
    pub struct Member {
        pub identity_commitment: u32,
        pub group_id: u32,
        pub index: u32,         // Position in the group
    }
