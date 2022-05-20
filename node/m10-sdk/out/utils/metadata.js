"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.convertMemoToAny = void 0;
const protobufs_1 = require("../../protobufs");
function convertMemoToAny(memo) {
    return new protobufs_1.google.protobuf.Any({
        type_url: "m10.sdk.metadata.Memo",
        value: protobufs_1.m10.sdk.metadata.Memo.encode(memo).finish(),
    });
}
exports.convertMemoToAny = convertMemoToAny;
//# sourceMappingURL=metadata.js.map