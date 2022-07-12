import * as uuid from "uuid";

import { m10 } from "../../protobufs";
import type { LedgerClient } from "../client";
import * as collections from "../collections";
import * as utils from "../utils";

export async function getBankAdminAccount(
    client: LedgerClient,
    bankAdminSigner: utils.CryptoSigner,
    bankName: string,
): Promise<utils.AccountId> {

    const bankAccountOwner = bankAdminSigner.getPublicKey();

    const listAccountsRequest: m10.sdk.IListAccountSetsRequest = { owner: bankAccountOwner };
    const listAccountsResponse: m10.sdk.IListAccountsResponse = await client
        .listAccounts(bankAdminSigner, listAccountsRequest);

    const bankAccounts = (listAccountsResponse.accounts || [])
        .map(account => {
            return {
                id: account.id ? utils.getAccountIdFromUint8Array(account.id) : null,
                owner: account.owner ? utils.getPublicKeyFromUint8Array(account.owner) : null,
                name: account.name,
                publicName: account.publicName,
                profileImageUrl: account.profileImageUrl,
            };
        });

    const bankIssuanceAccount = bankAccounts.find(account => (account.publicName === bankName));

    if (!bankIssuanceAccount) {
        throw new Error(`Cannot find bank account for: ${bankName}`);
    }

    return utils.unwrap<string>(bankIssuanceAccount.id, "bankIssuanceAccount.id");
}

export async function createLedgerAccount(
    client: LedgerClient,
    signer: utils.CryptoSigner,
    parentId?: string,
    instrument?: m10.sdk.transaction.IInstrument,
    frozen?: boolean,
    issuance?: boolean,
): Promise<string> {

    if (parentId) {
        utils.validate(parentId, "id", utils.isValidAccountId);
    }

    const createLedgerAccountPayload = new m10.sdk.transaction.CreateLedgerAccount({
        parentId: parentId == undefined ? undefined : utils.getUint8ArrayFromAccountId(parentId),
        instrument,
        frozen,
        issuance,
    });

    const transactionData = new m10.sdk.transaction.TransactionData({
        createLedgerAccount: createLedgerAccountPayload,
    });
    const transactionRequestPayload = client.transactionRequest(transactionData);
    const response = await client.createTransaction(signer, transactionRequestPayload);

    const accountCreated = utils.unwrap<Uint8Array>(response.accountCreated, "response.accountCreated");

    return utils.getAccountIdFromUint8Array(accountCreated);
}

export async function createAccount(
    client: LedgerClient,
    signer: utils.CryptoSigner,
    accountId: utils.AccountId,
    name: string,
): Promise<utils.AccountId> {

    utils.validate(accountId, "id", utils.isValidAccountId);

    const account: m10.sdk.model.IAccount = {
        id: utils.getUint8ArrayFromAccountId(accountId),
        name,
        publicName: name,
        owner: signer.getPublicKey(),
    };
    const accountPayload = new m10.sdk.model.Account(account);
    const transactionData = collections.getCreateTransactionDataFromDocument(
        m10.sdk.model.Account.encode(accountPayload).finish(),
        collections.Collection.Account,
    );

    const transactionRequestPayload = client.transactionRequest(transactionData);
    const transactionResponse = await client.createTransaction(signer, transactionRequestPayload);

    utils.checkTransactionResponse(transactionResponse);

    return accountId;
}

/**
 * Create a bank that is represents a collection of accounts that represent a bank. Has things like short name as well.
 *
 * @param client
 * @param signer
 * @param owner
 * @param shortName
 * @param displayName
 * @param accounts
 * @param id
 * @returns
 */
export async function createBank(
    client: LedgerClient,
    signer: utils.CryptoSigner,
    owner: utils.AccountId,
    shortName: string,
    displayName: string,
    accounts: m10.sdk.model.IBankAccountRef[],
    id?: string,
): Promise<m10.sdk.model.IBank> {

    utils.validate(owner, "id", utils.isValidAccountId);

    const bank: m10.sdk.model.IBank = {
        id: utils.getUint8ArrayFromAccountId(id ? id : uuid.v4()),
        owner: utils.getUint8ArrayFromAccountId(owner),
        shortName,
        displayName,
        accounts,
    };

    const bankPayload = new m10.sdk.model.Bank(bank);
    const transactionData = collections.getCreateTransactionDataFromDocument(
        m10.sdk.model.Bank.encode(bankPayload).finish(),
        collections.Collection.Bank,
    );

    const transactionRequestPayload = client.transactionRequest(transactionData);
    const transactionResponse = await client.createTransaction(signer, transactionRequestPayload);

    utils.checkTransactionResponse(transactionResponse);

    return bank;
}
