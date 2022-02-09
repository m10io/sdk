mod builder;
mod contract_ext;

pub use builder::ContractBuilder;
pub use contract_ext::{ContractId, FinalizedContractExt, TransferInfo};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_flow() -> Result<(), m10_protos::prost::EncodeError> {
        let contract = ContractBuilder::default()
            .transfer(
                "usd.m10",
                vec![0, 0],
                vec![0, 1],
                4212,
                Some("my test transfer on USD"),
            )
            .transfer(
                "cad.m10",
                vec![0, 1],
                vec![0, 2],
                1242,
                Some("my test transfer on CAD"),
            )
            .build()?;
        assert!(!contract.id().is_empty(), "Contract ID is empty");
        Ok(())
    }
}
