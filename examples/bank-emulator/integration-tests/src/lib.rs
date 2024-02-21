#[cfg(test)]
mod accounts;
#[cfg(test)]
mod assets;
#[cfg(test)]
mod contacts;
#[cfg(test)]
mod documents;
#[cfg(test)]
mod fees;
#[cfg(test)]
mod keys;
#[cfg(test)]
mod notification_preferences;
#[cfg(test)]
mod transfer_methods;
#[cfg(test)]
mod utils;

#[cfg(test)]
fn base_url() -> String {
    std::env::var("BANK_EMULATOR_URL").unwrap_or_else(|_| "http://localhost:8080".to_string())
}

#[cfg(test)]
fn ledger_addr() -> String {
    std::env::var("LEDGER_ADDR").unwrap_or_else(|_| "https://develop.m10.net".to_string())
}

#[cfg(test)]
fn ledger_client(
    key_pair: m10_sdk::Ed25519,
) -> Box<dyn m10_sdk::M10CoreClient<Signer = m10_sdk::Ed25519> + Send + Sync> {
    Box::new(
        m10_sdk::GrpcClient::new(
            tonic::transport::Channel::from_shared(ledger_addr()).unwrap(),
            Some(std::sync::Arc::new(key_pair)),
        )
        .unwrap(),
    )
}

#[cfg(test)]
mod tests {}
