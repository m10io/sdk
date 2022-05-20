"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.isSome = exports.unwrapOr = exports.unwrap = exports.arrayIsNotEmpty = void 0;
function arrayIsNotEmpty(array) {
    return (Array.isArray(array) && array.length > 0);
}
exports.arrayIsNotEmpty = arrayIsNotEmpty;
function unwrap(value, name) {
    if (!isSome(value)) {
        throw new Error(`Error: Unexpectedly found None while unwrapping an Option value [${name}]`);
    }
    return value;
}
exports.unwrap = unwrap;
function unwrapOr(value, defaultValue) {
    return isSome(value) ? value : defaultValue;
}
exports.unwrapOr = unwrapOr;
function isSome(value) {
    return (value !== null) && (value !== undefined);
}
exports.isSome = isSome;
//# sourceMappingURL=common.js.map