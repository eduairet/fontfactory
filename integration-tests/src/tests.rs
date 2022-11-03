use near_units::parse_near;
use serde_json::json;
use std::{env, fs};
use workspaces::{Account, Contract};

// Hashing
use sha2::{Digest, Sha256};

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let wasm_arg: &str = &(env::args().nth(1).unwrap());
    let wasm_filepath = fs::canonicalize(env::current_dir()?.join(wasm_arg))?;

    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(wasm_filepath)?;
    let contract = worker.dev_deploy(&wasm).await?;

    // create accounts
    let account = worker.dev_create_account().await?;
    let alice = account
        .create_subaccount("alice")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    // begin tests
    test_default_font_id(&alice, &contract).await?;
    test_changes_font_id(&alice, &contract).await?;
    Ok(())
}

async fn test_default_font_id(user: &Account, contract: &Contract) -> anyhow::Result<()> {
    let fontid: String = user
        .call(contract.id(), "get_font_id")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    assert_eq!(fontid, hash_from_str("MyFont".to_string()));
    println!("      Passed ✅ gets default fontid");
    Ok(())
}

async fn test_changes_font_id(user: &Account, contract: &Contract) -> anyhow::Result<()> {
    user.call(contract.id(), "set_font_id")
        .args_json(json!({"message": hash_from_str("YourFont".to_string())}))
        .transact()
        .await?
        .into_result()?;

    let fontid: String = user
        .call(contract.id(), "get_font_id")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    assert_eq!(fontid, hash_from_str("YourFont".to_string()));
    println!("      Passed ✅ changes message");
    Ok(())
}
