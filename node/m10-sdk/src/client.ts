import { m10 } from "../protobufs";

import type { TransferBuilder, TransferFilter } from "./builders/transfer";
import type { CryptoSigner } from "./utils/signer";
import type { AccountBuilder, AccountFilter, ActionBuilder, ActionsFilter, DocumentBuilder, PageBuilder } from "./builders";
import { M10Error } from "./error";
import type { DocumentId, TxId } from "./ids";
import { AccountId } from "./ids";
import { LedgerClient } from "./ledger_client";
import type { ContextFilter, GroupingFilter } from "./types";
import { ExpandedTransfer } from "./types";
import { Account, AccountInfo, AccountMetadata, AccountSet, Action, Bank, Role, RoleBinding, Transaction, Transfer } from "./types";
import { unwrap } from "./utils";


export class M10Client {

    private client: LedgerClient;
    private signer: CryptoSigner;


    public constructor(ledgerUrl: string, signer: CryptoSigner, tls: boolean = true) {
        this.client = new LedgerClient(ledgerUrl, tls);
        this.signer = signer;
    }

    public getSigner(): CryptoSigner {
        return this.signer;
    }

    /**
     * @throws  {M10Error.Transaction}          if response contains an error
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     * @throws  {M10Error.InvalidAccountId}     if invalid account id was used
     */
    public async createAccount(builder: AccountBuilder, contextId?: Uint8Array): Promise<[ TxId, AccountId ]> {

        const request = this.client.transactionRequest(
            new m10.sdk.transaction.TransactionData({ createLedgerAccount: builder.toCreateLedgerAccount() }),
            contextId,
        );
        const transactionResponse = await this.client.createTransaction(this.signer, request);

        return [
            unwrap(transactionResponse.txId, M10Error.InvalidTransaction()),
            AccountId.fromUint8Array(unwrap(transactionResponse.accountCreated, M10Error.InvalidTransaction())),
        ];
    }

    /**
     * @throws  {M10Error.Transaction}          if response contains an error
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     */
    public async transfer(builder: TransferBuilder, contextId?: Uint8Array): Promise<TxId> {

        const request = this.client.transactionRequest(
            new m10.sdk.transaction.TransactionData({ transfer: builder.toCreateTransfer() }),
            contextId,
        );
        const transactionResponse = await this.client.createTransaction(this.signer, request);

        return unwrap(transactionResponse.txId, M10Error.InvalidTransaction());
    }

    /**
     * @throws  {M10Error.Transaction}          if response contains an error
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     */
    public async initiateTransfer(builder: TransferBuilder, contextId?: Uint8Array): Promise<TxId> {

        const request = this.client.transactionRequest(
            new m10.sdk.transaction.TransactionData({ initiateTransfer: builder.toCreateTransfer() }),
            contextId,
        );
        const transactionResponse = await this.client.createTransaction(this.signer, request);

        return unwrap(transactionResponse.txId, M10Error.InvalidTransaction());
    }

    /**
     * @throws  {M10Error.Transaction}          if response contains an error
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     */
    public async commitTransfer(txId: TxId, accept: boolean, contextId?: Uint8Array): Promise<TxId> {

        const payload = new m10.sdk.transaction.CommitTransfer({
            pendingTxId: txId,
            newState: (
                accept
                    ? m10.sdk.transaction.CommitTransfer.TransferState.ACCEPTED
                    : m10.sdk.transaction.CommitTransfer.TransferState.REJECTED
            ),
        });

        const request = this.client.transactionRequest(
            new m10.sdk.transaction.TransactionData({ commitTransfer: payload }),
            contextId,
        );
        const transactionResponse = await this.client.createTransaction(this.signer, request);

        return unwrap(transactionResponse.txId, M10Error.InvalidTransaction());
    }

