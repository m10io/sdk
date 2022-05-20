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
exports.accounts = exports.roleBindings = exports.createUser = void 0;
const uuid = __importStar(require("uuid"));
const protobufs_1 = require("../../protobufs");
const utils_1 = require("../utils");
const __1 = require("..");
// -----------------------------------------------------------------------------
// User
// -----------------------------------------------------------------------------
function createUser(client) {
    return __awaiter(this, void 0, void 0, function* () {
        const signer = utils_1.CryptoSigner.generateKeyPair();
        const userId = uuid.v4();
        const user = {
            owner: Uint8Array.from(signer.getPublicKey()),
            id: (0, utils_1.getUint8ArrayFromDocumentId)(userId),
        };
        const userPayload = new protobufs_1.m10.sdk.model.AccountSet(user);
        const transactionData = __1.collections.getCreateTransactionDataFromDocument(protobufs_1.m10.sdk.model.AccountSet.encode(userPayload).finish(), __1.collections.Collection.AccountSet);
        const transactionRequestPayload = client.transactionRequest(transactionData);
        const transactionResponse = yield client.createTransaction(signer, transactionRequestPayload);
        (0, utils_1.checkTransactionResponse)(transactionResponse);
        return [signer, userId];
    });
}
exports.createUser = createUser;
// -------------------------------------------------------------------------
// Exports
// -------------------------------------------------------------------------
exports.roleBindings = __importStar(require("./role-bindings"));
exports.accounts = __importStar(require("./accounts"));
//# sourceMappingURL=index.js.map