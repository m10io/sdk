use crate::account::AccountId;
use crate::builders::*;
use crate::error::{M10Error, M10Result};
use crate::types::*;
use crate::{DocumentId, LedgerClient};
use core::clone::Clone;
use core::convert::{Into, TryFrom};
use futures_core::Stream;
use futures_util::{StreamExt, TryStreamExt};
use m10_protos::sdk;
use m10_signing::{SignedRequest, Signer};
use std::sync::Arc;
use uuid::Uuid;

pub use tonic::transport::Channel;

pub struct M10Client<S: Signer> {
    pub client: LedgerClient,
    signer: Arc<S>,
}

impl<S: Signer> Clone for M10Client<S> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            signer: self.signer.clone(),
        }
    }
}

impl<S: Signer> M10Client<S> {
    pub fn new(signer: S, channel: Channel) -> Self {
        Self {
            signer: Arc::new(signer),
            client: LedgerClient::new(channel),
        }
    }

    async fn signed_transaction<D: Into<sdk::transaction_data::Data>>(
        &self,
        data: D,
        context_id: Vec<u8>,
    ) -> M10Result<SignedRequest<sdk::TransactionRequestPayload>> {
        let req = LedgerClient::transaction_request(data.into(), context_id);
        let signed = self.signer.sign_request(req).await?;
        Ok(signed)
    }

    pub async fn create_account(
        &self,
        parent_id: AccountId,
        issuance: bool,
        context_id: Vec<u8>,
    ) -> M10Result<(TxId, AccountId)> {
        let req = self
            .signed_transaction(
                sdk::CreateLedgerAccount {
                    parent_id: parent_id.to_vec(),
                    frozen: false,
                    issuance,
                    instrument: None,
                    balance_limit: 0,
                },
                context_id,
            )
            .await?;
        let response = self
            .client
            .clone()
            .create_transaction(req)
            .await?
            .tx_error()?;
        let account_id = AccountId::try_from_be_slice(&response.account_created)?;
        Ok((response.tx_id, account_id))
    }

    pub async fn transfer(&self, builder: TransferBuilder, context_id: Vec<u8>) -> M10Result<TxId> {
        let req = self
            .signed_transaction::<sdk::CreateTransfer>(builder.into(), context_id)
            .await?;
        let response = self
            .client
            .clone()
            .create_transaction(req)
            .await?
            .tx_error()?;
        Ok(response.tx_id)
    }

    pub async fn initiate_transfer(
        &self,
        builder: TransferBuilder,
        context_id: Vec<u8>,
    ) -> M10Result<TxId> {
        let req = self
            .signed_transaction(
                sdk::transaction_data::Data::InitiateTransfer(builder.into()),
                context_id,
            )
            .await?;
        let response = self
            .client
            .clone()
            .create_transaction(req)
            .await?
            .tx_error()?;
        Ok(response.tx_id)
    }

    pub async fn commit_transfer(
        &self,
        tx_id: TxId,
        accept: bool,
        context_id: Vec<u8>,
    ) -> M10Result<TxId> {
        let req = self
            .signed_transaction(
                sdk::CommitTransfer {
                    pending_tx_id: tx_id,
                    new_state: if accept {
                        sdk::commit_transfer::TransferState::Accepted
                    } else {
                        sdk::commit_transfer::TransferState::Rejected
                    } as i32,
                },
                context_id,
            )
            .await?;
        let response = self
            .client
            .clone()
            .create_transaction(req)
            .await?
            .tx_error()?;
        Ok(response.tx_id)
    }

    /// Sets the [`Account`] [`frozen`] status.
    /// Frozen accounts cannot participate in transactions.
    pub async fn freeze_account(
        &self,
        account_id: AccountId,
        frozen: bool,
        context_id: Vec<u8>,
    ) -> M10Result<TxId> {
        let req = self
            .signed_transaction(
                sdk::SetFreezeState {
                    account_id: account_id.to_vec(),
                    frozen,
                },
                context_id,
            )
            .await?;
        let response = self
            .client
            .clone()
            .create_transaction(req)
            .await?
            .tx_error()?;
        Ok(response.tx_id)
    }

