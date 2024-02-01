#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod numnft {

    use ink::prelude::string::String;
    // use ink_e2e::env_logger::fmt::Timestamp;

    #[ink(storage)]
    pub struct NFT {
        owner_name: String,
        owner: AccountId,
        creationtime: Timestamp,
        value: u32,
    }

    impl NFT {
        #[ink[constructor]]
        pub fn new(own_name: String, number: u32) -> Self {
            Self {
                owner_name: own_name,
                owner: Self::env().caller(),
                creationtime: Self::env().block_timestamp(),
                value: number,
            }
        }

        #[ink(message)]
        pub fn get_val(&self) -> u32 {
            self.value
        }

        #[ink(message)]
        pub fn get_owner_name(&self) -> String {
            let own = self.owner_name.clone();
            own
        }

        #[ink(message)]
        pub fn sell(&mut self, new_owner: AccountId, new_owner_name: String) {
            if Self::env().caller() == self.owner {
                self.owner_name = new_owner_name;
                self.owner = new_owner;
            } else {
                panic!("Error You are not the owner");
            }
        }
    }
}
