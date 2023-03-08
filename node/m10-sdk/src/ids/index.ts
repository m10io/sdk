import type { M10Error } from "../error";


export const BASE64_REGEX: RegExp = /^([0-9a-zA-Z+/]{4})*(([0-9a-zA-Z+/]{2}==)|([0-9a-zA-Z+/]{3}=))?$/;
export const HEX_REGEX: RegExp = /[0-9A-Fa-f]{6}/;


/**
 * Throws an exception if the value is not valid
 */
export function validate<T>(value: T, name: string, verifier: (value: T) => boolean): T {
    if (verifier(value)) {
        return value;
    }
    else {
        throw new Error(`Invalid ${name}`);
    }
}

/**
 * @throws  {M10Error}  if value is not valid
 */
export function _validate<T>(value: T, verifier: (value: T) => boolean, error: M10Error): T {
    if (verifier(value)) {
        return value;
    }
    else {
        throw error;
    }
}

export type TxId = LongNumber;

export * from "./account_id";
export * from "./document_id";
export * from "./public_key";
