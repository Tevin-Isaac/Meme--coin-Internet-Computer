// Import standard library HashMap for managing token ledger
use std::collections::HashMap;

// Import CandidType trait and related types for Candid serialization
use ic_cdk::export::candid::{CandidType, Principal};
use ic_cdk::export::candid::Nat;

// Import ICRCLedger canister for ledger management
use ic_cdk::api::call::ICRCLedger;

// Define the ICRC1 meme token canister
#[derive(CandidType)]
struct ICRC1MemeToken {
    id: Nat, // Token identifier
    name: String, // Token name
    symbol: String, // Token symbol
    total_supply: Nat, // Total token supply
    ledger: HashMap<Principal, Nat>, // Token balances ledger
    meme_url: String, // Meme URL associated with the token
    meme_description: String, // Description of the meme
    meme_creator: String, // Creator of the meme
}

impl ICRC1MemeToken {
    // Initialize the ICRC1 meme token with the specified parameters
    pub fn new(id: Nat, meme_url: String, meme_description: String, meme_creator: String) -> Self {
        // Total supply set to 10 billion tokens (10,000,000,000)
        let total_supply: Nat = 10_000_000_000;
        
        ICRC1MemeToken {
            id,
            name: "Kadudu".to_string(), // Set the token name to "Kadudu"
            symbol: "KD".to_string(), // Example symbol
            total_supply,
            ledger: HashMap::new(),
            meme_url,
            meme_description, // Assign the meme description
            meme_creator, // Assign the meme creator's name
        }
    }

    // Transfer tokens from one account to another
    pub fn transfer(&mut self, from: Principal, to: Principal, amount: Nat) -> Result<(), String> {
        if let Some(balance) = self.ledger.get_mut(&from) {
            if *balance >= amount {
                *balance -= amount;
                *self.ledger.entry(to).or_insert(0) += amount;
                Ok(())
            } else {
                Err("Insufficient balance".to_string())
            }
        } else {
            Err("Account not found".to_string())
        }
    }

    // Get the balance of tokens for a specific account
    pub fn balance_of(&self, account: Principal) -> Nat {
        *self.ledger.get(&account).unwrap_or(&0)
    }

    // Mint new tokens
    pub fn mint(&mut self, to: Principal, amount: Nat) {
        *self.ledger.entry(to).or_insert(0) += amount;
        self.total_supply += amount;
    }

    // Burn tokens
    pub fn burn(&mut self, from: Principal, amount: Nat) -> Result<(), String> {
        if let Some(balance) = self.ledger.get_mut(&from) {
            if *balance >= amount {
                *balance -= amount;
                self.total_supply -= amount;
                Ok(())
            } else {
                Err("Insufficient balance".to_string())
            }
        } else {
            Err("Account not found".to_string())
        }
    }

    // Get the meme creator
    pub fn get_meme_creator(&self) -> String {
        self.meme_creator.clone()
    }

    // Get the meme description
    pub fn get_meme_description(&self) -> String {
        self.meme_description.clone()
    }
}
