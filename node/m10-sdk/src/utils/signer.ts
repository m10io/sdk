import * as crypto from "crypto";

import { m10 } from "../../protobufs";


export enum Algorithm {
    ED25519 = 1,
}

export class CryptoSigner {

    private privateKey: crypto.KeyObject;
    private publicKey: crypto.KeyObject;

    private algorithm?: Algorithm;

    private static readonly PUBLIC_KEY_HEADER_LENGTH: number = 12;


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

    public getPublicKey(): Buffer {
        const publicKey = this.publicKey.export({ type: "spki", format: "der" });
        // NOTE: Remove header from buffer
        return publicKey.slice(CryptoSigner.PUBLIC_KEY_HEADER_LENGTH);
    }

    public getAlgorithm(): Option<Algorithm> {
        return this.algorithm;
    }

    public getSignature(payload: WithImplicitCoercion<ArrayBuffer | SharedArrayBuffer>): m10.sdk.Signature {
        return new m10.sdk.Signature({
            publicKey: Uint8Array.from(this.getPublicKey()),
            signature: Uint8Array.from(this.sign(payload)),
            algorithm: this.getAlgorithm(),
        });
    }

    public static generateKeyPair(): CryptoSigner {
        const { privateKey } = crypto.generateKeyPairSync("ed25519");
        return new CryptoSigner(privateKey.export({ type: "pkcs8", format: "pem" }));
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
