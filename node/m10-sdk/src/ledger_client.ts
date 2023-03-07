import * as grpc from "@grpc/grpc-js";

import { google, m10 } from "../protobufs";

import { M10Error } from "./error";
import type { EnhancedTransfer, EnhancedTransferStep } from "./transfer_ext";
import type { CryptoSigner } from "./utils";
import { getRPCImpl, getRPCImplStream, isSome, TransactionSigner } from "./utils";


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
export class LedgerClient {

    private txClient: m10.sdk.M10TxService;
    private queryClient: m10.sdk.M10QueryService;

    private ledgerUrl: string;
    private credentials: grpc.ChannelCredentials;

    public constructor(ledgerUrl: string, tls: boolean) {
        this.ledgerUrl = ledgerUrl;
        this.credentials = tls ? grpc.credentials.createSsl() : grpc.credentials.createInsecure();

        this.txClient = m10.sdk.M10TxService.create(
            getRPCImpl(this.ledgerUrl, this.credentials, "m10.sdk.M10TxService"),
        );
        this.queryClient = m10.sdk.M10QueryService.create(
            getRPCImpl(this.ledgerUrl, this.credentials, "m10.sdk.M10QueryService"),
        );
    }

    private getQueryServiceStream(): m10.sdk.M10QueryService {
        return m10.sdk.M10QueryService.create(
            getRPCImplStream(this.ledgerUrl, this.credentials, "m10.sdk.M10QueryService"),
        );
    }

    // -------------------------------------------------------------------------
    // Transactions
    // -------------------------------------------------------------------------

    public transactionRequest(
        data: m10.sdk.transaction.ITransactionData,
        contextId?: Uint8Array,
    ): m10.sdk.transaction.ITransactionRequestPayload {
        return new m10.sdk.transaction.TransactionRequestPayload({
            data: data,
            nonce: TransactionSigner.generateNonce(),
            timestamp: TransactionSigner.getTimestamp(),
            contextId: contextId,
        });
    }

    public async blockHeight(): Promise<LongNumber> {
        const emptyRequest = new google.protobuf.Empty();
        const chainInfo = await this.queryClient.getChainInfo(emptyRequest);
        return chainInfo.blockHeight;
    }

    /**
     * @throws  {M10Error.Transaction} if response contains an error
     */
    public async createTransaction(
        signer: CryptoSigner,
        request: m10.sdk.transaction.ITransactionRequestPayload,
    ): Promise<m10.sdk.transaction.ITransactionResponse> {
        const payload = m10.sdk.transaction.TransactionRequestPayload.encode(request).finish();
        const envelop = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
        const transactionResponse = await this.txClient.createTransaction(envelop);

        if (isSome(transactionResponse.error)) {
            throw M10Error.Transaction(transactionResponse.error);
        }

        return transactionResponse;
    }

    // -------------------------------------------------------------------------
    // Transfers
    // -------------------------------------------------------------------------

