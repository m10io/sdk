use core::convert::Into;
use std::{
    pin::Pin,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use futures_core::Stream;
use m10_protos::sdk;
use m10_protos::sdk::signature::Algorithm;
use m10_signing::{PreparedTransaction, SignedRequest, Signer};

use crate::{
    account::AccountId, builders::*, error::M10Result, transfer_ext::EnhancedTransfer, types::*,
    DocumentId, EnhancedTransferStep,
};

#[async_trait]
pub trait M10CoreClient {
    type Signer: Signer;

    // Signing
    fn signer(&self) -> M10Result<Arc<Self::Signer>>;

    fn set_signer(&mut self, signer: Arc<Self::Signer>);

    async fn signed_transaction(
        &self,
        data: sdk::transaction_data::Data,
        context_id: Vec<u8>,
    ) -> M10Result<SignedRequest<sdk::TransactionRequestPayload>> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_micros() as u64;
        let req = sdk::TransactionRequestPayload {
            nonce: fastrand::u64(..),
            timestamp,
            context_id,
            data: Some(sdk::TransactionData { data: Some(data) }),
        };
        sign_prepared_or_raw(req, self.signer()?.as_ref()).await
    }

    // Transactions
    async fn create_transaction(
        &self,
        payload: sdk::RequestEnvelope,
    ) -> M10Result<sdk::TransactionResponse>;

    async fn bulk_create_transactions(
        &self,
        payload: sdk::BulkTransactions,
    ) -> M10Result<sdk::BulkTransactionsResponse>;

    async fn transfer(&self, data: sdk::CreateTransfer, context_id: Vec<u8>) -> M10Result<TxId> {
        let req = self.signed_transaction(data.into(), context_id).await?;
        let response = self.create_transaction(req.into()).await?;
        Ok(response.tx_id)
    }

    async fn create_account(
        &self,
        data: sdk::CreateLedgerAccount,
        context_id: Vec<u8>,
    ) -> M10Result<(TxId, AccountId)> {
        let req = self.signed_transaction(data.into(), context_id).await?;
        let response = self.create_transaction(req.into()).await?;
        let account_id = AccountId::try_from_be_slice(&response.account_created)?;
        Ok((response.tx_id, account_id))
    }

    async fn initiate_transfer(
        &self,
        data: sdk::CreateTransfer,
        context_id: Vec<u8>,
    ) -> M10Result<TxId> {
        let req = self
            .signed_transaction(
                sdk::transaction_data::Data::InitiateTransfer(data),
                context_id,
            )
            .await?;
        let response = self.create_transaction(req.into()).await?;
        Ok(response.tx_id)
    }

    async fn commit_transfer(
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
                }
                .into(),
                context_id,
            )
            .await?;
        let response = self.create_transaction(req.into()).await?;
        Ok(response.tx_id)
    }

    async fn freeze_account(
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
                }
                .into(),
                context_id,
            )
            .await?;
        let response = self.create_transaction(req.into()).await?;
        Ok(response.tx_id)
    }

    async fn set_account_limit(
        &self,
        account_id: AccountId,
        limit: u64,
        context_id: Vec<u8>,
    ) -> M10Result<TxId> {
        let req = self
            .signed_transaction(
                sdk::SetBalanceLimit {
                    account_id: account_id.to_vec(),
                    balance_limit: limit,
                }
                .into(),
                context_id,
            )
            .await?;
        let response = self.create_transaction(req.into()).await?;
        Ok(response.tx_id)
    }

    async fn set_issuance_limit(
        &self,
        account_id: AccountId,
        limit: u64,
        context_id: Vec<u8>,
    ) -> M10Result<TxId> {
        let req = self
            .signed_transaction(
                sdk::SetIssuanceLimit {
                    account_id: account_id.to_vec(),
                    issuance_limit: limit,
                }
                .into(),
                context_id,
            )
            .await?;
        let response = self.create_transaction(req.into()).await?;
        Ok(response.tx_id)
    }

    async fn set_display_code(
        &self,
        account_id: AccountId,
        display_code: String,
        context_id: Vec<u8>,
    ) -> M10Result<TxId> {
        let req = self
            .signed_transaction(
                sdk::SetDisplayCode {
                    account_id: account_id.to_vec(),
                    display_code,
                }
                .into(),
                context_id,
            )
            .await?;
        let response = self.create_transaction(req.into()).await?;
        Ok(response.tx_id)
    }

    async fn set_account_instrument(
        &self,
        account_id: AccountId,
        code: String,
        decimals: u32,
        description: Option<String>,
        context_id: Vec<u8>,
        display_code: Option<String>,
    ) -> M10Result<TxId> {
        let req = self
            .signed_transaction(
                sdk::SetInstrument {
                    account_id: account_id.to_vec(),
                    code: code.clone(),
                    decimal_places: decimals,
                    description: description.unwrap_or_default(),
                    display_code: display_code.unwrap_or(code),
                }
                .into(),
                context_id,
            )
            .await?;
        let response = self.create_transaction(req.into()).await?;
        Ok(response.tx_id)
    }

    async fn action(&self, data: sdk::InvokeAction, context_id: Vec<u8>) -> M10Result<TxId> {
        let req = self.signed_transaction(data.into(), context_id).await?;
        let response = self.create_transaction(req.into()).await?;
        Ok(response.tx_id)
    }

    async fn documents(
        &self,
        data: sdk::DocumentOperations,
        context_id: Vec<u8>,
    ) -> M10Result<TxId> {
        let req = self.signed_transaction(data.into(), context_id).await?;
        let response = self.create_transaction(req.into()).await?;
        Ok(response.tx_id)
    }

    async fn create_token(
        &self,
        account: AccountId,
        value: u64,
        address: Option<Vec<u8>>,
        context_id: Vec<u8>,
    ) -> M10Result<(TxId, Option<sdk::OfflineToken>)> {
        let address = address.unwrap_or(self.signer()?.public_key().to_vec());
        let req = self
            .signed_transaction(
                sdk::CreateToken {
                    address,
                    account_id: account.to_vec(),
                    value,
                }
                .into(),
                context_id,
            )
            .await?;
        let response = self.create_transaction(req.into()).await?;
        Ok((response.tx_id, response.token))
    }

    async fn redeem_token(
        &self,
        token: sdk::RedeemableToken,
        account_id: AccountId,
        context_id: Vec<u8>,
    ) -> M10Result<TxId> {
        let req = self
            .signed_transaction(
                sdk::RedeemToken {
                    token: Some(token),
                    account_id: account_id.to_vec(),
                }
                .into(),
                context_id,
            )
            .await?;
        let response = self.create_transaction(req.into()).await?;
        Ok(response.tx_id)
    }

    // Queries
    async fn get_block_height(&self) -> M10Result<u64>;

    async fn get_offline_key(&self) -> M10Result<Vec<u8>>;

    async fn get_account(&self, id: AccountId) -> M10Result<Account>;

    async fn get_indexed_account(&self, id: AccountId) -> M10Result<sdk::IndexedAccount>;

    async fn get_account_info(&self, id: AccountId) -> M10Result<AccountInfo>;

    async fn get_raw_account_info(&self, id: Vec<u8>) -> M10Result<sdk::AccountInfo>;

    async fn list_accounts(
        &self,
        filter: PageBuilder<Vec<u8>, AccountMetadataFilter>,
    ) -> M10Result<Vec<Account>>;

    async fn get_transfer(&self, tx_id: TxId) -> M10Result<Transfer>;

    async fn get_raw_transfer(&self, tx_id: TxId) -> M10Result<sdk::FinalizedTransfer>;

    async fn get_enhanced_transfer(&self, tx_id: TxId) -> M10Result<ExpandedTransfer>;

    async fn get_enhanced_transfers(
        &self,
        filter: TxnFilter<TransferFilter>,
    ) -> M10Result<Vec<ExpandedTransfer>>;

    async fn list_transfers(&self, filter: TxnFilter<TransferFilter>) -> M10Result<Vec<Transfer>>;

    async fn list_raw_transfers(
        &self,
        filter: TxnFilter<TransferFilter>,
    ) -> M10Result<sdk::FinalizedTransfers>;

    async fn get_action(&self, tx_id: TxId) -> M10Result<Action>;

    async fn list_actions(&self, filter: TxnFilter<ActionsFilter>) -> M10Result<Vec<Action>>;

    async fn list_transactions(
        &self,
        filter: TxnFilter<ContextFilter>,
    ) -> M10Result<Vec<Transaction>>;

    async fn group_transactions(
        &self,
        filter: TxnFilter<GroupingFilter>,
    ) -> M10Result<Vec<Vec<Transaction>>>;

    async fn get_bank(&self, id: Vec<u8>) -> M10Result<Bank>;

    async fn list_banks(&self, builder: PageBuilder<Vec<u8>>) -> M10Result<Vec<Bank>>;

    async fn get_role(&self, id: Vec<u8>) -> M10Result<Role>;

    async fn list_roles(&self, builder: PageBuilder<Vec<u8>, RoleFilter>) -> M10Result<Vec<Role>>;

    async fn get_role_binding(&self, id: Vec<u8>) -> M10Result<RoleBinding>;

    async fn list_role_bindings(
        &self,
        builder: PageBuilder<Vec<u8>, RoleBindingFilter>,
    ) -> M10Result<Vec<RoleBinding>>;

    async fn get_account_set(&self, id: Vec<u8>) -> M10Result<AccountSet>;

    async fn list_account_sets(
        &self,
        builder: PageBuilder<Vec<u8>, NameOrOwnerFilter>,
    ) -> M10Result<Vec<AccountSet>>;

    async fn get_account_metadata(&self, id: Vec<u8>) -> M10Result<AccountMetadata>;

    async fn list_account_metadata(
        &self,
        builder: PageBuilder<Vec<u8>, AccountMetadataFilter>,
    ) -> M10Result<Vec<AccountMetadata>>;

    // Transfer Enhancing
    async fn enhance_transfers(
        &self,
        transfers: Vec<sdk::FinalizedTransfer>,
    ) -> M10Result<Vec<EnhancedTransfer>> {
        futures_util::future::try_join_all(
            transfers
                .into_iter()
                .map(|transfer| self.enhance_transfer(transfer)),
        )
        .await
    }

    async fn enhance_transfer(
        &self,
        transfer: sdk::FinalizedTransfer,
    ) -> M10Result<EnhancedTransfer> {
        let mut enhanced_steps = Vec::default();
        for transfer_step in &transfer.transfer_steps {
            enhanced_steps.push(self.enhance_transfer_step(transfer_step).await?);
        }
        Ok(EnhancedTransfer {
            enhanced_steps,
            transfer,
        })
    }

    async fn enhance_transfer_step(
        &self,
        transfer_step: &sdk::TransferStep,
    ) -> M10Result<EnhancedTransferStep> {
        let from = async {
            M10Result::Ok(
                self.get_raw_account_info(transfer_step.from_account_id.clone())
                    .await
                    .ok(),
            )
        };
        let to = async {
            Ok(self
                .get_raw_account_info(transfer_step.to_account_id.clone())
                .await
                .ok())
        };
        let (from, to) = futures_util::future::try_join(from, to).await?;
        let from_bank = async {
            if let Some(ref from) = from {
                if from.parent_account_id.is_empty() {
                    Ok(None)
                } else {
                    M10Result::Ok(
                        self.get_raw_account_info(from.parent_account_id.clone())
                            .await
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
                    M10Result::Ok(
                        self.get_raw_account_info(to.parent_account_id.clone())
                            .await
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

    // Observers
    async fn observe_accounts(
        &self,
        filter: AccountFilter,
    ) -> M10Result<Pin<Box<dyn Stream<Item = M10Result<Vec<AccountUpdate>>> + Send + Sync + 'static>>>;

    async fn observe_transfers(
        &self,
        filter: AccountFilter,
    ) -> M10Result<Pin<Box<dyn Stream<Item = M10Result<Vec<Transfer>>> + Send + Sync + 'static>>>;

    async fn observe_transactions(
        &self,
        filter: AccountFilter,
    ) -> M10Result<
        Pin<Box<dyn Stream<Item = M10Result<sdk::FinalizedTransactions>> + Send + Sync + 'static>>,
    >;

    async fn observe_actions(
        &self,
        filter: AccountFilter<NamedAction>,
    ) -> M10Result<Pin<Box<dyn Stream<Item = M10Result<Vec<Action>>> + Send + Sync + 'static>>>;

    async fn observe_raw_actions(
        &self,
        filter: AccountFilter<NamedAction>,
    ) -> M10Result<
        Pin<Box<dyn Stream<Item = M10Result<sdk::FinalizedTransactions>> + Send + Sync + 'static>>,
    >;

    async fn observe_resources(
        &self,
        request: sdk::ObserveResourcesRequest,
    ) -> M10Result<
        Pin<Box<dyn Stream<Item = M10Result<sdk::FinalizedTransactions>> + Send + Sync + 'static>>,
    >;

    async fn observe_metrics(
        &self,
        filter: AccountFilter,
    ) -> M10Result<
        Pin<Box<dyn Stream<Item = M10Result<sdk::TransactionMetrics>> + Send + Sync + 'static>>,
    >;
}

// Convenient functions with "impl" parameters.
// They had to be removed from the trait to make it object safe.

#[allow(clippy::borrowed_box)]
pub async fn signed_transaction<D: Into<sdk::transaction_data::Data>, S: Signer>(
    client: &Box<dyn M10CoreClient<Signer = S> + Send + Sync>,
    data: D,
    context_id: Vec<u8>,
) -> M10Result<SignedRequest<sdk::TransactionRequestPayload>> {
    client.signed_transaction(data.into(), context_id).await
}

// Routes signing through the digest path for P256 and Ed25519Ph, and falls back to
// raw signing for plain Ed25519 to preserve backward compatibility.
async fn sign_prepared_or_raw<S: Signer>(
    req: sdk::TransactionRequestPayload,
    signer: &S,
) -> M10Result<SignedRequest<sdk::TransactionRequestPayload>> {
    use m10_protos::prost::Message as _;

    match signer.algorithm() {
        Algorithm::Ed25519 => Ok(signer.sign_request(req).await?),
        algorithm => {
            let payload = req.encode_to_vec();
            let prepared = PreparedTransaction::new(payload, algorithm)?;
            prepared.verify_integrity()?;
            let sig_bytes = signer.sign_prepared_transaction(&prepared).await?;
            // Reject structurally invalid signature bytes before attaching to the envelope
            match algorithm {
                Algorithm::P256Sha256Asn1 => {
                    let _ = p256::ecdsa::Signature::from_der(&sig_bytes)
                        .map_err(|_| m10_signing::SigningError::MalFormedSignature)?;
                }
                Algorithm::Ed25519PhSha512 => {
                    if sig_bytes.len() != ed25519_dalek::SIGNATURE_LENGTH {
                        return Err(m10_signing::SigningError::MalFormedSignature.into());
                    }
                }
                Algorithm::Ed25519 => unreachable!("Ed25519 is handled in the outer match arm"),
            }
            let signature = sdk::Signature {
                algorithm: algorithm.into(),
                public_key: signer.public_key().to_vec(),
                signature: sig_bytes,
            };
            Ok(SignedRequest {
                request_envelope: sdk::RequestEnvelope {
                    payload: prepared.payload,
                    signature: Some(signature),
                },
                data: req,
            })
        }
    }
}

pub async fn signed_transaction_as<S: Signer>(
    data: sdk::transaction_data::Data,
    context_id: Vec<u8>,
    signer: Arc<S>,
) -> M10Result<SignedRequest<sdk::TransactionRequestPayload>> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_micros() as u64;
    let req = sdk::TransactionRequestPayload {
        nonce: fastrand::u64(..),
        timestamp,
        context_id,
        data: Some(sdk::TransactionData { data: Some(data) }),
    };
    sign_prepared_or_raw(req, signer.as_ref()).await
}

#[allow(clippy::borrowed_box)]
pub async fn create_transaction<S: Signer>(
    client: &Box<dyn M10CoreClient<Signer = S> + Send + Sync>,
    payload: impl Into<sdk::RequestEnvelope>,
) -> M10Result<sdk::TransactionResponse> {
    client.create_transaction(payload.into()).await
}

#[allow(clippy::borrowed_box)]
pub async fn bulk_create_transactions<S: Signer>(
    client: &Box<dyn M10CoreClient<Signer = S> + Send + Sync>,
    payload: impl Into<sdk::BulkTransactions>,
) -> M10Result<sdk::BulkTransactionsResponse> {
    client.bulk_create_transactions(payload.into()).await
}

#[allow(clippy::borrowed_box)]
pub async fn transfer<S: Signer>(
    client: &Box<dyn M10CoreClient<Signer = S> + Send + Sync>,
    builder: impl Into<Ctx<TransferBuilder>>,
) -> M10Result<TxId> {
    let builder = builder.into();
    let req = signed_transaction::<sdk::CreateTransfer, S>(
        client,
        builder.value.into(),
        builder.context_id,
    )
    .await?;
    let response = client.create_transaction(req.into()).await?;
    Ok(response.tx_id)
}

#[allow(clippy::borrowed_box)]
pub async fn create_account<S: Signer>(
    client: &Box<dyn M10CoreClient<Signer = S> + Send + Sync>,
    builder: impl Into<Ctx<AccountBuilder>>,
) -> M10Result<(TxId, AccountId)> {
    let builder = builder.into();
    let req = signed_transaction::<sdk::CreateLedgerAccount, S>(
        client,
        builder.value.into(),
        builder.context_id,
    )
    .await?;
    let response = client.create_transaction(req.into()).await?;
    let account_id = AccountId::try_from_be_slice(&response.account_created)?;
    Ok((response.tx_id, account_id))
}

#[allow(clippy::borrowed_box)]
pub async fn initiate_transfer<S: Signer>(
    client: &Box<dyn M10CoreClient<Signer = S> + Send + Sync>,
    builder: impl Into<Ctx<TransferBuilder>>,
) -> M10Result<TxId> {
    let builder = builder.into();
    let req = signed_transaction(
        client,
        sdk::transaction_data::Data::InitiateTransfer(builder.value.into()),
        builder.context_id,
    )
    .await?;
    let response = client.create_transaction(req.into()).await?;
    Ok(response.tx_id)
}

#[allow(clippy::borrowed_box)]
pub async fn set_account_instrument<S: Signer>(
    client: &Box<dyn M10CoreClient<Signer = S> + Send + Sync>,
    account_id: AccountId,
    code: impl Into<String>,
    decimals: u32,
    description: Option<impl Into<String>>,
    context_id: Vec<u8>,
    display_code: impl Into<String>,
) -> M10Result<TxId> {
    let req = signed_transaction(
        client,
        sdk::SetInstrument {
            account_id: account_id.to_vec(),
            code: code.into(),
            decimal_places: decimals,
            description: description.map(|d| d.into()).unwrap_or_default(),
            display_code: display_code.into(),
        },
        context_id,
    )
    .await?;
    let response = client.create_transaction(req.into()).await?;
    Ok(response.tx_id)
}

#[allow(clippy::borrowed_box)]
pub async fn action<S: Signer>(
    client: &Box<dyn M10CoreClient<Signer = S> + Send + Sync>,
    builder: impl Into<Ctx<ActionBuilder>>,
) -> M10Result<TxId> {
    let builder = builder.into();
    let req = signed_transaction::<sdk::InvokeAction, S>(
        client,
        builder.value.into(),
        builder.context_id,
    )
    .await?;
    let response = client.create_transaction(req.into()).await?;
    Ok(response.tx_id)
}

#[allow(clippy::borrowed_box)]
pub async fn documents<S: Signer>(
    client: &Box<dyn M10CoreClient<Signer = S> + Send + Sync>,
    builder: impl Into<Ctx<DocumentBuilder>>,
) -> M10Result<TxId> {
    let builder = builder.into();
    let req = signed_transaction::<sdk::DocumentOperations, S>(
        client,
        builder.value.into(),
        builder.context_id,
    )
    .await?;
    let response = client.create_transaction(req.into()).await?;
    Ok(response.tx_id)
}

#[allow(clippy::borrowed_box)]
pub async fn get_bank<S: Signer>(
    client: &Box<dyn M10CoreClient<Signer = S> + Send + Sync>,
    id: impl DocumentId,
) -> M10Result<Bank> {
    client.get_bank(id.into_vec()).await
}

#[allow(clippy::borrowed_box)]
pub async fn get_role<S: Signer>(
    client: &Box<dyn M10CoreClient<Signer = S> + Send + Sync>,
    id: impl DocumentId,
) -> M10Result<Role> {
    client.get_role(id.into_vec()).await
}

#[allow(clippy::borrowed_box)]
pub async fn get_role_binding<S: Signer>(
    client: &Box<dyn M10CoreClient<Signer = S> + Send + Sync>,
    id: impl DocumentId,
) -> M10Result<RoleBinding> {
    client.get_role_binding(id.into_vec()).await
}

#[allow(clippy::borrowed_box)]
pub async fn get_account_set<S: Signer>(
    client: &Box<dyn M10CoreClient<Signer = S> + Send + Sync>,
    id: impl DocumentId,
) -> M10Result<AccountSet> {
    client.get_account_set(id.into_vec()).await
}
