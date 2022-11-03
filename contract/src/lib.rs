// NEAR
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId};

// Font Engine
mod font_engine;
use font_engine::font_engine;

// Contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct FontFactory {
    pub owner: AccountId,
    pub font_id: u128,
}

impl Default for FontFactory {
    fn default() -> Self {
        Self {
            owner: "v1.fontfactory.eduairet.testnet".parse().unwrap(),
            font_id: 0,
        }
    }
}

#[near_bindgen]
impl FontFactory {
    #[init]
    #[private] // Public - but only callable by env::current_account_id()
    pub fn init(owner: AccountId) -> Self {
        Self { owner, font_id: 0 }
    }

    // Public - owner getter
    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    // Public - but only callable by env::current_account_id(). Sets the beneficiary
    #[private]
    #[payable]
    pub fn mint_font(&mut self, owner: AccountId) {
        self.owner = owner;
        font_engine(String::from("NFTHASH"));
    }
}

/* #[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;
    use near_sdk::Balance;

    const OWNER: &str = "owner";
    const NEAR: u128 = 1000000000000000000000000;

    #[test]
    fn initializes() {
        let contract = FontFactory::init(OWNER.parse().unwrap());
        assert_eq!(contract.owner, OWNER.parse().unwrap())
    }
} */
