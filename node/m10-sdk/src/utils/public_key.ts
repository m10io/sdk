
// NOTE: Represented by a base64 string
export type PublicKey = string;


export function isValidPublicKey(publicKey: PublicKey): boolean {
    return /^([0-9a-zA-Z+/]{4})*(([0-9a-zA-Z+/]{2}==)|([0-9a-zA-Z+/]{3}=))?$/.test(publicKey);
}

export function getPublicKeyFromUint8Array(uint8arr: Uint8Array): PublicKey {
    return Buffer.from(uint8arr).toString("base64");
}

export function getUint8ArrayFromPublicKey(publicKey: PublicKey): Uint8Array {
    return Uint8Array.from(Buffer.from(publicKey, "base64"));
}
