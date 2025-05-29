import { Contract, Memo } from "../protobufs/sdk/metadata";
import { type CreateLedgerTransfer,CreateLedgerTransfers } from "../protobufs/sdk/transaction/transaction";
import { convertMemoToAny, TransactionSigner } from "../utils";

import { FinalizedContractExt } from "./contract_ext";



/**
 * A builder for Contract
 *
 * A Contract in M10's system is an agreement amoung multiple parties to complete a series of transactions.
 * Each contract contains series of endorsements from each of the parties. Contracts are designed
 * to be executed across ledgers, for scenarios like FX swaps or other multi-currency transactions.
 */
export class ContractBuilder {

    public transfers: CreateLedgerTransfer[];
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
        memo?: string,
    ): ContractBuilder {

        this.transfers.push({
            ledgerId: ledgerId,
            nonce: BigInt(TransactionSigner.generateNonce()),
            transfer: {
                transferSteps: [{
                    fromAccountId: fromAccountId,
                    toAccountId: toAccountId,
                    amount: BigInt(amount),
                    metadata: (
                        memo
                            ? [convertMemoToAny(Memo.create({ plaintext: memo }))]
                            : []
                    ),
                }],
            },
        });

        return this;
    }

    /**
     * Builds the Contract
     */
    public build(): FinalizedContractExt {

        const validUntil = TransactionSigner.getTimestamp(this.validFor);
        const transferReqs = CreateLedgerTransfers.toBinary({ transfers: this.transfers, validUntil: BigInt(validUntil) });

        const contract = Contract.create({
            transactions: transferReqs,
            endorsements: [],
        });

        return new FinalizedContractExt(contract);
    }

    public static default(): ContractBuilder {
        return new ContractBuilder();
    }
}
