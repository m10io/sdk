"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.getUint8ArrayFromAccountId = exports.getAccountIdFromUint8Array = exports.isValidAccountId = void 0;
function isValidAccountId(accountId) {
    return /[0-9A-Fa-f]{6}/.test(accountId);
}
exports.isValidAccountId = isValidAccountId;
function getAccountIdFromUint8Array(uint8arr) {
    return Buffer.from(uint8arr).toString("hex");
}
exports.getAccountIdFromUint8Array = getAccountIdFromUint8Array;
function getUint8ArrayFromAccountId(accountId) {
    return Uint8Array.from(Buffer.from(accountId, "hex"));
}
exports.getUint8ArrayFromAccountId = getUint8ArrayFromAccountId;
//# sourceMappingURL=account_id.js.map