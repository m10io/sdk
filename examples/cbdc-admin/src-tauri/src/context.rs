use std::sync::Arc;

use m10_sdk::{
    client::Channel,
    directory::directory_service_client::DirectoryServiceClient,
    prost::Message,
    sdk::{self, transaction_data::Data},
    DocumentUpdate, Ed25519, LedgerClient, SignedRequest, Signer, SigningError,
};
use uuid::Uuid;

use crate::{config::Config, types::*};

#[derive(Clone)]
pub(crate) struct Context {
    ledger: LedgerClient,
    signer: ProxySigner,
    _directory: DirectoryServiceClient<Channel>,
    config: Config,
}

impl Context {
    pub(crate) fn new(config: Config) -> anyhow::Result<Self> {
        Ok(Self {
            ledger: LedgerClient::new(config.ledger_addr.connect_lazy()?),
            signer: ProxySigner::new(&config),
            _directory: DirectoryServiceClient::new(config.directory_addr.connect_lazy()?),
            config,
        })
    }

    pub(crate) async fn ledger_info(&self) -> anyhow::Result<LedgerInfo> {
        let mut ledger = self.ledger.clone();
        let height = ledger.block_height().await?;
        Ok(LedgerInfo {
            url: self.config.ledger_addr.uri().to_string(),
            height,
        })
    }

    pub(crate) async fn block_height(&self) -> anyhow::Result<u64> {
        let mut ledger = self.ledger.clone();
        Ok(ledger.block_height().await?)
    }

    // Create a new asset with operator key
    pub(crate) async fn create_assets(
        &self,
        code: String,
        decimal_places: Option<u32>,
    ) -> anyhow::Result<String> {
        // Create Ledger Account
        let request = sdk::CreateLedgerAccount {
            parent_id: vec![],
            issuance: true,
            frozen: false,
            instrument: Some(sdk::Instrument {
                code: code.to_uppercase(),
                decimal_places: decimal_places.unwrap_or(2),
                ..Default::default()
            }),
        };
        let id = self
            .submit_transaction(request, vec![], SignerLevel::Operator)
            .await?
            .account_created;
        let encoded_id = hex::encode(&id);
        let owner = self.signer.public_key(SignerLevel::CentralBank).to_vec();

        // Create Account Meta Data
        let account = sdk::Account {
            id,
            owner,
            name: code.clone(),
            public_name: code.to_uppercase(),
            profile_image_url: String::new(),
        };

        self.submit_transaction(
            sdk::Operation::insert(account),
            vec![],
            SignerLevel::Operator,
        )
        .await?;

        // Add new Central Bank Key to Issuer Role Binding
        self.update_role_binding(
            self.config.sandbox_issuer_role,
            self.signer.public_key(SignerLevel::CentralBank).to_vec(),
            SignerLevel::Operator,
        )
        .await?;

        Ok(encoded_id)
    }

    pub(crate) async fn get_accounts(
        &self,
        owner: SignerLevel,
        signer: SignerLevel,
    ) -> anyhow::Result<Vec<AssetInfo>> {
        let mut ledger = self.ledger.clone();
        let request = sdk::ListAccountsRequest {
            filter: Some(sdk::list_accounts_request::Filter::Owner(
                self.signer.public_key(owner).to_vec(),
            )),
            page: None,
        };
        let request = self.signer.sign_request(request, signer).await?;
        let accounts = ledger.list_accounts(request).await?;
        let mut result = vec![];
        for account in accounts.accounts {
            let encoded_id = hex::encode(&account.id);
            let request = self
                .signer
                .sign_request(
                    sdk::GetAccountRequest { id: account.id },
                    SignerLevel::Operator,
                )
                .await?;
            let index_account = ledger.get_indexed_account(request.clone()).await?;
            let instrument = index_account.instrument.unwrap_or(sdk::Instrument {
                code: String::new(),
                decimal_places: 2,
                ..Default::default()
            });
            result.push(AssetInfo {
                account_id: encoded_id,
                name: account.public_name.clone(),
                issued: index_account
                    .issuance
                    .as_ref()
                    .map(|a| a.issued_balance)
                    .unwrap_or_default(),
                code: instrument.code,
                decimals: instrument.decimal_places,
            });
        }
        Ok(result)
    }

