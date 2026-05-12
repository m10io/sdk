use crate::vault::parse_vault_signature;
use crate::vault::VaultTransit;
use crate::{Ed25519, PreparedDigest, PreparedTransaction, Signer, P256};
use m10_protos::sdk;
use m10_protos::sdk::signature::Algorithm;
use serde_json::json;
use sha2::Digest as _;
use std::time::Duration;
use testcontainers::{runners::AsyncRunner, GenericImage, ImageExt};
use tokio::time::sleep;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Unit tests

#[test]
fn canonical_payload_encoding_is_stable() {
    let payload = b"canonical payload bytes".to_vec();

    let prepared1 = PreparedTransaction::new(payload.clone(), Algorithm::P256Sha256Asn1).unwrap();
    let prepared2 = PreparedTransaction::new(payload.clone(), Algorithm::P256Sha256Asn1).unwrap();

    match (&prepared1.digest, &prepared2.digest) {
        (PreparedDigest::P256Sha256(d1), PreparedDigest::P256Sha256(d2)) => {
            assert_eq!(
                d1, d2,
                "P256 digest must be stable for the same payload bytes"
            );
        }
        _ => panic!("unexpected digest variants"),
    }

    let prepared3 = PreparedTransaction::new(payload.clone(), Algorithm::Ed25519PhSha512).unwrap();
    let prepared4 = PreparedTransaction::new(payload.clone(), Algorithm::Ed25519PhSha512).unwrap();

    match (&prepared3.digest, &prepared4.digest) {
        (PreparedDigest::Ed25519PhSha512(d3), PreparedDigest::Ed25519PhSha512(d4)) => {
            assert_eq!(
                d3, d4,
                "Ed25519Ph digest must be stable for the same payload bytes"
            );
        }
        _ => panic!("unexpected digest variants"),
    }
}

#[test]
fn prepared_transaction_digest_matches_algorithm() {
    let payload = b"test payload".to_vec();

    let p256 = PreparedTransaction::new(payload.clone(), Algorithm::P256Sha256Asn1).unwrap();
    assert!(matches!(p256.digest, PreparedDigest::P256Sha256(_)));
    assert_eq!(p256.algorithm, Algorithm::P256Sha256Asn1);

    let ed = PreparedTransaction::new(payload, Algorithm::Ed25519PhSha512).unwrap();
    assert!(matches!(ed.digest, PreparedDigest::Ed25519PhSha512(_)));
    assert_eq!(ed.algorithm, Algorithm::Ed25519PhSha512);
}

#[test]
fn p256_digest_matches_sha256_of_payload() {
    let payload = b"test transaction payload".to_vec();
    let prepared = PreparedTransaction::new(payload.clone(), Algorithm::P256Sha256Asn1).unwrap();
    let expected = sha2::Sha256::digest(&payload);
    match &prepared.digest {
        PreparedDigest::P256Sha256(d) => assert_eq!(d.as_ref(), &expected[..]),
        _ => panic!("expected P256Sha256 digest variant"),
    }
}

#[test]
fn ed25519ph_digest_matches_sha512_of_payload() {
    let payload = b"test transaction payload".to_vec();
    let prepared = PreparedTransaction::new(payload.clone(), Algorithm::Ed25519PhSha512).unwrap();
    let expected = sha2::Sha512::digest(&payload);
    match &prepared.digest {
        PreparedDigest::Ed25519PhSha512(d) => assert_eq!(d.as_ref(), &expected[..]),
        _ => panic!("expected Ed25519PhSha512 digest variant"),
    }
}

#[test]
fn final_envelope_uses_exact_prepared_payload_bytes() {
    let payload = b"exact payload bytes".to_vec();
    let prepared = PreparedTransaction::new(payload.clone(), Algorithm::P256Sha256Asn1).unwrap();

    assert_eq!(
        prepared.payload, payload,
        "prepared transaction must preserve exact input payload bytes"
    );
    prepared
        .verify_integrity()
        .expect("digest must match the preserved payload");
}

#[test]
fn sdk_rejects_digest_signing_for_plain_ed25519() {
    let result = PreparedTransaction::new(b"any payload".to_vec(), Algorithm::Ed25519);
    assert!(
        result.is_err(),
        "PreparedTransaction::new must reject plain ED25519"
    );
}

