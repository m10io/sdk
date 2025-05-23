// eslint-disable-next-line import/no-nodejs-modules
import { Buffer } from "buffer";

export class PublicKey {

    private value: Uint8Array;

    public constructor(value: string | Buffer) {
        if (typeof value === "string") {
            this.value = Uint8Array.from(Buffer.from(value, "base64"));
        }
        else {
            this.value = Uint8Array.from(value);
        }
    }

    public toUint8Array(): Uint8Array {
        return this.value;
    }

    public toString(): string {
        return Buffer.from(this.value).toString("base64");
    }

    public static fromUint8Array(value: Uint8Array): PublicKey {
        return new PublicKey(Buffer.from(value).toString("base64"));
    }
}
