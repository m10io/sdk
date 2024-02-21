use core::convert::{Into, TryFrom};
use std::{pin::Pin, sync::Arc};

use async_trait::async_trait;
use futures_core::Stream;
use futures_util::{future::try_join_all, StreamExt, TryStreamExt};
use m10_protos::sdk::{
    self, m10_query_service_client::M10QueryServiceClient,
    m10_tx_service_client::M10TxServiceClient,
};
use m10_signing::Signer;
use tonic::{
    transport::{Channel, Endpoint},
    Request,
};

use crate::{
    account::AccountId,
    builders::*,
    error::{M10Error, M10Result},
    types::*,
};

pub struct GrpcClient<S> {
    tx_client: M10TxServiceClient<Channel>,
    query_client: M10QueryServiceClient<Channel>,
    signer: Option<Arc<S>>,
}

impl<S> Clone for GrpcClient<S> {
    fn clone(&self) -> Self {
        Self {
            tx_client: self.tx_client.clone(),
            query_client: self.query_client.clone(),
            signer: self.signer.clone(),
        }
    }
}

impl<S> GrpcClient<S> {
    pub fn new(endpoint: Endpoint, signer: Option<Arc<S>>) -> M10Result<Self> {
        let channel = endpoint.connect_lazy()?;
        let tx_client = M10TxServiceClient::new(channel.clone());
        let query_client = M10QueryServiceClient::new(channel);
        Ok(Self {
            tx_client,
            query_client,
            signer,
        })
    }

    pub async fn connect(endpoint: Endpoint, signer: Option<Arc<S>>) -> M10Result<Self> {
        let channel = endpoint.connect().await?;
        let tx_client = M10TxServiceClient::new(channel.clone());
        let query_client = M10QueryServiceClient::new(channel);
        Ok(Self {
            tx_client,
            query_client,
            signer,
        })
    }
}

#[async_trait]
impl<S: Signer> crate::m10_core_client::M10CoreClient for GrpcClient<S> {
    type Signer = S;

    fn signer(&self) -> M10Result<Arc<S>> {
        self.signer.clone().ok_or(M10Error::NoSigner)
    }

    fn set_signer(&mut self, signer: Arc<Self::Signer>) {
        self.signer = Some(signer);
    }

    // Transactions
    async fn create_transaction(
        &self,
        payload: sdk::RequestEnvelope,
    ) -> M10Result<sdk::TransactionResponse> {
        Ok(self
            .tx_client
            .clone()
            .create_transaction(Request::new(payload))
            .await
            .map(|res| res.into_inner())?
            .tx_error()?)
    }

    async fn bulk_create_transactions(
        &self,
        payload: sdk::BulkTransactions,
    ) -> M10Result<sdk::BulkTransactionsResponse> {
        Ok(self
            .tx_client
            .clone()
            .bulk_create_transactions(Request::new(payload))
            .await
            .map(|res| res.into_inner())?)
    }

    // Queries
    async fn get_block_height(&self) -> M10Result<u64> {
        Ok(self
            .query_client
            .clone()
            .get_chain_info(Request::new(()))
            .await?
            .into_inner()
            .block_height)
    }

    async fn get_offline_key(&self) -> M10Result<Vec<u8>> {
        Ok(self
            .query_client
            .clone()
            .get_offline_key(Request::new(()))
            .await?
            .into_inner()
            .offline_pk)
    }

    async fn get_account(&self, id: AccountId) -> M10Result<Account> {
        let account = self.get_indexed_account(id).await?;
        Account::try_from(account)
    }

    async fn get_indexed_account(&self, id: AccountId) -> M10Result<sdk::IndexedAccount> {
        let req = self
            .signer()?
            .sign_request(sdk::GetAccountRequest { id: id.to_vec() })
            .await?;
        let account = self
            .query_client
            .clone()
            .get_indexed_account(Request::new(req.into()))
            .await
            .map(|res| res.into_inner())?;
        Ok(account)
    }

    async fn get_account_info(&self, id: AccountId) -> M10Result<AccountInfo> {
        let account = self.get_raw_account_info(id.to_vec()).await?;
        AccountInfo::try_from(account)
    }

