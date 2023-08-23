/* eslint-disable @typescript-eslint/no-shadow */
import { m10 } from "../../protobufs";
import type { ITxnFilter } from "../builders/txn_filter";
import { TxnFilter } from "../builders/txn_filter";
import { M10Error } from "../error";
import type { AccountId, TxId } from "../ids";
import { unwrap } from "../utils";

import { AccountUpdate } from "./account";
import { Action } from "./action";
import { Transfer } from "./transfer";


// type TransactionTypeString =  "transfer" | "createLedgerAccount" | "setFreezeState" | "documentOperations" |
//     "invokeAction" | "initiateTransfer" | "commitTransfer" | "setInstrument" | "setBalanceLimit";

export enum TransactionType {
    Transfer,
    InitiateTransfer,
    CommitTransfer,
    AccountUpdate,
    // TODO: Not yet implemented
    DocumentOperations,
    Action,
}

type TransactionValue = Transfer | AccountUpdate | Action;

export interface ITransaction {
    type: TransactionType;
    value: Option<TransactionValue>;
}

export class Transaction implements ITransaction {

    public type: TransactionType;
    public value: Option<TransactionValue>;

    public constructor(properties: ITransaction) {
        this.type = properties.type;
        this.value = properties.value;
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any of the required fields are missing
     */
    public static tryFrom(txn: m10.sdk.IFinalizedTransaction): Transaction {
        const txnData = new m10.sdk.transaction.TransactionData(
            unwrap(txn.request?.data, M10Error.InvalidTransaction()),
        );
        switch (unwrap(txnData.data, M10Error.InvalidTransaction())) {
            case "transfer":
            case "createToken":
            case "redeemToken":
                return new Transaction({
                    type: TransactionType.Transfer,
                    value: Transfer.tryFromTxn(txn),
                });
            case "initiateTransfer":
                return new Transaction({
                    type: TransactionType.InitiateTransfer,
                    value: Transfer.tryFromTxn(txn),
                });
            case "commitTransfer":
                return new Transaction({
                    type: TransactionType.CommitTransfer,
                    value: Transfer.tryFromTxn(txn),
                });
            case "createLedgerAccount":
            case "setInstrument":
            case "setBalanceLimit":
            case "setFreezeState":
                return new Transaction({
                    type: TransactionType.AccountUpdate,
                    value: AccountUpdate.tryFrom(txn),
                });
            case "invokeAction":
                return new Transaction({
                    type: TransactionType.Action,
                    value: Action.tryFromFinalizedTransaction(txn),
                });
            case "documentOperations":
                return new Transaction({
                    type: TransactionType.DocumentOperations,
                    value: null,
                });
        }
    }
}

interface IContextFilter extends ITxnFilter {
    min: TxId;
    max: TxId;
    limit: number;
    contextId: Uint8Array;
}

export class ContextFilter extends TxnFilter {
    private _contextId: Uint8Array;

    public constructor(properties: IContextFilter) {
        super(properties);
        this._contextId = properties.contextId;
    }

    public toListTransactionsRequest(): m10.sdk.ListTransactionsRequest {
        return new m10.sdk.ListTransactionsRequest({
            contextId: this._contextId,
            minTxId: this.getMinTxId(),
            maxTxId: this.getMaxTxId(),
            limit: this.getLimit(),
        });
    }

    public static byContextId(contextId: Uint8Array): ContextFilter {
        return new ContextFilter({
            min: 0,
            max: Number.MAX_SAFE_INTEGER,
            limit: TxnFilter.DEFAULT_TXN_LIMIT,
            contextId: contextId,
        });
    }
}


interface IGroupingFilter extends ITxnFilter {
    min: TxId;
    max: TxId;
    limit: number;
    accountId: AccountId;
}

export class GroupingFilter extends TxnFilter {
    private _accountId: AccountId;

    public constructor(properties: IGroupingFilter) {
        super(properties);
        this._accountId = properties.accountId;
    }

    public toGroupTransactionsRequest(): m10.sdk.GroupTransactionsRequest {
        return new m10.sdk.GroupTransactionsRequest({
            accountId: this._accountId.toUint8Array(),
            minTxId: this.getMinTxId(),
            maxTxId: this.getMaxTxId(),
            limitGroups: this.getLimit(),
        });
    }

    public static byAccountId(accountId: AccountId): GroupingFilter {
        return new GroupingFilter({
            min: 0,
            max: Number.MAX_SAFE_INTEGER,
            limit: TxnFilter.DEFAULT_TXN_LIMIT,
            accountId: accountId,
        });
    }
}
