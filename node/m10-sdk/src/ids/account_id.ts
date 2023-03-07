import { M10Error } from "../error";

import { _validate, HEX_REGEX } from ".";


export class AccountId {

    private value: Uint8Array;

    /**
     * @param value - hex `string` (throws exception otherwise)
     * @throws  {M10Error.InvalidAccountId}     if invalid account id was used
     */
    public constructor(value: string) {
        _validate(value, HEX_REGEX.test.bind(HEX_REGEX), M10Error.InvalidAccountId());
        this.value = Uint8Array.from(Buffer.from(value, "hex"));
    }

    public toUint8Array(): Uint8Array {
        return this.value;
    }

    public toString(): string {
        return Buffer.from(this.value).toString("hex");
    }

    /**
     * @throws  {M10Error.InvalidAccountId}     if invalid account id was used
     */
    public static fromUint8Array(value: Uint8Array): AccountId {
        return new AccountId(Buffer.from(value).toString("hex"));
    }
}
