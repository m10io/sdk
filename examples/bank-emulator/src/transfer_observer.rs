use flume::Sender;
use futures_util::StreamExt;
use m10_sdk::{
    sdk::{self, FinalizedTransaction},
    Signer,
};
use pala_types::TxId;
use sqlx::Acquire;
use tokio::time::Duration;
use tracing::{debug, info};

use crate::{
    context::Context,
    models::{LedgerTransfer, TransferHandler},
};

#[derive(Clone)]
pub(crate) struct TransferObserver {
    transfer_tx: Sender<LedgerTransfer>,
    handler_type: TransferHandler,
    currency_code: String,
}

impl TransferObserver {
    pub fn new(
        transfer_tx: Sender<LedgerTransfer>,
        handler_type: TransferHandler,
        currency_code: &str,
    ) -> Self {
        Self {
            transfer_tx,
            handler_type,
            currency_code: currency_code.to_string(),
        }
    }

    pub async fn run<F>(&self, context: Context, filter: F) -> eyre::Result<()>
    where
        F: Fn(&FinalizedTransaction, &[u8], &str) -> Option<LedgerTransfer>,
    {
        let account_id = match self.handler_type {
            TransferHandler::DrcReserves => {
                context
                    .get_currency_regulated_account(&self.currency_code)
                    .await?
            }
            _ => {
                context
                    .get_currency_cbdc_account(&self.currency_code)
                    .await?
            }
        };

        let mut conn = context.db_pool.get().await?;

        // Inject unhandled transfers
        let transfers = LedgerTransfer::find_unhandled(self.handler_type)
            .fetch_all(&mut *conn)
            .await?;
        for transfer in transfers {
            let _ = self.transfer_tx.send_async(transfer).await;
        }

        // Check for last transfer as start for observer
        let last_tx_id = if let Some(transfer) = LedgerTransfer::find_latest(self.handler_type)
            .fetch_optional(&mut *conn)
            .await?
        {
            TxId::try_from(&transfer.tx_id[..])?
        } else {
            TxId::new(1, 0).unwrap() // Block 0 is the genesis block, and so can't be observed
        };

        info!(?last_tx_id, "Observing transfers");
        let request = context
            .signer
            .sign_request(sdk::ObserveAccountsRequest {
                involved_accounts: vec![account_id.clone()],
                starting_from: Some(sdk::TxId {
                    tx_id: u64::from(last_tx_id) + 1,
                }),
            })
            .await?;

        let channel =
            m10_sdk::ledger_client::Channel::builder(context.config.ledger_addr.uri().clone())
                .keep_alive_while_idle(true)
                .http2_keep_alive_interval(Duration::from_secs(30))
                .connect_lazy()?;
        let ledger = m10_sdk::LedgerClient::new(channel);

        let mut stream = ledger.observe_transfers(request).await?;
        while let Some(msg) = stream.next().await {
            let sdk::FinalizedTransactions { transactions } = msg?;
            debug!(transfers = %transactions.len(),"observation");
            let mut conn = context.db_pool.get().await?;
            for transaction in transactions {
                let response = transaction.response.as_ref().unwrap();
                debug!("transfer {}", response.tx_id);
                // Ignore failed transfers
                if response.error.is_some() {
                    continue;
                }

                if let Some(mut transfer) = filter(&transaction, &account_id, &self.currency_code) {
                    // Check if a transaction of same block was already queued for the handler
                    if !LedgerTransfer::find_block(
                        self.handler_type,
                        pala_types::TxId::from(response.tx_id),
                    )
                    .fetch_all(&mut *conn)
                    .await?
                    .is_empty()
                    {
                        debug!("transfer for same block already queued");
                        continue;
                    }

                    {
                        let mut txn = conn.begin().await?;
                        transfer.insert(&mut txn).await?;
                        txn.commit().await?;
                    }

                    debug!("queueing transfer {}", response.tx_id);
                    if self.transfer_tx.send_async(transfer).await.is_err() {
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    }
}
