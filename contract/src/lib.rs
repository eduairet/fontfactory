/*
FontFactory Contract
*/

// NEAR
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env;
use near_sdk::{log, near_bindgen};

// Default FontID before digest
const DEFAULT_FONTID: &str = "MyFont";

// FontFactory structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub fontid: String,
}

// Default Initialization
impl Default for Contract {
    fn default() -> Self {
        Self {
            fontid: DEFAULT_FONTID.to_string(),
        }
    }
}

// FontFactory Contract
#[near_bindgen]
impl Contract {
    // Initialization
    #[init]
    #[private]
    pub fn init(fontid: String) -> Self {
        Self { fontid }
    }

    // Public method - returns the hashed Font ID saved
    pub fn get_font_id(&self) -> String {
        return self.fontid.clone();
    }

    // Public method - accepts a Font ID, and makes a hash from it and the signer NEAR address
    pub fn create_custom_font(&mut self, fontid: String) {
        // Font ID
        log!("Saving Font ID {}", fontid);
        self.fontid = fontid.clone();

        // Font engine
        use fonttools::font::{self, Table};
        use fonttools::name::NameRecord;
        use std::fs::File;

        // Hashing
        use sha2::{Digest, Sha256};

        // Util
        pub fn hash_from_str(message: String) -> String {
            let mut sha256 = Sha256::new();
            sha256.update(format!("{}", message));
            let result: String = format!("{:X}", sha256.finalize());
            return result;
        }

        // Font manipulation
        // Source font file
        let fontfile = File::open("Paradisio-Regular.otf").unwrap();
        let mut source_font = font::load(fontfile).expect("Could not load font");
        // Access the name table
        // Just fonts with name table will work
        if let Table::Name(name_table) = source_font
            .get_table(b"name")
            .expect("Error reading name table")
            .expect("There was no name table")
        {
            // Change the unique identifier
            let mut identifier_string = String::from("");
            let mut removable: usize = 0;
            for (i, name_record) in name_table.records.iter().enumerate() {
                if name_record.nameID == 3 {
                    // Manipulate the name table
                    identifier_string = String::from(
                        name_record.string.replace(
                            "UKWN",
                            hash_from_str(format!(
                                "{}{}",
                                self.fontid.clone(),
                                String::from(env::signer_account_id())
                            ))
                            .as_str(),
                        ),
                    );
                    removable = i;
                }
            }
            // Name record
            let nft_identifier = NameRecord {
                platformID: 3,
                encodingID: 1,
                languageID: 1033,
                nameID: 3,
                string: identifier_string.clone(),
            };
            // Set the table
            name_table.records.remove(removable);
            name_table.records.push(nft_identifier);
            // Check that the field has changed
            for name_record in name_table.records.iter() {
                if name_record.nameID == 3 {
                    assert_eq!(name_record.string, identifier_string.clone());
                }
            }
        }
        // New File Generator
        let home = std::env::var("HOME").unwrap();
        let out_file = format!("{}/Downloads/Paradisio-Regular-NFT.otf", home);
        let mut nft_font = File::create(out_file).expect("Could not create file");
        source_font.save(&mut nft_font);
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
        assert_eq!(contract.get_font_id(), "MyFont".to_string());
    }

    #[test]
    fn create_custom_font() {
        let mut contract = Contract::default();
        contract.create_custom_font("YourFont".to_string());
        assert_eq!(contract.get_font_id(), "YourFont".to_string());
    }

    // Font Tests
    use fonttools::font::{self, Table};
    use std::fs::File;

    #[test]
    fn compare_fonts() {
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
        // Different data
        assert_ne!(source_font_id, out_font_id);
    }
}
