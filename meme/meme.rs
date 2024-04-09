use ic_cdk::export::candid::{CandidType, Principal};
use ic_cdk::export::candid::Nat;
use ic_cdk::storage;

#[derive(CandidType, Clone)]
struct Balance {
    account: Principal,
    balance: Nat,
}

#[derive(CandidType, Clone)]
struct MemeToken {
    balances: Vec<Balance>,
    name: String,
    symbol: String,
    total_supply: Nat,
    transfers_paused: bool,
}

impl MemeToken {
    pub fn new(name: String, symbol: String) -> Self {
        MemeToken {
            balances: Vec::new(),
            name,
            symbol,
            total_supply: 0,
            transfers_paused: false,
        }
    }

    // ... (previous functions for mint, transfer, balance_of)

    pub fn burn(&mut self, caller: Principal, amount: Nat) -> bool {
        assert_eq!(caller, ic_cdk::caller());
        assert!(amount > 0);
        if let Some(index) = self.find_index(caller) {
            let balance = self.balances[index].balance;
            if balance >= amount {
                let new_total_supply = self.total_supply - amount;
                self.total_supply = new_total_supply;
                self.balances[index].balance = balance - amount;
                return true;
            }
        }
        false
    }

    pub fn pause_transfers(&mut self, caller: Principal) -> bool {
        assert_eq!(caller, ic_cdk::caller());
        self.transfers_paused = true;
        true
    }

    pub fn unpause_transfers(&mut self, caller: Principal) -> bool {
        assert_eq!(caller, ic_cdk::caller());
        self.transfers_paused = false;
        true
    }

    pub fn get_token_metadata(&self) -> (String, String, Nat) {
        (self.name.clone(), self.symbol.clone(), self.total_supply)
    }
}