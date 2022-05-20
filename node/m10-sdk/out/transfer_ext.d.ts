import type { m10 } from "../protobufs";
export interface EnhancedTransfer {
    transfer: m10.sdk.transaction.FinalizedTransfer;
    enhanced_steps: EnhancedTransferStep[];
}
export interface EnhancedTransferStep {
    from: Option<m10.sdk.model.IAccountInfo>;
    from_bank: Option<m10.sdk.model.IAccountInfo>;
    to: Option<m10.sdk.model.IAccountInfo>;
    to_bank: Option<m10.sdk.model.IAccountInfo>;
}
