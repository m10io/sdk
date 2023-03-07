import { m10 } from "../../protobufs";
import type { TxId } from "../ids";
import type { AccountId } from "../ids";
import { Target } from "../types";

import type { ITxnFilter } from "./txn_filter";
import { TxnFilter } from "./txn_filter";


export class ActionBuilder {

    private _name: string;
    private _from: AccountId;
    private _to: Target;
    private _payload?: Option<Uint8Array>;


    private constructor(name: string, from: AccountId, to: Target, payload?: Uint8Array) {
        this._name = name;
        this._from = from;
        this._to = to;
        this._payload = payload;
    }

    public static forAccount(name: string, from: AccountId, to: AccountId): ActionBuilder {
        return new ActionBuilder(
            name,
            from,
            new Target(to),
        );
    }

    public static forAll(name: string, from: AccountId): ActionBuilder {
        return new ActionBuilder(
            name,
            from,
            new Target(),
        );
    }

    // TODO: find usecase
    // public metadata(value: google.protobuf.Any): ActionBuilder {
    //     this._payload = ...
    //     return this;
    // }

    public payload(payload: Uint8Array): ActionBuilder {
        this._payload = payload;
        return this;
    }


    public toInvokeAction(): m10.sdk.transaction.InvokeAction {
        return new m10.sdk.transaction.InvokeAction({
            name: this._name,
            fromAccount: this._from.toUint8Array(),
            target: this._to.into(),
            payload: this._payload,
        });
    }
}

export interface IActionsFilter extends ITxnFilter {
    min: TxId;
    max: TxId;
    limit: number;
    name: string;
    accountId: Option<AccountId>;
    contextId: Option<Uint8Array>;
}

export class ActionsFilter extends TxnFilter {

    private _name: string;
    private _accountId: Option<AccountId>;
    private _contextId: Option<Uint8Array>;


    public constructor(properties: IActionsFilter) {
        super(properties);

        this._name = properties.name;
        this._accountId = properties.accountId;
        this._contextId = properties.contextId;
    }


    public min(txId: TxId): ActionsFilter {
        super.min(txId);
        return this;
    }

    public max(txId: TxId): ActionsFilter {
        super.max(txId);
        return this;
    }

    public limit(count: number): ActionsFilter {
        super.limit(count);
        return this;
    }

    public toListActionsRequest(): m10.sdk.transaction.ListActionsRequest {
        return new m10.sdk.transaction.ListActionsRequest({
            name: this._name,
            minTxId: this.getMinTxId(),
            maxTxId: this.getMaxTxId(),
            limit: this.getLimit(),
            accountId: this._accountId?.toUint8Array(),
            contextId: this._contextId,
        });
    }

    public static byAccount(name: string, accountId: AccountId): ActionsFilter {
        return new ActionsFilter({
            min: 0,
            max: Number.MAX_SAFE_INTEGER,
            limit: TxnFilter.DEFAULT_TXN_LIMIT,
            name: name,
            accountId: accountId,
            contextId: null,
        });
    }

    public static byContextId(name: string, contextId: Uint8Array): ActionsFilter {
        return new ActionsFilter({
            min: 0,
            max: Number.MAX_SAFE_INTEGER,
            limit: TxnFilter.DEFAULT_TXN_LIMIT,
            name: name,
            accountId: null,
            contextId: contextId,
        });
    }
}
