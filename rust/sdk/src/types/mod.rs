mod account;
mod account_set;
mod action;
mod bank;
mod role;
mod role_binding;
mod transfer;

pub use account::*;
pub use account_set::*;
pub use action::*;
pub use bank::*;
pub use role::*;
pub use role_binding::*;
pub use transfer::*;

pub type TxId = u64;

#[derive(Debug, Clone)]
pub struct PublicKey(pub Vec<u8>);

#[cfg(feature = "format")]
impl std::fmt::Display for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", base64::encode(&self.0))
    }
}
