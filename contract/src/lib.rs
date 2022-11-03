/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::signer_account_id;
use near_sdk::{log, near_bindgen};

// Hashing
use sha2::{Digest, Sha256};

// Font Engine
mod font_engine;
use font_engine::set_font_name;

// Util
// Private function
fn hash_from_str(message: String) -> String {
    // create a Sha256 object
    let mut sha256 = Sha256::new();

    // write input message
    sha256.update(format!("{}", message));

    // read hash digest and consume hasher
    let result: String = format!("{:X}", sha256.finalize());

    return result;
}

// Default FontID before digest
const DEFAULT_FONTID: &str = "MyFont";

// FontFactory structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    fontid: String,
}

// Default Initialization
impl Default for Contract {
    fn default() -> Self {
        Self {
            fontid: hash_from_str(DEFAULT_FONTID.to_string()),
        }
    }
}

// FontFactory Contract
#[near_bindgen]
impl Contract {
    // Public method - returns the hashed FontID saved
    pub fn get_font_id(&self) -> String {
        return self.fontid.clone();
    }

    // Public method - accepts a Tx hash, and makes a hash from it and the signer NEAR address
    pub fn set_font_id(&mut self, fontid: String) {
        // Use env::log to record logs permanently to the blockchain!
        log!("Saving greeting {}", fontid);
        self.fontid = fontid.clone();
        set_font_name(hash_from_str(format!(
            "{}{}",
            fontid.clone(),
            String::from(signer_account_id())
        )));
    }
}

// Contract tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_font_id() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(contract.get_font_id(), hash_from_str("MyFont".to_string()));
    }

    #[test]
    fn set_font_id() {
        let mut contract = Contract::default();
        contract.set_font_id(hash_from_str("YourFont".to_string()));
        assert_eq!(
            contract.get_font_id(),
            hash_from_str("YourFont".to_string())
        );
    }
}
