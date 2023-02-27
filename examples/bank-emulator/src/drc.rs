use futures_util::StreamExt;
use m10_sdk::{
    sdk::{self, transaction_data::Data, FinalizedTransaction},
    Signer, TransactionExt,
};
use pala_types::TxId;
use sqlx::Acquire;
use tokio::time::Duration;
use tracing::{debug, error};

use crate::{
    context::Context,
    liquidity::{release_drc_holding, reserve_drc_holding},
    models::{LedgerTransfer, TransferHandler},
    utils::is_common_parent,
};

const TRANSFER_ADJUSTMENT_TIMEOUT: Duration = Duration::from_secs(30);

pub(crate) struct DrcReserveHandler {
    transfers: flume::r#async::RecvStream<'static, LedgerTransfer>,
    context: Context,
}

impl DrcReserveHandler {
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
            Data::Transfer(transfer) | Data::InitiateTransfer(transfer) => {
                Some(&transfer.transfer_steps)
            }
            _ => None,
        });

        transfer_steps
            .and_then(|ts| {
                ts.iter().find(|s| {
                    !is_common_parent(account_id, &s.from_account_id, &s.to_account_id).unwrap()
                })
            })
            .map(|_| {
                LedgerTransfer::new(
                    account_id,
                    currency_code,
                    response,
                    TransferHandler::DrcReserves,
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
            if let Some(drc_config) = &currency.drc_config {
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
                    hex::encode(&target),
                    indexed_account.balance
                );
                let issued = indexed_account.issuance.map(|i| i.issued_balance);
                if let Some(issued_balance) = issued {
                    if indexed_account.balance < drc_config.reserve_low_bound(issued_balance) {
                        debug!("DRC reserves low");
                        let adjustment = drc_config
                            .nominal_reserve(issued_balance)
                            .saturating_sub(indexed_account.balance);
                        reserve_drc_holding(adjustment, target.to_vec(), currency, &self.context)
                            .await?;
                    } else if indexed_account.balance
                        > drc_config.reserve_high_bound(issued_balance)
                    {
                        debug!("DRC reserves high");
                        let adjustment = indexed_account
                            .balance
                            .saturating_sub(drc_config.nominal_reserve(issued_balance));
                        release_drc_holding(adjustment, target.to_vec(), currency, &self.context)
                            .await?;
                    }
                }
            }
        }
        Ok(())
    }
}
