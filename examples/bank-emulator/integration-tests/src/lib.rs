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
fn ledger_client() -> m10_sdk::LedgerClient {
    m10_sdk::LedgerClient::new(
        tonic::transport::Channel::from_shared(ledger_addr())
            .unwrap()
            .connect_lazy()
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {}