    public async getTransfer(
        signer: CryptoSigner,
        properties: m10.sdk.transaction.IGetTransferRequest,
    ): Promise<m10.sdk.transaction.IFinalizedTransfer> {

        const request = new m10.sdk.transaction.GetTransferRequest(properties);
        const payload = m10.sdk.transaction.GetTransferRequest.encode(request).finish();

        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });

        return this.queryClient.getTransfer(envelope);
    }

    public async listTransfers(
        signer: CryptoSigner,
        properties: m10.sdk.transaction.IListTransferRequest,
    ): Promise<m10.sdk.transaction.IFinalizedTransfers> {

        const request = new m10.sdk.transaction.ListTransferRequest(properties);
        const payload = m10.sdk.transaction.ListTransferRequest.encode(request).finish();

        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });

        return this.queryClient.listTransfers(envelope);
    }

    // -------------------------------------------------------------------------
    // Actions
    // -------------------------------------------------------------------------

    public async getAction(
        signer: CryptoSigner,
        properties: m10.sdk.transaction.IGetActionRequest,
    ): Promise<m10.sdk.transaction.IAction> {

        const request = new m10.sdk.transaction.GetActionRequest(properties);
        const payload = m10.sdk.transaction.GetActionRequest.encode(request).finish();

        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });

        return this.queryClient.getAction(envelope);
    }

    public async listActions(
        signer: CryptoSigner,
        properties: m10.sdk.transaction.IListActionsRequest,
    ): Promise<m10.sdk.transaction.IActions> {

        const request = new m10.sdk.transaction.ListActionsRequest(properties);
        const payload = m10.sdk.transaction.ListActionsRequest.encode(request).finish();

        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });

        return this.queryClient.listActions(envelope);
    }

    public async getTransaction(
        signer: CryptoSigner,
        properties: m10.sdk.IGetTransactionRequest,
    ): Promise<m10.sdk.IFinalizedTransaction> {

        const request = new m10.sdk.GetTransactionRequest(properties);
        const payload = m10.sdk.GetTransactionRequest.encode(request).finish();

        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });

        return this.queryClient.getTransaction(envelope);
    }

    public async listTransactions(
        signer: CryptoSigner,
        properties: m10.sdk.IListTransactionsRequest,
    ): Promise<m10.sdk.IFinalizedTransactions> {

        const request = new m10.sdk.ListTransactionsRequest(properties);
        const payload = m10.sdk.ListTransactionsRequest.encode(request).finish();

        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });

        return this.queryClient.listTransactions(envelope);
    }

    public async groupTransactions(
        signer: CryptoSigner,
        properties: m10.sdk.IGroupTransactionsRequest,
    ): Promise<m10.sdk.IGroupedFinalizedTransactions> {

        const request = new m10.sdk.GroupTransactionsRequest(properties);
        const payload = m10.sdk.GroupTransactionsRequest.encode(request).finish();

        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });

        return this.queryClient.groupTransactions(envelope);
    }

    // -------------------------------------------------------------------------
    // Indexed Accounts
    // -------------------------------------------------------------------------

    public async getIndexedAccount(
        signer: CryptoSigner,
        properties: m10.sdk.transaction.IGetAccountRequest,
    ): Promise<m10.sdk.transaction.IIndexedAccount> {

        const request = new m10.sdk.transaction.GetAccountRequest(properties);
        const payload = m10.sdk.transaction.GetAccountRequest.encode(request).finish();

        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });

        return this.queryClient.getIndexedAccount(envelope);
    }

    // -------------------------------------------------------------------------
    // AccountSets
    // -------------------------------------------------------------------------

    public async getAccountSet(
        signer: CryptoSigner,
        properties: m10.sdk.IGetAccountSetRequest,
    ): Promise<m10.sdk.model.IAccountSet> {

        const request = new m10.sdk.GetAccountSetRequest(properties);
        const payload = m10.sdk.GetAccountSetRequest.encode(request).finish();

        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });

        return this.queryClient.getAccountSet(envelope);
    }


    public async listAccountSets(
        signer: CryptoSigner,
        properties: m10.sdk.IListAccountSetsRequest,
    ): Promise<m10.sdk.IListAccountSetsResponse> {

        const request = new m10.sdk.ListAccountSetsRequest(properties);
        const payload = m10.sdk.ListAccountSetsRequest.encode(request).finish();

        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });

        return this.queryClient.listAccountSets(envelope);
    }

    // -------------------------------------------------------------------------
    // Accounts
    // -------------------------------------------------------------------------

    public async getAccountMetadata(
        signer: CryptoSigner,
        properties: m10.sdk.transaction.IGetAccountRequest,
    ): Promise<m10.sdk.model.IAccountMetadata> {

        const request = new m10.sdk.transaction.GetAccountRequest(properties);
        const payload = m10.sdk.transaction.GetAccountRequest.encode(request).finish();

        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });

        return this.queryClient.getAccountMetadata(envelope);
    }

    public async getAccountInfo(
        signer: CryptoSigner,
        request: m10.sdk.transaction.GetAccountRequest,
    ): Promise<m10.sdk.model.IAccountInfo> {
        const payload = m10.sdk.transaction.GetAccountRequest.encode(request).finish();
        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
        return this.queryClient.getAccountInfo(envelope);
    }

    public async listAccountMetadata(
        signer: CryptoSigner,
        request: m10.sdk.ListAccountMetadataRequest,
    ): Promise<m10.sdk.IListAccountMetadataResponse> {
        const payload = m10.sdk.ListAccountMetadataRequest.encode(request).finish();
        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
        return this.queryClient.listAccountMetadata(envelope);
    }

    // -------------------------------------------------------------------------
    // Role Bindings
    // -------------------------------------------------------------------------

    public async getRoleBinding(
        signer: CryptoSigner,
        properties: m10.sdk.IGetRoleBindingRequest,
    ): Promise<m10.sdk.IRoleBinding> {

        const request = new m10.sdk.GetRoleBindingRequest(properties);
        const payload = m10.sdk.GetRoleBindingRequest.encode(request).finish();

        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });

        return this.queryClient.getRoleBinding(envelope);
    }

    public async listRoleBindings(
        signer: CryptoSigner,
        properties: m10.sdk.IListRoleBindingsRequest,
    ): Promise<m10.sdk.IListRoleBindingsResponse> {

        const request = new m10.sdk.ListRoleBindingsRequest(properties);
        const payload = m10.sdk.ListRoleBindingsRequest.encode(request).finish();

        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });

        return this.queryClient.listRoleBindings(envelope);
    }

    // -------------------------------------------------------------------------
    // Banks
    // -------------------------------------------------------------------------

    public async getBank(
        signer: CryptoSigner,
        request: m10.sdk.GetBankRequest,
    ): Promise<m10.sdk.model.IBank> {
        const payload = m10.sdk.GetBankRequest.encode(request).finish();
        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
        return this.queryClient.getBank(envelope);
    }

    public async listBanks(
        signer: CryptoSigner,
        properties: m10.sdk.IListBanksRequest,
    ): Promise<m10.sdk.IListBanksResponse> {

        const request = new m10.sdk.ListBanksRequest(properties);
        const payload = m10.sdk.ListBanksRequest.encode(request).finish();

        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });

        return this.queryClient.listBanks(envelope);
    }

    // -------------------------------------------------------------------------
    // Roles
    // -------------------------------------------------------------------------

    public async getRole(
        signer: CryptoSigner,
        request: m10.sdk.GetRoleRequest,
    ): Promise<m10.sdk.Role> {
        const payload = m10.sdk.GetRoleRequest.encode(request).finish();
        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
        return this.queryClient.getRole(envelope);
    }

    public async listRoles(
        signer: CryptoSigner,
        request: m10.sdk.ListRolesRequest,
    ): Promise<m10.sdk.IListRolesResponse> {
        const payload = m10.sdk.ListRolesRequest.encode(request).finish();
        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });
        return this.queryClient.listRoles(envelope);
    }

    // -------------------------------------------------------------------------
    // Observations (can be improved after https://github.com/protobufjs/protobuf.js/pull/1115 is merged)
    // -------------------------------------------------------------------------

    public observeTransfers(
        signer: CryptoSigner,
        properties: m10.sdk.IObserveAccountsRequest,
    ): [m10.sdk.M10QueryService, () => void] {

        const request = new m10.sdk.ObserveAccountsRequest(properties);
        const payload = m10.sdk.ObserveAccountsRequest.encode(request).finish();

        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });

        const service = this.getQueryServiceStream();
        const startObserver = (): void => { service.observeTransfers(envelope); };

        return [service, startObserver];
    }

    public observeResources(
        signer: CryptoSigner,
        properties: m10.sdk.IObserveResourcesRequest,
    ): [m10.sdk.M10QueryService, () => void] {

        const request = new m10.sdk.ObserveResourcesRequest(properties);
        const payload = m10.sdk.ObserveResourcesRequest.encode(request).finish();

        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });

        const service = this.getQueryServiceStream();
        const startObserver = (): void => { service.observeResources(envelope); };

        return [service, startObserver];
    }

    public observeAccounts(
        signer: CryptoSigner,
        properties: m10.sdk.IObserveAccountsRequest,
    ): [m10.sdk.M10QueryService, () => void] {

        const request = new m10.sdk.ObserveAccountsRequest(properties);
        const payload = m10.sdk.ObserveAccountsRequest.encode(request).finish();

        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });

        const service = this.getQueryServiceStream();
        const startObserver = (): void => { service.observeAccounts(envelope); };

        return [service, startObserver];
    }

    public observeActions(
        signer: CryptoSigner,
        properties: m10.sdk.IObserveActionsRequest,
    ): [m10.sdk.M10QueryService, () => void] {

        const request = new m10.sdk.ObserveActionsRequest(properties);
        const payload = m10.sdk.ObserveActionsRequest.encode(request).finish();

        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });

        const service = this.getQueryServiceStream();
        const startObserver = (): void => { service.observeActions(envelope); };

        return [service, startObserver];
    }

    public observeMetrics(
        signer: CryptoSigner,
        properties: m10.sdk.IObserveAccountsRequest,
    ): [m10.sdk.M10QueryService, () => void] {

        const request = new m10.sdk.ObserveAccountsRequest(properties);
        const payload = m10.sdk.ObserveAccountsRequest.encode(request).finish();

        const envelope = new m10.sdk.RequestEnvelope({ payload, signature: signer.getSignature(payload) });

        const service = this.getQueryServiceStream();
        const startObserver = (): void => { service.observeMetrics(envelope); };

        return [service, startObserver];
    }

    // -------------------------------------------------------------------------
    // EnhancedTransfers
    // -------------------------------------------------------------------------

    public async enhanceTransfers(
        signer: CryptoSigner,
        transfers: m10.sdk.transaction.IFinalizedTransfer[],
    ): Promise<EnhancedTransfer[]> {

        const enhancedTransfersPromise = transfers.map((transfer) => this.enhanceTransfer(signer, transfer));
        const enhancedTransfers = await Promise.all(enhancedTransfersPromise);

        return enhancedTransfers;
    }

    public async enhanceTransfer(
        signer: CryptoSigner,
        transfer: m10.sdk.transaction.IFinalizedTransfer,
    ): Promise<EnhancedTransfer> {

        const enhancedTransferStepsPromise = (transfer.transferSteps || [])
            .map((transferStep) => this.enhanceTransferStep(signer, transferStep));

        const enhancedTransferSteps = await Promise.all(enhancedTransferStepsPromise);

        return {
            transfer: new m10.sdk.transaction.FinalizedTransfer(transfer),
            enhanced_steps: enhancedTransferSteps,
        };
    }

    public async enhanceTransferStep(
        signer: CryptoSigner,
        transferStep: m10.sdk.transaction.ITransferStep,
    ): Promise<EnhancedTransferStep> {

        const fromPromise = this.getAccountInfo(
            signer,
            new m10.sdk.transaction.GetAccountRequest({ id: transferStep.fromAccountId }),
        );
        const toPromise = this.getAccountInfo(
            signer,
            new m10.sdk.transaction.GetAccountRequest({ id: transferStep.toAccountId }),
        );

        const [from, to] = await Promise.all([fromPromise, toPromise]);

        const fromBankPromise = (
            isSome(from.parentAccountId)
                ? await this.getAccountInfo(signer, new m10.sdk.transaction.GetAccountRequest({ id: from.parentAccountId }))
                : null
        );
        const toBankPromise = (
            isSome(to.parentAccountId)
                ? await this.getAccountInfo(signer, new m10.sdk.transaction.GetAccountRequest({ id: to.parentAccountId }))
                : null
        );

        const [fromBank, toBank] = await Promise.all([fromBankPromise, toBankPromise].filter(isSome));

        return {
            from,
            to,
            from_bank: fromBank,
            to_bank: toBank,
        };
    }
}
