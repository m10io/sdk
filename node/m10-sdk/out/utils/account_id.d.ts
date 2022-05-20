export declare type AccountId = string;
export declare function isValidAccountId(accountId: AccountId): boolean;
export declare function getAccountIdFromUint8Array(uint8arr: Uint8Array): AccountId;
export declare function getUint8ArrayFromAccountId(accountId: AccountId): Uint8Array;
