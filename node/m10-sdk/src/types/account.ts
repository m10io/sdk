import { m10 } from "../../protobufs";
import { M10Error } from "../error";
import type { TxId } from "../ids";
import { AccountId } from "../ids/account_id";
import { isNone, isSome, unwrap } from "../utils";


export interface IAccount {
    id: AccountId;
    balance: LongNumber;
    frozen: boolean;
    code: string;
    decimals: number;
    balanceLimit: LongNumber;
    issuance: Option<Issuance>;
}

export class Account implements IAccount {
    public id: AccountId;
    public balance: LongNumber;
    public frozen: boolean;
    public code: string;
    public decimals: number;
    public balanceLimit: LongNumber;
    public issuance: Option<Issuance>;


    public constructor(properties: IAccount) {
        this.id = properties.id;
        this.balance = properties.balance;
        this.frozen = properties.frozen;
        this.code = properties.code;
        this.decimals = properties.decimals;
        this.balanceLimit = properties.balanceLimit;
        this.issuance = properties.issuance;
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any of the required fields are missing
     */
    public static tryFrom(indexedAccount: m10.sdk.transaction.IIndexedAccount): Account {
        return new Account({
            id: AccountId.fromUint8Array(unwrap(indexedAccount.id, M10Error.InvalidTransaction())),
            balance: unwrap(indexedAccount.balance, M10Error.InvalidTransaction()),
            frozen: unwrap(indexedAccount.frozen, M10Error.InvalidTransaction()),
            code: unwrap(indexedAccount.instrument?.code, M10Error.InvalidTransaction()),
            decimals: unwrap(indexedAccount.instrument?.decimalPlaces, M10Error.InvalidTransaction()),
            balanceLimit: unwrap(indexedAccount.balanceLimit, M10Error.InvalidTransaction()),
            issuance: isSome(indexedAccount.issuance) ? Issuance.tryFrom(indexedAccount.issuance) : null,
        });
    }
}

export interface IIssuance {
    balance: LongNumber;
    issuanceAccounts: LongNumber;
    holdingAccounts: LongNumber;
}

export class Issuance implements IIssuance {
    public balance: LongNumber;
    public issuanceAccounts: LongNumber;
    public holdingAccounts: LongNumber;


    public constructor(properties: IIssuance) {
        this.balance = properties.balance;
        this.issuanceAccounts = properties.issuanceAccounts;
        this.holdingAccounts = properties.holdingAccounts;
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any of the required fields are missing
     */
    public static tryFrom(issuance: m10.sdk.transaction.IndexedAccount.IIssuance): Issuance {
        return new Issuance({
            balance: unwrap(issuance.issuedBalance, M10Error.InvalidTransaction()),
            issuanceAccounts: unwrap(issuance.nonLeafChildren, M10Error.InvalidTransaction()),
            holdingAccounts: unwrap(issuance.leafChildren, M10Error.InvalidTransaction()),
        });
    }
}

export interface IAccountInfo {
    id: AccountId;
    parentId: AccountId;
    publicName: string;
    profileImageUrl: string;
    code: string;
    decimals: number;
}

export class AccountInfo implements IAccountInfo {
    public id: AccountId;
    public parentId: AccountId;
    public publicName: string;
    public profileImageUrl: string;
    public code: string;
    public decimals: number;


