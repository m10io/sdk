use crate::collections::roles::{RuleArgs, Verb};
use crate::collections::{LEDGER_ACCOUNTS, ROLES, ROLE_BINDINGS};
use anyhow::bail;
use anyhow::Context;
use m10_sdk::account::{AccountId, Builder as AccountIdBuilder, LeafAccountIndex, RawAccountIndex};
use m10_sdk::sdk::rule::Ty;
use m10_sdk::{sdk::signature::Algorithm, Ed25519, Signer, P256};
use std::fs;
use std::io::Read;
use std::path::Path;
use std::{path::PathBuf, str::FromStr};

pub(crate) fn m10_config_path() -> PathBuf {
    dirs::home_dir()
        .expect("XDG_CONFIG_HOME")
        .join(".config/m10/")
}

pub(crate) fn account_id_builder_from_chain(
    chain: &[u64],
) -> Result<AccountIdBuilder, anyhow::Error> {
    let mut chain_iter = chain.iter();
    let root = chain_iter.next().unwrap_or(&0u64);
    let mut builder = AccountIdBuilder::from_root_account_index(*root as RawAccountIndex)?;
    for id in chain_iter {
        builder.push(*id as RawAccountIndex)?;
    }
    Ok(builder)
}

pub(crate) fn account_id_from_str(account_chain: &str) -> Result<AccountId, anyhow::Error> {
    let trimmed = account_chain.trim();
    if !(trimmed.starts_with('[') && trimmed.ends_with(']')) {
        return Err(anyhow::anyhow!(
            "invalid account chain format: expected '[<issuance_chain>;<leaf_id>]', e.g. '[1,2;3]', got '{}'",
            trimmed
        ));
    }
    let mut parts = trimmed[1..trimmed.len() - 1].split(';');
    if let Some(issuance) = parts.next() {
        let chain = issuance
            .split(',')
            .map(|i| Ok(u64::from_str(i.trim())?))
            .collect::<Result<Vec<u64>, anyhow::Error>>()?;
        let builder = account_id_builder_from_chain(&chain)?;
        if let Some(leaf) = parts.next() {
            let leaf_id = u64::from_str(leaf.trim())?;
            Ok(builder.leaf_id(LeafAccountIndex::new(leaf_id as RawAccountIndex)?)?)
        } else {
            Ok(builder.issuance_id())
        }
    } else {
        Err(anyhow::anyhow!(
            "invalid account chain format: expected '[<issuance_chain>;<leaf_id>]', e.g. '[1,2;3]', got '{}'",
            trimmed
        ))
    }
}

pub(crate) fn pprint_account_id(account_id: &AccountId) {
    let mut chain: Vec<u64> = vec![account_id.root_account_index().as_raw()];
    let mut indecies = account_id
        .issuance_account_indexes()
        .map(|i| i.as_raw())
        .collect();
    chain.append(&mut indecies);
    if account_id.is_issuance() {
        println!("{:?}", chain);
    } else {
        print!("[");
        for (n, i) in chain.iter().enumerate() {
            if n > 0 {
                print!(", ");
            }
            print!("{}", i);
        }
        println!("; {}]", account_id.leaf_account_index().unwrap().as_raw());
    }
}

pub(crate) fn vec_from_int_array(int_array_str: &str) -> Result<Vec<u8>, anyhow::Error> {
    let trimmed = int_array_str.trim();
    if !(trimmed.starts_with('[') && trimmed.ends_with(']')) {
        return Err(anyhow::anyhow!("not a valid int array: '{}'", trimmed));
    }
    let parts = trimmed[1..trimmed.len() - 1]
        .split(',')
        .map(|i| Ok(u8::from_str(i.trim())?))
        .collect::<Result<Vec<u8>, anyhow::Error>>()?;
    Ok(parts)
}

pub(crate) fn vec_from_hex_array(hex_array_str: &str) -> Result<Vec<u8>, anyhow::Error> {
    let trimmed = hex_array_str.trim();
    if !(trimmed.starts_with('[') && trimmed.ends_with(']')) {
        return Err(anyhow::anyhow!("not a valid hex array: '{}'", trimmed));
    }
    let parts = trimmed[1..trimmed.len() - 1]
        .split(',')
        .map(|i| Ok(u8::from_str_radix(&i.trim()[2..], 16)?))
        .collect::<Result<Vec<u8>, anyhow::Error>>()?;
    Ok(parts)
}

pub(crate) fn create_key_pair(key_file: &str, method: Algorithm) -> Result<Vec<u8>, anyhow::Error> {
    match method {
        Algorithm::Ed25519 => {
            let key = Ed25519::new_key_pair(Some(key_file))?;
            eprintln!("created key pair file: {:?}", key_file);
            Ok(key.public_key().to_vec())
        }
        Algorithm::P256Sha256Asn1 => {
            let key = P256::new_key_pair(Some(key_file))?;
            eprintln!("created key pair file: {:?}", key_file);
            Ok(key.public_key().to_vec())
        }
        Algorithm::Ed25519PhSha512 => {
            let key = Ed25519::new_key_pair_ph(Some(key_file))?;
            eprintln!("created key pair file: {:?}", key_file);
            Ok(key.public_key().to_vec())
        }
    }
}

