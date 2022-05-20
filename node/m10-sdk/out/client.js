"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.LedgerClient = void 0;
const grpc = __importStar(require("@grpc/grpc-js"));
const protobufs_1 = require("../protobufs");
const utils = __importStar(require("./utils"));
/**
 * A client for the M10 Ledger.
 *
 * This client allows you to query and transact on the M10 ledger.
 *
 * @example
 * const ledgerUrl = "test.m10.net";
 * const ledgerClient = new LedgerClient(ledgerUrl, true);
 * const block_height = await ledgerClient.blockHeight();
 */
class LedgerClient {
    constructor(ledger_url, tls) {
        const credentials = tls ? grpc.credentials.createSsl() : grpc.credentials.createInsecure();
        this.txClient = protobufs_1.m10.sdk.M10TxService.create(utils.getRPCImpl(ledger_url, credentials, "m10.sdk.M10TxService"));
        this.queryClient = protobufs_1.m10.sdk.M10QueryService.create(utils.getRPCImpl(ledger_url, credentials, "m10.sdk.M10QueryService"));
        this.queryClientStream = protobufs_1.m10.sdk.M10QueryService.create(utils.getRPCImplStream(ledger_url, credentials, "m10.sdk.M10QueryService"));
    }
    // -------------------------------------------------------------------------
    // Transactions
    // -------------------------------------------------------------------------
    transactionRequest(data, contextId) {
        return new protobufs_1.m10.sdk.transaction.TransactionRequestPayload({
            data: data,
            nonce: utils.TransactionSigner.generateNonce(),
            timestamp: utils.TransactionSigner.getTimestamp(),
            contextId: contextId,
        });
    }
    blockHeight() {
        return __awaiter(this, void 0, void 0, function* () {
            const emptyRequest = new protobufs_1.google.protobuf.Empty();
            const chainInfo = yield this.queryClient.getChainInfo(emptyRequest);
            return chainInfo.blockHeight;
        });
    }
    createTransaction(signer, request) {
        return __awaiter(this, void 0, void 0, function* () {
            const payload = protobufs_1.m10.sdk.transaction.TransactionRequestPayload.encode(request).finish();
            const envelop = new protobufs_1.m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
            return this.txClient.createTransaction(envelop);
        });
    }
    // -------------------------------------------------------------------------
    // Transfers
    // -------------------------------------------------------------------------
    getTransfer(signer, properties) {
        return __awaiter(this, void 0, void 0, function* () {
            const request = new protobufs_1.m10.sdk.transaction.GetTransferRequest(properties);
            const payload = protobufs_1.m10.sdk.transaction.GetTransferRequest.encode(request).finish();
            const envelope = new protobufs_1.m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
            return this.queryClient.getTransfer(envelope);
        });
    }
    listTransfers(signer, properties) {
        return __awaiter(this, void 0, void 0, function* () {
            const request = new protobufs_1.m10.sdk.transaction.ListTransferRequest(properties);
            const payload = protobufs_1.m10.sdk.transaction.ListTransferRequest.encode(request).finish();
            const envelope = new protobufs_1.m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
            return this.queryClient.listTransfers(envelope);
        });
    }
    // -------------------------------------------------------------------------
    // Actions
    // -------------------------------------------------------------------------
    getAction(signer, properties) {
        return __awaiter(this, void 0, void 0, function* () {
            const request = new protobufs_1.m10.sdk.transaction.GetActionRequest(properties);
            const payload = protobufs_1.m10.sdk.transaction.GetActionRequest.encode(request).finish();
            const envelope = new protobufs_1.m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
            return this.queryClient.getAction(envelope);
        });
    }
    listActions(signer, properties) {
        return __awaiter(this, void 0, void 0, function* () {
            const request = new protobufs_1.m10.sdk.transaction.ListActionsRequest(properties);
            const payload = protobufs_1.m10.sdk.transaction.ListActionsRequest.encode(request).finish();
            const envelope = new protobufs_1.m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
            return this.queryClient.listActions(envelope);
        });
    }
    getTransaction(signer, properties) {
        return __awaiter(this, void 0, void 0, function* () {
            const request = new protobufs_1.m10.sdk.GetTransactionRequest(properties);
            const payload = protobufs_1.m10.sdk.GetTransactionRequest.encode(request).finish();
            const envelope = new protobufs_1.m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
            return this.queryClient.getTransaction(envelope);
        });
    }
    listTransactions(signer, properties) {
        return __awaiter(this, void 0, void 0, function* () {
            const request = new protobufs_1.m10.sdk.ListTransactionsRequest(properties);
            const payload = protobufs_1.m10.sdk.ListTransactionsRequest.encode(request).finish();
            const envelope = new protobufs_1.m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
            return this.queryClient.listTransactions(envelope);
        });
    }
    groupTransactions(signer, properties) {
        return __awaiter(this, void 0, void 0, function* () {
            const request = new protobufs_1.m10.sdk.GroupTransactionsRequest(properties);
            const payload = protobufs_1.m10.sdk.GroupTransactionsRequest.encode(request).finish();
            const envelope = new protobufs_1.m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
            return this.queryClient.groupTransactions(envelope);
        });
    }
    // -------------------------------------------------------------------------
    // Indexed Accounts
    // -------------------------------------------------------------------------
    getIndexedAccount(signer, properties) {
        return __awaiter(this, void 0, void 0, function* () {
            const request = new protobufs_1.m10.sdk.transaction.GetAccountRequest(properties);
            const payload = protobufs_1.m10.sdk.transaction.GetAccountRequest.encode(request).finish();
            const envelope = new protobufs_1.m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
            return this.queryClient.getIndexedAccount(envelope);
        });
    }
    // -------------------------------------------------------------------------
    // AccountSets
    // -------------------------------------------------------------------------
    getAccountSet(signer, properties) {
        return __awaiter(this, void 0, void 0, function* () {
            const request = new protobufs_1.m10.sdk.GetAccountSetRequest(properties);
            const payload = protobufs_1.m10.sdk.GetAccountSetRequest.encode(request).finish();
            const envelope = new protobufs_1.m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
            return this.queryClient.getAccountSet(envelope);
        });
    }
    listAccountSets(signer, properties) {
        return __awaiter(this, void 0, void 0, function* () {
            const request = new protobufs_1.m10.sdk.ListAccountSetsRequest(properties);
            const payload = protobufs_1.m10.sdk.ListAccountSetsRequest.encode(request).finish();
            const envelope = new protobufs_1.m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
            return this.queryClient.listAccountSets(envelope);
        });
    }
    // -------------------------------------------------------------------------
    // Accounts
    // -------------------------------------------------------------------------
    getAccount(signer, properties) {
        return __awaiter(this, void 0, void 0, function* () {
            const request = new protobufs_1.m10.sdk.transaction.GetAccountRequest(properties);
            const payload = protobufs_1.m10.sdk.transaction.GetAccountRequest.encode(request).finish();
            const envelope = new protobufs_1.m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
            return this.queryClient.getAccount(envelope);
        });
    }
    getAccountInfo(signer, properties) {
        return __awaiter(this, void 0, void 0, function* () {
            const request = new protobufs_1.m10.sdk.transaction.GetAccountRequest(properties);
            const payload = protobufs_1.m10.sdk.transaction.GetAccountRequest.encode(request).finish();
            const envelope = new protobufs_1.m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
            return this.queryClient.getAccountInfo(envelope);
        });
    }
    listAccounts(signer, properties) {
        return __awaiter(this, void 0, void 0, function* () {
            const request = new protobufs_1.m10.sdk.ListAccountsRequest(properties);
            const payload = protobufs_1.m10.sdk.ListAccountsRequest.encode(request).finish();
            const envelope = new protobufs_1.m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
            return this.queryClient.listAccounts(envelope);
        });
    }
    // -------------------------------------------------------------------------
    // Role Bindings
    // -------------------------------------------------------------------------
    getRoleBinding(signer, properties) {
        return __awaiter(this, void 0, void 0, function* () {
            const request = new protobufs_1.m10.sdk.GetRoleBindingRequest(properties);
            const payload = protobufs_1.m10.sdk.GetRoleBindingRequest.encode(request).finish();
            const envelope = new protobufs_1.m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
            return this.queryClient.getRoleBinding(envelope);
        });
    }
    listRoleBindings(signer, properties) {
        return __awaiter(this, void 0, void 0, function* () {
            const request = new protobufs_1.m10.sdk.ListRoleBindingsRequest(properties);
            const payload = protobufs_1.m10.sdk.ListRoleBindingsRequest.encode(request).finish();
            const envelope = new protobufs_1.m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
            return this.queryClient.listRoleBindings(envelope);
        });
    }
    // -------------------------------------------------------------------------
    // Roles
    // -------------------------------------------------------------------------
    getRole(signer, properties) {
        return __awaiter(this, void 0, void 0, function* () {
            const request = new protobufs_1.m10.sdk.GetRoleRequest(properties);
            const payload = protobufs_1.m10.sdk.GetRoleRequest.encode(request).finish();
            const envelope = new protobufs_1.m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
            return this.queryClient.getRole(envelope);
        });
    }
    listRoles(signer, properties) {
        return __awaiter(this, void 0, void 0, function* () {
            const request = new protobufs_1.m10.sdk.ListRolesRequest(properties);
            const payload = protobufs_1.m10.sdk.ListRolesRequest.encode(request).finish();
            const envelope = new protobufs_1.m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
            return this.queryClient.listRoles(envelope);
        });
    }
    // -------------------------------------------------------------------------
    // Observations
    // -------------------------------------------------------------------------
    getObserveTransfers(signer, properties) {
        return __awaiter(this, void 0, void 0, function* () {
            const request = new protobufs_1.m10.sdk.ObserveAccountsRequest(properties);
            const payload = protobufs_1.m10.sdk.ObserveAccountsRequest.encode(request).finish();
            const envelope = new protobufs_1.m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
            return this.queryClientStream.observeTransfers(envelope);
        });
    }
    getObserveResources(signer, properties) {
        return __awaiter(this, void 0, void 0, function* () {
            const request = new protobufs_1.m10.sdk.ObserveResourcesRequest(properties);
            const payload = protobufs_1.m10.sdk.ObserveResourcesRequest.encode(request).finish();
            const envelope = new protobufs_1.m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
            return this.queryClientStream.observeResources(envelope);
        });
    }
    getObserveAccounts(signer, properties) {
        return __awaiter(this, void 0, void 0, function* () {
            const request = new protobufs_1.m10.sdk.ObserveAccountsRequest(properties);
            const payload = protobufs_1.m10.sdk.ObserveAccountsRequest.encode(request).finish();
            const envelope = new protobufs_1.m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
            return this.queryClientStream.observeAccounts(envelope);
        });
    }
    getObserveActions(signer, properties) {
        return __awaiter(this, void 0, void 0, function* () {
            const request = new protobufs_1.m10.sdk.ObserveActionsRequest(properties);
            const payload = protobufs_1.m10.sdk.ObserveActionsRequest.encode(request).finish();
            const envelope = new protobufs_1.m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
            return this.queryClientStream.observeActions(envelope);
        });
    }
    // -------------------------------------------------------------------------
    // EnhancedTransfers
    // -------------------------------------------------------------------------
    enhanceTransfers(signer, transfers) {
        return __awaiter(this, void 0, void 0, function* () {
            const enhancedTransfersPromise = transfers.map((transfer) => this.enhanceTransfer(signer, transfer));
            const enhancedTransfers = yield Promise.all(enhancedTransfersPromise);
            return enhancedTransfers;
        });
    }
    enhanceTransfer(signer, transfer) {
        return __awaiter(this, void 0, void 0, function* () {
            const enhancedTransferStepsPromise = (transfer.transferSteps || [])
                .map((transferStep) => this.enhanceTransferStep(signer, transferStep));
            const enhancedTransferSteps = yield Promise.all(enhancedTransferStepsPromise);
            return {
                transfer: new protobufs_1.m10.sdk.transaction.FinalizedTransfer(transfer),
                enhanced_steps: enhancedTransferSteps,
            };
        });
    }
    enhanceTransferStep(signer, transferStep) {
        return __awaiter(this, void 0, void 0, function* () {
            const fromPromise = this.getAccountInfo(signer, { id: transferStep.fromAccountId });
            const toPromise = this.getAccountInfo(signer, { id: transferStep.toAccountId });
            const [from, to] = yield Promise.all([fromPromise, toPromise]);
            const fromBankPromise = (utils.isSome(from.parentAccountId)
                ? yield this.getAccountInfo(signer, { id: from.parentAccountId })
                : null);
            const toBankPromise = (utils.isSome(to.parentAccountId)
                ? yield this.getAccountInfo(signer, { id: to.parentAccountId })
                : null);
            const [fromBank, toBank] = yield Promise.all([fromBankPromise, toBankPromise].filter(utils.isSome));
            return {
                from,
                to,
                from_bank: fromBank,
                to_bank: toBank,
            };
        });
    }
}
exports.LedgerClient = LedgerClient;
//# sourceMappingURL=client.js.map