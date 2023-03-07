import type { m10 } from "../../protobufs";
import { M10Error } from "../error";
import { AccountId } from "../ids/account_id";
import { DocumentId } from "../ids/document_id";
import { PublicKey } from "../ids/public_key";
import { unwrap } from "../utils";


export interface IAccountSet {
    id: DocumentId;
    owner: PublicKey;
    accounts: AccountId[];
}

export class AccountSet implements IAccountSet {
    public id: DocumentId;
    public owner: PublicKey;
    public accounts: AccountId[];


    public constructor(properties: IAccountSet) {
        this.id = properties.id;
        this.owner = properties.owner;
        this.accounts = properties.accounts;
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any of the required fields are missing
     * @throws  {M10Error.InvalidAccountId}     if invalid account id was used
     */
    public static tryFrom(accountSet: m10.sdk.model.IAccountSet): AccountSet {
        return new AccountSet({
            id: DocumentId.fromUint8Array("account-set", unwrap(accountSet.id, M10Error.InvalidTransaction())),
            owner: PublicKey.fromUint8Array(unwrap(accountSet.owner, M10Error.InvalidTransaction())),
            accounts: unwrap(accountSet.accounts, M10Error.InvalidTransaction()).map(
                (accountRef) => AccountId.fromUint8Array(unwrap(accountRef.accountId, M10Error.InvalidTransaction())),
            ),
        });
    }
}
