export declare type PublicKey = string;
export declare function isValidPublicKey(publicKey: PublicKey): boolean;
export declare function getPublicKeyFromUint8Array(uint8arr: Uint8Array): PublicKey;
export declare function getUint8ArrayFromPublicKey(publicKey: PublicKey): Uint8Array;
