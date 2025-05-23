import { sha256 } from "@noble/hashes/sha256";

import type { Contract } from "../protobufs/sdk/metadata";
import { CreateLedgerTransfers } from "../protobufs/sdk/transaction/transaction";


export interface TransferInfo {
    ledgerId?: string;
    fromAccountId?: Uint8Array;
    toAccountId?: Uint8Array;
    amount?: bigint;
    nonce?: bigint;
}

export class FinalizedContractExt {

    public contract: Contract;

    public constructor(contract: Contract) {
        this.contract = contract;
    }

    /**
     * Calculates the contract ID based on the transactions
     */
    public id(): Uint8Array {
        return sha256(new Uint8Array(this.contract.transactions.buffer));
    }

    /**
     * Extracts a list of the proposed transfers
     */
    public transferInfo(): TransferInfo[] {

        const { transfers } = CreateLedgerTransfers.fromBinary(this.contract.transactions);

        return transfers
            .flatMap((createLedgerTransfer) =>
                (createLedgerTransfer.transfer?.transferSteps || [])
                    .map((transferStep) => ({
                        ledgerId: createLedgerTransfer.ledgerId,
                        fromAccountId: transferStep.fromAccountId,
                        toAccountId: transferStep.toAccountId,
                        amount: transferStep.amount,
                        nonce: createLedgerTransfer.nonce,
                    } as TransferInfo)),
            );
    }
}
