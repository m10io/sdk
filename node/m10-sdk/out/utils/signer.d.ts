/// <reference types="node" />
import { m10 } from "../../protobufs";
export declare enum Algorithm {
    ED25519 = 1
}
export declare class CryptoSigner {
    private privateKey;
    private publicKey;
    private algorithm?;
    private static readonly PUBLIC_KEY_HEADER_LENGTH;
    constructor(privateKeyString: string | Buffer);
    sign(payload: WithImplicitCoercion<ArrayBuffer | SharedArrayBuffer>): Buffer;
    getPublicKey(): Buffer;
    getAlgorithm(): Option<Algorithm>;
    getSignature(payload: WithImplicitCoercion<ArrayBuffer | SharedArrayBuffer>): m10.sdk.Signature;
    static generateKeyPair(): CryptoSigner;
}
export declare class CryptoHasher {
    private hasher;
    constructor();
    hash(payload: WithImplicitCoercion<ArrayBuffer | SharedArrayBuffer>): Buffer;
}
export declare class TransactionSigner {
    private static readonly TIMESTAMP_MULTIPLIER;
    private static readonly DEFAULT_OFFSET;
    static generateNonce(): number;
    /**
     * @param offset in milliseconds
     * @returns timestamp in microseconds
     */
    static getTimestamp(offset?: number): number;
}
