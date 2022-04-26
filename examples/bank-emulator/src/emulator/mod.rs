use m10_rds_pool::{bb8, RdsManager};
use serde_json::Value;
use sqlx::Connection;
use uuid::Uuid;

use crate::{bank::Bank, config::BankEmulatorConfig, error::Error, models::ContactType};

use self::model::{bank_accounts::*, bank_contacts::*};

mod model;

#[derive(Clone)]
pub(crate) struct BankEmulator {
    checking_account_range: i32,
    holding_account: BankAccount,
    currency: String,
    db_pool: bb8::Pool<RdsManager>,
}

impl BankEmulator {
    pub(crate) async fn new_from_config(
        config: &BankEmulatorConfig,
        db_pool: bb8::Pool<RdsManager>,
    ) -> Result<Self, Error> {
        let currency_conf = config
            .currencies
            .first()
            .ok_or_else(|| Error::not_found("Bank holding account"))?;
        let holding_account = {
            let mut conn = db_pool.get().await?;
            BankAccount::find_by_name(&currency_conf.account_name)
                .fetch_optional(&mut *conn)
                .await?
                .ok_or_else(|| Error::not_found("Bank holding account"))?
        };
        Ok(Self {
            checking_account_range: config.checking_account_start,
            holding_account,
            currency: currency_conf.currency.clone(),
            db_pool,
        })
    }
}

#[async_trait::async_trait]
impl Bank for BankEmulator {
    type Contact = BankContact;
    type Account = BankAccount;

    async fn create_account(&self, display_name: &str) -> Result<Value, Error> {
        let conn = self.db_pool.get().await?;
        let account = BankAccount::new(
            self.checking_account_range,
            display_name,
            &self.currency,
            conn,
        )
        .await?;
        Ok(account.into())
    }

    async fn create_contact(
        &self,
        account_ref: &Value,
        contact_type: ContactType,
        data: Value,
    ) -> Result<Value, Error> {
        let account_id = BankAccount::try_id_from(account_ref)?;
        let mut conn = self.db_pool.get().await?;
        let mut txn = conn.begin().await?;
        let contact = BankContact::new(account_id, contact_type.into(), data, &mut txn).await?;
        txn.commit().await?;
        Ok(contact.into())
    }

    async fn update_contact(
        &self,
        contact_ref: &Value,
        data: Value,
    ) -> Result<Self::Contact, Error> {
        let id = BankContact::try_id_from(contact_ref)?;
        let mut conn = self.db_pool.get().await?;
        let mut txn = conn.begin().await?;
        let mut contact = BankContact::find_by_id(id)
            .fetch_optional(&mut txn)
            .await?
            .ok_or_else(|| Error::not_found("Bank contact"))?;
        contact.data = Some(data);
        contact.update_data(&mut txn).await?;
        txn.commit().await?;
        Ok(contact)
    }

    async fn retire_contact(&self, contact_ref: &Value) -> Result<Self::Contact, Error> {
        let id = BankContact::try_id_from(contact_ref)?;
        let mut conn = self.db_pool.get().await?;
        let mut txn = conn.begin().await?;
        let mut contact = BankContact::find_by_id(id)
            .fetch_optional(&mut txn)
            .await?
            .ok_or_else(|| Error::not_found("Bank contact"))?;
        contact.contact_status = BankContactStatus::Retired;
        contact.update_status(&mut txn).await?;
        txn.commit().await?;
        Ok(contact)
    }

    async fn approve_contact(&self, contact_ref: &Value) -> Result<Self::Contact, Error> {
        let id = BankContact::try_id_from(contact_ref)?;
        let mut conn = self.db_pool.get().await?;
        let mut txn = conn.begin().await?;
        let mut contact = BankContact::find_by_id(id)
            .fetch_optional(&mut txn)
            .await?
            .ok_or_else(|| Error::not_found("Bank contact"))?;
        contact.contact_status = BankContactStatus::Approved;
        contact.update_status(&mut txn).await?;
        txn.commit().await?;
        Ok(contact)
    }

    async fn freeze_contact(&self, contact_ref: &Value) -> Result<Self::Contact, Error> {
        let id = BankContact::try_id_from(contact_ref)?;
        let mut conn = self.db_pool.get().await?;
        let mut txn = conn.begin().await?;
        let mut contact = BankContact::find_by_id(id)
            .fetch_optional(&mut txn)
            .await?
            .ok_or_else(|| Error::not_found("Bank contact"))?;
        contact.contact_status = BankContactStatus::Frozen;
        contact.update_status(&mut txn).await?;
        txn.commit().await?;
        Ok(contact)
    }

