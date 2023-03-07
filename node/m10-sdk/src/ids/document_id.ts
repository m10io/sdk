import * as uuid from "uuid";

import { isSome } from "../utils/common";

import { validate } from ".";


type DocumentType = "role" | "role-binding" | "account-set" | "bank";


export class DocumentId {

    private value: Uint8Array;
    private type: DocumentType;

    /**
     * @param value - uuid `string` (throws exception otherwise)
     */
    public constructor(type: DocumentType, value?: string) {
        this.type = type;

        switch (this.type) {
            case "role":
            case "role-binding":
            case "account-set":
                if (isSome(value)) {
                    validate(value, "uuid", uuid.validate);
                    this.value = Uint8Array.from(uuid.parse(value));
                }
                else {
                    this.value = Uint8Array.from(uuid.parse(uuid.v4()));
                }
                break;
            case "bank":
                this.value = (
                    isSome(value)
                        ? Uint8Array.from(Buffer.from(value))
                        : Uint8Array.from(Buffer.from(uuid.v4()))
                );
                break;
        }
    }

    public toUint8Array(): Uint8Array {
        return this.value;
    }

    public toString(): string {
        switch (this.type) {
            case "role":
            case "role-binding":
            case "account-set":
                return uuid.stringify(this.value);
            case "bank":
                return Buffer.from(this.value).toString();
        }
    }

    public static fromUint8Array(type: DocumentType, value: Uint8Array): DocumentId {
        switch (type) {
            case "role":
            case "role-binding":
            case "account-set":
                return new DocumentId(type, uuid.stringify(value));
            case "bank":
                return new DocumentId(type, Buffer.from(value).toString());
        }
    }
}
