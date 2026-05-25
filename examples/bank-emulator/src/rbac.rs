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
use m10_sdk::{sdk, PageBuilder, PublicKey, RoleFilter};
use tokio::time::{sleep, Duration};
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
    let role_binding = get_role_binding_retry(role_id, context).await?;
    Ok(role_binding.subjects.contains(&Bytes::from(key.to_vec())))
}

pub(crate) async fn add_key(role_id: Uuid, key: &[u8], context: &Context) -> Result<(), Error> {
    let role_binding = get_role_binding_retry(role_id, context).await?;

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
        expressions: vec![],
        is_universal: false,
        created_at: 0,
        updated_at: 0,
        created_by: role_binding.created_by.to_vec().into(),
        description: role_binding.description,
        labels: role_binding.labels,
        expires_at: role_binding.expires_at,
        attributes: vec![],
    };

    let ops = vec![delete_op, sdk::Operation::insert(role_binding)];

    submit_transaction(ops, vec![], context)
        .await
        .internal_error("failed to change role binding key")?;

    Ok(())
}

async fn get_role_binding_retry(
    role_id: Uuid,
    context: &Context,
) -> Result<m10_sdk::RoleBinding, Error> {
    const RETRIES: usize = 10;

    let mut attempts = 0usize;
    loop {
        match m10_sdk::get_role_binding(&context.ledger, role_id).await {
            Ok(role_binding) => return Ok(role_binding),
            Err(err) => {
                attempts += 1;
                if attempts > RETRIES {
                    return Err(err).internal_error("getting role_binding");
                }
                sleep(Duration::from_millis((attempts * 200) as u64)).await;
            }
        }
    }
}

pub(crate) async fn find_role_by_name(
    name: &str,
    context: &Context,
) -> Result<Option<Role>, Error> {
    let request = PageBuilder::<_, RoleFilter>::name(name);
    let roles = context.ledger.clone().list_roles(request).await?;
    let role = roles.into_iter().next();

    Ok(role.map(|r| Role {
        id: Bytes::copy_from_slice(&r.id.to_vec()),
        name: r.name,
        owner: Bytes::copy_from_slice(&r.owner.to_vec()),
        rules: r.rules,
        created_at: r.created_at,
        updated_at: r.updated_at,
        created_by: Bytes::copy_from_slice(&r.created_by.to_vec()),
        description: r.description,
        labels: r.labels,
        immutable: r.immutable,
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
        verbs: vec![Verb::Read as i32, Verb::Update as i32, Verb::Delete as i32],
        when: None,
        types: std::collections::HashMap::new(),
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
            when: None,
            types: std::collections::HashMap::new(),
        });
        rules.push(Rule {
            collection: LEDGER_ACCOUNT_COLLECTION.to_string(),
            instance_keys: ledger_accounts,
            verbs: vec![Verb::Initiate as i32, Verb::Transact as i32],
            when: None,
            types: std::collections::HashMap::new(),
        });
    }
    let role = Role {
        id: role_id.clone(),
        name: format!("owner-{}", contact_name),
        owner: Bytes::copy_from_slice(context.ledger.signer()?.public_key()),
        rules,
        created_at: 0,
        updated_at: 0,
        created_by: Bytes::copy_from_slice(context.ledger.signer()?.public_key()),
        description: String::new(),
        labels: std::collections::HashMap::new(),
        immutable: false,
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
        expressions: vec![],
        is_universal: false,
        created_at: 0,
        updated_at: 0,
        created_by: Bytes::copy_from_slice(context.ledger.signer()?.public_key()),
        description: String::new(),
        labels: std::collections::HashMap::new(),
        expires_at: None,
        attributes: vec![],
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
        created_at: 0,
        updated_at: 0,
        created_by: owner.clone(),
        description: String::new(),
        labels: std::collections::HashMap::new(),
        immutable: false,
    };

    let role_binding = RoleBinding {
        id: role_id.clone(),
        name,
        owner: owner.clone(),
        role: role_id.clone(),
        subjects: vec![Bytes::copy_from_slice(public_key.to_vec().as_slice())],
        expressions: vec![],
        is_universal: false,
        created_at: 0,
        updated_at: 0,
        created_by: owner,
        description: String::new(),
        labels: std::collections::HashMap::new(),
        expires_at: None,
        attributes: vec![],
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
            verbs: vec![Verb::Read as i32],
            when: None,
            types: std::collections::HashMap::new(),
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
            when: None,
            types: std::collections::HashMap::new(),
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
