import type { m10 } from "../../protobufs";
import { M10Error } from "../error";
import { DocumentId } from "../ids/document_id";
import { PublicKey } from "../ids/public_key";
import { unwrap } from "../utils";


export interface IRoleBinding {
    id: DocumentId;
    owner: PublicKey;
    name: string;
    roleId: DocumentId;
    subjects: PublicKey[];
    expressions: m10.sdk.IExpression[];
    isUniversal: boolean;
}

export class RoleBinding implements IRoleBinding {
    public id: DocumentId;
    public owner: PublicKey;
    public name: string;
    public roleId: DocumentId;
    public subjects: PublicKey[];
    public expressions: m10.sdk.IExpression[];
    public isUniversal: boolean;


    public constructor(properties: IRoleBinding) {
        this.id = properties.id;
        this.owner = properties.owner;
        this.name = properties.name;
        this.roleId = properties.roleId;
        this.subjects = properties.subjects;
        this.expressions = properties.expressions;
        this.isUniversal = properties.isUniversal;
    }


    /**
     * @throws  {M10Error.InvalidTransaction}   if any of the required fields are missing
     */
    public static tryFrom(roleBinding: m10.sdk.IRoleBinding): RoleBinding {
        return new RoleBinding({
            id: DocumentId.fromUint8Array("role-binding", unwrap(roleBinding.id, M10Error.InvalidTransaction())),
            owner: PublicKey.fromUint8Array(unwrap(roleBinding.owner, M10Error.InvalidTransaction())),
            name: unwrap(roleBinding.name, M10Error.InvalidTransaction()),
            roleId: DocumentId.fromUint8Array("role", unwrap(roleBinding.role, M10Error.InvalidTransaction())),
            subjects: unwrap(roleBinding.subjects, M10Error.InvalidTransaction()).map(PublicKey.fromUint8Array),
            expressions: unwrap(roleBinding.expressions, M10Error.InvalidTransaction()),
            isUniversal: unwrap(roleBinding.isUniversal, M10Error.InvalidTransaction()),
        });
    }
}
