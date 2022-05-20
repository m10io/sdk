/// <reference types="long" />
/// <reference types="node" />
import { m10 } from "../../protobufs";
export interface TransferInfo {
    ledgerId?: Option<string>;
    fromAccountId?: Option<Uint8Array>;
    toAccountId?: Option<Uint8Array>;
    amount?: Option<number | Long>;
    nonce?: Option<number | Long>;
}
export declare class FinalizedContractExt {
    contract: m10.sdk.metadata.Contract;
    constructor(contract: m10.sdk.metadata.Contract);
    /**
     * Calculates the contract ID based on the transactions
     */
    id(): Buffer;
    /**
     * Extracts a list of the proposed transfers
     */
    transferInfo(): TransferInfo[];
}
