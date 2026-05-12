use crate::m10_core_client::signed_transaction_as;
use m10_protos::sdk;
use m10_signing::{Ed25519, P256};
use std::sync::Arc;

fn test_data() -> sdk::transaction_data::Data {
    sdk::transaction_data::Data::CreateLedgerAccount(Default::default())
}

#[tokio::test]
async fn p256_signed_transaction_envelope_verifies() {
    let signer = Arc::new(P256::new_key_pair(None).unwrap());
    let signed = signed_transaction_as(test_data(), vec![], signer)
        .await
        .unwrap();

    let signature = signed.request_envelope.signature.unwrap();
    signature
        .verify(&signed.request_envelope.payload)
        .expect("P256 envelope signature must verify against envelope payload");
}

#[tokio::test]
async fn ed25519ph_signed_transaction_envelope_verifies() {
    let signer = Arc::new(Ed25519::new_key_pair_ph(None).unwrap());
    let signed = signed_transaction_as(test_data(), vec![], signer)
        .await
        .unwrap();

    let signature = signed.request_envelope.signature.unwrap();
    signature
        .verify(&signed.request_envelope.payload)
        .expect("Ed25519Ph envelope signature must verify against envelope payload");
}

#[tokio::test]
async fn plain_ed25519_signed_transaction_backward_compat() {
    let signer = Arc::new(Ed25519::new_key_pair(None).unwrap());
    let signed = signed_transaction_as(test_data(), vec![], signer)
        .await
        .unwrap();

    let signature = signed.request_envelope.signature.unwrap();
    signature
        .verify(&signed.request_envelope.payload)
        .expect("plain Ed25519 envelope signature must still verify");
}
