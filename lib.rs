#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod numnft {

    use ink::prelude::string::String;

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
            self.owner_name.clone()
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner.clone()
        }

        #[ink(message)]
        pub fn get_timestamp(&self) -> Timestamp {
            self.creationtime
        }

        #[ink(message)]
        pub fn sell(&mut self, new_owner: AccountId, new_owner_name: String) {
            if Self::env().caller() == self.owner {
                self.owner_name = new_owner_name;
                self.owner = new_owner;
            } else {
                panic!("Only the legitimate owner of this NFT can sell this");
            }
        }
    }
}
