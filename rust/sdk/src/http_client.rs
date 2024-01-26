use core::convert::{Into, TryFrom};
use std::{pin::Pin, sync::Arc};

use async_trait::async_trait;
use futures_core::Stream;
use futures_util::future::try_join_all;
use m10_protos::{prost::Message, sdk};
use m10_signing::Signer;
use reqwest::Client;
use tonic::transport::Endpoint;
use uuid::Uuid;

use crate::{
    account::AccountId,
    builders::*,
    error::{M10Error, M10Result},
    types::*,
};

#[derive(Clone)]
pub struct HttpClient<S> {
    endpoint: Endpoint,
    client: Client,
    signer: Option<Arc<S>>,
}

impl<S> HttpClient<S> {
    pub fn new(endpoint: Endpoint, signer: Option<Arc<S>>) -> Self {
        Self {
            endpoint,
            client: Client::default(),
            signer,
        }
    }

    async fn get_with_request(
        &self,
        ep: &str,
        req: sdk::RequestEnvelope,
    ) -> M10Result<bytes::Bytes> {
        let mut req_body = vec![];
        req.encode(&mut req_body)?;
        Ok(self
            .client
            .get(format!("{}ledger/api/v1/{}", self.endpoint.uri(), ep,))
            .body(req_body)
            .send()
            .await?
            .bytes()
            .await?)
    }
}

#[async_trait]
impl<S: Signer> crate::m10_core_client::M10CoreClient for HttpClient<S> {
    type Signer = S;

    fn signer(&self) -> M10Result<Arc<S>> {
        self.signer.clone().ok_or(M10Error::NoSigner)
    }

    fn set_signer(&mut self, signer: Arc<Self::Signer>) {
        self.signer = Some(signer);
    }

    async fn create_transaction(
        &self,
        payload: sdk::RequestEnvelope,
    ) -> M10Result<sdk::TransactionResponse> {
        let mut req_body = vec![];
        payload.encode(&mut req_body)?;
        let mut res_body = self
            .client
            .post(format!("{}ledger/api/v1/transaction", self.endpoint.uri()))
            .body(req_body)
            .send()
            .await?
            .bytes()
            .await?;
        let res = sdk::TransactionResponse::decode(&mut res_body)?;
        Ok(res)
    }

    async fn bulk_create_transactions(
        &self,
        payload: sdk::BulkTransactions,
    ) -> M10Result<sdk::BulkTransactionsResponse> {
        let mut req_body = vec![];
        payload.encode(&mut req_body)?;
        let mut res_body = self
            .client
            .post(format!(
                "{}ledger/api/v1/bulk-transactions",
                self.endpoint.uri()
            ))
            .body(req_body)
            .send()
            .await?
            .bytes()
            .await?;
        let res = sdk::BulkTransactionsResponse::decode(&mut res_body)?;
        Ok(res)
    }

    // Queries
    async fn get_block_height(&self) -> M10Result<u64> {
        let mut msg = self
            .client
            .get(format!("{}ledger/api/v1/chain-info", self.endpoint.uri()))
            .send()
            .await?
            .bytes()
            .await?;
        let res = sdk::ChainInfo::decode(&mut msg)?;
        Ok(res.block_height)
    }

