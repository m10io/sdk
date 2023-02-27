#![allow(dead_code)]
use crate::context::Context;
use crate::{
    error::{Error, ResultExt},
    utils::submit_transaction,
};

use m10_sdk::{
    prost::bytes::Bytes,
    sdk::{self, rule::Verb, Operation, Role, RoleBinding, Rule, Value},
    Collection, DocumentUpdate, Signer,
};
use uuid::Uuid;

const LEDGER_ACCOUNT_COLLECTION: &str = "ledger-accounts";

pub(crate) async fn is_key_known(
    role_id: Uuid,
    key: &[u8],
    context: &Context,
) -> Result<bool, Error> {
    let request = context
        .signer
        .sign_request(sdk::GetRoleBindingRequest {
            id: role_id.as_bytes().to_vec(),
        })
        .await?;
    let mut client = context.ledger.clone();
    let role_binding = client
        .get_role_binding(request)
        .await
        .internal_error("getting role_binding")?;
    Ok(role_binding.subjects.contains(&Bytes::from(key.to_vec())))
}

pub(crate) async fn add_key(role_id: Uuid, key: &[u8], context: &Context) -> Result<(), Error> {
    let mut builder = DocumentUpdate::<RoleBinding>::new(role_id);
    builder.subject(key.to_vec());
    submit_transaction(builder.operation(), vec![], context).await?;
    Ok(())
}

pub(crate) async fn create_contact_rbac_role(
    contact_name: &str,
    account_set_id: Uuid,
    ledger_accounts: Vec<Vec<u8>>,
    context: &Context,
) -> Result<Uuid, Error> {
    let mut rules = Vec::default();
    let role_uuid = Uuid::new_v4();
    let role_id: Bytes = role_uuid.as_bytes().to_vec().into();

    // Create Role
    rules.push(Rule {
        collection: Collection::AccountSets.to_string(),
        instance_keys: vec![Bytes::copy_from_slice(account_set_id.as_bytes()).into()],
        verbs: vec![Verb::Read as i32, Verb::Update as i32, Verb::Delete as i32],
    });
    if !ledger_accounts.is_empty() {
        let ledger_accounts: Vec<Value> = ledger_accounts
            .iter()
            .map(|account_id| Bytes::copy_from_slice(account_id).into())
            .collect();
        rules.push(Rule {
            collection: Collection::AccountMetadata.to_string(),
            instance_keys: ledger_accounts.clone(),
            verbs: vec![Verb::Read as i32],
        });
        rules.push(Rule {
            collection: LEDGER_ACCOUNT_COLLECTION.to_string(),
            instance_keys: ledger_accounts,
            verbs: vec![Verb::Initiate as i32, Verb::Transact as i32],
        });
    }
    let role = Role {
        id: role_id.clone(),
        name: format!("owner-{}", contact_name),
        owner: Bytes::copy_from_slice(context.signer.public_key()),
        rules,
    };

    // Create RoleBinding
    let role_binding = RoleBinding {
        id: role_id.clone(),
        name: format!("owner-{}", contact_name),
        owner: Bytes::copy_from_slice(context.signer.public_key()),
        role: role_id.clone(),
        ..Default::default()
    };

    // Submit request
    let ops = vec![Operation::insert(role), Operation::insert(role_binding)];
    submit_transaction(ops, vec![], context)
        .await
        .internal_error("adding customer RBAC")?;
    Ok(role_uuid)
}

pub(crate) async fn add_accounts_to_role(
    role_id: Uuid,
    ledger_accounts: Vec<Value>,
    context: &Context,
) -> Result<(), Error> {
    let request = context
        .signer
        .sign_request(sdk::GetRoleRequest {
            id: role_id.as_bytes().to_vec(),
        })
        .await?;
    let mut client = context.ledger.clone();
    let role = client
        .get_role(request)
        .await
        .internal_error("getting customer role")?;
    let mut account_rule = role
        .rules
        .iter()
        .find(|r| r.collection == Collection::AccountMetadata.to_string())
        .unwrap_or(&Rule {
            collection: Collection::AccountMetadata.to_string(),
            instance_keys: vec![],
            verbs: vec![Verb::Read as i32],
        })
        .clone();
    let mut ledger_account_rule = role
        .rules
        .iter()
        .find(|r| r.collection == *LEDGER_ACCOUNT_COLLECTION.to_string())
        .unwrap_or(&Rule {
            collection: LEDGER_ACCOUNT_COLLECTION.to_string(),
            instance_keys: vec![],
            verbs: vec![Verb::Initiate as i32, Verb::Transact as i32],
        })
        .clone();
    for ledger_account_id in ledger_accounts {
        account_rule.instance_keys.push(ledger_account_id.clone());
        ledger_account_rule.instance_keys.push(ledger_account_id);
    }
    let mut builder = DocumentUpdate::<Role>::new(role_id);
    builder.rule(account_rule);
    builder.rule(ledger_account_rule);
    submit_transaction(builder.operation(), vec![], context)
        .await
        .internal_error("updating customer role")?;
    Ok(())
}
