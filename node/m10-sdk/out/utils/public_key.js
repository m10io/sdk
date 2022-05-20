"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.getUint8ArrayFromPublicKey = exports.getPublicKeyFromUint8Array = exports.isValidPublicKey = void 0;
function isValidPublicKey(publicKey) {
    return /^([0-9a-zA-Z+/]{4})*(([0-9a-zA-Z+/]{2}==)|([0-9a-zA-Z+/]{3}=))?$/.test(publicKey);
}
exports.isValidPublicKey = isValidPublicKey;
function getPublicKeyFromUint8Array(uint8arr) {
    return Buffer.from(uint8arr).toString("base64");
}
exports.getPublicKeyFromUint8Array = getPublicKeyFromUint8Array;
function getUint8ArrayFromPublicKey(publicKey) {
    return Uint8Array.from(Buffer.from(publicKey, "base64"));
}
exports.getUint8ArrayFromPublicKey = getUint8ArrayFromPublicKey;
//# sourceMappingURL=public_key.js.map