    pub async fn action(&self, builder: ActionBuilder, context_id: Vec<u8>) -> M10Result<TxId> {
        let req = self
            .signed_transaction::<sdk::InvokeAction>(builder.into(), context_id)
            .await?;
        let response = self
            .client
            .clone()
            .create_transaction(req)
            .await?
            .tx_error()?;
        Ok(response.tx_id)
    }

    pub async fn documents(
        &self,
        builder: DocumentBuilder,
        context_id: Vec<u8>,
    ) -> M10Result<TxId> {
        let req = self
            .signed_transaction::<sdk::DocumentOperations>(builder.into(), context_id)
            .await?;
        let response = self
            .client
            .clone()
            .create_transaction(req)
            .await?
            .tx_error()?;
        Ok(response.tx_id)
    }

    // Accounts

    pub async fn get_account(&self, id: AccountId) -> M10Result<Account> {
        let req = self
            .signer
            .sign_request(sdk::GetAccountRequest { id: id.to_vec() })
            .await?;
        let account = self.client.clone().get_indexed_account(req).await?;
        Account::try_from(account)
    }

    pub async fn observe_accounts(
        &self,
        filter: AccountFilter,
    ) -> M10Result<impl Stream<Item = M10Result<Vec<AccountUpdate>>>> {
        let req = self.signer.sign_request(filter.into()).await?;
        let stream = self
            .client
            .clone()
            .observe_accounts(req)
            .await?
            .map(|res| match res {
                Ok(txs) => Ok(txs
                    .transactions
                    .into_iter()
                    .map(AccountUpdate::try_from)
                    .collect::<M10Result<Vec<_>>>()?),
                Err(err) => Err(M10Error::from(err)),
            });
        Ok(stream)
    }

    // Transfers

    pub async fn get_transfer(&self, tx_id: TxId) -> M10Result<Transfer> {
        let req = self
            .signer
            .sign_request(sdk::GetTransferRequest { tx_id })
            .await?;
        let transfer = self.client.clone().get_transfer(req).await?;
        Transfer::try_from(transfer)
    }

    pub async fn get_enhanced_transfer(&self, tx_id: TxId) -> M10Result<ExpandedTransfer> {
        let req = self
            .signer
            .sign_request(sdk::GetTransferRequest { tx_id })
            .await?;
        let transfer = self.client.clone().get_transfer(req).await?;
        let enhanced = self
            .client
            .clone()
            .enhance_transfer(transfer, self.signer.as_ref())
            .await?;
        ExpandedTransfer::try_from(enhanced)
    }

    pub async fn get_enhanced_transfers(
        &self,
        filter: TxnFilter<TransferFilter>,
    ) -> M10Result<Vec<ExpandedTransfer>> {
        let req = self.signer.sign_request(filter.into()).await?;
        let transfers = self.client.clone().list_transfers(req).await?;
        self.client
            .clone()
            .enhance_transfers(transfers.transfers, self.signer.as_ref())
            .await?
            .into_iter()
            .map(ExpandedTransfer::try_from)
            .collect::<M10Result<_>>()
    }

    pub async fn list_transfers(
        &self,
        filter: TxnFilter<TransferFilter>,
    ) -> M10Result<Vec<Transfer>> {
        let req = self.signer.sign_request(filter.into()).await?;
        let transfers = self.client.clone().list_transfers(req).await?;
        let transfers = transfers
            .transfers
            .into_iter()
            .map(Transfer::try_from)
            .collect::<M10Result<_>>()?;
        Ok(transfers)
    }

    pub async fn observe_transfers(
        &self,
        filter: AccountFilter,
    ) -> M10Result<impl Stream<Item = M10Result<Vec<Transfer>>>> {
        let req = self.signer.sign_request(filter.into()).await?;
        let stream = self
            .client
            .clone()
            .observe_transfers(req)
            .await?
            .map(|res| match res {
                Ok(txs) => Ok(txs
                    .transactions
                    .into_iter()
                    .map(Transfer::try_from)
                    .collect::<M10Result<Vec<_>>>()?),
                Err(err) => Err(M10Error::from(err)),
            });
        Ok(stream)
    }

    // Actions

    pub async fn get_action(&self, tx_id: TxId) -> M10Result<Action> {
        let req = self
            .signer
            .sign_request(sdk::GetActionRequest { tx_id })
            .await?;
        let action = self.client.clone().get_action(req).await?;
        Action::try_from(action)
    }

