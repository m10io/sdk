use futures_core::Stream;
use m10_sdk_protos::arcadius2;
use m10_sdk_protos::sdk::{
    self, m10_query_service_client::M10QueryServiceClient,
    m10_tx_service_client::M10TxServiceClient, transaction_request_payload::Data,
};
use m10_signing::SignedRequest;
use std::ops::Range;
use std::time::{Duration, Instant};
use tonic::{Request, Response, Status};

use crate::account::AccountId;
use crate::document_id::DocumentId;
use crate::transfer_ext::EnhancedTransfer;
use crate::{EnhancedTransferStep, Signer};
pub use tonic::transport::{Channel, ClientTlsConfig, Uri};

const DEFAULT_UPDATE_INTERVAL: Duration = Duration::from_secs(1);
const DEFAULT_BLOCK_WINDOW_SIZE: u64 = 2000;

pub fn page_from(last_id: impl DocumentId, limit: u32) -> sdk::Page {
    sdk::Page {
        last_id: last_id.into_vec(),
        limit,
    }
}

#[derive(Clone)]
pub struct Client {
    tx_client: M10TxServiceClient<Channel>,
    query_client: M10QueryServiceClient<Channel>,
    // chain metadata used for replay protection
    block_height: u64,
    update_interval: Duration,
    last_updated: Instant,
}

impl Client {
    pub fn new(grpc_channel: Channel) -> Self {
        let tx_client = M10TxServiceClient::new(grpc_channel.clone());
        let query_client = M10QueryServiceClient::new(grpc_channel);
        let block_height = 0;
        let update_interval = DEFAULT_UPDATE_INTERVAL;
        let last_updated = Instant::now() - update_interval;
        Self {
            tx_client,
            query_client,
            block_height,
            update_interval,
            last_updated,
        }
    }

    // TODO: this is not necessary once block window is replaced by timestamps
    pub async fn transaction_request(
        &mut self,
        data: impl Into<Data>,
        context_id: Vec<u8>,
    ) -> sdk::TransactionRequestPayload {
        let block_window = self.block_window().await.unwrap();
        sdk::TransactionRequestPayload {
            nonce: fastrand::u64(..),
            after_height: block_window.start,
            before_height: block_window.end,
            context_id,
            data: Some(data.into()),
        }
    }

    pub async fn block_height(&mut self) -> Result<u64, tonic::Status> {
        self.update_block_height().await?;
        Ok(self.block_height)
    }

    pub async fn create_transaction(
        &mut self,
        payload: SignedRequest<sdk::TransactionRequestPayload>,
    ) -> Result<sdk::TransactionResponse, Status> {
        self.tx_client
            .create_transaction(Request::new(payload.into()))
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
    pub async fn get_account(
        &mut self,
        request: SignedRequest<sdk::GetAccountRequest>,
    ) -> Result<sdk::Account, Status> {
        self.query_client
            .get_account(Request::new(request.into()))
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

    pub async fn list_accounts(
        &mut self,
        request: SignedRequest<sdk::ListAccountsRequest>,
    ) -> Result<sdk::ListAccountsResponse, Status> {
        self.query_client
            .list_accounts(Request::new(request.into()))
            .await
            .map(|res| res.into_inner())
    }

    // Role Bindings
    pub async fn get_role_binding(
        &mut self,
        request: SignedRequest<sdk::GetRoleBindingRequest>,
    ) -> Result<arcadius2::RoleBinding, Status> {
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
    ) -> Result<arcadius2::Role, Status> {
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
        };
        let to = async {
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
        };
        let from_parent_id = AccountId::try_from_be_slice(&transfer_step.from_account_id)
            .ok()
            .and_then(AccountId::parent_id);
        let from_bank = async {
            if let Some(id) = from_parent_id {
                Result::<_, Status>::Ok(
                    self.query_client
                        .clone()
                        .get_account_info(Request::new(
                            signer
                                .sign_request(sdk::GetAccountRequest {
                                    id: id.to_be_bytes().to_vec(),
                                })
                                .await
                                .map_err(|err| Status::internal(err.to_string()))?
                                .into(),
                        ))
                        .await
                        .map(|res| res.into_inner())
                        .ok(),
                )
            } else {
                Ok(None)
            }
        };
        let to_parent_id = AccountId::try_from_be_slice(&transfer_step.to_account_id)
            .ok()
            .and_then(AccountId::parent_id);
        let to_bank = async {
            if let Some(id) = to_parent_id {
                Result::<_, Status>::Ok(
                    self.query_client
                        .clone()
                        .get_account_info(Request::new(
                            signer
                                .sign_request(sdk::GetAccountRequest {
                                    id: id.to_be_bytes().to_vec(),
                                })
                                .await
                                .map_err(|err| Status::internal(err.to_string()))?
                                .into(),
                        ))
                        .await
                        .map(|res| res.into_inner())
                        .ok(),
                )
            } else {
                Ok(None)
            }
        };
        let (from, to, from_bank, to_bank) =
            futures_util::future::join4(from, to, from_bank, to_bank).await;

        Ok(EnhancedTransferStep {
            from: from?,
            to: to?,
            from_bank: from_bank?,
            to_bank: to_bank?,
        })
    }

    async fn block_window(&mut self) -> Result<Range<u64>, tonic::Status> {
        self.update_block_height().await?;
        Ok(self.block_height..self.block_height + DEFAULT_BLOCK_WINDOW_SIZE)
    }

    async fn update_block_height(&mut self) -> Result<(), tonic::Status> {
        let now = Instant::now();
        if (now - self.last_updated) > self.update_interval {
            let chain_info = self
                .query_client
                .get_chain_info(Request::new(()))
                .await?
                .into_inner();
            self.block_height = chain_info.block_height;
            self.last_updated = now;
        }
        Ok(())
    }
}
