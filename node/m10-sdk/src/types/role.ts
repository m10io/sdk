import type { m10 } from "../../protobufs";
import { M10Error } from "../error";
import { DocumentId } from "../ids/document_id";
import { PublicKey } from "../ids/public_key";
import { unwrap } from "../utils";


export interface IRole {
    id: DocumentId;
    owner: PublicKey;
    name: string;
    rules: m10.sdk.IRule[];
}

export class Role implements IRole {
    public id: DocumentId;
    public owner: PublicKey;
    public name: string;
    public rules: m10.sdk.IRule[];


    public constructor(properties: IRole) {
        this.id = properties.id;
        this.owner = properties.owner;
        this.name = properties.name;
        this.rules = properties.rules;
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any of the required fields are missing
     */
    public static tryFrom(role: m10.sdk.IRole): Role {
        return new Role({
            id: DocumentId.fromUint8Array("role", unwrap(role.id, M10Error.InvalidTransaction())),
            owner: PublicKey.fromUint8Array(unwrap(role.owner, M10Error.InvalidTransaction())),
            name: unwrap(role.name, M10Error.InvalidTransaction()),
            rules: unwrap(role.rules, M10Error.InvalidTransaction()),
        });
    }
}
