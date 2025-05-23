use chrono::DateTime;
use m10_sdk::{
    contract::{FinalizedContractExt, TransferInfo},
    prost::Message,
    sdk,
    sdk::signature::Algorithm,
    Format,
};
use serde::Serialize;
use std::{convert::TryFrom, fmt::Debug};

#[derive(Serialize, Debug)]
pub struct ContractContent {
    pub id: String,
    pub valid_until: String,
    pub transfers: Vec<TransferInfo>,
    pub endorsements: Vec<Endorsement>,
}

#[derive(Serialize, Debug)]
pub struct ContractEndorsement {
    ledger_id: String,
    #[serde(serialize_with = "as_b64", skip_serializing_if = "Vec::is_empty")]
    public_key: Vec<u8>,
    #[serde(serialize_with = "as_b64", skip_serializing_if = "Vec::is_empty")]
    signature: Vec<u8>,
    #[serde(skip_serializing_if = "String::is_empty")]
    algorithm: String,
}

#[derive(Serialize, Debug)]
pub struct Endorsement {
    #[serde(flatten)]
    endorsement: Option<ContractEndorsement>,
}

impl TryFrom<sdk::Endorsement> for Endorsement {
    type Error = anyhow::Error;

    fn try_from(other: sdk::Endorsement) -> Result<Endorsement, Self::Error> {
        let sdk::Endorsement {
            ledger_id,
            signature,
        } = other;
        Ok(Endorsement {
            endorsement: signature.map(|sig| ContractEndorsement {
                ledger_id,
                public_key: sig.public_key,
                signature: sig.signature,
                algorithm: match Algorithm::try_from(sig.algorithm).unwrap() {
                    Algorithm::Ed25519 => "Ed25519",
                    Algorithm::P256Sha256Asn1 => "P256",
                }
                .to_string(),
            }),
        })
    }
}

pub(crate) async fn show_contract(path: &str, formatter: Format) -> anyhow::Result<()> {
    let file = std::fs::read(path)?;
    let contract = sdk::Contract::decode(file.as_slice())?;
    let id = contract.id();
    let transfers = contract.transfer_info()?;
    let transfer_requests = sdk::CreateLedgerTransfers::decode(contract.transactions.as_slice())?;
    let content = ContractContent {
        id: hex::encode(id).to_uppercase(),
        valid_until: DateTime::from_timestamp(
            (transfer_requests.valid_until / 1_000_000) as i64,
            ((transfer_requests.valid_until % 1_000_000) * 1000) as u32,
        )
        .expect("expected valid timestamp")
        .to_string(),
        transfers,
        endorsements: contract
            .endorsements
            .into_iter()
            .map(Endorsement::try_from)
            .collect::<anyhow::Result<_>>()?,
    };
    let content_str = match formatter {
        Format::Yaml => serde_yml::to_string(&content)?,
        Format::Json => serde_json::to_string_pretty(&content)?,
        Format::Raw => format!("{:#?}", content),
        Format::Csv => {
            return Err(anyhow::anyhow!("csv not supported on contracts"));
        }
    };
    println!("{}", content_str);

    Ok(())
}

fn as_b64<S: serde::Serializer>(x: &[u8], s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&base64::encode(x))
}
