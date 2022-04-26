use serde_json::Value;
use uuid::Uuid;

use crate::{error::Error, models::ContactType};

#[async_trait::async_trait]
pub(crate) trait Bank {
    type Contact;
    type Account;

    async fn create_account(&self, display_name: &str) -> Result<Value, Error>;
    async fn create_contact(
        &self,
        account_ref: &Value,
        contact_type: ContactType,
        data: Value,
    ) -> Result<Value, Error>;
    async fn update_contact(
        &self,
        contact_ref: &Value,
        data: Value,
    ) -> Result<Self::Contact, Error>;
    async fn retire_contact(&self, contact_ref: &Value) -> Result<Self::Contact, Error>;
    async fn approve_contact(&self, contact_ref: &Value) -> Result<Self::Contact, Error>;
    async fn freeze_contact(&self, contact_ref: &Value) -> Result<Self::Contact, Error>;
    async fn unfreeze_contact(&self, contact_ref: &Value) -> Result<Self::Contact, Error>;
    async fn deny_contact(&self, contact_ref: &Value) -> Result<Self::Contact, Error>;
    async fn get_account(&self, account_ref: &Value) -> Result<Self::Account, Error>;
    async fn get_contact(&self, contact_ref: &Value) -> Result<Self::Contact, Error>;
    async fn open_account(&self, account_ref: &Value) -> Result<Self::Account, Error>;
    async fn close_account(&self, account_ref: &Value) -> Result<Self::Account, Error>;
    async fn deposit(&mut self, amount: u64, contact_ref: &Value) -> Result<Uuid, Error>;
    async fn withdraw(&mut self, amount: u64, contact_ref: &Value) -> Result<Uuid, Error>;
    async fn account_deposit(&mut self, amount: u64, account_ref: &Value) -> Result<Uuid, Error>;
    async fn account_withdraw(&mut self, amount: u64, account_ref: &Value) -> Result<Uuid, Error>;
    async fn transfer(&self) -> Result<Value, Error>;
    async fn fund(&mut self, amount: u64, contact_ref: &Value) -> Result<Uuid, Error>;
    async fn fund_account(&mut self, amount: u64, account_ref: &Value) -> Result<Uuid, Error>;
    async fn settle_deposit(&mut self, txn_id: Uuid) -> Result<u64, Error>;
    async fn settle_withdraw(&mut self, txn_id: Uuid) -> Result<u64, Error>;
    async fn reverse_deposit(&mut self, txn_id: Uuid) -> Result<u64, Error>;
    async fn reverse_withdraw(&mut self, txn_id: Uuid) -> Result<u64, Error>;
    async fn create_transfer_method(&self) -> Result<Value, Error>;
    async fn get_transfer_method(&self) -> Result<Value, Error>;
    async fn deactivate_transfer_method(&self) -> Result<Value, Error>;
}
