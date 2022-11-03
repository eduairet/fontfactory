# FontFactory Contract

## Functions

-   Takes a font from a decentralized source (IPFS)
-   When transaction is successfull it creates an instance from the previous font
-   The font engine inside the contract is triggered
    -   It uses the [fonttools](https://docs.rs/fonttools/0.1.0/fonttools/) crate behind the scenes
-   A new NFT linked to the new font file with the custom metadata is generated

## Structure