// Interoperability tests

#[tokio::test]
async fn local_p256_end_to_end_through_server_verifier() {
    let signer = P256::new_key_pair(None).unwrap();
    let payload = b"p256 round-trip payload".to_vec();
    let prepared = PreparedTransaction::new(payload.clone(), Algorithm::P256Sha256Asn1).unwrap();
    prepared.verify_integrity().unwrap();

    let sig_bytes = signer.sign_prepared_transaction(&prepared).await.unwrap();

    let signature = sdk::Signature {
        algorithm: Algorithm::P256Sha256Asn1.into(),
        public_key: signer.public_key().to_vec(),
        signature: sig_bytes,
    };
    signature
        .verify(&payload)
        .expect("local P256 digest signature must verify through server verifier");
}

#[tokio::test]
async fn local_ed25519ph_end_to_end_through_server_verifier() {
    let signer = Ed25519::new_key_pair_ph(None).unwrap();
    let payload = b"ed25519ph round-trip payload".to_vec();
    let prepared = PreparedTransaction::new(payload.clone(), Algorithm::Ed25519PhSha512).unwrap();
    prepared.verify_integrity().unwrap();

    let sig_bytes = signer.sign_prepared_transaction(&prepared).await.unwrap();

    let signature = sdk::Signature {
        algorithm: Algorithm::Ed25519PhSha512.into(),
        public_key: signer.public_key().to_vec(),
        signature: sig_bytes,
    };
    signature
        .verify(&payload)
        .expect("local Ed25519Ph digest signature must verify through server verifier");
}

#[tokio::test]
async fn plain_ed25519_backward_compat() {
    let signer = Ed25519::new_key_pair(None).unwrap();
    let payload = b"plain ed25519 payload";

    let sig_bytes = signer.sign(payload).await.unwrap();

    let signature = sdk::Signature {
        algorithm: Algorithm::Ed25519.into(),
        public_key: signer.public_key().to_vec(),
        signature: sig_bytes,
    };
    signature
        .verify(payload)
        .expect("plain Ed25519 sign() must still verify through server verifier");
}

#[tokio::test]
async fn sign_request_p256_uses_digest_path() {
    let signer = P256::new_key_pair(None).expect("Failed to create P256 keypair");
    let msg = sdk::GetAccountRequest {
        id: b"test-account".to_vec(),
    };

    let signed = signer
        .sign_request(msg)
        .await
        .expect("sign_request must succeed for P256");

    let sig = signed
        .request_envelope
        .signature
        .expect("signature must be present");
    sig.verify(&signed.request_envelope.payload)
        .expect("P256 sign_request must verify through server verifier");
}

#[tokio::test]
async fn sign_request_ed25519ph_uses_digest_path() {
    let signer = Ed25519::new_key_pair_ph(None).expect("Failed to create Ed25519Ph keypair");
    let msg = sdk::GetAccountRequest {
        id: b"test-account".to_vec(),
    };

    let signed = signer
        .sign_request(msg)
        .await
        .expect("sign_request must succeed for Ed25519Ph");

    let sig = signed
        .request_envelope
        .signature
        .expect("signature must be present");
    sig.verify(&signed.request_envelope.payload)
        .expect("Ed25519Ph sign_request must verify through server verifier");
}

#[tokio::test]
async fn sign_request_ed25519_backward_compat() {
    let signer = Ed25519::new_key_pair(None).expect("Failed to create Ed25519 keypair");
    let msg = sdk::GetAccountRequest {
        id: b"test-account".to_vec(),
    };

    let signed = signer
        .sign_request(msg)
        .await
        .expect("sign_request must succeed for plain Ed25519");

    let sig = signed
        .request_envelope
        .signature
        .expect("signature must be present");
    sig.verify(&signed.request_envelope.payload)
        .expect("Ed25519 sign_request must verify through server verifier");
}

// Vault tests

