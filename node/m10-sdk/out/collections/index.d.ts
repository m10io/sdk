import { google } from "../../protobufs";
import { m10 } from "../../protobufs";
/**
 * A struct for building a document update operation
 */
export declare class DocumentUpdate<D> {
    document: D;
    mask: google.protobuf.IFieldMask;
    merge_repeated: boolean;
    /**
     * Creates a new document update operation with the passed id inside of the document,
     * and a list of fields to update.
     */
    constructor(document: D, fields: string[]);
}
export declare enum Collection {
    Account = "accounts",
    AccountSet = "account-sets",
    RoleBinding = "role-bindings",
    Role = "roles"
}
/**
 * Assembles a TransactionData object for a Create action based on the provided document and a collection name
 */
export declare function getCreateTransactionDataFromDocument(document: Uint8Array, collection: Collection): m10.sdk.transaction.TransactionData;
/**
 * Assembles a TransactionData object for an Update action based on the provided document and a collection name
 */
export declare function getUpdateTransactionDataFromDocument<T>(document: Uint8Array, id: Uint8Array, documentUpdate: DocumentUpdate<T>, collection: Collection): m10.sdk.transaction.TransactionData;
