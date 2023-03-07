import { BASE64_REGEX, validate } from ".";


export class PublicKey {

    private value: Uint8Array;

    public constructor(value: string | Buffer) {
        if (typeof value === "string") {
            validate(value, "base64", BASE64_REGEX.test.bind(BASE64_REGEX));
            this.value = Uint8Array.from(Buffer.from(value, "base64"));
        }
        else {
            validate(Buffer.from(value).toString("base64"), "base64", BASE64_REGEX.test.bind(BASE64_REGEX));
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
