import { getPublicKeyAsync, signAsync as edSign, utils } from "@noble/ed25519";
import * as asn1js from "asn1js";
// eslint-disable-next-line import/no-nodejs-modules
import { Buffer } from "buffer";
import {
    AlgorithmIdentifier,
    PrivateKeyInfo,
} from "pkijs";

import { PublicKey } from "../ids";
import * as sdkTransaction from "../protobufs/sdk/transaction/transaction";

function pemToArrayBuffer(pem: string): Uint8Array {
    const b64 = pem
        .replace(/-----BEGIN .*-----/, "")
        .replace(/-----END .*-----/, "")
        .replace(/\s+/g, "");

    return new Uint8Array(Buffer.from(b64, "base64"));
}

function arrayBufferToPem(buffer: ArrayBuffer, label: string): string {
    const b64 = Buffer.from(buffer).toString("base64");
    const lines = b64.match(/.{1,64}/g)?.join("\n") ?? "";
    return `-----BEGIN ${label}-----\n${lines}\n-----END ${label}-----`;
}

export enum Algorithm {
  ED25519 = 1,
}

export class CryptoSigner {
    private seed: Uint8Array;
    private pub: Uint8Array;
    private algorithm = sdkTransaction.Signature_Algorithm.ED25519;

    public constructor(seed: Uint8Array, pub: Uint8Array) {
        this.seed = seed;
        this.pub = pub;
    }

    public static async generateKeyPair(): Promise<CryptoSigner> {
        const seed = utils.randomPrivateKey();
        const pub = await getPublicKeyAsync(seed);
        const signer = new CryptoSigner(seed, pub);
        return signer;
    }

    public static async fromSeed(seed: Uint8Array): Promise<CryptoSigner> {
        const pub = await getPublicKeyAsync(seed);
        return new CryptoSigner(seed, pub);
    }

    public getPublicKey(): PublicKey {
        return new PublicKey(Buffer.from(this.pub));
    }

    public getAlgorithm(): sdkTransaction.Signature_Algorithm {
        return this.algorithm;
    }

    public async getSignature(
        payload: Uint8Array<ArrayBufferLike>,
    ): Promise<sdkTransaction.Signature> {
        return sdkTransaction.Signature.create({
            publicKey: this.getPublicKey().toUint8Array(),
            signature: Uint8Array.from(await this.sign(payload)),
            algorithm: this.algorithm,
        });
    }

    public async sign(payload: ArrayBuffer | Uint8Array): Promise<Buffer> {
        const data = payload instanceof Uint8Array ? payload : new Uint8Array(payload);
        const sig = await edSign(data, this.seed);
        return Buffer.from(sig);
    }

    public static ed25519BytesFromPkcs8Pem(pem: string): Uint8Array {
        const buffer = pemToArrayBuffer(pem);

        const asn1 = asn1js.fromBER(buffer);

        // eslint-disable-next-line @typescript-eslint/no-magic-numbers
        if (asn1.offset === -1) throw new Error("Invalid ASN.1 format");

        // eslint-disable-next-line @typescript-eslint/no-magic-numbers
        const octetString = asn1.result instanceof asn1js.Sequence && asn1.result.valueBlock.value[2];

        if (!octetString || !(octetString instanceof asn1js.OctetString)) throw new Error("Invalid ASN.1 format");

        // eslint-disable-next-line @typescript-eslint/no-magic-numbers
        return Buffer.from(octetString.valueBlock.valueHexView.slice(2));
    }

    /**
     * Import a PKCS#8 PEM (v1) private key.
     * this function should support format like "MC4CAQAwBQYDK2VwBCIEIIpBmydRGZPDKGaE55+lsCHVHamx/NqKDQSjsRzB98nJ"
     * or "MFMCAQEwBQYDK2VwBCIEIHyr+m5Z4gy9JxoMdgrrX/EE8uhzkj3ztWx28zJxpStqoSMDIQAAIwpWR4i34vnPf3GTlge6ONw3tsuGer5QiQsGXKY0zg=="
     * and also both wrapped by ASN.1 tags
     */
    public static async fromPkcs8Pem(b64Pem: string): Promise<CryptoSigner> {
        const seed = CryptoSigner.ed25519BytesFromPkcs8Pem(b64Pem);
        const pub  = await getPublicKeyAsync(seed);
        return new CryptoSigner(seed, pub);
    }

    public async toPkcs8Pem(): Promise<string> {
        const privateKeyInfo = new PrivateKeyInfo({
            version: 0,
            privateKeyAlgorithm: new AlgorithmIdentifier({
                algorithmId: "1.3.101.112", // OID for Ed25519
            }),
            privateKey: new asn1js.OctetString({
                valueHex: new asn1js.OctetString({ valueHex: this.seed }).toBER(),
            }),
        });

        const schema = privateKeyInfo.toSchema();
        const pkcs8Raw = schema.toBER(false);
        return arrayBufferToPem(pkcs8Raw, "PRIVATE KEY");
    }
}

export class TransactionSigner {
    private static readonly TIMESTAMP_MULTIPLIER = 1000;
    private static readonly DEFAULT_OFFSET = 0;

    public static generateNonce(): number {
        return Math.floor(Math.random() * Number.MAX_SAFE_INTEGER);
    }

    /** @param offset  milliseconds → returns microseconds */
    public static getTimestamp(offset = TransactionSigner.DEFAULT_OFFSET): number {
        return (
            (Date.now() + offset) * TransactionSigner.TIMESTAMP_MULTIPLIER
        );
    }
}