    /**
     * Sets the [`Account`] [`frozen`] status.
     * Frozen accounts cannot participate in transactions.
     *
     * @throws  {M10Error.Transaction}          if response contains an error
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     */
    public async freezeAccount(accountId: AccountId, frozen: boolean, contextId?: Uint8Array): Promise<TxId> {

        const payload = new m10.sdk.transaction.SetFreezeState({ accountId: accountId.toUint8Array(), frozen });
        const request = this.client.transactionRequest(
            new m10.sdk.transaction.TransactionData({ setFreezeState: payload }),
            contextId,
        );
        const transactionResponse = await this.client.createTransaction(this.signer, request);

        return unwrap(transactionResponse.txId, M10Error.InvalidTransaction());
    }

    /**
     * @throws  {M10Error.Transaction}          if response contains an error
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     */
    public async setAccountLimit(accountId: AccountId, limit: number, contextId?: Uint8Array): Promise<TxId> {

        const payload = new m10.sdk.transaction.SetBalanceLimit({
            accountId: accountId.toUint8Array(),
            balanceLimit: limit,
        });

        const request = this.client.transactionRequest(
            new m10.sdk.transaction.TransactionData({ setBalanceLimit: payload }),
            contextId,
        );
        const transactionResponse = await this.client.createTransaction(this.signer, request);

        return unwrap(transactionResponse.txId, M10Error.InvalidTransaction());
    }

    /**
     * @throws  {M10Error.Transaction}          if response contains an error
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     */
    public async setAccountInstrument(
        accountId: AccountId,
        code: string,
        decimals: number,
        description?: Option<string>,
        contextId?: Uint8Array,
    ): Promise<TxId> {

        const payload = new m10.sdk.transaction.SetInstrument({
            accountId: accountId.toUint8Array(),
            code: code,
            decimalPlaces: decimals,
            description: description,
        });

        const request = this.client.transactionRequest(
            new m10.sdk.transaction.TransactionData({ setInstrument: payload }),
            contextId,
        );
        const transactionResponse = await this.client.createTransaction(this.signer, request);

        return unwrap(transactionResponse.txId, M10Error.InvalidTransaction());
    }

    /**
     * @throws  {M10Error.Transaction}          if response contains an error
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     */
    public async action(builder: ActionBuilder, contextId?: Uint8Array): Promise<TxId> {

        const request = this.client.transactionRequest(
            new m10.sdk.transaction.TransactionData({ invokeAction: builder.toInvokeAction() }),
            contextId,
        );
        const transactionResponse = await this.client.createTransaction(this.signer, request);

        return unwrap(transactionResponse.txId, M10Error.InvalidTransaction());
    }

    /**
     * @throws  {M10Error.Transaction}          if response contains an error
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     */
    public async documents(builder: DocumentBuilder, contextId?: Uint8Array): Promise<TxId> {

        const request = this.client.transactionRequest(
            new m10.sdk.transaction.TransactionData({ documentOperations: builder.toDocumentOperations() }),
            contextId,
        );
        const transactionResponse = await this.client.createTransaction(this.signer, request);

        return unwrap(transactionResponse.txId, M10Error.InvalidTransaction());
    }

    // Accounts

