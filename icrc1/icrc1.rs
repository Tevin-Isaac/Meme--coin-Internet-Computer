use std::collections::HashMap;
use candid::CandidType;
use candid::Nat;
use ic_cdk::candid::{CandidType, Principal};
use icrc1_ledger_canister::ICRCLedger;

#[derive(CandidType)]
struct ICRC1MemeToken {
    id: Nat,
    name: String,
    symbol: String,
    total_supply: Nat,
    ledger: HashMap<Principal, Nat>,
    meme_url: String,
    meme_description: String,
    meme_creator: Principal,
}

impl ICRC1MemeToken {
    pub fn new(
        id: Nat,
        total_supply: Nat,
        meme_url: String,
        meme_description: String,
        meme_creator: Principal,
    ) -> Self {
        ICRC1MemeToken {
            id,
            name: "Kadudu".to_string(),
            symbol: "KD".to_string(),
            total_supply,
            ledger: HashMap::new(),
            meme_url,
            meme_description,
            meme_creator,
        }
    }

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

    pub fn balance_of(&self, account: Principal) -> Nat {
        *self.ledger.get(&account).unwrap_or(&0)
    }

    pub fn mint(&mut self, to: Principal, amount: Nat) {
        *self.ledger.entry(to).or_insert(0) += amount;
        self.total_supply += amount;
    }

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

    pub fn get_meme_creator(&self) -> Principal {
        self.meme_creator
    }

    pub fn get_meme_description(&self) -> String {
        self.meme_description.clone()
    }
}