    pub async fn list_actions(&self, filter: TxnFilter<ActionsFilter>) -> M10Result<Vec<Action>> {
        let req = self.signer.sign_request(filter.into()).await?;
        let actions = self.client.clone().list_actions(req).await?;
        let actions = actions
            .actions
            .into_iter()
            .map(Action::try_from)
            .collect::<M10Result<_>>()?;
        Ok(actions)
    }

    pub async fn observe_actions(
        &self,
        filter: AccountFilter<NamedAction>,
    ) -> M10Result<impl Stream<Item = M10Result<Vec<Action>>>> {
        let req = self.signer.sign_request(filter.into()).await?;
        let stream = self
            .client
            .clone()
            .observe_actions(req)
            .await?
            .map(|res| match res {
                Ok(txs) => Ok(txs
                    .transactions
                    .into_iter()
                    .map(Action::try_from)
                    .collect::<M10Result<Vec<_>>>()?),
                Err(err) => Err(M10Error::from(err)),
            });
        Ok(stream)
    }

    // Metrics
    pub async fn observe_metrics(
        &self,
        filter: AccountFilter,
    ) -> M10Result<impl Stream<Item = M10Result<sdk::TransactionMetrics>>> {
        let req = self.signer.sign_request(filter.into()).await?;
        let stream = self
            .client
            .clone()
            .observe_metrics(req)
            .await?
            .map_err(M10Error::from);
        Ok(stream)
    }

    // Banks
    pub async fn list_banks(&self, builder: PageBuilder<Uuid>) -> M10Result<Vec<Bank>> {
        let req = self
            .signer
            .sign_request(sdk::ListBanksRequest {
                page: Some(builder.into()),
            })
            .await?;
        let banks = self.client.clone().list_banks(req).await?;
        let banks = banks
            .banks
            .into_iter()
            .map(Bank::try_from)
            .collect::<M10Result<_>>()?;
        Ok(banks)
    }

    // Roles

    pub async fn get_role(&self, id: Uuid) -> M10Result<Role> {
        let req = self
            .signer
            .sign_request(sdk::GetRoleRequest { id: id.into_vec() })
            .await?;
        let role = self.client.clone().get_role(req).await?;
        Role::try_from(role)
    }

    pub async fn list_roles(&self, builder: PageBuilder<Uuid, NameFilter>) -> M10Result<Vec<Role>> {
        let req = self.signer.sign_request(builder.into()).await?;
        let roles = self.client.clone().list_roles(req).await?;
        let roles = roles
            .roles
            .into_iter()
            .map(Role::try_from)
            .collect::<M10Result<_>>()?;
        Ok(roles)
    }

    // Role-bindings

    pub async fn get_role_binding(&self, id: Uuid) -> M10Result<RoleBinding> {
        let req = self
            .signer
            .sign_request(sdk::GetRoleBindingRequest { id: id.into_vec() })
            .await?;
        let role_binding = self.client.clone().get_role_binding(req).await?;
        RoleBinding::try_from(role_binding)
    }

    pub async fn list_role_bindings(
        &self,
        builder: PageBuilder<Uuid, NameFilter>,
    ) -> M10Result<Vec<RoleBinding>> {
        let req = self.signer.sign_request(builder.into()).await?;
        let role_bindings = self.client.clone().list_role_bindings(req).await?;
        let role_bindings = role_bindings
            .role_bindings
            .into_iter()
            .map(RoleBinding::try_from)
            .collect::<M10Result<_>>()?;
        Ok(role_bindings)
    }

    // AccountSets

    pub async fn get_account_set(&self, id: Uuid) -> M10Result<AccountSet> {
        let req = self
            .signer
            .sign_request(sdk::GetAccountSetRequest { id: id.into_vec() })
            .await?;
        let set = self.client.clone().get_account_set(req).await?;
        AccountSet::try_from(set)
    }

    pub async fn list_account_sets(
        &self,
        builder: PageBuilder<Uuid, NameOrOwnerFilter>,
    ) -> M10Result<Vec<AccountSet>> {
        let req = self.signer.sign_request(builder.into()).await?;
        let account_sets = self.client.clone().list_account_sets(req).await?;
        let account_sets = account_sets
            .account_sets
            .into_iter()
            .map(AccountSet::try_from)
            .collect::<M10Result<_>>()?;
        Ok(account_sets)
    }
}
