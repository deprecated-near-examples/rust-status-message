#![cfg(test)]

use serde_json::json;
use workspaces::prelude::*;
use workspaces::DevNetwork;

const STATUS_MSG_WASM_FILEPATH: &str = "../res/status_message.wasm";

async fn setup() -> anyhow::Result<(workspaces::Worker<impl DevNetwork>, workspaces::Contract)> {
    let worker = workspaces::sandbox();
    let wasm = std::fs::read(STATUS_MSG_WASM_FILEPATH)?;
    let contract = worker.dev_deploy(wasm).await?;
    Ok((worker, contract))
}

async fn set_message(
    worker: &workspaces::Worker<impl DevNetwork>,
    contract: &workspaces::Contract,
    message: &str,
) -> anyhow::Result<()> {
    let outcome = contract
        .call(&worker, "set_status")
        .args_json(json!({
            "message": message,
        }))?
        .transact()
        .await?;
    println!("set_status: {:?}", outcome);
    Ok(())
}

async fn view_message(
    worker: &workspaces::Worker<impl DevNetwork>,
    contract: &workspaces::Contract,
    account_id: &workspaces::AccountId,
) -> anyhow::Result<Option<String>> {
    contract
        .view(
            &worker,
            "get_status",
            json!({
                "account_id": account_id,
            })
            .to_string()
            .into_bytes(),
        )
        .await?
        .json()
}

#[tokio::test]
async fn set_get_message() -> anyhow::Result<()> {
    let (worker, contract) = setup().await?;
    let message = "hello world!";
    set_message(&worker, &contract, message).await?;

    let result: Option<String> = view_message(&worker, &contract, &contract.id()).await?;

    println!("status: {:?}", result);
    assert_eq!(result.unwrap(), message);
    Ok(())
}

#[tokio::test]
async fn get_nonexistent_message() -> anyhow::Result<()> {
    let (worker, contract) = setup().await?;
    let result: Option<String> = view_message(&worker, &contract, &contract.id()).await?;
    println!("status: {:?}", result);
    assert_eq!(result, None);
    Ok(())
}
