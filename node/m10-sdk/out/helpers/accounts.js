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
exports.createAccount = exports.createLedgerAccount = exports.getBankAdminAccount = void 0;
const protobufs_1 = require("../../protobufs");
const collections = __importStar(require("../collections"));
const utils = __importStar(require("../utils"));
function getBankAdminAccount(client, bankAdminSigner, bankName) {
    return __awaiter(this, void 0, void 0, function* () {
        const bankAccountOwner = bankAdminSigner.getPublicKey();
        const listAccountsRequest = { owner: bankAccountOwner };
        const listAccountsResponse = yield client
            .listAccounts(bankAdminSigner, listAccountsRequest);
        const bankAccounts = (listAccountsResponse.accounts || [])
            .map(account => {
            return {
                id: account.id ? utils.getAccountIdFromUint8Array(account.id) : null,
                owner: account.owner ? utils.getPublicKeyFromUint8Array(account.owner) : null,
                name: account.name,
                publicName: account.publicName,
                profileImageUrl: account.profileImageUrl,
            };
        });
        const bankIssuanceAccount = bankAccounts.find(account => (account.publicName === bankName));
        if (!bankIssuanceAccount) {
            throw new Error(`Cannot find bank account for: ${bankName}`);
        }
        return utils.unwrap(bankIssuanceAccount.id, "bankIssuanceAccount.id");
    });
}
exports.getBankAdminAccount = getBankAdminAccount;
function createLedgerAccount(client, signer, parentId) {
    return __awaiter(this, void 0, void 0, function* () {
        utils.validate(parentId, "id", utils.isValidAccountId);
        const createLedgerAccountPayload = new protobufs_1.m10.sdk.transaction.CreateLedgerAccount({
            parentId: utils.getUint8ArrayFromAccountId(parentId),
        });
        const transactionData = new protobufs_1.m10.sdk.transaction.TransactionData({
            createLedgerAccount: createLedgerAccountPayload,
        });
        const transactionRequestPayload = client.transactionRequest(transactionData);
        const response = yield client.createTransaction(signer, transactionRequestPayload);
        const accountCreated = utils.unwrap(response.accountCreated, "response.accountCreated");
        return utils.getAccountIdFromUint8Array(accountCreated);
    });
}
exports.createLedgerAccount = createLedgerAccount;
function createAccount(client, signer, accountId, name) {
    return __awaiter(this, void 0, void 0, function* () {
        utils.validate(accountId, "id", utils.isValidAccountId);
        const account = {
            id: utils.getUint8ArrayFromAccountId(accountId),
            name,
            publicName: name,
            owner: signer.getPublicKey(),
        };
        const accountPayload = new protobufs_1.m10.sdk.model.Account(account);
        const transactionData = collections.getCreateTransactionDataFromDocument(protobufs_1.m10.sdk.model.Account.encode(accountPayload).finish(), collections.Collection.Account);
        const transactionRequestPayload = client.transactionRequest(transactionData);
        const transactionResponse = yield client.createTransaction(signer, transactionRequestPayload);
        utils.checkTransactionResponse(transactionResponse);
        return accountId;
    });
}
exports.createAccount = createAccount;
//# sourceMappingURL=accounts.js.map