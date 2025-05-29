import type { AccountInfo } from "./protobufs/sdk/model/model";
import type { FinalizedTransfer } from "./protobufs/sdk/transaction/transaction";

/**
 * Represents a transfer that includes additional enhanced details.
 */
export interface EnhancedTransfer {
    /**
     * The finalized transfer data.
     */
    transfer: FinalizedTransfer;

    /**
     * A list of steps involved in the transfer, with enhanced details.
     */
    enhanced_steps: EnhancedTransferStep[];
}

/**
 * Represents a step in an enhanced transfer with detailed account information.
 */
export type EnhancedTransferStep = Partial<{
    /**
     * The account information for the sender, if available.
     */
    from: AccountInfo;

    /**
     * The bank account information for the sender, if available.
     */
    from_bank: AccountInfo;

    /**
     * The account information for the recipient, if available.
     */
    to: AccountInfo;

    /**
     * The bank account information for the recipient, if available.
     */
    to_bank: AccountInfo;
}>;
