// Font Engine Required Crates
use fonttools::font::{self, Table};
use fonttools::name::NameRecord;
use std::fs::File;

// Font Engine
pub fn mint_font(nft_hash: String) {
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
                identifier_string =
                    String::from(name_record.string.replace("UKWN", nft_hash.as_str()));
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
