import { m10 } from "../../protobufs";
import { FinalizedContractExt } from "./contract_ext";
/**
 * A builder for Contract
 *
 * A Contract in M10's system is an agreement amoung multiple parties to complete a series of transactions.
 * Each contract contains series of endorsements from each of the parties. Contracts are designed
 * to be executed across ledgers, for scenarios like FX swaps or other multi-currency transactions.
 */
export declare class ContractBuilder {
    transfers: m10.sdk.transaction.ICreateLedgerTransfer[];
    /**
     * Timeout of the contract in milliseconds
     */
    validFor: number;
    /**
     * Default contract duration in milliseconds
     */
    static readonly DEFAULT_CONTRACT_DURATION: number;
    constructor();
    /**
     * Adds a transfer to the contract between two accounts on a particular ledger
     *
     * @returns {ContractBuilder} with modified transfers
     */
    transfer(ledgerId: string, fromAccountId: Uint8Array, toAccountId: Uint8Array, amount: number, memo: Option<string>): ContractBuilder;
    /**
     * Builds the Contract
     */
    build(): FinalizedContractExt;
    static default(): ContractBuilder;
}
