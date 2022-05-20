/// <reference types="long" />
import { m10 } from "../protobufs";
import type { EnhancedTransfer, EnhancedTransferStep } from "./transfer_ext";
import * as utils from "./utils";
/**
 * A client for the M10 Ledger.
 *
 * This client allows you to query and transact on the M10 ledger.
 *
 * @example
 * const ledgerUrl = "test.m10.net";
 * const ledgerClient = new LedgerClient(ledgerUrl, true);
 * const block_height = await ledgerClient.blockHeight();
 */
export declare class LedgerClient {
    private txClient;
    private queryClient;
    private queryClientStream;
    constructor(ledger_url: string, tls: boolean);
    transactionRequest(data: m10.sdk.transaction.ITransactionData, contextId?: Uint8Array): m10.sdk.transaction.ITransactionRequestPayload;
    blockHeight(): Promise<number | Long>;
    createTransaction(signer: utils.CryptoSigner, request: m10.sdk.transaction.ITransactionRequestPayload): Promise<m10.sdk.transaction.ITransactionResponse>;
    getTransfer(signer: utils.CryptoSigner, properties: m10.sdk.transaction.IGetTransferRequest): Promise<m10.sdk.transaction.IFinalizedTransfer>;
    listTransfers(signer: utils.CryptoSigner, properties: m10.sdk.transaction.IListTransferRequest): Promise<m10.sdk.transaction.IFinalizedTransfers>;
    getAction(signer: utils.CryptoSigner, properties: m10.sdk.transaction.IGetActionRequest): Promise<m10.sdk.transaction.IAction>;
    listActions(signer: utils.CryptoSigner, properties: m10.sdk.transaction.IListActionsRequest): Promise<m10.sdk.transaction.IActions>;
    getTransaction(signer: utils.CryptoSigner, properties: m10.sdk.IGetTransactionRequest): Promise<m10.sdk.IFinalizedTransaction>;
    listTransactions(signer: utils.CryptoSigner, properties: m10.sdk.IListTransactionsRequest): Promise<m10.sdk.IFinalizedTransactions>;
    groupTransactions(signer: utils.CryptoSigner, properties: m10.sdk.IGroupTransactionsRequest): Promise<m10.sdk.IGroupedFinalizedTransactions>;
    getIndexedAccount(signer: utils.CryptoSigner, properties: m10.sdk.transaction.IGetAccountRequest): Promise<m10.sdk.transaction.IIndexedAccount>;
    getAccountSet(signer: utils.CryptoSigner, properties: m10.sdk.IGetAccountSetRequest): Promise<m10.sdk.model.IAccountSet>;
    listAccountSets(signer: utils.CryptoSigner, properties: m10.sdk.IListAccountSetsRequest): Promise<m10.sdk.IListAccountSetsResponse>;
    getAccount(signer: utils.CryptoSigner, properties: m10.sdk.transaction.IGetAccountRequest): Promise<m10.sdk.model.IAccount>;
    getAccountInfo(signer: utils.CryptoSigner, properties: m10.sdk.transaction.IGetAccountRequest): Promise<m10.sdk.model.IAccountInfo>;
    listAccounts(signer: utils.CryptoSigner, properties: m10.sdk.IListAccountsRequest): Promise<m10.sdk.IListAccountsResponse>;
    getRoleBinding(signer: utils.CryptoSigner, properties: m10.sdk.IGetRoleBindingRequest): Promise<m10.sdk.IRoleBinding>;
    listRoleBindings(signer: utils.CryptoSigner, properties: m10.sdk.IListRoleBindingsRequest): Promise<m10.sdk.IListRoleBindingsResponse>;
    getRole(signer: utils.CryptoSigner, properties: m10.sdk.IGetRoleRequest): Promise<m10.sdk.IRole>;
    listRoles(signer: utils.CryptoSigner, properties: m10.sdk.IListRolesRequest): Promise<m10.sdk.IListRolesResponse>;
    getObserveTransfers(signer: utils.CryptoSigner, properties: m10.sdk.IObserveAccountsRequest): Promise<m10.sdk.IFinalizedTransactions>;
    getObserveResources(signer: utils.CryptoSigner, properties: m10.sdk.IObserveResourcesRequest): Promise<m10.sdk.IFinalizedTransactions>;
    getObserveAccounts(signer: utils.CryptoSigner, properties: m10.sdk.IObserveAccountsRequest): Promise<m10.sdk.IFinalizedTransactions>;
    getObserveActions(signer: utils.CryptoSigner, properties: m10.sdk.IObserveActionsRequest): Promise<m10.sdk.IFinalizedTransactions>;
    enhanceTransfers(signer: utils.CryptoSigner, transfers: m10.sdk.transaction.IFinalizedTransfer[]): Promise<EnhancedTransfer[]>;
    enhanceTransfer(signer: utils.CryptoSigner, transfer: m10.sdk.transaction.IFinalizedTransfer): Promise<EnhancedTransfer>;
    enhanceTransferStep(signer: utils.CryptoSigner, transferStep: m10.sdk.transaction.ITransferStep): Promise<EnhancedTransferStep>;
}