    async fn get_raw_account_info(&self, id: Vec<u8>) -> M10Result<sdk::AccountInfo> {
        let req = self
            .signer()?
            .sign_request(sdk::GetAccountRequest { id })
            .await?;
        let account = self
            .query_client
            .clone()
            .get_account_info(Request::new(req.into()))
            .await
            .map(|res| res.into_inner())?;
        Ok(account)
    }

    async fn list_accounts(
        &self,
        filter: PageBuilder<Vec<u8>, NameOrOwnerFilter>,
    ) -> M10Result<Vec<Account>> {
        let req = self
            .signer()?
            .sign_request::<sdk::ListAccountMetadataRequest>(filter.into())
            .await?;
        let accounts = self
            .query_client
            .clone()
            .list_account_metadata(Request::new(req.into()))
            .await
            .map(|res| res.into_inner())?;
        let accounts = try_join_all(
            accounts
                .accounts
                .into_iter()
                // TODO: Remove indexed vs non-indexed differentiation
                .filter_map(|account| match AccountId::try_from_be_slice(&account.id) {
                    Ok(id) => Some(async move { self.get_account(id).await }),
                    Err(_) => None,
                }),
        )
        .await?;
        Ok(accounts)
    }

    // Transfers

    async fn get_transfer(&self, tx_id: TxId) -> M10Result<Transfer> {
        let transfer = self.get_raw_transfer(tx_id).await?;
        Transfer::try_from(transfer)
    }

    async fn get_raw_transfer(&self, tx_id: TxId) -> M10Result<sdk::FinalizedTransfer> {
        let req = self
            .signer()?
            .sign_request(sdk::GetTransferRequest { tx_id })
            .await?;
        let transfer = self
            .query_client
            .clone()
            .get_transfer(Request::new(req.into()))
            .await
            .map(|res| res.into_inner())?;
        Ok(transfer)
    }

    async fn get_enhanced_transfer(&self, tx_id: TxId) -> M10Result<ExpandedTransfer> {
        let req = self
            .signer()?
            .sign_request(sdk::GetTransferRequest { tx_id })
            .await?;
        let transfer = self
            .query_client
            .clone()
            .get_transfer(Request::new(req.into()))
            .await
            .map(|res| res.into_inner())?;
        let enhanced = self.enhance_transfer(transfer).await?;
        ExpandedTransfer::try_from(enhanced)
    }

    async fn get_enhanced_transfers(
        &self,
        filter: TxnFilter<TransferFilter>,
    ) -> M10Result<Vec<ExpandedTransfer>> {
        let req = self
            .signer()?
            .sign_request::<sdk::ListTransferRequest>(filter.into())
            .await?;
        let transfers = self
            .query_client
            .clone()
            .list_transfers(Request::new(req.into()))
            .await
            .map(|res| res.into_inner())?;
        self.enhance_transfers(transfers.transfers)
            .await?
            .into_iter()
            .map(ExpandedTransfer::try_from)
            .collect::<M10Result<_>>()
    }

    async fn list_transfers(&self, filter: TxnFilter<TransferFilter>) -> M10Result<Vec<Transfer>> {
        let transfers = self.list_raw_transfers(filter).await?;
        let transfers = transfers
            .transfers
            .into_iter()
            .map(Transfer::try_from)
            .collect::<M10Result<_>>()?;
        Ok(transfers)
    }

    async fn list_raw_transfers(
        &self,
        filter: TxnFilter<TransferFilter>,
    ) -> M10Result<sdk::FinalizedTransfers> {
        let req = self
            .signer()?
            .sign_request::<sdk::ListTransferRequest>(filter.into())
            .await?;
        let transfers = self
            .query_client
            .clone()
            .list_transfers(Request::new(req.into()))
            .await
            .map(|res| res.into_inner())?;
        Ok(transfers)
    }

    // Actions

    async fn get_action(&self, tx_id: TxId) -> M10Result<Action> {
        let req = self
            .signer()?
            .sign_request(sdk::GetActionRequest { tx_id })
            .await?;
        let action = self
            .query_client
            .clone()
            .get_action(Request::new(req.into()))
            .await
            .map(|res| res.into_inner())?;
        Action::try_from(action)
    }

