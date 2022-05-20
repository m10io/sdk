"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.getUpdateTransactionDataFromDocument = exports.getCreateTransactionDataFromDocument = exports.Collection = exports.DocumentUpdate = void 0;
const protobufs_1 = require("../../protobufs");
const protobufs_2 = require("../../protobufs");
/**
 * A struct for building a document update operation
 */
class DocumentUpdate {
    /**
     * Creates a new document update operation with the passed id inside of the document,
     * and a list of fields to update.
     */
    constructor(document, fields) {
        this.merge_repeated = false;
        this.document = document;
        this.mask = new protobufs_1.google.protobuf.FieldMask({ paths: fields });
    }
}
exports.DocumentUpdate = DocumentUpdate;
var Collection;
(function (Collection) {
    Collection["Account"] = "accounts";
    Collection["AccountSet"] = "account-sets";
    Collection["RoleBinding"] = "role-bindings";
    Collection["Role"] = "roles";
})(Collection = exports.Collection || (exports.Collection = {}));
/**
 * Assembles a TransactionData object for a Create action based on the provided document and a collection name
 */
function getCreateTransactionDataFromDocument(document, collection) {
    const insertDocument = new protobufs_2.m10.sdk.Operation.InsertDocument({ collection, document });
    const operation = new protobufs_2.m10.sdk.Operation({ insertDocument });
    const documentOperations = new protobufs_2.m10.sdk.DocumentOperations({ operations: [operation] });
    const transactionData = new protobufs_2.m10.sdk.transaction.TransactionData({ documentOperations });
    return transactionData;
}
exports.getCreateTransactionDataFromDocument = getCreateTransactionDataFromDocument;
/**
 * Assembles a TransactionData object for an Update action based on the provided document and a collection name
 */
function getUpdateTransactionDataFromDocument(document, id, documentUpdate, collection) {
    const primaryKey = new protobufs_2.m10.sdk.Value({ bytesValue: id });
    const updateDocument = new protobufs_2.m10.sdk.Operation.UpdateDocument({
        collection, primaryKey, document, fieldMask: documentUpdate.mask,
    });
    const operation = new protobufs_2.m10.sdk.Operation({ updateDocument });
    const documentOperations = new protobufs_2.m10.sdk.DocumentOperations({ operations: [operation] });
    const transactionData = new protobufs_2.m10.sdk.transaction.TransactionData({ documentOperations });
    return transactionData;
}
exports.getUpdateTransactionDataFromDocument = getUpdateTransactionDataFromDocument;
//# sourceMappingURL=index.js.map