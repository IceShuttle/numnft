#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod numnft {

    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct NFT {
        author: String,
        owner: String,
        value: u32,
    }

    impl NFT {
        #[ink[constructor]]
        pub fn new(auth: String, number: u32) -> Self {
            let own = auth.clone();
            Self {
                author: auth,
                owner: own,
                value: number,
            }
        }

        #[ink(message)]
        pub fn get_val(&self) -> u32 {
            self.value
        }

        #[ink(message)]
        pub fn get_author(&self) -> String {
            let auth = self.author.clone();
            auth
        }

        #[ink(message)]
        pub fn get_owner(&self) -> String {
            let own = self.owner.clone();
            own
        }

        #[ink(message)]
        pub fn sell(&mut self, own: String) {
            self.owner = own;
        }
    }
}