    // create a new bank with CentralBank key
    pub(crate) async fn create_bank(
        &self,
        code: String,
        name: String,
        public_key: Option<String>,
    ) -> anyhow::Result<String> {
        let assets = self
            .get_accounts(SignerLevel::CentralBank, SignerLevel::Operator)
            .await?;
        let parent_id = assets
            .iter()
            .find(|a| a.code == code.to_uppercase())
            .ok_or_else(|| anyhow::anyhow!("Asset not found"))?;
        let request = sdk::CreateLedgerAccount {
            parent_id: hex::decode(&parent_id.account_id)?,
            issuance: true,
            frozen: false,
            instrument: None,
        };
        let id = self
            .submit_transaction(request, vec![], SignerLevel::CentralBank)
            .await?
            .account_created;
        let encoded_id = hex::encode(&id);
        let owner = public_key
            .map(|k| base64::decode(k))
            .unwrap_or(Ok(self.signer.public_key(SignerLevel::Bank).to_vec()))?;

        let account = sdk::Account {
            id,
            owner: owner.clone(),
            name: name.clone(),
            public_name: name,
            profile_image_url: String::new(),
        };

        self.submit_transaction(
            sdk::Operation::insert(account),
            vec![],
            SignerLevel::CentralBank,
        )
        .await?;

        // Add new Bank Key to Nostro Role Binding
        self.update_role_binding(
            self.config.sandbox_nostro_role,
            owner,
            SignerLevel::CentralBank,
        )
        .await?;

        Ok(encoded_id)
    }

    pub(crate) async fn transfer(
        &self,
        parent_account: String,
        account: String,
        amount: u64,
        reference: String,
        signer: SignerLevel,
    ) -> anyhow::Result<u64> {
        let parent_account = hex::decode(parent_account)?;
        let account = hex::decode(account)?;
        if parent_account.len() != 16 || account.len() != 16 {
            return Err(anyhow::anyhow!("Invalid account id"));
        }
        let metadata = vec![m10_sdk::memo(&reference)];
        let transfer = sdk::CreateTransfer {
            transfer_steps: vec![sdk::TransferStep {
                from_account_id: parent_account,
                to_account_id: account,
                amount,
                metadata,
            }],
        };
        let transfer = Data::Transfer(transfer);

        let response = self.submit_transaction(transfer, vec![], signer).await?;
        Ok(response.tx_id)
    }

    pub(crate) async fn get_account(&self, id: String) -> anyhow::Result<Account> {
        let id = hex::decode(&id)?;
        let signer = if id.ends_with(&[0]) {
            SignerLevel::CentralBank
        } else {
            SignerLevel::Bank
        };
        let mut ledger = self.ledger.clone();
        let request = self
            .signer
            .sign_request(sdk::GetAccountRequest { id }, signer)
            .await?;
        let index_account = ledger.get_indexed_account(request.clone()).await?;
        let account = ledger.get_account(request).await?;
        Ok((index_account, account).try_into()?)
    }

    async fn submit_transaction(
        &self,
        data: impl Into<Data>,
        context_id: Vec<u8>,
        level: SignerLevel,
    ) -> anyhow::Result<sdk::TransactionResponse> {
        let payload = LedgerClient::transaction_request(data, context_id);
        let signed_request = self.signer.sign_request(payload, level).await?;
        let mut ledger = self.ledger.clone();
        Ok(ledger
            .create_transaction(signed_request)
            .await?
            .tx_error()?)
    }

    async fn update_role_binding(
        &self,
        id: Uuid,
        subject: Vec<u8>,
        level: SignerLevel,
    ) -> anyhow::Result<()> {
        let request = self
            .signer
            .sign_request(
                sdk::GetRoleBindingRequest {
                    id: id.as_bytes().to_vec(),
                },
                level,
            )
            .await?;
        let mut ledger = self.ledger.clone();
        let doc = ledger.get_role_binding(request).await?;
        if !doc
            .subjects
            .iter()
            .any(|s| s.as_ref() == subject.as_slice())
        {
            let mut builder = DocumentUpdate::<sdk::RoleBinding>::new(id);
            builder.subject(subject);
            self.submit_transaction(builder.operation(), vec![], level)
                .await?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub(crate) enum SignerLevel {
    Operator,
    CentralBank,
    Bank,
}

#[derive(Clone)]
struct ProxySigner {
    operator: Arc<Ed25519>,
    central_bank: Arc<Ed25519>,
    banks: Arc<Ed25519>,
}

impl ProxySigner {
    fn new(config: &Config) -> Self {
        Self {
            operator: config.operator_key.clone(),
            central_bank: config.central_bank_key.clone(),
            banks: config.banks_key.clone(),
        }
    }

    fn public_key(&self, level: SignerLevel) -> &[u8] {
        match level {
            SignerLevel::Operator => self.operator.public_key(),
            SignerLevel::CentralBank => self.central_bank.public_key(),
            SignerLevel::Bank => self.banks.public_key(),
        }
    }

    async fn sign_request<P: Message>(
        &self,
        data: P,
        level: SignerLevel,
    ) -> Result<SignedRequest<P>, SigningError> {
        match level {
            SignerLevel::Operator => self.operator.sign_request(data).await,
            SignerLevel::CentralBank => self.central_bank.sign_request(data).await,
            SignerLevel::Bank => self.banks.sign_request(data).await,
        }
    }
}
