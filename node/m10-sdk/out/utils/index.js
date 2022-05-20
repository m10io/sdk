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
var __exportStar = (this && this.__exportStar) || function(m, exports) {
    for (var p in m) if (p !== "default" && !Object.prototype.hasOwnProperty.call(exports, p)) __createBinding(exports, m, p);
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.getPrivateKey = exports.checkTransactionResponse = exports.validate = exports.getRPCImplStream = exports.getRPCImpl = void 0;
const grpc = __importStar(require("@grpc/grpc-js"));
const common_1 = require("./common");
function getRPCImpl(ledgerUrl, credentials, serviceName) {
    const Client = grpc.makeGenericClientConstructor({}, serviceName);
    const client = new Client(ledgerUrl, credentials);
    return (method, requestData, callback) => {
        client.makeUnaryRequest(`/${serviceName}/${method.name}`, arg => Buffer.from(arg), arg => arg, requestData, callback);
    };
}
exports.getRPCImpl = getRPCImpl;
function getRPCImplStream(ledgerUrl, credentials, serviceName) {
    const Client = grpc.makeGenericClientConstructor({}, serviceName);
    const client = new Client(ledgerUrl, credentials);
    return (method, requestData) => {
        client.makeServerStreamRequest(`/${serviceName}/${method.name}`, arg => Buffer.from(arg), arg => arg, requestData);
    };
}
exports.getRPCImplStream = getRPCImplStream;
/**
 * Throws an exception if the value is not valid
 */
function validate(value, name, verifier) {
    if (verifier(value)) {
        return value;
    }
    else {
        throw new Error(`Invalid ${name}`);
    }
}
exports.validate = validate;
/**
 * Throws an exception if the transaction contains an error
 */
function checkTransactionResponse(response) {
    if ((0, common_1.isSome)(response.error)) {
        throw new Error(`TransactionError: ${response.error.message}`);
    }
}
exports.checkTransactionResponse = checkTransactionResponse;
function getPrivateKey(privateKey) {
    const KEY_HEADER = "-----BEGIN PRIVATE KEY-----\n";
    const KEY_FOOTER = "-----END PRIVATE KEY-----\n";
    return `${KEY_HEADER}${privateKey}\n${KEY_FOOTER}`;
}
exports.getPrivateKey = getPrivateKey;
// -------------------------------------------------------------------------
// Exports
// -------------------------------------------------------------------------
__exportStar(require("./signer"), exports);
__exportStar(require("./metadata"), exports);
__exportStar(require("./account_id"), exports);
__exportStar(require("./document_id"), exports);
__exportStar(require("./public_key"), exports);
__exportStar(require("./common"), exports);
//# sourceMappingURL=index.js.map