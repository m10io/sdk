use futures_util::StreamExt;
use m10_sdk::{
    sdk::{self, transaction_data::Data, FinalizedTransaction},
    Metadata, Signer, TransactionExt,
};
use pala_types::TxId;
use sqlx::Acquire;
use tokio::time::Duration;
use tracing::{debug, error};

const TRANSFER_ADJUSTMENT_TIMEOUT: Duration = Duration::from_secs(30);

use crate::{
    context::Context,
    liquidity::{release_cbdc, reserve_cbdc},
    models::{Asset, AssetType, LedgerTransfer, TransferHandler},
    utils::{is_common_parent, is_parent, ledger_transfer},
};

pub(crate) struct CbdcAdjustmentHandler {
    transfers: flume::r#async::RecvStream<'static, LedgerTransfer>,
    context: Context,
}

impl CbdcAdjustmentHandler {
    pub fn new(
        transfers: flume::r#async::RecvStream<'static, LedgerTransfer>,
        context: Context,
    ) -> Self {
        Self { transfers, context }
    }

    pub fn find_matching_transfer(
        transaction: &FinalizedTransaction,
        account_id: &[u8],
        currency_code: &str,
    ) -> Option<LedgerTransfer> {
        let response = transaction.response.as_ref().unwrap();
        let transfer_steps = transaction.data().and_then(|d| match d {
            Data::Transfer(transfer) => Some(&transfer.transfer_steps),
            Data::CommitTransfer(_) => response
                .transfer_committed
                .as_ref()
                .map(|t| &t.transfer_steps),
            _ => None,
        });

        transfer_steps
            .and_then(|tss| {
                tss.iter()
                    .find(|ts| is_parent(&ts.to_account_id, account_id).unwrap())
            })
            .map(|ts| {
                LedgerTransfer::new(
                    &ts.to_account_id,
                    currency_code,
                    response,
                    TransferHandler::CbdcLimits,
                )
            })
    }

    pub async fn start(mut self) -> eyre::Result<()> {
        while let Some(mut transfer) = self.transfers.next().await {
            let tx_id = TxId::try_from(transfer.tx_id.as_slice());

            match tokio::time::timeout(
                TRANSFER_ADJUSTMENT_TIMEOUT,
                self.check_and_adjust(&transfer),
            )
            .await
            {
                Ok(Ok(())) => {
                    let mut conn = self.context.db_pool.get().await?;
                    let mut txn = conn.begin().await?;
                    transfer.set_handled(&mut txn).await?;
                    txn.commit().await?;
                }
                Ok(Err(err)) => {
                    error!(%err, "Failed to adjust transfer");
                }
                Err(_) => {
                    let txn_id = tx_id
                        .map(|id| id.to_string())
                        .unwrap_or_else(|_| "invalid txId".to_string());
                    error!(%txn_id, "adjustment timed out after 30s");
                }
            }
        }
        Ok(())
    }

    async fn check_and_adjust(&self, issuing_transfer: &LedgerTransfer) -> eyre::Result<()> {
        let LedgerTransfer {
            target,
            currency_code,
            ..
        } = issuing_transfer;
        if let Some(cbdc_config) = self
            .context
            .config
            .currencies
            .get(currency_code)
            .and_then(|c| c.cbdc_config.as_ref())
        {
            let mut client = self.context.ledger.clone();

            // Get balance of target account
            let request = self
                .context
                .signer
                .sign_request(sdk::GetAccountRequest {
                    id: target.to_vec(),
                })
                .await?;
            let indexed_account = client.get_indexed_account(request).await?;
            debug!(
                "account {} balance {}",
                hex::encode(target),
                indexed_account.balance
            );
            // If balance is over limit transfer from CBDC to DRC account
            if indexed_account.balance > cbdc_config.customer_limit {
                debug!("account over limit");
                let mut conn = self.context.db_pool.get().await?;
                if let Some(asset) = Asset::find_by_ledger_account_id(target)
                    .fetch_optional(&mut *conn)
                    .await?
                {
                    debug!("cbdc asset {}", asset.id);
                    let regulated_account = Asset::find_by_account_id_instrument_type(
                        asset.linked_account,
                        currency_code,
                        AssetType::Regulated,
                    )
                    .fetch_one(&mut *conn)
                    .await?;
                    debug!(
                        "regulated asset {}/{}",
                        asset.id,
                        hex::encode(&regulated_account.ledger_account_id)
                    );
                    ledger_transfer(
                        target.to_vec(),
                        regulated_account.ledger_account_id,
                        indexed_account.balance - cbdc_config.customer_limit,
                        vec![sdk::RebalanceTransfer::default().any()],
                        &self.context,
                    )
                    .await?;
                    debug!(
                        "amount adjsuted {}",
                        indexed_account.balance - cbdc_config.customer_limit
                    );
                }
            }
        }
        Ok(())
    }
}