    async fn list_actions(&self, filter: TxnFilter<ActionsFilter>) -> M10Result<Vec<Action>> {
        let req = self
            .signer()?
            .sign_request::<sdk::ListActionsRequest>(filter.into())
            .await?;
        let actions = self
            .query_client
            .clone()
            .list_actions(Request::new(req.into()))
            .await
            .map(|res| res.into_inner())?;
        let actions = actions
            .actions
            .into_iter()
            .map(Action::try_from)
            .collect::<M10Result<_>>()?;
        Ok(actions)
    }

    // Transactions

    async fn list_transactions(
        &self,
        filter: TxnFilter<ContextFilter>,
    ) -> M10Result<Vec<Transaction>> {
        let req = self
            .signer()?
            .sign_request::<sdk::ListTransactionsRequest>(filter.into())
            .await?;
        let txs = self
            .query_client
            .clone()
            .list_transactions(Request::new(req.into()))
            .await
            .map(|res| res.into_inner())?;
        let txs = txs
            .transactions
            .into_iter()
            .map(Transaction::try_from)
            .collect::<M10Result<_>>()?;
        Ok(txs)
    }

    async fn group_transactions(
        &self,
        filter: TxnFilter<GroupingFilter>,
    ) -> M10Result<Vec<Vec<Transaction>>> {
        let req = self
            .signer()?
            .sign_request::<sdk::GroupTransactionsRequest>(filter.into())
            .await?;
        let groups = self
            .query_client
            .clone()
            .group_transactions(Request::new(req.into()))
            .await
            .map(|res| res.into_inner())?;
        let groups = groups
            .groups
            .into_iter()
            .map(|txs| {
                txs.transactions
                    .into_iter()
                    .map(Transaction::try_from)
                    .collect::<M10Result<Vec<_>>>()
            })
            .collect::<M10Result<_>>()?;
        Ok(groups)
    }

    // Banks

    async fn get_bank(&self, id: Vec<u8>) -> M10Result<Bank> {
        let req = self
            .signer()?
            .sign_request(sdk::GetBankRequest { id })
            .await?;
        let bank = self
            .query_client
            .clone()
            .get_bank(Request::new(req.into()))
            .await
            .map(|res| res.into_inner())?;
        Bank::try_from(bank)
    }

    async fn list_banks(&self, builder: PageBuilder<Vec<u8>>) -> M10Result<Vec<Bank>> {
        let req = self
            .signer()?
            .sign_request(sdk::ListBanksRequest {
                page: builder.into(),
            })
            .await?;
        let banks = self
            .query_client
            .clone()
            .list_banks(Request::new(req.into()))
            .await
            .map(|res| res.into_inner())?;
        let banks = banks
            .banks
            .into_iter()
            .map(Bank::try_from)
            .collect::<M10Result<_>>()?;
        Ok(banks)
    }

    // Roles

    async fn get_role(&self, id: Vec<u8>) -> M10Result<Role> {
        let req = self
            .signer()?
            .sign_request(sdk::GetRoleRequest { id })
            .await?;
        let role = self
            .query_client
            .clone()
            .get_role(Request::new(req.into()))
            .await
            .map(|res| res.into_inner())?;
        Role::try_from(role)
    }

    async fn list_roles(&self, builder: PageBuilder<Vec<u8>, NameFilter>) -> M10Result<Vec<Role>> {
        let req = self
            .signer()?
            .sign_request::<sdk::ListRolesRequest>(builder.into())
            .await?;
        let roles = self
            .query_client
            .clone()
            .list_roles(Request::new(req.into()))
            .await
            .map(|res| res.into_inner())?;
        let roles = roles
            .roles
            .into_iter()
            .map(Role::try_from)
            .collect::<M10Result<_>>()?;
        Ok(roles)
    }

    // Role-bindings

    async fn get_role_binding(&self, id: Vec<u8>) -> M10Result<RoleBinding> {
        let req = self
            .signer()?
            .sign_request(sdk::GetRoleBindingRequest { id })
            .await?;
        let role_binding = self
            .query_client
            .clone()
            .get_role_binding(Request::new(req.into()))
            .await
            .map(|res| res.into_inner())?;
        RoleBinding::try_from(role_binding)
    }

