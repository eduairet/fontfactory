/*
FontFactory Contract Tests
*/

// NEAR
use near_units::parse_near;
use serde_json::json;
use std::{env, fs};
use workspaces::{Account, Contract};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let wasm_arg: &str = &(env::args().nth(1).unwrap());
    let wasm_filepath = fs::canonicalize(env::current_dir()?.join(wasm_arg))?;

    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(wasm_filepath)?;
    let contract = worker.dev_deploy(&wasm).await?;

    // create accounts
    let account = worker.dev_create_account().await?;
    let bitdoni = account
        .create_subaccount("bitdoni")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    // begin tests
    test_default_font_id(&bitdoni, &contract).await?;
    test_create_custom_font(&bitdoni, &contract).await?;
    Ok(())
}

async fn test_default_font_id(user: &Account, contract: &Contract) -> anyhow::Result<()> {
    let fontid: String = user
        .call(contract.id(), "get_font_id")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    assert_eq!(fontid, "MyFont".to_string());
    println!("      Passed ✅ gets default fontid");
    Ok(())
}

async fn test_create_custom_font(user: &Account, contract: &Contract) -> anyhow::Result<()> {
    user.call(contract.id(), "create_custom_font")
        .args_json(json!({"fontid": "YourFont".to_string()}))
        .transact()
        .await?
        .into_result()?;

    let fontid: String = user
        .call(contract.id(), "get_font_id")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    assert_eq!(fontid, "YourFont".to_string());
    println!("      Passed ✅ custom fontid");
    Ok(())
}
