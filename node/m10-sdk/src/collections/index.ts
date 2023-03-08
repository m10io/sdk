import { google, m10 } from "../../protobufs";


export type CollectionDocument = m10.sdk.model.AccountMetadata | m10.sdk.model.AccountSet |
    m10.sdk.RoleBinding | m10.sdk.Role | m10.sdk.model.Bank;

/**
 * A struct for building a document update operation
 */
export class DocumentUpdate<D extends CollectionDocument> {

    public collection: Collection;
    public document: D;
    public mask: google.protobuf.IFieldMask;
    public merge_repeated: boolean = false;

    /**
     * Creates a new document update operation with the passed id inside of the document,
     * and a list of fields to update.
     */
    public constructor(collection: Collection, document: D, fields: string[]) {
        this.collection = collection;
        this.document = document;
        this.mask = new google.protobuf.FieldMask({ paths: fields });
    }

    public getUint8Array(): Uint8Array {
        return getUint8ArrayFromDocument(this.collection, this.document);
    }

    public toOperation(): m10.sdk.Operation {
        return new m10.sdk.Operation({
            updateDocument: new m10.sdk.Operation.UpdateDocument({
                collection: this.collection,
                primaryKey: new m10.sdk.Value({ bytesValue: this.document.id }),
                document: this.getUint8Array(),
                fieldMask: this.mask,
                mergeRepeated: this.merge_repeated,
            }),
        });
    }

    /**
     * Assembles a TransactionData object for an Update action based on the provided document and a collection name
     */
    public getTransactionData(): m10.sdk.transaction.TransactionData {
        const primaryKey = new m10.sdk.Value({ bytesValue: this.document.id });
        const updateDocument = new m10.sdk.Operation.UpdateDocument({
            collection: this.collection,
            primaryKey,
            document: this.getUint8Array(),
            fieldMask: this.mask,
        });
        const operation = new m10.sdk.Operation({ updateDocument });
        const documentOperations = new m10.sdk.DocumentOperations({ operations: [operation] });
        const transactionData = new m10.sdk.transaction.TransactionData({ documentOperations });
        return transactionData;
    }
}

export function getUint8ArrayFromDocument(collection: Collection, document: CollectionDocument): Uint8Array {
    switch (collection) {
        case Collection.AccountMetadata:
            return m10.sdk.model.AccountMetadata.encode(document as m10.sdk.model.AccountMetadata).finish();
        case Collection.AccountSet:
            return m10.sdk.model.AccountSet.encode(document as m10.sdk.model.AccountSet).finish();
        case Collection.RoleBinding:
            return m10.sdk.RoleBinding.encode(document as m10.sdk.RoleBinding).finish();
        case Collection.Role:
            return m10.sdk.Role.encode(document as m10.sdk.Role).finish();
        case Collection.Bank:
            return m10.sdk.model.Bank.encode(document as m10.sdk.model.Bank).finish();
        default:
            throw new Error(`${collection} collection does not have an encoding function`);
    }
}

export enum Collection {
    AccountMetadata = "account-metadata",
    AccountSet = "account-sets",
    LedgerAccount = "ledger-accounts",
    RoleBinding = "role-bindings",
    Role = "roles",
    Bank = "banks",
}