pub async fn setup_real_vault(
    key_name: &str,
    algorithm: Algorithm,
) -> (VaultTransit, testcontainers::ContainerAsync<GenericImage>) {
    let token = "my-test-token";
    let image = GenericImage::new("hashicorp/vault", "latest")
        .with_exposed_port(testcontainers::core::ContainerPort::Tcp(8200))
        .with_env_var("VAULT_DEV_ROOT_TOKEN_ID", token)
        .with_env_var("VAULT_DISABLE_MLOCK", "1");

    let container = image
        .start()
        .await
        .expect("Failed to start Vault container");

    let host_port = container
        .get_host_port_ipv4(8200)
        .await
        .expect("Failed to get port");
    let vault_url = format!("http://127.0.0.1:{}", host_port);

    let client = reqwest::Client::new();

    let mut is_ready = false;
    for _ in 0..20 {
        if let Ok(res) = client
            .get(format!("{}/v1/sys/health", vault_url))
            .send()
            .await
        {
            if res.status().is_success() {
                is_ready = true;
                break;
            }
        }
        sleep(Duration::from_millis(500)).await;
    }

    if !is_ready {
        panic!("Vault container never became fully ready (did not return HTTP 200 OK)");
    }

    let res_mount = client
        .post(format!("{}/v1/sys/mounts/transit", vault_url))
        .header("X-Vault-Token", token)
        .json(&serde_json::json!({ "type": "transit" }))
        .send()
        .await
        .unwrap();

    if !res_mount.status().is_success() {
        let status = res_mount.status();
        let error_body = res_mount.text().await.unwrap_or_default();

        if !error_body.contains("path is already in use") {
            panic!(
                "Failed to enable transit engine! Status: {}, Body: {}",
                status, error_body
            );
        }
    }

    sleep(Duration::from_millis(500)).await;

    let vault_transit = VaultTransit::new(
        &vault_url,
        token,
        key_name.to_string(),
        None,
        Some(algorithm),
        None,
    )
    .await
    .expect("Failed to initialize VaultTransit");

    (vault_transit, container)
}

async fn setup_wiremock_vault() -> MockServer {
    let mock_server = MockServer::start().await;
    let vault_response = json!({
        "request_id": "mock-req-id-12345",
        "lease_id": "",
        "renewable": false,
        "lease_duration": 0,
        "data": {
            "signature": "vault:v1:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=="
        },
        "wrap_info": null,
        "warnings": null,
        "auth": null
    });

    Mock::given(method("POST"))
        .and(path("/v1/transit/sign/test-key"))
        .and(header("X-Vault-Token", "some-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(vault_response))
        .mount(&mock_server)
        .await;

    mock_server
}

#[tokio::test]
async fn test_mismatched_algorithms_p256_key_ed25519_digest() {
    let mock_server = setup_wiremock_vault().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": {
                "signature": "vault:v1:ZmFrZXNpZ25hdHVyZQ=="
            }
        })))
        .mount(&mock_server)
        .await;

    let vault = VaultTransit::new_mock(&mock_server.uri(), Algorithm::Ed25519PhSha512);
    let payload = vec![];
    let prepared =
        PreparedTransaction::new(payload, Algorithm::P256Sha256Asn1).expect("Failed to prepare");

    let result = Signer::sign_prepared_transaction(&vault, &prepared).await;
    assert!(result.is_err(), "Expected error for mismatched algorithms");
}

#[tokio::test]
async fn test_mismatched_algorithms_ed25519ph_key_p256_digest() {
    let mock_server = setup_wiremock_vault().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": {
                "signature": "vault:v1:ZmFrZXNpZ25hdHVyZQ=="
            }
        })))
        .mount(&mock_server)
        .await;

    let vault = VaultTransit::new_mock(&mock_server.uri(), Algorithm::P256Sha256Asn1);

    let payload = vec![];

    let prepared = PreparedTransaction::new(payload, Algorithm::Ed25519PhSha512)
        .expect("Failed to prepare transaction");

    let result = vault.sign_prepared_transaction(&prepared).await;
    assert!(result.is_err(), "Expected error for mismatched algorithms");
}

