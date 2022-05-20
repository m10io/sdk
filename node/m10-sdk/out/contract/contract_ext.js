"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.FinalizedContractExt = void 0;
const protobufs_1 = require("../../protobufs");
const utils_1 = require("../utils");
class FinalizedContractExt {
    constructor(contract) {
        this.contract = contract;
    }
    /**
     * Calculates the contract ID based on the transactions
     */
    id() {
        return new utils_1.CryptoHasher().hash(this.contract.transactions);
    }
    /**
     * Extracts a list of the proposed transfers
     */
    transferInfo() {
        const { transfers } = protobufs_1.m10.sdk.transaction.CreateLedgerTransfers.decode(this.contract.transactions);
        return transfers
            .flatMap((createLedgerTransfer) => {
            var _a;
            return (((_a = createLedgerTransfer.transfer) === null || _a === void 0 ? void 0 : _a.transferSteps) || [])
                .map((transferStep) => ({
                ledgerId: createLedgerTransfer.ledgerId,
                fromAccountId: transferStep.fromAccountId,
                toAccountId: transferStep.toAccountId,
                amount: transferStep.amount,
                nonce: createLedgerTransfer.nonce,
            }));
        });
    }
}
exports.FinalizedContractExt = FinalizedContractExt;
//# sourceMappingURL=contract_ext.js.map