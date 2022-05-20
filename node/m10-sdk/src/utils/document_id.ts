import * as uuid from "uuid";


export type DocumentId = string;


export function getDocumentIdFromUint8Array(uint8arr: Uint8Array): DocumentId {
    return uuid.stringify(uint8arr);
}

export function getUint8ArrayFromDocumentId(documentId: DocumentId): Uint8Array {
    return Uint8Array.from(uuid.parse(documentId));
}