    async fn list_role_bindings(
        &self,
        builder: PageBuilder<Vec<u8>, NameFilter>,
    ) -> M10Result<Vec<RoleBinding>> {
        let req = self
            .signer()?
            .sign_request::<sdk::ListRoleBindingsRequest>(builder.into())
            .await?;
        let role_bindings = self
            .query_client
            .clone()
            .list_role_bindings(Request::new(req.into()))
            .await
            .map(|res| res.into_inner())?;
        let role_bindings = role_bindings
            .role_bindings
            .into_iter()
            .map(RoleBinding::try_from)
            .collect::<M10Result<_>>()?;
        Ok(role_bindings)
    }

    // AccountSets

    async fn get_account_set(&self, id: Vec<u8>) -> M10Result<AccountSet> {
        let req = self
            .signer()?
            .sign_request(sdk::GetAccountSetRequest { id })
            .await?;
        let set = self
            .query_client
            .clone()
            .get_account_set(Request::new(req.into()))
            .await
            .map(|res| res.into_inner())?;
        AccountSet::try_from(set)
    }

    async fn list_account_sets(
        &self,
        builder: PageBuilder<Vec<u8>, NameOrOwnerFilter>,
    ) -> M10Result<Vec<AccountSet>> {
        let req = self
            .signer()?
            .sign_request::<sdk::ListAccountSetsRequest>(builder.into())
            .await?;
        let account_sets = self
            .query_client
            .clone()
            .list_account_sets(Request::new(req.into()))
            .await
            .map(|res| res.into_inner())?;
        let account_sets = account_sets
            .account_sets
            .into_iter()
            .map(AccountSet::try_from)
            .collect::<M10Result<_>>()?;
        Ok(account_sets)
    }
    // Account Metadata

    async fn get_account_metadata(&self, id: AccountId) -> M10Result<AccountMetadata> {
        let req = self
            .signer()?
            .sign_request(sdk::GetAccountRequest { id: id.to_vec() })
            .await?;
        let metadata = self
            .query_client
            .clone()
            .get_account_metadata(Request::new(req.into()))
            .await
            .map(|res| res.into_inner())?;
        AccountMetadata::try_from(metadata)
    }

    async fn list_account_metadata(
        &self,
        builder: PageBuilder<Vec<u8>, NameOrOwnerFilter>,
    ) -> M10Result<Vec<AccountMetadata>> {
        let req = self
            .signer()?
            .sign_request::<sdk::ListAccountMetadataRequest>(builder.into())
            .await?;
        let account_metadata = self
            .query_client
            .clone()
            .list_account_metadata(Request::new(req.into()))
            .await
            .map(|res| res.into_inner())?;
        let account_sets = account_metadata
            .accounts
            .into_iter()
            .map(AccountMetadata::try_from)
            .collect::<M10Result<_>>()?;
        Ok(account_sets)
    }

    // Observers
    async fn observe_accounts(
        &self,
        filter: AccountFilter,
    ) -> M10Result<Pin<Box<dyn Stream<Item = M10Result<Vec<AccountUpdate>>> + Send + Sync + 'static>>>
    {
        let req = self
            .signer()?
            .sign_request::<sdk::ObserveAccountsRequest>(filter.into())
            .await?;
        let stream = self
            .query_client
            .clone()
            .observe_accounts(Request::new(req.into()))
            .await
            .map(tonic::Response::into_inner)?
            .into_stream()
            .map(|res| match res {
                Ok(txs) => Ok(txs
                    .transactions
                    .into_iter()
                    .map(AccountUpdate::try_from)
                    .collect::<M10Result<Vec<_>>>()?),
                Err(err) => Err(M10Error::from(err)),
            });
        Ok(Box::pin(stream))
    }