    async fn unfreeze_contact(&self, contact_ref: &Value) -> Result<Self::Contact, Error> {
        let id = BankContact::try_id_from(contact_ref)?;
        let mut conn = self.db_pool.get().await?;
        let mut txn = conn.begin().await?;
        let mut contact = BankContact::find_by_id(id)
            .fetch_optional(&mut txn)
            .await?
            .ok_or_else(|| Error::not_found("Bank contact"))?;
        contact.contact_status = BankContactStatus::Approved;
        contact.update_status(&mut txn).await?;
        txn.commit().await?;
        Ok(contact)
    }

    async fn deny_contact(&self, contact_ref: &Value) -> Result<Self::Contact, Error> {
        let id = BankContact::try_id_from(contact_ref)?;
        let mut conn = self.db_pool.get().await?;
        let mut txn = conn.begin().await?;
        let mut contact = BankContact::find_by_id(id)
            .fetch_optional(&mut txn)
            .await?
            .ok_or_else(|| Error::not_found("Bank contact"))?;
        contact.contact_status = BankContactStatus::Denied;
        contact.update_status(&mut txn).await?;
        txn.commit().await?;
        Ok(contact)
    }

    async fn get_contact(&self, contact_ref: &Value) -> Result<Self::Contact, Error> {
        let id = BankContact::try_id_from(contact_ref)?;
        let mut conn = self.db_pool.get().await?;
        let contact = BankContact::find_by_id(id)
            .fetch_optional(&mut *conn)
            .await?
            .ok_or_else(|| Error::not_found("Bank contact"))?;
        Ok(contact)
    }

    async fn get_account(&self, account_ref: &Value) -> Result<Self::Account, Error> {
        let id = BankAccount::try_id_from(account_ref)?;
        let mut conn = self.db_pool.get().await?;
        let account = BankAccount::find_by_id(id)
            .fetch_optional(&mut *conn)
            .await?
            .ok_or_else(|| Error::not_found("Bank account"))?;
        Ok(account)
    }

    async fn open_account(&self, account_ref: &Value) -> Result<Self::Account, Error> {
        let id = BankAccount::try_id_from(account_ref)?;
        let mut conn = self.db_pool.get().await?;
        let mut txn = conn.begin().await?;
        let mut account = BankAccount::find_by_id(id)
            .fetch_optional(&mut txn)
            .await?
            .ok_or_else(|| Error::not_found("Bank account"))?;
        account.account_status = BankAccountStatus::Open;
        account.update_status(&mut txn).await?;
        txn.commit().await?;
        Ok(account)
    }

    async fn close_account(&self, account_ref: &Value) -> Result<Self::Account, Error> {
        let id = BankAccount::try_id_from(account_ref)?;
        let mut conn = self.db_pool.get().await?;
        let mut txn = conn.begin().await?;
        let mut account = BankAccount::find_by_id(id)
            .fetch_optional(&mut txn)
            .await?
            .ok_or_else(|| Error::not_found("Bank account"))?;
        account.account_status = BankAccountStatus::PendingClosure;
        account.update_status(&mut txn).await?;
        txn.commit().await?;
        Ok(account)
    }

    async fn deposit(&mut self, amount: u64, contact_ref: &Value) -> Result<Uuid, Error> {
        let contact_id = BankContact::try_id_from(contact_ref)?;
        let mut conn = self.db_pool.get().await?;
        let mut txn = conn.begin().await?;
        let txn_id = self
            .holding_account
            .deposit_for_contact(contact_id, amount.try_into()?, &mut txn)
            .await?;
        self.holding_account.refresh(&mut txn).await?;
        txn.commit().await?;
        Ok(txn_id)
    }

    async fn withdraw(&mut self, amount: u64, contact_ref: &Value) -> Result<Uuid, Error> {
        let contact_id = BankContact::try_id_from(contact_ref)?;
        let mut conn = self.db_pool.get().await?;
        let mut txn = conn.begin().await?;
        let txn_id = self
            .holding_account
            .withdraw_for_contact(contact_id, amount.try_into()?, &mut txn)
            .await?;
        self.holding_account.refresh(&mut txn).await?;
        txn.commit().await?;
        Ok(txn_id)
    }