    async fn get_offline_key(&self) -> M10Result<Vec<u8>> {
        let mut msg = self
            .client
            .get(format!("{}ledger/api/v1/offline-key", self.endpoint.uri()))
            .send()
            .await?
            .bytes()
            .await?;
        let res = sdk::OfflineKey::decode(&mut msg)?;
        Ok(res.offline_pk)
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
        let mut msg = self.get_with_request("indexed-account", req.into()).await?;
        let account = sdk::IndexedAccount::decode(&mut msg)?;
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
        let mut msg = self.get_with_request("account-info", req.into()).await?;
        let account = sdk::AccountInfo::decode(&mut msg)?;
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
        let mut msg = self
            .get_with_request("accounst-metadata", req.into())
            .await?;
        let accounts = sdk::ListAccountMetadataResponse::decode(&mut msg)?;
        let accounts = try_join_all(
            accounts
                .accounts
                .into_iter()
                // TODO: Remove indexed vs non-indexed differentiation
                .map(|account| async move {
                    match AccountId::try_from_be_slice(&account.id) {
                        Ok(id) => self.get_account(id).await,
                        Err(err) => Err(M10Error::from(err)),
                    }
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
        let mut msg = self.get_with_request("transfer", req.into()).await?;
        let transfer = sdk::FinalizedTransfer::decode(&mut msg)?;
        Ok(transfer)
    }

    async fn get_enhanced_transfer(&self, tx_id: TxId) -> M10Result<ExpandedTransfer> {
        let req = self
            .signer()?
            .sign_request(sdk::GetTransferRequest { tx_id })
            .await?;
        let mut msg = self.get_with_request("transfer", req.into()).await?;
        let transfer = sdk::FinalizedTransfer::decode(&mut msg)?;
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
        let mut msg = self.get_with_request("transfers", req.into()).await?;
        let transfers = sdk::FinalizedTransfers::decode(&mut msg)?;
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
        let mut msg = self.get_with_request("transfers", req.into()).await?;
        let transfers = sdk::FinalizedTransfers::decode(&mut msg)?;
        Ok(transfers)
    }

    // Actions

    async fn get_action(&self, tx_id: TxId) -> M10Result<Action> {
        let req = self
            .signer()?
            .sign_request(sdk::GetActionRequest { tx_id })
            .await?;
        let mut msg = self.get_with_request("action", req.into()).await?;
        let action = sdk::Action::decode(&mut msg)?;
        Action::try_from(action)
    }

    async fn list_actions(&self, filter: TxnFilter<ActionsFilter>) -> M10Result<Vec<Action>> {
        let req = self
            .signer()?
            .sign_request::<sdk::ListActionsRequest>(filter.into())
            .await?;
        let mut msg = self.get_with_request("actions", req.into()).await?;
        let actions = sdk::Actions::decode(&mut msg)?;
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
        let mut msg = self.get_with_request("transactions", req.into()).await?;
        let txs = sdk::FinalizedTransactions::decode(&mut msg)?;
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
        let mut msg = self
            .get_with_request("group-transactions", req.into())
            .await?;
        let groups = sdk::GroupedFinalizedTransactions::decode(&mut msg)?;
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
        let mut msg = self.get_with_request("bank", req.into()).await?;
        let bank = sdk::Bank::decode(&mut msg)?;
        Bank::try_from(bank)
    }

    async fn list_banks(&self, builder: PageBuilder<Uuid>) -> M10Result<Vec<Bank>> {
        let req = self
            .signer()?
            .sign_request(sdk::ListBanksRequest {
                page: builder.into(),
            })
            .await?;
        let mut msg = self.get_with_request("banks", req.into()).await?;
        let banks = sdk::ListBanksResponse::decode(&mut msg)?;
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
        let mut msg = self.get_with_request("role", req.into()).await?;
        let role = sdk::Role::decode(&mut msg)?;
        Role::try_from(role)
    }

    async fn list_roles(&self, builder: PageBuilder<Uuid, NameFilter>) -> M10Result<Vec<Role>> {
        let req = self
            .signer()?
            .sign_request::<sdk::ListRolesRequest>(builder.into())
            .await?;
        let mut msg = self.get_with_request("roles", req.into()).await?;
        let roles = sdk::ListRolesResponse::decode(&mut msg)?;
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
        let mut msg = self.get_with_request("role-binding", req.into()).await?;
        let role_binding = sdk::RoleBinding::decode(&mut msg)?;
        RoleBinding::try_from(role_binding)
    }

    async fn list_role_bindings(
        &self,
        builder: PageBuilder<Uuid, NameFilter>,
    ) -> M10Result<Vec<RoleBinding>> {
        let req = self
            .signer()?
            .sign_request::<sdk::ListRoleBindingsRequest>(builder.into())
            .await?;
        let mut msg = self.get_with_request("role-bindings", req.into()).await?;
        let role_bindings = sdk::ListRoleBindingsResponse::decode(&mut msg)?;
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
        let mut msg = self.get_with_request("account-set", req.into()).await?;
        let set = sdk::AccountSet::decode(&mut msg)?;
        AccountSet::try_from(set)
    }

    async fn list_account_sets(
        &self,
        builder: PageBuilder<Uuid, NameOrOwnerFilter>,
    ) -> M10Result<Vec<AccountSet>> {
        let req = self
            .signer()?
            .sign_request::<sdk::ListAccountSetsRequest>(builder.into())
            .await?;
        let mut msg = self.get_with_request("account-sets", req.into()).await?;
        let account_sets = sdk::ListAccountSetsResponse::decode(&mut msg)?;
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
        let mut msg = self
            .get_with_request("account-metadata", req.into())
            .await?;
        let metadata = sdk::AccountMetadata::decode(&mut msg)?;
        AccountMetadata::try_from(metadata)
    }

    async fn list_account_metadata(
        &self,
        builder: PageBuilder<Uuid, NameOrOwnerFilter>,
    ) -> M10Result<Vec<AccountMetadata>> {
        let req = self
            .signer()?
            .sign_request::<sdk::ListAccountMetadataRequest>(builder.into())
            .await?;
        let mut msg = self
            .get_with_request("accounts-metadata", req.into())
            .await?;
        let account_metadata = sdk::ListAccountMetadataResponse::decode(&mut msg)?;
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
        _filter: AccountFilter,
    ) -> M10Result<Pin<Box<dyn Stream<Item = M10Result<Vec<AccountUpdate>>> + Send + Sync + 'static>>>
    {
        unimplemented!()
    }

    async fn observe_transfers(
        &self,
        _filter: AccountFilter,
    ) -> M10Result<Pin<Box<dyn Stream<Item = M10Result<Vec<Transfer>>> + Send + Sync + 'static>>>
    {
        unimplemented!()
    }

    async fn observe_transactions(
        &self,
        _filter: AccountFilter,
    ) -> M10Result<
        Pin<Box<dyn Stream<Item = M10Result<sdk::FinalizedTransactions>> + Send + Sync + 'static>>,
    > {
        unimplemented!()
    }

    async fn observe_actions(
        &self,
        _filter: AccountFilter<NamedAction>,
    ) -> M10Result<Pin<Box<dyn Stream<Item = M10Result<Vec<Action>>> + Send + Sync + 'static>>>
    {
        unimplemented!()
    }

    async fn observe_raw_actions(
        &self,
        _filter: AccountFilter<NamedAction>,
    ) -> M10Result<
        Pin<Box<dyn Stream<Item = M10Result<sdk::FinalizedTransactions>> + Send + Sync + 'static>>,
    > {
        unimplemented!()
    }

    async fn observe_resources(
        &self,
        _request: sdk::ObserveResourcesRequest,
    ) -> M10Result<
        Pin<Box<dyn Stream<Item = M10Result<sdk::FinalizedTransactions>> + Send + Sync + 'static>>,
    > {
        unimplemented!()
    }

    async fn observe_metrics(
        &self,
        _filter: AccountFilter,
    ) -> M10Result<
        Pin<Box<dyn Stream<Item = M10Result<sdk::TransactionMetrics>> + Send + Sync + 'static>>,
    > {
        unimplemented!()
    }
}