    async fn observe_transfers(
        &self,
        filter: AccountFilter,
    ) -> M10Result<Pin<Box<dyn Stream<Item = M10Result<Vec<Transfer>>> + Send + Sync + 'static>>>
    {
        let req = self
            .signer()?
            .sign_request::<sdk::ObserveAccountsRequest>(filter.into())
            .await?;
        let stream = self
            .query_client
            .clone()
            .observe_transfers(Request::new(req.into()))
            .await
            .map(tonic::Response::into_inner)?
            .into_stream()
            .map(|res| match res {
                Ok(txs) => Ok(txs
                    .transactions
                    .into_iter()
                    .map(Transfer::try_from)
                    .collect::<M10Result<Vec<_>>>()?),
                Err(err) => Err(M10Error::from(err)),
            });
        Ok(Box::pin(stream))
    }

    async fn observe_transactions(
        &self,
        filter: AccountFilter,
    ) -> M10Result<
        Pin<Box<dyn Stream<Item = M10Result<sdk::FinalizedTransactions>> + Send + Sync + 'static>>,
    > {
        let req = self
            .signer()?
            .sign_request::<sdk::ObserveAccountsRequest>(filter.into())
            .await?;
        let stream = self
            .query_client
            .clone()
            .observe_transfers(Request::new(req.into()))
            .await
            .map(tonic::Response::into_inner)?
            .into_stream()
            .map(|res| match res {
                Ok(txs) => Ok(txs),
                Err(err) => Err(M10Error::from(err)),
            });
        Ok(Box::pin(stream))
    }

    async fn observe_actions(
        &self,
        filter: AccountFilter<NamedAction>,
    ) -> M10Result<Pin<Box<dyn Stream<Item = M10Result<Vec<Action>>> + Send + Sync + 'static>>>
    {
        let req = self
            .signer()?
            .sign_request::<sdk::ObserveActionsRequest>(filter.into())
            .await?;
        let stream = self
            .query_client
            .clone()
            .observe_actions(Request::new(req.into()))
            .await
            .map(tonic::Response::into_inner)?
            .into_stream()
            .map(|res| match res {
                Ok(txs) => Ok(txs
                    .transactions
                    .into_iter()
                    .map(Action::try_from)
                    .collect::<M10Result<Vec<_>>>()?),
                Err(err) => Err(M10Error::from(err)),
            });
        Ok(Box::pin(stream))
    }

    async fn observe_raw_actions(
        &self,
        filter: AccountFilter<NamedAction>,
    ) -> M10Result<
        Pin<Box<dyn Stream<Item = M10Result<sdk::FinalizedTransactions>> + Send + Sync + 'static>>,
    > {
        let req = self
            .signer()?
            .sign_request::<sdk::ObserveActionsRequest>(filter.into())
            .await?;
        let stream = self
            .query_client
            .clone()
            .observe_actions(Request::new(req.into()))
            .await
            .map(tonic::Response::into_inner)?
            .into_stream()
            .map(|res| match res {
                Ok(txs) => Ok(txs),
                Err(err) => Err(M10Error::from(err)),
            });
        Ok(Box::pin(stream))
    }

    async fn observe_resources(
        &self,
        request: sdk::ObserveResourcesRequest,
    ) -> M10Result<
        Pin<Box<dyn Stream<Item = M10Result<sdk::FinalizedTransactions>> + Send + Sync + 'static>>,
    > {
        let req = self.signer()?.sign_request(request).await?;
        let stream = self
            .query_client
            .clone()
            .observe_resources(Request::new(req.into()))
            .await
            .map(tonic::Response::into_inner)?
            .into_stream()
            .map(|res| match res {
                Ok(txs) => Ok(txs),
                Err(err) => Err(M10Error::from(err)),
            });
        Ok(Box::pin(stream))
    }

    async fn observe_metrics(
        &self,
        filter: AccountFilter,
    ) -> M10Result<
        Pin<Box<dyn Stream<Item = M10Result<sdk::TransactionMetrics>> + Send + Sync + 'static>>,
    > {
        let req = self
            .signer()?
            .sign_request::<sdk::ObserveAccountsRequest>(filter.into())
            .await?;
        let stream = self
            .query_client
            .clone()
            .observe_metrics(Request::new(req.into()))
            .await
            .map(tonic::Response::into_inner)?
            .into_stream()
            .map_err(M10Error::from);
        Ok(Box::pin(stream))
    }
}
