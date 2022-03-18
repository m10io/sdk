use m10_protos::sdk;

pub struct EnhancedTransfer {
    pub transfer: sdk::FinalizedTransfer,
    pub enhanced_steps: Vec<EnhancedTransferStep>,
}

pub struct EnhancedTransferStep {
    pub from: Option<sdk::AccountInfo>,
    pub from_bank: Option<sdk::AccountInfo>,
    pub to: Option<sdk::AccountInfo>,
    pub to_bank: Option<sdk::AccountInfo>,
}