pub(crate) fn create_exportable_key_pair(method: Algorithm) -> Result<Vec<u8>, anyhow::Error> {
    match method {
        Algorithm::Ed25519 | Algorithm::Ed25519PhSha512 => {
            let (kp, _) = Ed25519::new_key_pair_exportable()?;
            Ok(kp)
        }
        Algorithm::P256Sha256Asn1 => {
            let (kp, _) = P256::new_key_pair_exportable()?;
            Ok(kp)
        }
    }
}

pub fn secure_read_file<P: AsRef<Path>>(rel: P) -> anyhow::Result<Vec<u8>> {
    let cwd = std::env::current_dir()?;
    let full = cwd.join(rel);
    let canon = full
        .canonicalize()
        .with_context(|| format!("File not found: {}", full.display()))?;
    let meta = fs::symlink_metadata(&canon)
        .with_context(|| format!("Could not stat: {}", canon.display()))?;
    if !meta.is_file() {
        anyhow::bail!("Not a regular file: {}", canon.display());
    }
    if meta.file_type().is_symlink() {
        anyhow::bail!("Symlinks not allowed: {}", canon.display());
    }

    let mut f = fs::OpenOptions::new()
        .read(true)
        .open(&canon)
        .with_context(|| format!("Cannot open file: {}", canon.display()))?;

    let mut buf = Vec::new();
    f.read_to_end(&mut buf).context("Failed to read file")?;
    Ok(buf)
}

pub fn parse_key_value(s: &str) -> Result<(String, String), String> {
    let mut parts = s.splitn(2, '=');
    let key = parts.next().ok_or("missing key")?.to_string();
    let val = parts.next().ok_or("missing value")?.to_string();
    Ok((key, val))
}

pub fn validate_labels(
    labels: &std::collections::HashMap<String, String>,
) -> Result<(), anyhow::Error> {
    for (k, v) in labels {
        if k.len() > 100 {
            return Err(anyhow::anyhow!(
                "Label key {} is too long ({} > 100)",
                k,
                k.len(),
            ));
        }

        if v.len() > 100 {
            return Err(anyhow::anyhow!(
                "Label value {} is too long ({} > 100)",
                v,
                v.len(),
            ));
        }
    }

    Ok(())
}

static RESERVED_WHEN_KEYWORDS: [&str; 2] = ["now", "transfer"];

pub fn is_reserved_when_keyword(input: &str) -> bool {
    let normalized = input.trim().to_lowercase();
    RESERVED_WHEN_KEYWORDS.contains(&normalized.as_str())
}

fn validate_when_type_names(types: &[(String, Ty)]) -> anyhow::Result<()> {
    for (name, _) in types {
        if is_reserved_when_keyword(name) {
            bail!("Reserved keyword '{name}' cannot be used as a when-statement type name");
        }
    }
    Ok(())
}

pub fn validate_rules(rules: &[RuleArgs]) -> anyhow::Result<()> {
    let forbidden_verbs = [
        (Verb::Transact, "TRANSACT"),
        (Verb::Initiate, "INITIATE"),
        (Verb::Commit, "COMMIT"),
    ];
    if rules.is_empty() {
        return Err(anyhow::anyhow!("Rules cannot be empty"));
    }
    for rule in rules {
        if rule.collection.is_empty() {
            return Err(anyhow::anyhow!("Collection name cannot be empty"));
        }
        if rule.verbs.is_empty() {
            return Err(anyhow::anyhow!(
                "Verbs cannot be empty for collection '{}'",
                rule.collection
            ));
        }
        if let Some(types) = &rule.types {
            validate_when_type_names(types)?;
        }
        let has_deny = rule.verbs.contains(&Verb::Deny);
        let has_other_verb = rule.verbs.iter().any(|&v| v != Verb::Deny);
        if has_deny && !has_other_verb {
            return Err(anyhow::anyhow!(
                "DENY must be combined with at least one other verb for collection '{}'",
                rule.collection
            ));
        }
        if !rule.collection.eq_ignore_ascii_case(LEDGER_ACCOUNTS) {
            for (verb, name) in forbidden_verbs {
                if rule.verbs.contains(&verb) {
                    return Err(anyhow::anyhow!(
                        "{} can not be applied to collection '{}'",
                        name,
                        rule.collection
                    ));
                }
            }
        }
        if rule.collection.eq_ignore_ascii_case(ROLES)
            || rule.collection.eq_ignore_ascii_case(ROLE_BINDINGS)
        {
            let has_grant = rule.verbs.contains(&Verb::Grant);

            if has_deny && has_grant {
                return Err(anyhow::anyhow!(
                    "DENY can not be combined with GRANT verb for collection '{}'",
                    rule.collection
                ));
            }

            let has_revoke = rule.verbs.contains(&Verb::Revoke);

            if has_deny && has_revoke {
                return Err(anyhow::anyhow!(
                    "DENY can not be combined with REVOKE verb for collection '{}'",
                    rule.collection
                ));
            }
        }
    }
    Ok(())
}
