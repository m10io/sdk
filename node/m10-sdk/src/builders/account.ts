import { m10 } from "../../protobufs";
import type { AccountId, TxId } from "../ids";
import { isSome } from "../utils";


export class AccountFilter {

    private _accounts: AccountId[] = [];
    private _startingFrom: Option<TxId>;
    private _name: Option<string>;


    public involves(id: AccountId): AccountFilter {
        this._accounts.push(id);
        return this;
    }

    public startingFrom(txId: TxId): AccountFilter {
        this._startingFrom = txId;
        return this;
    }

    public name(name: string): AccountFilter {
        this._name = name;
        return this;
    }


    public toObserveAccountsRequest(): m10.sdk.ObserveAccountsRequest {
        return new m10.sdk.ObserveAccountsRequest({
            involvedAccounts: this._accounts.map((account) => account.toUint8Array()),
            startingFrom: isSome(this._startingFrom) ? new m10.sdk.TxId({ txId: this._startingFrom }) : null,
        });
    }

    public toObserveActionsRequest(): m10.sdk.ObserveActionsRequest {
        return new m10.sdk.ObserveActionsRequest({
            involvesAccounts: this._accounts.map((account) => account.toUint8Array()),
            startingFrom: isSome(this._startingFrom) ? new m10.sdk.TxId({ txId: this._startingFrom }) : null,
            name: this._name,
        });
    }
}


export class AccountBuilder {

    private _parentId: Option<AccountId>;
    private _issuance: boolean;
    private _frozen: boolean;
    private _balanceLimit: number;
    private _instrument: Option<m10.sdk.transaction.IInstrument>;


    public constructor(parentId?: AccountId) {
        this._parentId = parentId;
        this._issuance = false;
        this._frozen = false;
        this._balanceLimit = 0;
        this._instrument = null;
    }

    public issuance(flag: boolean): AccountBuilder {
        this._issuance = flag;
        return this;
    }

    public frozen(flag: boolean): AccountBuilder {
        this._frozen = flag;
        return this;
    }

    public balanceLimit(limit: number): AccountBuilder {
        this._balanceLimit = limit;
        return this;
    }

    public instrument(code: string, decimals: number, description: Option<string>): AccountBuilder {
        this._instrument = {
            code: code,
            decimalPlaces: decimals,
            description: description,
        };
        return this;
    }

    public toCreateLedgerAccount(): m10.sdk.transaction.CreateLedgerAccount {
        return new m10.sdk.transaction.CreateLedgerAccount({
            parentId: isSome(this._parentId) ? this._parentId.toUint8Array() : null,
            frozen: this._frozen,
            issuance: this._issuance,
            instrument: this._instrument,
            balanceLimit: this._balanceLimit,
        });
    }
}
