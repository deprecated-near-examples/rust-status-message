use serde_json::json;
use near_units::parse_near;
use workspaces::prelude::*; 
use workspaces::{network::Sandbox, Account, Contract, Worker};

const WASM_FILEPATH: &str = "../../res/status_message.wasm";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(WASM_FILEPATH)?;
    let contract = worker.dev_deploy(&wasm).await?;

    // create accounts
    let owner = worker.root_account().unwrap();
    let alice = owner
    .create_subaccount(&worker, "alice")
    .initial_balance(parse_near!("30 N"))
    .transact()
    .await?
    .into_result()?;

    // begin tests  
    test_set_message(&owner, &alice, &contract, &worker).await?;
    test_null_messages(&owner, &alice, &contract, &worker).await?;
    test_differing_statuses(&owner, &alice, &contract, &worker).await?;
    Ok(())
}   

async fn test_set_message(
    owner: &Account,
    user: &Account,
    contract: &Contract,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<()> {
    user
        .call(&worker, contract.id(), "set_status")
        .args_json(json!({ "message": "hello" }))?
        .transact()
        .await?;

    let alice_status: String = owner
        .call(&worker, contract.id(), "get_status")
        .args_json(json!({ "account_id": user.id() }))?
        .transact()
        .await?
        .json()?;

    assert_eq!(alice_status, "hello");
    println!("      Passed ✅ set get message");
    Ok(())
}

async fn test_null_messages(
    owner: &Account,
    user: &Account,
    contract: &Contract,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<()> {
    let owner_status: Option<String> = user
        .call(&worker, contract.id(), "get_status")
        .args_json(json!({ "account_id": owner.id() }))?
        .transact()
        .await?
        .json()?;

    assert_eq!(owner_status, None);
    println!("      Passed ✅ get nonexistent message");
    Ok(())
}

async fn test_differing_statuses(
    owner: &Account,
    user: &Account,
    contract: &Contract,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<()> {
    owner
        .call(&worker, contract.id(), "set_status")
        .args_json(json!({ "message": "world" }))?
        .transact()
        .await?;

    let alice_status: String = owner
        .call(&worker, contract.id(), "get_status")
        .args_json(json!({ "account_id": user.id() }))?
        .transact()
        .await?
        .json()?;

    assert_eq!(alice_status, "hello");

    let owner_status: String = owner
        .call(&worker, contract.id(), "get_status")
        .args_json(json!({ "account_id": owner.id() }))?
        .transact()
        .await?
        .json()?;

    assert_eq!(owner_status, "world");
    println!("      Passed ✅ root and alice have different statuses");
    Ok(())
}