// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{ext_contract, log, near_bindgen, Promise, AccountId};


#[ext_contract(ext_greeting)]
trait Greeting {
    fn get_greeting(&self) -> String;
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[borsh(crate = "near_sdk::borsh")]
pub struct CrossContract {}

// Define the Default
impl Default for CrossContract {
    fn default() -> Self {
        Self {}
    }
}



// Implement the contract structure
#[near_bindgen]
impl CrossContract {
    pub fn cross_contract_call(&self) -> Promise  {
        let greeting_account_id: AccountId = "real-support.testnet".parse().unwrap();
        ext_greeting::ext(greeting_account_id)
            .get_greeting()
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
