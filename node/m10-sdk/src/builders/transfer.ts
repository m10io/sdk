import { google, m10 } from "../../protobufs";
import type { AccountId, TxId } from "../ids";

import type { ITxnFilter } from "./txn_filter";
import { TxnFilter } from "./txn_filter";


export class StepBuilder {
    private _from: AccountId;
    private _to: AccountId;
    private _amount: number;
    private _metadata: google.protobuf.Any[];


    public constructor(from: AccountId, to: AccountId, amount: number) {
        this._from = from;
        this._to = to;
        this._amount = amount;
        this._metadata = [];
    }

    public metadata(value: google.protobuf.Any): StepBuilder {
        this._metadata.push(value);
        return this;
    }

    public custom_metadata(typeUrl: string, payload: Uint8Array): StepBuilder {
        this._metadata.push(new google.protobuf.Any({ type_url: typeUrl, value: payload }));
        return this;
    }

    public toTransferStep(): m10.sdk.transaction.TransferStep {
        return new m10.sdk.transaction.TransferStep({
            fromAccountId: this._from.toUint8Array(),
            toAccountId: this._to.toUint8Array(),
            amount: this._amount,
            metadata: this._metadata,
        });
    }
}


export class TransferBuilder {
    private _steps: StepBuilder[];

    public constructor() {
        this._steps = [];
    }

    public step(step: StepBuilder): TransferBuilder {
        this._steps.push(step);
        return this;
    }

    public toCreateTransfer(): m10.sdk.transaction.CreateTransfer {
        return new m10.sdk.transaction.CreateTransfer({
            transferSteps: this._steps.map((step) => step.toTransferStep()),
        });
    }
}

export interface ITransferFilter extends ITxnFilter {
    min: TxId;
    max: TxId;
    limit: number;
    includeChildAccounts: boolean;
    accountId: Option<AccountId>;
    contextId: Option<Uint8Array>;
}

export class TransferFilter extends TxnFilter {

    private _includeChildAccounts: boolean;
    private _accountId: Option<AccountId>;
    private _contextId: Option<Uint8Array>;


    public constructor(properties: ITransferFilter) {
        super(properties);

        this._includeChildAccounts = properties.includeChildAccounts;
        this._accountId = properties.accountId;
        this._contextId = properties.contextId;
    }


    public includeChildAccounts(enable: boolean): TransferFilter {
        this._includeChildAccounts = enable;
        return this;
    }

    public toListTransferRequest(): m10.sdk.transaction.ListTransferRequest {
        return new m10.sdk.transaction.ListTransferRequest({
            includeChildAccounts: this._includeChildAccounts,
            minTxId: this.getMinTxId(),
            maxTxId: this.getMaxTxId(),
            limit: this.getLimit(),
            accountId: this._accountId?.toUint8Array(),
            contextId: this._contextId,
        });
    }

    public limit(count: number): TransferFilter {
        super.limit(count);
        return this;
    }

    public minTxId(txId: LongNumber): TransferFilter {
        super.min(txId);
        return this;
    }

    public maxTxId(txId: LongNumber): TransferFilter {
        super.max(txId);
        return this;
    }

    public static byAccount(accountId: AccountId): TransferFilter {
        return new TransferFilter({
            min: 0,
            max: Number.MAX_SAFE_INTEGER,
            limit: TxnFilter.DEFAULT_TXN_LIMIT,
            includeChildAccounts: false,
            accountId: accountId,
            contextId: null,
        });
    }

    public static byContextId(contextId: Uint8Array): TransferFilter {
        return new TransferFilter({
            min: 0,
            max: Number.MAX_SAFE_INTEGER,
            limit: TxnFilter.DEFAULT_TXN_LIMIT,
            includeChildAccounts: false,
            accountId: null,
            contextId: contextId,
        });
    }
}
