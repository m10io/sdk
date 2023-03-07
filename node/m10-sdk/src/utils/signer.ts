import * as crypto from "crypto";

import { m10 } from "../../protobufs";
import { PublicKey } from "../ids";


export enum Algorithm {
    ED25519 = 1,
}

export class CryptoSigner {

    private privateKey: crypto.KeyObject;
    private publicKey: crypto.KeyObject;

    private algorithm?: Algorithm;

    private static readonly PUBLIC_KEY_HEADER_LENGTH: number = 12;

    private static readonly BEGIN_PRIV_KEY_PREFIX: number = 8;
    private static readonly END_PRIV_KEY_PREFIX: number = 56;


    public constructor(privateKeyString: string | Buffer) {
        this.privateKey = crypto.createPrivateKey(privateKeyString);
        this.publicKey = crypto.createPublicKey(privateKeyString);

        if (this.privateKey.asymmetricKeyType === "ed25519") {
            this.algorithm = Algorithm.ED25519;
        }
    }

    public sign(payload: WithImplicitCoercion<ArrayBuffer | SharedArrayBuffer>): Buffer {
        return crypto.sign(null, Buffer.from(payload), this.privateKey);
    }

    public getPublicKey(): PublicKey {
        const publicKey = this.publicKey.export({ type: "spki", format: "der" });
        // NOTE: Remove header from buffer
        return new PublicKey(publicKey.slice(CryptoSigner.PUBLIC_KEY_HEADER_LENGTH));
    }

    public getAlgorithm(): Option<Algorithm> {
        return this.algorithm;
    }

    public getSignature(payload: WithImplicitCoercion<ArrayBuffer | SharedArrayBuffer>): m10.sdk.Signature {
        return new m10.sdk.Signature({
            publicKey: this.getPublicKey().toUint8Array(),
            signature: Uint8Array.from(this.sign(payload)),
            algorithm: this.getAlgorithm(),
        });
    }

    /**
    * Converts a PKCS#8 v2 key to the v1 format with only the PRV and version.
    */
    private static convertPkcs8V2KeyToV1(pkcs8v2Key: string): string {
        const v1header = "MC4CAQAw";
        const keyWithoutV2HeaderAndPublicKey = pkcs8v2Key
            .substring(CryptoSigner.BEGIN_PRIV_KEY_PREFIX, CryptoSigner.BEGIN_PRIV_KEY_PREFIX + CryptoSigner.END_PRIV_KEY_PREFIX);

        return `${v1header}${keyWithoutV2HeaderAndPublicKey}`;
    }

    private static getWrappedPrivateKey(privateKey: string): string {
        const KEY_HEADER = "-----BEGIN PRIVATE KEY-----\n";
        const KEY_FOOTER = "-----END PRIVATE KEY-----\n";
        return `${KEY_HEADER}${privateKey}\n${KEY_FOOTER}`;
    }

    public static generateKeyPair(): CryptoSigner {
        const { privateKey } = crypto.generateKeyPairSync("ed25519");
        return new CryptoSigner(privateKey.export({ type: "pkcs8", format: "pem" }));
    }

    /**
    * Creates a `CryptoSigner` from PKCS#8 v2 key
    */
    public static getSignerFromPkcs8V2(pkcs8V2Key: string): CryptoSigner {
        const pkcs8V1 = CryptoSigner.convertPkcs8V2KeyToV1(pkcs8V2Key);
        return CryptoSigner.getSignerFromPkcs8V1(pkcs8V1);
    }

    /**
    * Creates a `CryptoSigner` from PKCS#8 v1 key
    */
    public static getSignerFromPkcs8V1(pkcs8V1: string): CryptoSigner {
        const wrappedKey = CryptoSigner.getWrappedPrivateKey(pkcs8V1);
        return new CryptoSigner(wrappedKey);
    }
}

export class CryptoHasher {

    private hasher: crypto.Hash;

    public constructor() {
        this.hasher = crypto.createHash("sha256");
    }

    public hash(payload: WithImplicitCoercion<ArrayBuffer | SharedArrayBuffer>): Buffer {
        this.hasher.update(Buffer.from(payload));
        return this.hasher.digest();
    }
}



export class TransactionSigner {

    private static readonly TIMESTAMP_MULTIPLIER: number = 1000;
    private static readonly DEFAULT_OFFSET: number = 0;

    public static generateNonce(): number {
        return Math.floor(Math.random() * Number.MAX_SAFE_INTEGER);
    }

    /**
     * @param offset in milliseconds
     * @returns timestamp in microseconds
     */
    public static getTimestamp(offset: number = TransactionSigner.DEFAULT_OFFSET): number {
        return (Date.now() + offset) * TransactionSigner.TIMESTAMP_MULTIPLIER;
    }
}
