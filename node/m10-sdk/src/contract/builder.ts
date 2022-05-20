import { m10 } from "../../protobufs";
import { convertMemoToAny, isSome, TransactionSigner } from "../utils";

import { FinalizedContractExt } from "./contract_ext";



/**
 * A builder for Contract
 *
 * A Contract in M10's system is an agreement amoung multiple parties to complete a series of transactions.
 * Each contract contains series of endorsements from each of the parties. Contracts are designed
 * to be executed across ledgers, for scenarios like FX swaps or other multi-currency transactions.
 */
export class ContractBuilder {

    public transfers: m10.sdk.transaction.ICreateLedgerTransfer[];
    /**
     * Timeout of the contract in milliseconds
     */
    public validFor: number;
    /**
     * Default contract duration in milliseconds
     */
    public static readonly DEFAULT_CONTRACT_DURATION: number = 300_000;


    public constructor() {
        this.transfers = [];
        this.validFor = ContractBuilder.DEFAULT_CONTRACT_DURATION;
    }

    /**
     * Adds a transfer to the contract between two accounts on a particular ledger
     *
     * @returns {ContractBuilder} with modified transfers
     */
    public transfer(
        ledgerId: string,
        fromAccountId: Uint8Array,
        toAccountId: Uint8Array,
        amount: number,
        memo: Option<string>,
    ): ContractBuilder {

        this.transfers.push({
            ledgerId: ledgerId,
            nonce: TransactionSigner.generateNonce(),
            transfer: {
                transferSteps: [ {
                    fromAccountId: fromAccountId,
                    toAccountId: toAccountId,
                    amount: amount,
                    metadata: (
                        isSome(memo)
                            ? [ convertMemoToAny(new m10.sdk.metadata.Memo({ plaintext: memo })) ]
                            : null
                    ),
                } ],
            },
        });

        return this;
    }

    /**
     * Builds the Contract
     */
    public build(): FinalizedContractExt {

        const validUntil = TransactionSigner.getTimestamp(this.validFor);
        const transferReqs = m10.sdk.transaction.CreateLedgerTransfers
            .encode({ transfers: this.transfers, validUntil })
            .finish();

        const contract = new m10.sdk.metadata.Contract({
            transactions: transferReqs,
            endorsements: [],
        });

        return new FinalizedContractExt(contract);
    }

    public static default(): ContractBuilder {
        return new ContractBuilder();
    }
}
