import { google } from "../../protobufs";
import { m10 } from "../../protobufs";

/**
 * A struct for building a document update operation
 */
export class DocumentUpdate<D> {

    public document: D;
    public mask: google.protobuf.IFieldMask;
    public merge_repeated: boolean = false;

    /**
     * Creates a new document update operation with the passed id inside of the document,
     * and a list of fields to update.
     */
    public constructor(document: D, fields: string[]) {
        this.document = document;
        this.mask = new google.protobuf.FieldMask({ paths: fields });
    }
}


export enum Collection {
    Account = "accounts",
    AccountSet = "account-sets",
    RoleBinding = "role-bindings",
    Role = "roles",
}

/**
 * Assembles a TransactionData object for a Create action based on the provided document and a collection name
 */
export function getCreateTransactionDataFromDocument(
    document: Uint8Array,
    collection: Collection,
): m10.sdk.transaction.TransactionData {

    const insertDocument = new m10.sdk.Operation.InsertDocument({ collection, document });
    const operation = new m10.sdk.Operation({ insertDocument });
    const documentOperations = new m10.sdk.DocumentOperations({ operations: [ operation ] });
    const transactionData = new m10.sdk.transaction.TransactionData({ documentOperations });
    return transactionData;
}

/**
 * Assembles a TransactionData object for an Update action based on the provided document and a collection name
 */
export function getUpdateTransactionDataFromDocument<T>(
    document: Uint8Array,
    id: Uint8Array,
    documentUpdate: DocumentUpdate<T>,
    collection: Collection,
): m10.sdk.transaction.TransactionData {
    const primaryKey = new m10.sdk.Value({ bytesValue: id });
    const updateDocument = new m10.sdk.Operation.UpdateDocument({
        collection, primaryKey, document, fieldMask: documentUpdate.mask,
    });
    const operation = new m10.sdk.Operation({ updateDocument });
    const documentOperations = new m10.sdk.DocumentOperations({ operations: [ operation ] });
    const transactionData = new m10.sdk.transaction.TransactionData({ documentOperations });
    return transactionData;
}

