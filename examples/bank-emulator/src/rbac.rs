#![allow(dead_code)]
use crate::context::Context;
use crate::{
    error::{Error, ResultExt},
    utils::submit_transaction,
};

use m10_sdk::{
    prost::bytes::Bytes,
    sdk::{rule::Verb, Operation, Role, RoleBinding, Rule, Value},
    Collection, DocumentUpdate, Signer,
};
use m10_sdk::{sdk, NameFilter, PageBuilder, PublicKey};
use tracing::debug;
use uuid::Uuid;

const LEDGER_ACCOUNT_COLLECTION: &str = "ledger-accounts";

pub fn get_requiem_service_name(bank_name: String) -> String {
    format!(
        "{}-{}",
        bank_name.replace(' ', "-").to_lowercase(),
        "requiem-service"
    )
}

pub(crate) async fn is_key_known(
    role_id: Uuid,
    key: &[u8],
    context: &Context,
) -> Result<bool, Error> {
    let role_binding = m10_sdk::get_role_binding(&context.ledger, role_id)
        .await
        .internal_error("getting role_binding")?;
    Ok(role_binding.subjects.contains(&Bytes::from(key.to_vec())))
}

pub(crate) async fn add_key(role_id: Uuid, key: &[u8], context: &Context) -> Result<(), Error> {
    let role_binding = m10_sdk::get_role_binding(&context.ledger, role_id)
        .await
        .internal_error("getting role_binding")?;

    if role_binding.subjects.contains(&Bytes::from(key.to_vec())) {
        return Err(Error::validation("public_key", "key already exists"));
    }

    // Basic update operation to add a key to a role binding doesn't replace the existing subjects
    // with the new one. Instead, it appends the new key to the existing list of subjects.
    // To set new subjects, we need to delete the existing role binding and create a new one.
    let delete_op = sdk::Operation::delete::<RoleBinding>(role_id.into());

    let role_binding = RoleBinding {
        id: role_id.as_bytes().to_vec().into(),
        name: role_binding.name,
        owner: role_binding.owner.to_vec().into(),
        role: role_binding.role_id.to_vec().into(),
        subjects: vec![Bytes::copy_from_slice(key)],
        ..Default::default()
    };

    let ops = vec![delete_op, sdk::Operation::insert(role_binding)];

    submit_transaction(ops, vec![], context)
        .await
        .internal_error("failed to change role binding key")?;

    Ok(())
}

pub(crate) async fn find_role_by_name(
    name: &str,
    context: &Context,
) -> Result<Option<Role>, Error> {
    let request = PageBuilder::<_, NameFilter>::name(name);
    let roles = context.ledger.clone().list_roles(request).await?;
    let role = roles.into_iter().next();

    Ok(role.map(|r| Role {
        id: Bytes::copy_from_slice(&r.id.to_vec()),
        name: r.name,
        owner: Bytes::copy_from_slice(&r.owner.to_vec()),
        rules: r.rules,
    }))
}

pub(crate) async fn create_contact_rbac_role(
    contact_name: &str,
    account_set_id: Uuid,
    ledger_accounts: Vec<Vec<u8>>,
    keys: Vec<PublicKey>,
    context: &Context,
) -> Result<Uuid, Error> {
    let mut rules = Vec::default();
    let role_uuid = Uuid::new_v4();
    let role_id: Bytes = role_uuid.as_bytes().to_vec().into();

    debug!("Creating RBAC for {}", contact_name);

    // Create Role
    rules.push(Rule {
        collection: Collection::AccountSets.to_string(),
        instance_keys: vec![Bytes::copy_from_slice(account_set_id.as_bytes()).into()],
        excluded_instance_keys: vec![],
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
            excluded_instance_keys: vec![],
            verbs: vec![Verb::Read as i32],
        });
        rules.push(Rule {
            collection: LEDGER_ACCOUNT_COLLECTION.to_string(),
            instance_keys: ledger_accounts,
            excluded_instance_keys: vec![],
            verbs: vec![Verb::Initiate as i32, Verb::Transact as i32],
        });
    }
    let role = Role {
        id: role_id.clone(),
        name: format!("owner-{}", contact_name),
        owner: Bytes::copy_from_slice(context.ledger.signer()?.public_key()),
        rules,
    };

    debug!("Role created with id: {}", role_uuid.clone());

    // Create RoleBinding
    let role_binding = RoleBinding {
        id: role_id.clone(),
        name: format!("owner-{}", contact_name),
        owner: Bytes::copy_from_slice(context.ledger.signer()?.public_key()),
        role: role_id.clone(),
        subjects: keys
            .into_iter()
            .map(|key| Bytes::copy_from_slice(key.to_vec().as_slice()))
            .collect(),
        ..Default::default()
    };

    let ops = vec![Operation::insert(role), Operation::insert(role_binding)];

    submit_transaction(ops, vec![], context)
        .await
        .internal_error("adding customer RBAC")?;
    Ok(role_uuid)
}

pub(crate) async fn create_requiem_rbac_role(
    public_key: PublicKey,
    context: &Context,
) -> Result<(), Error> {
    let role_uuid = Uuid::new_v4();
    let role_id: Bytes = role_uuid.as_bytes().to_vec().into();

    let rules = vec![
        Rule {
            collection: Collection::Roles.to_string(),
            verbs: vec![Verb::Read as i32],
            ..Default::default()
        },
        Rule {
            collection: Collection::RoleBindings.to_string(),
            verbs: vec![Verb::Read as i32],
            ..Default::default()
        },
    ];

    let name = get_requiem_service_name(context.config.bank_name.clone());
    let owner = Bytes::copy_from_slice(context.ledger.signer()?.public_key());

    let role = Role {
        id: role_id.clone(),
        name: name.clone(),
        owner: owner.clone(),
        rules,
    };

    let role_binding = RoleBinding {
        id: role_id.clone(),
        name,
        owner,
        role: role_id.clone(),
        subjects: vec![Bytes::copy_from_slice(public_key.to_vec().as_slice())],
        ..Default::default()
    };

    let ops = vec![Operation::insert(role), Operation::insert(role_binding)];
    submit_transaction(ops, vec![], context)
        .await
        .internal_error("adding recovery service RBAC")?;
    Ok(())
}

pub(crate) async fn add_accounts_to_role(
    role_id: Uuid,
    ledger_accounts: Vec<Value>,
    context: &Context,
) -> Result<(), Error> {
    let role = m10_sdk::get_role(&context.ledger, role_id)
        .await
        .internal_error("getting customer role")?;
    let mut account_rule = role
        .rules
        .iter()
        .find(|r| r.collection == Collection::AccountMetadata.to_string())
        .unwrap_or(&Rule {
            collection: Collection::AccountMetadata.to_string(),
            instance_keys: vec![],
            excluded_instance_keys: vec![],
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
            excluded_instance_keys: vec![],
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
