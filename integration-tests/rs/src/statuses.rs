use serde_json::json;
use near_units::parse_near;
use workspaces::prelude::*;

const WASM_FILEPATH: &str = "../../res/status_message.wasm";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(WASM_FILEPATH)?;
    let contract = worker.dev_deploy(&wasm).await?;

    // create accounts
    let owner = worker.root_account();
    let alice = owner
    .create_subaccount(&worker, "alice")
    .initial_balance(parse_near!("30 N"))
    .transact()
    .await?
    .into_result()?;

    // begin test
    alice
        .call(&worker, contract.id(), "set_status")
        .args_json(json!({ "message": "world" }))?
        .transact()
        .await?;
    
    let alice_status: String = owner
        .call(&worker, contract.id(), "get_status")
        .args_json(json!({ "account_id": alice.id() }))?
        .transact()
        .await?
        .json()?;

    assert_eq!(alice_status, "world");

    let owner_status: Option<String> = owner
    .call(&worker, contract.id(), "get_status")
    .args_json(json!({ "account_id": owner.id() }))?
    .transact()
    .await?
    .json()?;

    assert_eq!(owner_status, None);

    println!("Passed ✅");
    Ok(())
}