pub(crate) struct CbdcReserveHandler {
    transfers: flume::r#async::RecvStream<'static, LedgerTransfer>,
    context: Context,
}

impl CbdcReserveHandler {
    pub fn new(
        transfers: flume::r#async::RecvStream<'static, LedgerTransfer>,
        context: Context,
    ) -> Self {
        Self { transfers, context }
    }

    pub fn find_matching_transfer(
        transaction: &FinalizedTransaction,
        account_id: &[u8],
        currency_code: &str,
    ) -> Option<LedgerTransfer> {
        let response = transaction.response.as_ref().unwrap();
        let transfer_steps = transaction.data().and_then(|d| match d {
            Data::Transfer(transfer) => Some(&transfer.transfer_steps),
            Data::CommitTransfer(_) => response
                .transfer_committed
                .as_ref()
                .map(|t| &t.transfer_steps),
            _ => None,
        });

        transfer_steps
            .and_then(|tss| {
                tss.iter().find(|ts| {
                    !is_common_parent(account_id, &ts.from_account_id, &ts.to_account_id).unwrap()
                })
            })
            .map(|_| {
                LedgerTransfer::new(
                    account_id,
                    currency_code,
                    response,
                    TransferHandler::CbdcReserves,
                )
            })
    }

    pub async fn start(mut self) -> eyre::Result<()> {
        while let Some(mut transfer) = self.transfers.next().await {
            let tx_id = TxId::try_from(transfer.tx_id.as_slice());

            match tokio::time::timeout(
                TRANSFER_ADJUSTMENT_TIMEOUT,
                self.check_and_adjust_holdings(&transfer),
            )
            .await
            {
                Ok(Ok(())) => {
                    let mut conn = self.context.db_pool.get().await?;
                    let mut txn = conn.begin().await?;
                    transfer.set_handled(&mut txn).await?;
                    txn.commit().await?;
                }
                Ok(Err(err)) => {
                    error!(%err, "Failed to adjust transfer");
                }
                Err(_) => {
                    let txn_id = tx_id
                        .map(|id| id.to_string())
                        .unwrap_or_else(|_| "invalid txId".to_string());
                    error!(%txn_id, "adjustment timed out after 30s");
                }
            }
        }
        Ok(())
    }

    async fn check_and_adjust_holdings(
        &self,
        issuing_transfer: &LedgerTransfer,
    ) -> eyre::Result<()> {
        let LedgerTransfer {
            target,
            currency_code,
            ..
        } = issuing_transfer;
        if let Some(currency) = self.context.config.currencies.get(currency_code) {
            if let Some(cbdc_config) = &currency.cbdc_config {
                let mut client = self.context.ledger.clone();

                // Get balance of target account
                let request = self
                    .context
                    .signer
                    .sign_request(sdk::GetAccountRequest {
                        id: target.to_vec(),
                    })
                    .await?;
                let indexed_account = client.get_indexed_account(request).await?;
                debug!(
                    "account {} balance {}",
                    hex::encode(target),
                    indexed_account.balance
                );
                let issued = indexed_account.issuance.map(|i| i.issued_balance);
                if let Some(issued_balance) = issued {
                    // Difference between holding and issuance balance.
                    // Note: If account was never funded this difference can be negative.
                    let delta: i64 = if issued_balance <= indexed_account.balance {
                        (indexed_account.balance - issued_balance).try_into()?
                    } else {
                        -i64::try_from(issued_balance - indexed_account.balance)?
                    };

                    if delta < cbdc_config.reserve_balance_low_bound().try_into()? {
                        debug!("CBDC reserves low");
                        let (adjustment, overflow) = if delta.is_negative() {
                            cbdc_config
                                .nominal_margin
                                .overflowing_add(delta.abs().try_into()?)
                        } else {
                            cbdc_config
                                .nominal_margin
                                .overflowing_sub(delta.try_into()?)
                        };
                        if overflow {
                            return Err(eyre::Report::msg("required amount too large"));
                        }
                        reserve_cbdc(adjustment, target.to_vec(), currency, &self.context).await?;
                    } else if delta > cbdc_config.reserve_balance_high_bound().try_into()? {
                        debug!("CBDC reserves high");
                        let adjustment = u64::try_from(delta)? - cbdc_config.nominal_margin;
                        release_cbdc(adjustment, target.to_vec(), currency, &self.context).await?;
                    }
                }
            }
        }
        Ok(())
    }
}