#[tokio::test]
#[ignore]
async fn vault_p256_end_to_end_through_server_verifier() {
    let key_name = "test-key-p256-e2e";

    let (vault, _container) = setup_real_vault(key_name, Algorithm::P256Sha256Asn1).await;

    let payload = b"vault p256 end-to-end payload".to_vec();
    let prepared = PreparedTransaction::new(payload.clone(), Algorithm::P256Sha256Asn1).unwrap();

    let sig_bytes = vault.sign_prepared_transaction(&prepared).await.unwrap();

    let signature = sdk::Signature {
        algorithm: Algorithm::P256Sha256Asn1.into(),
        public_key: vault.public_key().to_vec(),
        signature: sig_bytes,
    };
    signature
        .verify(&payload)
        .expect("Vault P256 signature must verify locally");
}

#[tokio::test]
async fn test_ed25519_vault_mock_end_to_end() {
    let mock_server = setup_wiremock_vault().await;

    let vault = VaultTransit::new_mock(&mock_server.uri(), Algorithm::Ed25519PhSha512);

    let payload = b"test payload for mock vault".to_vec();
    let prepared = PreparedTransaction::new(payload, Algorithm::Ed25519PhSha512)
        .expect("Failed to prepare transaction");
    let sig_bytes = vault
        .sign_prepared_transaction(&prepared)
        .await
        .expect("Failed to get signature from mock Vault");

    assert_eq!(sig_bytes.len(), 64, "Signature should be exactly 64 bytes");
}

#[test]
fn test_rejects_wrong_signature_encoding() {
    let no_prefix = parse_vault_signature("some_random_base64_string");
    assert!(
        no_prefix.is_err(),
        "Must reject signature without vault:v1: prefix"
    );
    let wrong_prefix = parse_vault_signature("vault:v2:base64string");
    assert!(
        wrong_prefix.is_err(),
        "Must reject signature with wrong version prefix"
    );
    let bad_base64 = parse_vault_signature("vault:v1:not_a_valid_base64_!@#");
    assert!(bad_base64.is_err(), "Must reject invalid base64 encoding");
}

#[test]
fn test_mutate_payload_after_digest_creation_fails() {
    let payload = b"original transaction data".to_vec();

    let mut prepared = PreparedTransaction::new(payload, Algorithm::Ed25519PhSha512)
        .expect("Failed to create prepared transaction");

    assert!(
        prepared.verify_integrity().is_ok(),
        "Should be valid initially"
    );

    prepared.payload.push(0xFF);

    let result = prepared.verify_integrity();
    assert!(
        result.is_err(),
        "Verification MUST fail if payload was mutated after digest creation"
    );
}

#[tokio::test]
async fn label_ed25519ph_signature_as_plain_ed25519_fails() {
    let payload = b"test payload for ph to plain".to_vec();

    let ph_signer = Ed25519::new_key_pair_ph(None).expect("Failed to create Ed25519Ph keypair");

    let prepared = PreparedTransaction::new(payload.clone(), Algorithm::Ed25519PhSha512)
        .expect("Failed to prepare transaction");

    let valid_signature_bytes = ph_signer
        .sign_prepared_transaction(&prepared)
        .await
        .expect("Failed to sign locally with Ed25519Ph");

    let manipulated_signature = m10_protos::sdk::Signature {
        public_key: ph_signer.public_key().to_vec(),
        signature: valid_signature_bytes,
        algorithm: Algorithm::Ed25519 as i32,
    };

    let result = manipulated_signature.verify(&payload);

    assert!(
        result.is_err(),
        "Local Ed25519Ph signature labeled as plain ED25519 SHOULD fail verification"
    );
}

#[tokio::test]
async fn label_plain_ed25519_signature_as_ed25519_ph_sha_512_fails() {
    let payload = b"test payload for plain to ph".to_vec();

    let plain_signer = Ed25519::new_key_pair(None).expect("Failed to create plain Ed25519 keypair");

    let valid_signature_bytes = plain_signer
        .sign(&payload)
        .await
        .expect("Failed to sign locally with plain Ed25519");

    let manipulated_signature = m10_protos::sdk::Signature {
        public_key: plain_signer.public_key().to_vec(),
        signature: valid_signature_bytes,
        algorithm: Algorithm::Ed25519PhSha512 as i32,
    };

    let result = manipulated_signature.verify(&payload);

    assert!(
        result.is_err(),
        "Plain ED25519 signature labeled as ED25519_PH_SHA_512 SHOULD fail verification"
    );
}
