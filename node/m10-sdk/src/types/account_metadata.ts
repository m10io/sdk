import type { m10 } from "../../protobufs";
import { M10Error } from "../error";
import { AccountId } from "../ids/account_id";
import { PublicKey } from "../ids/public_key";
import { unwrap } from "../utils";


export interface IAccountMetadata {
    id: AccountId;
    owner: PublicKey;
    profileImageUrl: string;
    name: string;
    publicName: string;
}

export class AccountMetadata implements IAccountMetadata {
    public id: AccountId;
    public owner: PublicKey;
    public profileImageUrl: string;
    public name: string;
    public publicName: string;


    public constructor(properties: IAccountMetadata) {
        this.id = properties.id;
        this.owner = properties.owner;
        this.profileImageUrl = properties.profileImageUrl;
        this.name = properties.name;
        this.publicName = properties.publicName;
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any of the required fields are missing
     */
    public static tryFrom(accountMetadata: m10.sdk.model.IAccountMetadata): AccountMetadata {
        return new AccountMetadata({
            id: AccountId.fromUint8Array(unwrap(accountMetadata.id, M10Error.InvalidTransaction())),
            owner: PublicKey.fromUint8Array(unwrap(accountMetadata.owner, M10Error.InvalidTransaction())),
            profileImageUrl: unwrap(accountMetadata.profileImageUrl, M10Error.InvalidTransaction()),
            name: unwrap(accountMetadata.name, M10Error.InvalidTransaction()),
            publicName: unwrap(accountMetadata.publicName, M10Error.InvalidTransaction()),
        });
    }
}
