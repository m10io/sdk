#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use context::{Context, SignerLevel};
use tauri::{command, Manager, Result, State};

use crate::{config::Config, types::*};

mod config;
mod context;
mod types;

#[command]
async fn block_height(context: State<'_, Context>) -> Result<u64> {
    context.block_height().await.map_err(to_tauri_error)
}

#[command]
async fn ledger_info(context: State<'_, Context>) -> Result<LedgerInfo> {
    context.ledger_info().await.map_err(to_tauri_error)
}

#[command]
async fn get_assets(context: State<'_, Context>) -> Result<Vec<AssetInfo>> {
    context
        .get_accounts(SignerLevel::CentralBank, SignerLevel::Operator)
        .await
        .map_err(to_tauri_error)
}

#[command]
async fn create_asset(
    code: String,
    decimal_places: Option<u32>,
    context: State<'_, Context>,
) -> Result<String> {
    context
        .create_assets(code, decimal_places)
        .await
        .map_err(to_tauri_error)
}

#[command]
async fn create_bank(
    code: String,
    name: String,
    public_key: Option<String>,
    context: State<'_, Context>,
) -> Result<String> {
    context
        .create_bank(code, name, public_key)
        .await
        .map_err(to_tauri_error)
}

#[command]
async fn get_banks(context: State<'_, Context>) -> Result<Vec<AssetInfo>> {
    context
        .get_accounts(SignerLevel::Bank, SignerLevel::Bank)
        .await
        .map_err(to_tauri_error)
}

#[command]
async fn issue_to_account(
    parent_account: String,
    account: String,
    amount: u64,
    reference: String,
    context: State<'_, Context>,
) -> Result<u64> {
    context
        .transfer(
            parent_account,
            account,
            amount,
            reference,
            SignerLevel::CentralBank,
        )
        .await
        .map_err(to_tauri_error)
}

#[command]
async fn redeem_to_account(
    parent_account: String,
    account: String,
    amount: u64,
    reference: String,
    context: State<'_, Context>,
) -> Result<u64> {
    context
        .transfer(
            account,
            parent_account,
            amount,
            reference,
            SignerLevel::Bank,
        )
        .await
        .map_err(to_tauri_error)
}

#[command]
async fn get_account(id: String, context: State<'_, Context>) -> Result<Account> {
    context.get_account(id).await.map_err(to_tauri_error)
}

async fn init() -> anyhow::Result<Context> {
    let config = Config::new()?;
    println!("{:?}", config);
    println!("ledger:  {}", config.ledger_addr.uri().to_string());

    let context = Context::new(config)?;
    // Note: There is one async call required to get runtime started
    // before it can be used by tauri.
    {
        let height = context.block_height().await?;
        println!("Connected to ledger at height: {}", height);
    }
    Ok(context)
}

#[tokio::main]
async fn main() {
    match init().await {
        Ok(context) => {
            tauri::async_runtime::set(tokio::runtime::Handle::current());

            tauri::Builder::default()
                .manage(context)
                .invoke_handler(tauri::generate_handler![
                    ledger_info,
                    block_height,
                    get_assets,
                    get_account,
                    create_asset,
                    create_bank,
                    get_banks,
                    issue_to_account,
                    redeem_to_account,
                ])
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
        Err(err) => {
            tauri::Builder::default()
                .setup(move |app| {
                    app.emit_all(
                        "init-error",
                        InitError {
                            message: err.to_string(),
                        },
                    )
                    .unwrap();
                    Ok(())
                })
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
    }
}