    async fn account_deposit(&mut self, amount: u64, account_ref: &Value) -> Result<Uuid, Error> {
        let account_id = BankAccount::try_id_from(account_ref)?;
        let mut conn = self.db_pool.get().await?;
        let mut txn = conn.begin().await?;
        let txn_id = self
            .holding_account
            .deposit_into(account_id, amount.try_into()?, None, &mut txn)
            .await?;
        self.holding_account.refresh(&mut txn).await?;
        txn.commit().await?;
        Ok(txn_id)
    }

    async fn account_withdraw(&mut self, amount: u64, account_ref: &Value) -> Result<Uuid, Error> {
        let account_id = BankAccount::try_id_from(account_ref)?;
        let mut conn = self.db_pool.get().await?;
        let mut txn = conn.begin().await?;
        let txn_id = self
            .holding_account
            .withdraw_from(account_id, amount.try_into()?, None, &mut txn)
            .await?;
        self.holding_account.refresh(&mut txn).await?;
        txn.commit().await?;
        Ok(txn_id)
    }

    async fn transfer(&self) -> Result<Value, Error> {
        unimplemented!()
    }

    async fn fund(&mut self, amount: u64, contact_ref: &Value) -> Result<Uuid, Error> {
        let contact_id = BankContact::try_id_from(contact_ref)?;
        let mut conn = self.db_pool.get().await?;
        let mut txn = conn.begin().await?;
        let txn_id = self
            .holding_account
            .deposit_for_contact(contact_id, amount.try_into()?, &mut txn)
            .await?;
        self.holding_account
            .settle_deposit(txn_id, &mut txn)
            .await?;
        self.holding_account.refresh(&mut txn).await?;
        txn.commit().await?;
        Ok(txn_id)
    }

    async fn fund_account(&mut self, amount: u64, account_ref: &Value) -> Result<Uuid, Error> {
        let account_id = BankAccount::try_id_from(account_ref)?;
        let mut conn = self.db_pool.get().await?;
        let mut txn = conn.begin().await?;
        let txn_id = self
            .holding_account
            .deposit_into(account_id, amount.try_into()?, None, &mut txn)
            .await?;
        self.holding_account
            .settle_deposit(txn_id, &mut txn)
            .await?;
        self.holding_account.refresh(&mut txn).await?;
        txn.commit().await?;
        Ok(txn_id)
    }

    async fn settle_deposit(&mut self, txn_id: Uuid) -> Result<u64, Error> {
        let mut conn = self.db_pool.get().await?;
        let mut txn = conn.begin().await?;
        let amount = self
            .holding_account
            .settle_deposit(txn_id, &mut txn)
            .await?;
        self.holding_account.refresh(&mut txn).await?;
        txn.commit().await?;
        Ok(amount.try_into()?)
    }

    async fn settle_withdraw(&mut self, txn_id: Uuid) -> Result<u64, Error> {
        let mut conn = self.db_pool.get().await?;
        let mut txn = conn.begin().await?;
        let amount = self
            .holding_account
            .settle_withdraw(txn_id, &mut txn)
            .await?;
        self.holding_account.refresh(&mut txn).await?;
        txn.commit().await?;
        Ok(amount.try_into()?)
    }

    async fn reverse_deposit(&mut self, txn_id: Uuid) -> Result<u64, Error> {
        let mut conn = self.db_pool.get().await?;
        let mut txn = conn.begin().await?;
        let amount = self
            .holding_account
            .reverse_deposit(txn_id, &mut txn)
            .await?;
        self.holding_account.refresh(&mut txn).await?;
        txn.commit().await?;
        Ok(amount.try_into()?)
    }

    async fn reverse_withdraw(&mut self, txn_id: Uuid) -> Result<u64, Error> {
        let mut conn = self.db_pool.get().await?;
        let mut txn = conn.begin().await?;
        let amount = self
            .holding_account
            .reverse_withdraw(txn_id, &mut txn)
            .await?;
        self.holding_account.refresh(&mut txn).await?;
        txn.commit().await?;
        Ok(amount.try_into()?)
    }

    async fn create_transfer_method(&self) -> Result<Value, Error> {
        unimplemented!()
    }
    async fn get_transfer_method(&self) -> Result<Value, Error> {
        unimplemented!()
    }
    async fn deactivate_transfer_method(&self) -> Result<Value, Error> {
        unimplemented!()
    }
}