    public constructor(properties: IAccountInfo) {
        this.id = properties.id;
        this.parentId = properties.parentId;
        this.publicName = properties.publicName;
        this.profileImageUrl = properties.profileImageUrl;
        this.code = properties.code;
        this.decimals = properties.decimals;
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any of the required fields are missing
     */
    public static tryFrom(accountInfo: m10.sdk.model.IAccountInfo): AccountInfo {
        const accountId = AccountId.fromUint8Array(unwrap(accountInfo.accountId, M10Error.InvalidTransaction()));
        return new AccountInfo({
            id: accountId,
            parentId: isSome(accountInfo.parentAccountId) ? AccountId.fromUint8Array(accountInfo.parentAccountId) : accountId,
            publicName: unwrap(accountInfo.publicName, M10Error.InvalidTransaction()),
            profileImageUrl: unwrap(accountInfo.profileImageUrl, M10Error.InvalidTransaction()),
            code: unwrap(accountInfo.code, M10Error.InvalidTransaction()),
            decimals: unwrap(accountInfo.decimalPlaces, M10Error.InvalidTransaction()),
        });
    }
}


interface SetFrozen {
    accountId: AccountId;
    isFrozen: boolean;
}

interface SetBalanceLimit {
    accountId: AccountId;
    limit: LongNumber;
}

interface SetInstrument {
    accountId: AccountId;
    code: string;
    decimals: number;
}

interface NewAccount {
    accountId: Option<AccountId>;
}

enum AccountUpdateType {
    SetFrozen,
    SetBalanceLimit,
    SetInstrument,
    NewAccount,
}

type AccountUpdateData = SetFrozen | SetBalanceLimit | SetInstrument | NewAccount;

export interface IAccountUpdate {
    txId: TxId;
    contextId: Uint8Array;
    success: boolean;
    timestamp: LongNumber;
    updateType: AccountUpdateType;
    updateData: AccountUpdateData;
}

export class AccountUpdate implements IAccountUpdate {
    public txId: TxId;
    public contextId: Uint8Array;
    public success: boolean;
    public timestamp: LongNumber;
    public updateType: AccountUpdateType;
    public updateData: AccountUpdateData;

    public constructor(properties: IAccountUpdate) {
        this.txId = properties.txId;
        this.contextId = properties.contextId;
        this.success = properties.success;
        this.timestamp = properties.timestamp;
        this.updateType = properties.updateType;
        this.updateData = properties.updateData;
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any of the required fields are missing
     */
    private static getAccountUpdateType(
        txn: m10.sdk.IFinalizedTransaction,
    ): [AccountUpdateType, AccountUpdateData] {
        const txnData = new m10.sdk.transaction.TransactionData(
            unwrap(txn.request?.data, M10Error.InvalidTransaction()),
        );
        switch (txnData.data) {
            case "createLedgerAccount":
                return [
                    AccountUpdateType.NewAccount,
                    {
                        accountId: AccountId.fromUint8Array(
                            unwrap(txn.response?.accountCreated, M10Error.InvalidTransaction()),
                        ),
                    },
                ];
            case "setFreezeState":

                return [
                    AccountUpdateType.SetFrozen,
                    {
                        accountId: AccountId.fromUint8Array(
                            unwrap(txnData.setFreezeState?.accountId, M10Error.InvalidTransaction()),
                        ),
                        isFrozen: unwrap(txnData.setFreezeState?.frozen, M10Error.InvalidTransaction()),
                    },
                ];
            case "setBalanceLimit":
                return [
                    AccountUpdateType.SetBalanceLimit,
                    {
                        accountId: AccountId.fromUint8Array(
                            unwrap(txnData.setBalanceLimit?.accountId, M10Error.InvalidTransaction()),
                        ),
                        limit: unwrap(txnData.setBalanceLimit?.balanceLimit, M10Error.InvalidTransaction()),
                    },
                ];
            case "setInstrument":
                return [
                    AccountUpdateType.SetInstrument,
                    {
                        accountId: AccountId.fromUint8Array(
                            unwrap(txnData.setInstrument?.accountId, M10Error.InvalidTransaction()),
                        ),
                        code: unwrap(txnData.setInstrument?.code, M10Error.InvalidTransaction()),
                        decimals: unwrap(txnData.setInstrument?.decimalPlaces, M10Error.InvalidTransaction()),
                    },
                ];
            default:
                throw M10Error.InvalidTransaction();
        }
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any of the required fields are missing
     */
    public static tryFrom(txn: m10.sdk.IFinalizedTransaction): AccountUpdate {
        const [type, data] = AccountUpdate.getAccountUpdateType(txn);
        return new AccountUpdate({
            txId: unwrap(txn.response?.txId, M10Error.InvalidTransaction()),
            contextId: unwrap(txn.request?.contextId, M10Error.InvalidTransaction()),
            success: isNone(txn.response?.error),
            timestamp: unwrap(txn.response?.timestamp, M10Error.InvalidTransaction()),
            updateType: type,
            updateData: data,
        });
    }
}

