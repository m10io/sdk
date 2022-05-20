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
Object.defineProperty(exports, "__esModule", { value: true });
exports.TransactionSigner = exports.CryptoHasher = exports.CryptoSigner = exports.Algorithm = void 0;
const crypto = __importStar(require("crypto"));
const protobufs_1 = require("../../protobufs");
var Algorithm;
(function (Algorithm) {
    Algorithm[Algorithm["ED25519"] = 1] = "ED25519";
})(Algorithm = exports.Algorithm || (exports.Algorithm = {}));
class CryptoSigner {
    constructor(privateKeyString) {
        this.privateKey = crypto.createPrivateKey(privateKeyString);
        this.publicKey = crypto.createPublicKey(privateKeyString);
        if (this.privateKey.asymmetricKeyType === "ed25519") {
            this.algorithm = Algorithm.ED25519;
        }
    }
    sign(payload) {
        return crypto.sign(null, Buffer.from(payload), this.privateKey);
    }
    getPublicKey() {
        const publicKey = this.publicKey.export({ type: "spki", format: "der" });
        // NOTE: Remove header from buffer
        return publicKey.slice(CryptoSigner.PUBLIC_KEY_HEADER_LENGTH);
    }
    getAlgorithm() {
        return this.algorithm;
    }
    getSignature(payload) {
        return new protobufs_1.m10.sdk.Signature({
            publicKey: Uint8Array.from(this.getPublicKey()),
            signature: Uint8Array.from(this.sign(payload)),
            algorithm: this.getAlgorithm(),
        });
    }
    static generateKeyPair() {
        const { privateKey } = crypto.generateKeyPairSync("ed25519");
        return new CryptoSigner(privateKey.export({ type: "pkcs8", format: "pem" }));
    }
}
exports.CryptoSigner = CryptoSigner;
CryptoSigner.PUBLIC_KEY_HEADER_LENGTH = 12;
class CryptoHasher {
    constructor() {
        this.hasher = crypto.createHash("sha256");
    }
    hash(payload) {
        this.hasher.update(Buffer.from(payload));
        return this.hasher.digest();
    }
}
exports.CryptoHasher = CryptoHasher;
class TransactionSigner {
    static generateNonce() {
        return Math.floor(Math.random() * Number.MAX_SAFE_INTEGER);
    }
    /**
     * @param offset in milliseconds
     * @returns timestamp in microseconds
     */
    static getTimestamp(offset = TransactionSigner.DEFAULT_OFFSET) {
        return (Date.now() + offset) * TransactionSigner.TIMESTAMP_MULTIPLIER;
    }
}
exports.TransactionSigner = TransactionSigner;
TransactionSigner.TIMESTAMP_MULTIPLIER = 1000;
TransactionSigner.DEFAULT_OFFSET = 0;
//# sourceMappingURL=signer.js.map