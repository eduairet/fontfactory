/*
FontFactory Contract
*/

// NEAR
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::signer_account_id;
use near_sdk::{log, near_bindgen};

// Hashing
use sha2::{Digest, Sha256};

// Font Engine
mod font_engine;
use font_engine::mint_font;

// Util
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
    pub fn create_custom_font(&mut self, fontid: String) {
        log!("Saving Font ID {}", fontid);
        self.fontid = fontid.clone();
        mint_font(hash_from_str(format!(
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
    fn create_custom_font() {
        use fonttools::font::{self, Table};
        use std::fs::File;
        let mut contract = Contract::default();
        contract.create_custom_font(hash_from_str("YourFont".to_string()));
        assert_eq!(
            contract.get_font_id(),
            hash_from_str("YourFont".to_string())
        );
        // Paths
        let home = std::env::var("HOME").unwrap();
        // Font files
        let source_fontfile = File::open("Paradisio-Regular.otf").unwrap();
        let mut source_font = font::load(source_fontfile).expect("Could not load font");
        let mut source_font_id = "".to_string();
        let out_fontfile =
            File::open(format!("{}/Downloads/Paradisio-Regular-NFT.otf", home)).unwrap();
        let mut out_font = font::load(out_fontfile).expect("Could not load font");
        let mut out_font_id = "".to_string();
        // Font table
        if let Table::Name(name_table_source) = source_font
            .get_table(b"name")
            .expect("Error reading name table")
            .expect("There was no name table")
        {
            for name_record_source in name_table_source.records.iter() {
                if name_record_source.nameID == 3 {
                    source_font_id = name_record_source.string.clone();
                }
            }
        }
        if let Table::Name(name_table_out) = out_font
            .get_table(b"name")
            .expect("Error reading name table")
            .expect("There was no name table")
        {
            for name_record_out in name_table_out.records.iter() {
                if name_record_out.nameID == 3 {
                    out_font_id = name_record_out.string.clone();
                }
            }
        }
        // Assertions
        assert_ne!(source_font_id, out_font_id);
    }
}
