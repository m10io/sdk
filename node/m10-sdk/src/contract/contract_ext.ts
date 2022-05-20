import { m10 } from "../../protobufs";
import { CryptoHasher } from "../utils";


export interface TransferInfo {
    ledgerId?: Option<string>;
    fromAccountId?: Option<Uint8Array>;
    toAccountId?: Option<Uint8Array>;
    amount?: Option<number | Long>;
    nonce?: Option<number | Long>;
}

export class FinalizedContractExt {

    public contract: m10.sdk.metadata.Contract;

    public constructor(contract: m10.sdk.metadata.Contract) {
        this.contract = contract;
    }

    /**
     * Calculates the contract ID based on the transactions
     */
    public id(): Buffer {
        return new CryptoHasher().hash(this.contract.transactions);
    }

    /**
     * Extracts a list of the proposed transfers
     */
    public transferInfo(): TransferInfo[] {

        const { transfers } = m10.sdk.transaction.CreateLedgerTransfers.decode(this.contract.transactions);

        return transfers
            .flatMap((createLedgerTransfer) =>
                (createLedgerTransfer.transfer?.transferSteps || [])
                    .map((transferStep) => ({
                        ledgerId: createLedgerTransfer.ledgerId,
                        fromAccountId: transferStep.fromAccountId,
                        toAccountId: transferStep.toAccountId,
                        amount: transferStep.amount,
                        nonce: createLedgerTransfer.nonce,
                    })),
            );
    }
}
