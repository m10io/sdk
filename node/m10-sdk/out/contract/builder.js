"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.ContractBuilder = void 0;
const protobufs_1 = require("../../protobufs");
const utils_1 = require("../utils");
const contract_ext_1 = require("./contract_ext");
/**
 * A builder for Contract
 *
 * A Contract in M10's system is an agreement amoung multiple parties to complete a series of transactions.
 * Each contract contains series of endorsements from each of the parties. Contracts are designed
 * to be executed across ledgers, for scenarios like FX swaps or other multi-currency transactions.
 */
class ContractBuilder {
    constructor() {
        this.transfers = [];
        this.validFor = ContractBuilder.DEFAULT_CONTRACT_DURATION;
    }
    /**
     * Adds a transfer to the contract between two accounts on a particular ledger
     *
     * @returns {ContractBuilder} with modified transfers
     */
    transfer(ledgerId, fromAccountId, toAccountId, amount, memo) {
        this.transfers.push({
            ledgerId: ledgerId,
            nonce: utils_1.TransactionSigner.generateNonce(),
            transfer: {
                transferSteps: [{
                        fromAccountId: fromAccountId,
                        toAccountId: toAccountId,
                        amount: amount,
                        metadata: ((0, utils_1.isSome)(memo)
                            ? [(0, utils_1.convertMemoToAny)(new protobufs_1.m10.sdk.metadata.Memo({ plaintext: memo }))]
                            : null),
                    }],
            },
        });
        return this;
    }
    /**
     * Builds the Contract
     */
    build() {
        const validUntil = utils_1.TransactionSigner.getTimestamp(this.validFor);
        const transferReqs = protobufs_1.m10.sdk.transaction.CreateLedgerTransfers
            .encode({ transfers: this.transfers, validUntil })
            .finish();
        const contract = new protobufs_1.m10.sdk.metadata.Contract({
            transactions: transferReqs,
            endorsements: [],
        });
        return new contract_ext_1.FinalizedContractExt(contract);
    }
    static default() {
        return new ContractBuilder();
    }
}
exports.ContractBuilder = ContractBuilder;
/**
 * Default contract duration in milliseconds
 */
ContractBuilder.DEFAULT_CONTRACT_DURATION = 300000;
//# sourceMappingURL=builder.js.map