    /**
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     * @throws  {M10Error.InvalidAccountId}     if invalid account id was used
     */
    public async getAccount(id: AccountId): Promise<Account> {
        const request = new m10.sdk.transaction.GetAccountRequest({ id: id.toUint8Array() });
        const response = await this.client.getIndexedAccount(this.signer, request);
        return Account.tryFrom(response);
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     * @throws  {M10Error.InvalidAccountId}     if invalid account id was used
     */
    public async getAccountInfo(id: AccountId): Promise<AccountInfo> {
        const request = new m10.sdk.transaction.GetAccountRequest({ id: id.toUint8Array() });
        const response = await this.client.getAccountInfo(this.signer, request);
        return AccountInfo.tryFrom(response);
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     * @throws  {M10Error.InvalidAccountId}     if invalid account id was used
     */
    public async listAccounts(filter: PageBuilder): Promise<Account[]> {
        const response = await this.client.listAccountMetadata(this.signer, filter.toListAccountMetadataRequest());
        const accounts = unwrap(response.accounts, M10Error.InvalidTransaction());
        const requests = accounts.map((account) => this.getAccount(
            AccountId.fromUint8Array(unwrap(account.id, M10Error.InvalidTransaction())),
        ));
        return Promise.all(requests);
    }

    public observeAccounts(filter: AccountFilter): [m10.sdk.M10QueryService, () => void] {
        return this.client.observeAccounts(this.signer, filter.toObserveAccountsRequest());
    }

    // Transfers

    /**
     * @throws  {M10Error.InvalidTransaction}   if any of the required fields are missing
     */
    public async getTransfer(txId: TxId): Promise<Transfer> {
        const request = new m10.sdk.transaction.GetTransferRequest({ txId });
        const response = await this.client.getTransfer(this.signer, request);
        return Transfer.tryFrom(response);
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     * @throws  {M10Error.InvalidAccountId}     if invalid account id was used
     */
    public async getEnhancedTransfer(txId: TxId): Promise<ExpandedTransfer> {
        const request = new m10.sdk.transaction.GetTransferRequest({ txId });
        const response = await this.client.getTransfer(this.signer, request);
        const enchanced = await this.client.enhanceTransfer(this.signer, response);
        return ExpandedTransfer.tryFrom(enchanced);
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     */
    public async listTransfers(filter: TransferFilter): Promise<Transfer[]> {
        const request = filter.toListTransferRequest();
        const response = await this.client.listTransfers(this.signer, request);
        return unwrap(response.transfers, M10Error.InvalidTransaction()).map(Transfer.tryFrom);
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     * @throws  {M10Error.InvalidAccountId}     if invalid account id was used
     */
    public async getEnhancedTransfers(filter: TransferFilter): Promise<ExpandedTransfer[]> {
        const request = filter.toListTransferRequest();
        const response = await this.client.listTransfers(this.signer, request);
        const enchanced = await this.client.enhanceTransfers(
            this.signer,
            unwrap(response.transfers, M10Error.InvalidTransaction()),
        );
        return enchanced.map(ExpandedTransfer.tryFrom);
    }

    public observeTransfers(filter: AccountFilter): [m10.sdk.M10QueryService, () => void] {
        return this.client.observeTransfers(this.signer, filter.toObserveAccountsRequest());
    }

    // Actions

    /**
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     * @throws  {M10Error.InvalidAccountId}     if invalid account id was used
     */
    public async getAction(txId: TxId): Promise<Action> {
        const request = new m10.sdk.transaction.GetActionRequest({ txId });
        const response = await this.client.getAction(this.signer, request);
        return Action.tryFrom(response);
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     * @throws  {M10Error.InvalidAccountId}     if invalid account id was used
     */
    public async listActions(filter: ActionsFilter): Promise<Action[]> {
        const request = filter.toListActionsRequest();
        const response = await this.client.listActions(this.signer, request);
        return unwrap(response.actions, M10Error.InvalidTransaction()).map(Action.tryFrom);
    }

    public observeActions(filter: AccountFilter): [m10.sdk.M10QueryService, () => void] {
        return this.client.observeActions(this.signer, filter.toObserveActionsRequest());
    }

    // Metrics

    public observeMetrics(filter: AccountFilter): [m10.sdk.M10QueryService, () => void] {
        return this.client.observeMetrics(this.signer, filter.toObserveAccountsRequest());
    }

    // Transactions

    /**
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     */
    public async listTransactions(filter: ContextFilter): Promise<Transaction[]> {
        const request = filter.toListTransactionsRequest();
        const response = await this.client.listTransactions(this.signer, request);
        return unwrap(response.transactions, M10Error.InvalidTransaction()).map(Transaction.tryFrom);
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     */
    public async groupTransactions(filter: GroupingFilter): Promise<Transaction[][]> {
        const request = filter.toGroupTransactionsRequest();
        const response = await this.client.groupTransactions(this.signer, request);
        const groups = unwrap(response.groups, M10Error.InvalidTransaction());
        return groups.map((group) => unwrap(group.transactions, M10Error.InvalidTransaction()).map(Transaction.tryFrom));
    }

    // Banks

    /**
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     */
    public async getBank(id: DocumentId): Promise<Bank> {
        const request = new m10.sdk.GetBankRequest({ id: id.toUint8Array() });
        const response = await this.client.getBank(this.signer, request);
        return Bank.tryFrom(response);
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     */
    public async listBanks(builder: PageBuilder): Promise<Bank[]> {
        const request = new m10.sdk.ListBanksRequest({ page: builder.toPage() });
        const response = await this.client.listBanks(this.signer, request);
        return unwrap(response.banks, M10Error.InvalidTransaction()).map(Bank.tryFrom);
    }

    // Roles

    /**
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     */
    public async getRole(id: DocumentId): Promise<Role> {
        const request = new m10.sdk.GetRoleRequest({ id: id.toUint8Array() });
        const response = await this.client.getRole(this.signer, request);
        return Role.tryFrom(response);
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     */
    public async listRoles(builder: PageBuilder): Promise<Role[]> {
        const request = builder.toListRolesRequest();
        const response = await this.client.listRoles(this.signer, request);
        return unwrap(response.roles, M10Error.InvalidTransaction()).map(Role.tryFrom);
    }

    // Role-bindings

    /**
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     */
    public async getRoleBindings(id: DocumentId): Promise<RoleBinding> {
        const request = new m10.sdk.GetRoleBindingRequest({ id: id.toUint8Array() });
        const response = await this.client.getRoleBinding(this.signer, request);
        return RoleBinding.tryFrom(response);
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     */
    public async listRoleBindings(builder: PageBuilder): Promise<RoleBinding[]> {
        const request = builder.toListRoleBindingsRequest();
        const response = await this.client.listRoleBindings(this.signer, request);
        return unwrap(response.roleBindings, M10Error.InvalidTransaction()).map(RoleBinding.tryFrom);
    }

    // AccountSets

    /**
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     * @throws  {M10Error.InvalidAccountId}     if invalid account id was used
     */
    public async getAccountSet(id: DocumentId): Promise<AccountSet> {
        const request = new m10.sdk.GetAccountSetRequest({ id: id.toUint8Array() });
        const response = await this.client.getAccountSet(this.signer, request);
        return AccountSet.tryFrom(response);
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     * @throws  {M10Error.InvalidAccountId}     if invalid account id was used
     */
    public async listAccountSets(builder: PageBuilder): Promise<AccountSet[]> {
        const request = builder.toListAccountSetsRequest();
        const response = await this.client.listAccountSets(this.signer, request);
        return unwrap(response.accountSets, M10Error.InvalidTransaction()).map(AccountSet.tryFrom);
    }

    // Account Metadata

    /**
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     * @throws  {M10Error.InvalidAccountId}     if invalid account id was used
     */
    public async getAccountMetadata(id: DocumentId): Promise<AccountMetadata> {
        const request = new m10.sdk.transaction.GetAccountRequest({ id: id.toUint8Array() });
        const response = await this.client.getAccountMetadata(this.signer, request);
        return AccountMetadata.tryFrom(response);
    }

    /**
     * @throws  {M10Error.InvalidTransaction}   if any part of the transaction is missing
     * @throws  {M10Error.InvalidAccountId}     if invalid account id was used
     */
    public async listAccountMetadatas(builder: PageBuilder): Promise<AccountMetadata[]> {
        const request = builder.toListAccountMetadataRequest();
        const response = await this.client.listAccountMetadata(this.signer, request);
        return unwrap(response.accounts, M10Error.InvalidTransaction()).map(AccountMetadata.tryFrom);
    }
}
