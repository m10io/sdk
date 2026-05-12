use anyhow::Result;
use serde::Deserialize;

#[derive(Clone)]
pub struct BlockExplorerClient {
    address: String,
    block_explorer_client: reqwest::Client,
}

#[derive(Debug, Deserialize)]
pub struct AuditLog {
    pub tx_id: String,
    pub action: String,
    pub entity_type: String,
    pub status: u8,
    pub public_key: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize)]
struct AuditLogResponse {
    data: AuditLog,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxnResponse {
    pub data: Vec<TxnData>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxnData {
    pub attributes: TxnAttributes,
}

#[derive(Debug, Deserialize)]
pub struct TxnAttributes {
    pub signer_public_key: String,
}

// TODO add query params
impl BlockExplorerClient {
    pub fn new(address: impl Into<String>) -> Self {
        let address = address.into();
        let block_explorer_client = reqwest::Client::new();
        Self {
            address,
            block_explorer_client,
        }
    }

    /// Gets a transaction by its id from the block explorer (v2)
    pub async fn get_transaction_v2(&self, id: &str) -> Result<TxnResponse> {
        let base_url = reqwest::Url::parse(&self.address)?;
        let url = base_url.join(&format!(
            "/block-explorer/api/v2/transactions?filter[id]={}",
            id
        ))?;
        let resp = self
            .block_explorer_client
            .get(url)
            .send()
            .await?
            .error_for_status()?;
        let txn: TxnResponse = resp.json().await?;
        Ok(txn)
    }

    pub async fn get_block(&mut self, block_id: &str) -> reqwest::Result<String> {
        let url = format!("{}block-explorer/api/v1/blocks/{}", self.address, block_id);
        let block = self
            .block_explorer_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        Ok(block)
    }

    pub async fn get_account_roles(&mut self, account_role_id: &str) -> reqwest::Result<String> {
        let url = format!(
            "{}block-explorer/api/v1/account_roles/{}",
            self.address, account_role_id
        );
        let account_roles = self
            .block_explorer_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        Ok(account_roles)
    }

    pub async fn get_account_stats(&mut self, account_id: &str) -> reqwest::Result<String> {
        let url = format!(
            "{}block-explorer/api/v1/account_stats/{}",
            self.address, account_id
        );
        let account_stats = self
            .block_explorer_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        Ok(account_stats)
    }

    pub async fn get_account(&mut self, account_id: &str) -> reqwest::Result<String> {
        let url = format!(
            "{}block-explorer/api/v1/accounts/{}",
            self.address, account_id
        );
        let account = self
            .block_explorer_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        Ok(account)
    }

    pub async fn get_assets(&mut self) -> reqwest::Result<String> {
        let url = format!("{}block-explorer/api/v1/assets", self.address);
        let assets = self
            .block_explorer_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        Ok(assets)
    }

    pub async fn get_audit_log(&mut self, tx_id: &str) -> reqwest::Result<AuditLog> {
        let url = format!("{}block-explorer/api/v1/audit_log/{}", self.address, tx_id);
        let response = self
            .block_explorer_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json::<AuditLogResponse>()
            .await?;
        Ok(response.data)
    }

    pub async fn get_blocks(&mut self) -> reqwest::Result<String> {
        let url = format!("{}block-explorer/api/v1/blocks", self.address);
        let blocks = self
            .block_explorer_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        Ok(blocks)
    }

    pub async fn get_existing_assets(&mut self) -> reqwest::Result<String> {
        let url = format!("{}block-explorer/api/v1/existing_assets", self.address);
        let existing_assets = self
            .block_explorer_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        Ok(existing_assets)
    }

    pub async fn get_health(&mut self) -> reqwest::Result<String> {
        let url = format!("{}block-explorer/api/v1/health", self.address);
        let health = self
            .block_explorer_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        Ok(health)
    }

    pub async fn get_histogram(&mut self) -> reqwest::Result<String> {
        let url = format!("{}block-explorer/api/v1/histogram", self.address);
        let histogram = self
            .block_explorer_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        Ok(histogram)
    }

    pub async fn get_metrics(&mut self) -> reqwest::Result<String> {
        let url = format!("{}block-explorer/api/v1/metrics", self.address);
        let metrics = self
            .block_explorer_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        Ok(metrics)
    }

    pub async fn get_transaction(&mut self, transaction_id: &str) -> reqwest::Result<String> {
        let url = format!(
            "{}block-explorer/api/v1/transactions/{}",
            self.address, transaction_id
        );
        let transaction = self
            .block_explorer_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        Ok(transaction)
    }

    pub async fn get_transfer(&mut self, transfer_id: &str) -> reqwest::Result<String> {
        let url = format!(
            "{}block-explorer/api/v1/transfers/{}",
            self.address, transfer_id
        );
        let transfer = self
            .block_explorer_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        Ok(transfer)
    }
}
