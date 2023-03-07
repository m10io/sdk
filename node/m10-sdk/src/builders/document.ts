import { m10 } from "../../protobufs";
import type { Collection, CollectionDocument, DocumentUpdate } from "../collections";
import { getUint8ArrayFromDocument } from "../collections";


export class DocumentBuilder {
    private _docs: m10.sdk.Operation[];


    public constructor() {
        this._docs = [];
    }

    public insert<D extends CollectionDocument>(collection: Collection, document: D): DocumentBuilder {
        this._docs.push(
            new m10.sdk.Operation({
                insertDocument: new m10.sdk.Operation.InsertDocument({
                    collection: collection,
                    document: getUint8ArrayFromDocument(collection, document),
                }),
            }),
        );
        return this;
    }

    public insertCustom(collection: Collection, document: Uint8Array): DocumentBuilder {
        this._docs.push(
            new m10.sdk.Operation({
                insertDocument: new m10.sdk.Operation.InsertDocument({
                    collection: collection,
                    document: document,
                }),
            }),
        );
        return this;
    }

    public delete<D extends CollectionDocument>(collection: Collection, document: D): DocumentBuilder {
        this._docs.push(
            new m10.sdk.Operation({
                deleteDocument: new m10.sdk.Operation.DeleteDocument({
                    collection: collection,
                    primaryKey: new m10.sdk.Value({ bytesValue: document.id }),
                }),
            }),
        );
        return this;
    }

    public deleteCustom(collection: Collection, id: Uint8Array): DocumentBuilder {
        this._docs.push(
            new m10.sdk.Operation({
                deleteDocument: new m10.sdk.Operation.DeleteDocument({
                    collection: collection,
                    primaryKey: new m10.sdk.Value({ bytesValue: id }),
                }),
            }),
        );
        return this;
    }

    public update<D extends CollectionDocument>(update: DocumentUpdate<D>): DocumentBuilder {
        this._docs.push(update.toOperation());
        return this;
    }


    public toDocumentOperations(): m10.sdk.DocumentOperations {
        return new m10.sdk.DocumentOperations({ operations: this._docs });
    }
}
