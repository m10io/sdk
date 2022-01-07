mod builder;
mod contract_ext;

pub use builder::ContractBuilder;
pub use contract_ext::{ContractId, FinalizedContractExt, TransferInfo};

// TODO: Move somewhere public
pub use crate::account::AccountId;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account_id;

    #[test]
    fn contract_flow() -> Result<(), m10_sdk_protos::prost::EncodeError> {
        let contract = ContractBuilder::default()
            .transfer(
                "usd.m10",
                account_id![0; 0].unwrap(),
                account_id![0; 1].unwrap(),
                4212,
                Some("my test transfer on USD"),
            )
            .transfer(
                "cad.m10",
                account_id![0; 1].unwrap(),
                account_id![0; 2].unwrap(),
                1242,
                Some("my test transfer on CAD"),
            )
            .build()?;
        assert!(!contract.id().is_empty(), "Contract ID is empty");
        Ok(())
    }
}
