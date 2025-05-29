// eslint-disable-next-line import/no-nodejs-modules
import { Buffer } from "buffer";

export class AccountId {
    private value: Uint8Array;

    private constructor(value: Uint8Array) {
        this.value = value;
    }

    public get bytes(): Uint8Array {
        return this.value;
    }

    public get hex(): string {
        return Buffer.from(this.value).toString("hex");
    }

    public static fromBytes(value: Uint8Array): AccountId {
        return new AccountId(Buffer.from(value));
    }

    public static fromHex(value: string): AccountId {
        return new AccountId(Buffer.from(value, "hex"));
    }
}
