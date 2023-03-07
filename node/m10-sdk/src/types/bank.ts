import { m10 } from "../../protobufs";
import { M10Error } from "../error";
import { AccountId } from "../ids/account_id";
import { DocumentId } from "../ids/document_id";
import { PublicKey } from "../ids/public_key";
import { unwrap } from "../utils";


export interface IBank {
    id: DocumentId;
    owner: PublicKey;
    shortName: string;
    displayName: string;
    accounts: BankAccount[];
}

export class Bank implements IBank {
    public id: DocumentId;
    public owner: PublicKey;
    public shortName: string;
    public displayName: string;
    public accounts: BankAccount[];

    public constructor(properties: IBank) {
        this.id = properties.id;
        this.owner = properties.owner;
        this.shortName = properties.shortName;
        this.displayName = properties.displayName;
        this.accounts = properties.accounts;
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any of the required fields are missing
     */
    public static tryFrom(bank: m10.sdk.model.IBank): Bank {
        return new Bank({
            id: DocumentId.fromUint8Array("bank", unwrap(bank.id, M10Error.InvalidTransaction())),
            owner: PublicKey.fromUint8Array(unwrap(bank.owner, M10Error.InvalidTransaction())),
            shortName: unwrap(bank.shortName, M10Error.InvalidTransaction()),
            displayName: unwrap(bank.displayName, M10Error.InvalidTransaction()),
            accounts: unwrap(bank.accounts, M10Error.InvalidTransaction()).map(BankAccount.tryFrom),
        });
    }
}

export enum BankAccountType {
    CentralBankDigitalCurrency = 0,
    DigitalRegulatedMoney = 1,
}

function convertBankAccountType(value: m10.sdk.model.BankAccountRef.BankAccountType): BankAccountType {
    switch (value) {
        case m10.sdk.model.BankAccountRef.BankAccountType.CBDC:
            return BankAccountType.CentralBankDigitalCurrency;
        case m10.sdk.model.BankAccountRef.BankAccountType.DRM:
            return BankAccountType.DigitalRegulatedMoney;
    }
}


export interface IBankAccount {
    id: AccountId;
    accountType: BankAccountType;
}

export class BankAccount implements IBankAccount {
    public id: AccountId;
    public accountType: BankAccountType;

    public constructor(properties: IBankAccount) {
        this.id = properties.id;
        this.accountType = properties.accountType;
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any of the required fields are missing
     */
    public static tryFrom(bankAccountRef: m10.sdk.model.IBankAccountRef): BankAccount {
        return new BankAccount({
            id: AccountId.fromUint8Array(unwrap(bankAccountRef.accountId, M10Error.InvalidTransaction())),
            accountType: convertBankAccountType(unwrap(bankAccountRef.accountType, M10Error.InvalidTransaction())),
        });
    }
}
