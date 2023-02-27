use m10_sdk::account::{AccountId, Builder as AccountIdBuilder, LeafAccountIndex, RawAccountIndex};
use m10_sdk::{sdk::signature::Algorithm, Ed25519, KeyPair, Signer, P256};
use std::{fs::File, io::Read};
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
        return Err(anyhow::anyhow!("account chain format"));
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
        Err(anyhow::anyhow!("account chain format"))
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
        return Err(anyhow::anyhow!("not a valid int array"));
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
        return Err(anyhow::anyhow!("not a valid hex array"));
    }
    let stripped = trimmed[1..trimmed.len() - 1]
        .split(',')
        .map(|i| i.trim())
        .collect::<Vec<&str>>();
    let stripped = &stripped.join("");

    Ok(hex::decode(stripped)?)
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
    }
}

pub(crate) fn create_exportable_key_pair(method: Algorithm) -> Result<Vec<u8>, anyhow::Error> {
    match method {
        Algorithm::Ed25519 => {
            let (kp, _) = Ed25519::new_key_pair_exportable()?;
            Ok(kp)
        }
        Algorithm::P256Sha256Asn1 => {
            let (kp, _) = P256::new_key_pair_exportable()?;
            Ok(kp)
        }
    }
}

pub(crate) fn key_pair_from_env(key: &str) -> Result<KeyPair, anyhow::Error> {
    let try_key_pair = Ed25519::from_str(key);
    if try_key_pair.is_err() {
        Ok(KeyPair::P256(P256::from_str(key).map_err(|err| {
            anyhow::anyhow!("Could not create key pair from env: {}", err)
        })?))
    } else {
        Ok(KeyPair::Ed25519(try_key_pair.map_err(|err| {
            anyhow::anyhow!("Could not create key pair from env: {}", err)
        })?))
    }
}

pub(crate) fn load_key_pair(path_str: &str) -> Result<KeyPair, anyhow::Error> {
    let try_key_pair = Ed25519::load_key_pair(path_str);
    if try_key_pair.is_err() {
        Ok(KeyPair::P256(P256::load_key_pair(path_str).map_err(
            |err| anyhow::anyhow!("Could not load key file: {}", err),
        )?))
    } else {
        Ok(KeyPair::Ed25519(try_key_pair.map_err(|err| {
            anyhow::anyhow!("Could not load key file: {}", err)
        })?))
    }
}

pub(crate) fn load_key_pair_exportable(path_str: &str) -> Result<Vec<u8>, anyhow::Error> {
    let mut key_file = File::open(path_str)?;
    let mut pkcs8_bytes: Vec<u8> = Vec::new();
    key_file.read_to_end(&mut pkcs8_bytes)?;
    Ok(pkcs8_bytes)
}
