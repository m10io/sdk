use futures_core::Stream;
use m10_protos::sdk::{
    self, m10_query_service_client::M10QueryServiceClient,
    m10_tx_service_client::M10TxServiceClient, transaction_data::Data, BulkTransactions,
    TransactionData,
};
use m10_protos::sdk::{AccountInfo, RequestEnvelope};
use m10_signing::SignedRequest;
use std::time::{SystemTime, UNIX_EPOCH};
use tonic::{Request, Response};

use crate::transfer_ext::EnhancedTransfer;
use crate::{EnhancedTransferStep, Signer};
pub use tonic::transport::{Channel, ClientTlsConfig, Endpoint, Uri};

// Re-export public error
pub use tonic::Status;

#[derive(Clone)]
/// A client for the M10 Ledger.
///
/// This client allows you to query and transact on the M10 ledger.
///
/// # Example
/// ```no_run
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() {
///   let ledger_url = "https://test.m10.net".to_string();
///   let mut client = m10_sdk::LedgerClient::new(
///     tonic::transport::Endpoint::from_shared(ledger_url)
///       .unwrap()
///       .connect_lazy()
///       .unwrap()
///    );
///
///   let block_height = client.block_height().await;
/// }
/// ```
pub struct LedgerClient {
    tx_client: M10TxServiceClient<Channel>,
    query_client: M10QueryServiceClient<Channel>,
}

impl LedgerClient {
    pub fn new(grpc_channel: Channel) -> Self {
        let tx_client = M10TxServiceClient::new(grpc_channel.clone());
        let query_client = M10QueryServiceClient::new(grpc_channel);
        Self {
            tx_client,
            query_client,
        }
    }

    pub fn transaction_request(
        data: impl Into<Data>,
        context_id: Vec<u8>,
    ) -> sdk::TransactionRequestPayload {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;
        sdk::TransactionRequestPayload {
            nonce: fastrand::u64(..),
            timestamp,
            context_id,
            data: Some(TransactionData {
                data: Some(data.into()),
            }),
        }
    }

    pub async fn block_height(&mut self) -> Result<u64, tonic::Status> {
        let chain_info = self
            .query_client
            .get_chain_info(Request::new(()))
            .await?
            .into_inner();
        Ok(chain_info.block_height)
    }

    pub async fn create_transaction(
        &mut self,
        payload: impl Into<RequestEnvelope>,
    ) -> Result<sdk::TransactionResponse, Status> {
        self.tx_client
            .create_transaction(Request::new(payload.into()))
            .await
            .map(|res| res.into_inner())
    }

    pub async fn bulk_create_transactions(
        &mut self,
        payload: impl Into<BulkTransactions>,
    ) -> Result<sdk::BulkTransactionsResponse, Status> {
        self.tx_client
            .bulk_create_transactions(Request::new(payload.into()))
            .await
            .map(|res| res.into_inner())
    }

    // Transfers
    pub async fn get_transfer(
        &mut self,
        request: SignedRequest<sdk::GetTransferRequest>,
    ) -> Result<sdk::FinalizedTransfer, Status> {
        self.query_client
            .get_transfer(Request::new(request.into()))
            .await
            .map(|res| res.into_inner())
    }

    pub async fn list_transfers(
        &mut self,
        request: SignedRequest<sdk::ListTransferRequest>,
    ) -> Result<sdk::FinalizedTransfers, Status> {
        self.query_client
            .list_transfers(Request::new(request.into()))
            .await
            .map(|res| res.into_inner())
    }

    // Actions
    pub async fn get_action(
        &mut self,
        request: SignedRequest<sdk::GetActionRequest>,
    ) -> Result<sdk::Action, Status> {
        self.query_client
            .get_action(Request::new(request.into()))
            .await
            .map(|res| res.into_inner())
    }

    pub async fn list_actions(
        &mut self,
        request: SignedRequest<sdk::ListActionsRequest>,
    ) -> Result<sdk::Actions, Status> {
        self.query_client
            .list_actions(Request::new(request.into()))
            .await
            .map(|res| res.into_inner())
    }

    pub async fn get_transaction(
        &mut self,
        request: SignedRequest<sdk::GetTransactionRequest>,
    ) -> Result<sdk::FinalizedTransaction, Status> {
        self.query_client
            .get_transaction(Request::new(request.into()))
            .await
            .map(Response::into_inner)
    }

    pub async fn list_transactions(
        &mut self,
        request: SignedRequest<sdk::ListTransactionsRequest>,
    ) -> Result<sdk::FinalizedTransactions, Status> {
        self.query_client
            .list_transactions(Request::new(request.into()))
            .await
            .map(Response::into_inner)
    }

    pub async fn group_transactions(
        &mut self,
        request: SignedRequest<sdk::GroupTransactionsRequest>,
    ) -> Result<sdk::GroupedFinalizedTransactions, Status> {
        self.query_client
            .group_transactions(Request::new(request.into()))
            .await
            .map(Response::into_inner)
    }

    //  Indexed Accounts
    pub async fn get_indexed_account(
        &mut self,
        request: SignedRequest<sdk::GetAccountRequest>,
    ) -> Result<sdk::IndexedAccount, Status> {
        self.query_client
            .get_indexed_account(Request::new(request.into()))
            .await
            .map(|res| res.into_inner())
    }

    // AccountSets
    pub async fn get_account_set(
        &mut self,
        request: SignedRequest<sdk::GetAccountSetRequest>,
    ) -> Result<sdk::AccountSet, Status> {
        self.query_client
            .get_account_set(Request::new(request.into()))
            .await
            .map(|res| res.into_inner())
    }

    pub async fn list_account_sets(
        &mut self,
        request: SignedRequest<sdk::ListAccountSetsRequest>,
    ) -> Result<sdk::ListAccountSetsResponse, Status> {
        self.query_client
            .list_account_sets(Request::new(request.into()))
            .await
            .map(|res| res.into_inner())
    }

    // Accounts
    pub async fn get_account_metadata(
        &mut self,
        request: SignedRequest<sdk::GetAccountRequest>,
    ) -> Result<sdk::AccountMetadata, Status> {
        self.query_client
            .get_account_metadata(Request::new(request.into()))
            .await
            .map(|res| res.into_inner())
    }

    pub async fn get_account_info(
        &mut self,
        request: SignedRequest<sdk::GetAccountRequest>,
    ) -> Result<sdk::AccountInfo, Status> {
        self.query_client
            .get_account_info(Request::new(request.into()))
            .await
            .map(|res| res.into_inner())
    }

    pub async fn list_account_metadata(
        &mut self,
        request: SignedRequest<sdk::ListAccountMetadataRequest>,
    ) -> Result<sdk::ListAccountMetadataResponse, Status> {
        self.query_client
            .list_account_metadata(Request::new(request.into()))
            .await
            .map(|res| res.into_inner())
    }

    // Role Bindings
    pub async fn get_role_binding(
        &mut self,
        request: SignedRequest<sdk::GetRoleBindingRequest>,
    ) -> Result<sdk::RoleBinding, Status> {
        self.query_client
            .get_role_binding(Request::new(request.into()))
            .await
            .map(|res| res.into_inner())
    }

    pub async fn list_role_bindings(
        &mut self,
        request: SignedRequest<sdk::ListRoleBindingsRequest>,
    ) -> Result<sdk::ListRoleBindingsResponse, Status> {
        self.query_client
            .list_role_bindings(Request::new(request.into()))
            .await
            .map(|res| res.into_inner())
    }

    // Roles
    pub async fn get_role(
        &mut self,
        request: SignedRequest<sdk::GetRoleRequest>,
    ) -> Result<sdk::Role, Status> {
        self.query_client
            .get_role(Request::new(request.into()))
            .await
            .map(|res| res.into_inner())
    }

    pub async fn list_roles(
        &mut self,
        request: SignedRequest<sdk::ListRolesRequest>,
    ) -> Result<sdk::ListRolesResponse, Status> {
        self.query_client
            .list_roles(Request::new(request.into()))
            .await
            .map(|res| res.into_inner())
    }

    // Observations
    pub async fn observe_transfers(
        &self,
        request: SignedRequest<sdk::ObserveAccountsRequest>,
    ) -> Result<impl Stream<Item = Result<sdk::FinalizedTransactions, Status>>, Status> {
        self.query_client
            .clone()
            .observe_transfers(Request::new(request.into()))
            .await
            .map(tonic::Response::into_inner)
    }

    pub async fn observe_resources(
        &self,
        request: SignedRequest<sdk::ObserveResourcesRequest>,
    ) -> Result<impl Stream<Item = Result<sdk::FinalizedTransactions, Status>>, Status> {
        self.query_client
            .clone()
            .observe_resources(Request::new(request.into()))
            .await
            .map(tonic::Response::into_inner)
    }

    pub async fn observe_accounts(
        &self,
        request: SignedRequest<sdk::ObserveAccountsRequest>,
    ) -> Result<impl Stream<Item = Result<sdk::FinalizedTransactions, Status>>, Status> {
        self.query_client
            .clone()
            .observe_accounts(Request::new(request.into()))
            .await
            .map(tonic::Response::into_inner)
    }

    pub async fn observe_actions(
        &self,
        request: SignedRequest<sdk::ObserveActionsRequest>,
    ) -> Result<impl Stream<Item = Result<sdk::FinalizedTransactions, Status>>, Status> {
        self.query_client
            .clone()
            .observe_actions(Request::new(request.into()))
            .await
            .map(tonic::Response::into_inner)
    }

    pub async fn observe_metrics(
        &self,
        request: SignedRequest<sdk::ObserveAccountsRequest>,
    ) -> Result<impl Stream<Item = Result<sdk::TransactionMetrics, Status>>, Status> {
        self.query_client
            .clone()
            .observe_metrics(Request::new(request.into()))
            .await
            .map(tonic::Response::into_inner)
    }

    pub async fn enhance_transfers(
        &self,
        transfers: Vec<sdk::FinalizedTransfer>,
        signer: &impl Signer,
    ) -> Result<Vec<EnhancedTransfer>, Status> {
        futures_util::future::try_join_all(
            transfers
                .into_iter()
                .map(|transfer| self.enhance_transfer(transfer, signer)),
        )
        .await
    }

    pub async fn enhance_transfer(
        &self,
        transfer: sdk::FinalizedTransfer,
        signer: &impl Signer,
    ) -> Result<EnhancedTransfer, Status> {
        let mut enhanced_steps = Vec::default();
        for transfer_step in &transfer.transfer_steps {
            enhanced_steps.push(self.enhance_transfer_step(transfer_step, signer).await?);
        }
        Ok(EnhancedTransfer {
            enhanced_steps,
            transfer,
        })
    }

    async fn enhance_transfer_step(
        &self,
        transfer_step: &sdk::TransferStep,
        signer: &impl Signer,
    ) -> Result<EnhancedTransferStep, Status> {
        let from = async {
            Result::<_, Status>::Ok(
                self.query_client
                    .clone()
                    .get_account_info(Request::new(
                        signer
                            .sign_request(sdk::GetAccountRequest {
                                id: transfer_step.from_account_id.clone(),
                            })
                            .await
                            .map_err(|err| Status::internal(err.to_string()))?
                            .into(),
                    ))
                    .await
                    .map(|res| res.into_inner())
                    .ok(),
            )
        };
        let to = async {
            Result::<_, Status>::Ok(
                self.query_client
                    .clone()
                    .get_account_info(Request::new(
                        signer
                            .sign_request(sdk::GetAccountRequest {
                                id: transfer_step.to_account_id.clone(),
                            })
                            .await
                            .map_err(|err| Status::internal(err.to_string()))?
                            .into(),
                    ))
                    .await
                    .map(|res| res.into_inner())
                    .ok(),
            )
        };
        let (from, to) = futures_util::future::try_join(from, to).await?;
        let from_bank = async {
            if let Some(ref from) = from {
                if from.parent_account_id.is_empty() {
                    Ok(None)
                } else {
                    Result::<Option<AccountInfo>, Status>::Ok(
                        self.query_client
                            .clone()
                            .get_account_info(Request::new(
                                signer
                                    .sign_request(sdk::GetAccountRequest {
                                        id: from.parent_account_id.clone(),
                                    })
                                    .await
                                    .map_err(|err| Status::internal(err.to_string()))?
                                    .into(),
                            ))
                            .await
                            .map(|res| res.into_inner())
                            .ok(),
                    )
                }
            } else {
                Ok(None)
            }
        };
        let to_bank = async {
            if let Some(ref to) = to {
                if to.parent_account_id.is_empty() {
                    Ok(None)
                } else {
                    Result::<Option<AccountInfo>, Status>::Ok(
                        self.query_client
                            .clone()
                            .get_account_info(Request::new(
                                signer
                                    .sign_request(sdk::GetAccountRequest {
                                        id: to.parent_account_id.clone(),
                                    })
                                    .await
                                    .map_err(|err| Status::internal(err.to_string()))?
                                    .into(),
                            ))
                            .await
                            .map(|res| res.into_inner())
                            .ok(),
                    )
                }
            } else {
                Ok(None)
            }
        };
        let (from_bank, to_bank) = futures_util::future::try_join(from_bank, to_bank).await?;

        Ok(EnhancedTransferStep {
            from,
            to,
            from_bank,
            to_bank,
        })
    }

    pub async fn list_banks(
        &self,
        request: SignedRequest<sdk::ListBanksRequest>,
    ) -> Result<sdk::ListBanksResponse, Status> {
        self.query_client
            .clone()
            .list_banks(Request::new(request.into()))
            .await
            .map(tonic::Response::into_inner)
    }

    pub async fn get_bank(
        &self,
        request: SignedRequest<sdk::GetBankRequest>,
    ) -> Result<sdk::Bank, Status> {
        self.query_client
            .clone()
            .get_bank(Request::new(request.into()))
            .await
            .map(tonic::Response::into_inner)
    }
}
