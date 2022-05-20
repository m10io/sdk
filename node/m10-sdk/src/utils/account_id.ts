
// NOTE: Represented by a hex string
export type AccountId = string;


export function isValidAccountId(accountId: AccountId): boolean {
    return /[0-9A-Fa-f]{6}/.test(accountId);
}

export function getAccountIdFromUint8Array(uint8arr: Uint8Array): AccountId {
    return Buffer.from(uint8arr).toString("hex");
}

export function getUint8ArrayFromAccountId(accountId: AccountId): Uint8Array {
    return Uint8Array.from(Buffer.from(accountId, "hex"));
}
