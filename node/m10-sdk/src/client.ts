import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";
import type { PartialMessage } from "@protobuf-ts/runtime";
import type { RpcInterceptor, RpcOptions, ServerStreamingCall } from "@protobuf-ts/runtime-rpc";

import type {
    FinalizedTransactions,
    GroupTransactionsRequest,
    ListAccountMetadataResponse,
    ListTransactionsRequest,
    RequestEnvelope,
    TransactionMetrics,
} from "./protobufs/sdk/api";
import * as sdkApi from "./protobufs/sdk/api";
import { M10QueryServiceClient, M10TxServiceClient } from "./protobufs/sdk/api.client";
import { DocumentOperations, Operation } from "./protobufs/sdk/document";
import type { AccountInfo,AccountMetadata,AccountSet } from "./protobufs/sdk/model/model";
import type { Bank } from "./protobufs/sdk/model/model";
import type { RoleBinding } from "./protobufs/sdk/rbac";
import type { Role } from "./protobufs/sdk/rbac";
import type {
    Action } from "./protobufs/sdk/transaction/transaction";
import * as sdkTransaction from "./protobufs/sdk/transaction/transaction";
import {
    CommitTransfer_TransferState,
    SetBalanceLimit,
    SetFreezeState,
    SetInstrument,
} from "./protobufs/sdk/transaction/transaction";
import { AccountId } from "./ids";
import type { EnhancedTransfer, EnhancedTransferStep } from "./transfer_ext";
import type { CryptoSigner } from "./utils";
import { TransactionSigner } from "./utils";


export class M10Client {
    private ledgerUrl: string;

    public txClient: M10TxServiceClient;
    public queryClient: M10QueryServiceClient;

    private signer?: CryptoSigner;

    public constructor(ledgerUrl: string, signer?: CryptoSigner, accessToken?: string) {
        this.ledgerUrl = ledgerUrl;

        const accessTokenInterceptor: RpcInterceptor = {
            interceptUnary(next, method, input, options) {
                if (!options.meta) {
                    options.meta = {};
                }

                options.meta["Authorization"] = `Bearer ${accessToken}`;
                return next(method, input, options);
            },
        };

        this.txClient = M10Client.createServiceClient(this.ledgerUrl, accessToken ? [accessTokenInterceptor] : []);

        this.queryClient = M10Client.createQueryClient(this.ledgerUrl, accessToken ? [accessTokenInterceptor] : []);

        this.signer = signer;
    }

    public static createServiceClient(ledgerUrl: string, interceptors?: RpcInterceptor[]): M10TxServiceClient {
        return new M10TxServiceClient(
            new GrpcWebFetchTransport({
                format: "binary",
                baseUrl: ledgerUrl,
                interceptors: interceptors,
            }),
        );
    }

    public static createQueryClient(ledgerUrl: string, interceptors?: RpcInterceptor[]): M10QueryServiceClient {
        return new M10QueryServiceClient(
            new GrpcWebFetchTransport({
                format: "binary",
                baseUrl: ledgerUrl,
                interceptors: interceptors,
            }),
        );
    }

    public getSigner(): CryptoSigner {
        if (!this.signer) {
            throw new TypeError("Signer not set");
        }

        return this.signer;
    }

    public buildTransactionRequest(data: sdkTransaction.TransactionData, contextId?: Uint8Array): sdkTransaction.TransactionRequestPayload {
        return sdkTransaction.TransactionRequestPayload.create({
            data: data,
            nonce: BigInt(TransactionSigner.generateNonce()),
            timestamp: BigInt(TransactionSigner.getTimestamp()),
            contextId: contextId,
        });
    }

    public async createTransaction(payload: sdkTransaction.TransactionRequestPayload): Promise<sdkTransaction.TransactionResponse> {
        if (!this.signer) {
            throw new TypeError("Signer not set");
        }

        const encodedPayload = sdkTransaction.TransactionRequestPayload.toBinary(
            sdkTransaction.TransactionRequestPayload.create(payload),
        );
        const envelop = sdkApi.RequestEnvelope.create({
            payload: encodedPayload,
            signature: await this.signer.getSignature(new Uint8Array(encodedPayload.buffer)),
        });
        const { response: transactionResponse } = await this.txClient.createTransaction(envelop);

        if (transactionResponse.error) {
            throw transactionResponse.error;
        }

        return transactionResponse;
    }

    public async createAccount(opts: PartialMessage<sdkTransaction.CreateLedgerAccount>, contextId?: Uint8Array): Promise<[bigint, AccountId]> {
        const request = this.buildTransactionRequest(
            sdkTransaction.TransactionData.create({
                data: {
                    oneofKind: "createLedgerAccount",
                    createLedgerAccount: sdkTransaction.CreateLedgerAccount.create({
                        parentId: opts.parentId ?? new Uint8Array(),
                        frozen: opts.frozen,
                        issuance: opts.issuance,
                        ...(opts.instrument && { instrument: opts.instrument }),
                        balanceLimit: opts.balanceLimit,
                    }),
                },
            }),
            contextId,
        );
        const transactionResponse = await this.createTransaction(request);

        return [
            transactionResponse.txId,
            AccountId.fromBytes(transactionResponse.accountCreated),
        ];
    }

    public async transfer(steps: PartialMessage<sdkTransaction.TransferStep>[], contextId?: Uint8Array): Promise<bigint> {
        const request = this.buildTransactionRequest(
            sdkTransaction.TransactionData.create({
                data: {
                    oneofKind: "transfer",
                    transfer: sdkTransaction.CreateTransfer.create({
                        transferSteps: steps.map(el => sdkTransaction.TransferStep.create(el)),
                    }),
                },
            }),
            contextId,
        );
        const transactionResponse = await this.createTransaction(request);

        return transactionResponse.txId;
    }

    public async initiateTransfer(steps: sdkTransaction.TransferStep[], contextId?: Uint8Array): Promise<bigint> {
        const request = this.buildTransactionRequest(
            sdkTransaction.TransactionData.create({
                data: {
                    oneofKind: "initiateTransfer",
                    initiateTransfer: sdkTransaction.CreateTransfer.create({
                        transferSteps: steps,
                    }),
                },
            }),
            contextId,
        );
        const transactionResponse = await this.createTransaction(request);

        return transactionResponse.txId;
    }

    public async commitTransfer(txId: bigint, accept: boolean, contextId?: Uint8Array): Promise<bigint> {
        const payload = sdkTransaction.CommitTransfer.create({
            pendingTxId: txId,
            newState: (
                accept
                    ? CommitTransfer_TransferState.ACCEPTED
                    : CommitTransfer_TransferState.REJECTED
            ),
        });

        const request = this.buildTransactionRequest(
            sdkTransaction.TransactionData.create({
                data: {
                    oneofKind: "commitTransfer",
                    commitTransfer: payload,
                },
            }),
            contextId,
        );
        const transactionResponse = await this.createTransaction(request);

        return transactionResponse.txId;
    }

    public async freezeAccount(accountId: AccountId, frozen: boolean, contextId?: Uint8Array): Promise<bigint> {

        const payload = SetFreezeState.create({ accountId: accountId.bytes, frozen });
        const request = this.buildTransactionRequest(
            sdkTransaction.TransactionData.create({
                data: {
                    oneofKind: "setFreezeState",
                    setFreezeState: payload,
                },
            }),
            contextId,
        );
        const transactionResponse = await this.createTransaction(request);

        return transactionResponse.txId;
    }

    public async setAccountLimit(accountId: AccountId, limit: number, contextId?: Uint8Array): Promise<bigint> {
        const payload = SetBalanceLimit.create({
            accountId: accountId.bytes,
            balanceLimit: BigInt(limit),
        });

        const request = this.buildTransactionRequest(
            sdkTransaction.TransactionData.create({
                data: {
                    oneofKind: "setBalanceLimit",
                    setBalanceLimit: payload,
                },
            }),
            contextId,
        );
        const transactionResponse = await this.createTransaction(request);

        return transactionResponse.txId;
    }

    public async setAccountInstrument(
        accountId: AccountId,
        code: string,
        decimals: number,
        description?: string,
        contextId?: Uint8Array,
    ): Promise<bigint> {
        const payload = SetInstrument.create({
            accountId: accountId.bytes,
            code: code,
            decimalPlaces: decimals,
            description: description ?? "",
        });

        const request = this.buildTransactionRequest(
            sdkTransaction.TransactionData.create({
                data: {
                    oneofKind: "setInstrument",
                    setInstrument: payload,
                },
            }),
            contextId,
        );
        const transactionResponse = await this.createTransaction(request);

        return transactionResponse.txId;
    }

    public async action(properties: PartialMessage<sdkTransaction.InvokeAction>, contextId?: Uint8Array): Promise<bigint> {
        const request = this.buildTransactionRequest(
            sdkTransaction.TransactionData.create({
                data: {
                    invokeAction: sdkTransaction.InvokeAction.create(properties),
                    oneofKind: "invokeAction",
                },
            }),
            contextId,
        );
        const transactionResponse = await this.createTransaction(request);

        return transactionResponse.txId;
    }

    public async documents(operations: Operation["operation"][], contextId?: Uint8Array): Promise<bigint> {
        const request = this.buildTransactionRequest(
            sdkTransaction.TransactionData.create({
                data: {
                    documentOperations: DocumentOperations.create({
                        operations: operations.map((el) => Operation.create({
                            operation: el,
                        })),
                    }),
                    oneofKind: "documentOperations",
                },
            }),
            contextId,
        );
        const transactionResponse = await this.createTransaction(request);

        return transactionResponse.txId;
    }

    public async getAccount(id: Uint8Array | string | AccountId): Promise<sdkTransaction.IndexedAccount> {
        const request = sdkTransaction.GetAccountRequest.create({
            id: id instanceof AccountId ? id.bytes : typeof id === "string" ? AccountId.fromHex(id).bytes : id,
        });
        const payload = sdkTransaction.GetAccountRequest.toBinary(request);

        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });

        const { response } = await this.queryClient.getIndexedAccount(envelope);

        return response;
    }

    public async getAccountInfo(id: Uint8Array | string | AccountId): Promise<AccountInfo> {
        const request = sdkTransaction.GetAccountRequest.create({
            id: id instanceof AccountId ? id.bytes : typeof id === "string" ? AccountId.fromHex(id).bytes : id,
        });
        const payload = sdkTransaction.GetAccountRequest.toBinary(request);
        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });
        const { response } = await this.queryClient.getAccountInfo(envelope);

        return response;
    }

    public async listAccounts(properties: PartialMessage<sdkApi.ListAccountMetadataRequest>): Promise<sdkTransaction.IndexedAccount[]> {
        const response = await this.listAccountMetadata(properties);
        const requests = response.accounts.map((account) => this.getAccount(
            AccountId.fromBytes(account.id),
        ));
        return Promise.all(requests);
    }

    public async observeAccounts(
        properties: PartialMessage<sdkApi.ObserveAccountsRequest>,
        options?: RpcOptions,
    ): Promise<() => ServerStreamingCall<RequestEnvelope, FinalizedTransactions>> {
        const request = sdkApi.ObserveAccountsRequest.create(properties);
        const payload = sdkApi.ObserveAccountsRequest.toBinary(request);

        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });

        return () => this.queryClient.observeAccounts(envelope, options);
    }

    public async getTransfer(txId: bigint): Promise<sdkTransaction.FinalizedTransfer> {
        const request = sdkTransaction.GetTransferRequest.create({ txId });
        const payload = sdkTransaction.GetTransferRequest.toBinary(request);

        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });

        const { response } = await this.queryClient.getTransfer(envelope);

        return response;
    }

    public async listTransfers(properties: PartialMessage<sdkTransaction.ListTransferRequest>): Promise<sdkTransaction.FinalizedTransfers> {
        const request = sdkTransaction.ListTransferRequest.create(properties);
        const payload = sdkTransaction.ListTransferRequest.toBinary(request);

        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });

        const { response } = await this.queryClient.listTransfers(envelope);

        return response;
    }

    public async getEnhancedTransfers(properties: PartialMessage<sdkTransaction.ListTransferRequest>): Promise<EnhancedTransfer[]> {
        const response = await this.listTransfers(properties);

        const enhancedTransfersPromise = response.transfers.map((transfer) => this.enhanceTransfer(transfer));
        const enhancedTransfers = await Promise.all(enhancedTransfersPromise);

        return enhancedTransfers;
    }

    public async enhanceTransfer(
        transfer: sdkTransaction.FinalizedTransfer,
    ): Promise<EnhancedTransfer> {

        const enhancedTransferStepsPromise = (transfer.transferSteps || [])
            .map((transferStep) => this.enhanceTransferStep(transferStep));

        const enhancedTransferSteps = await Promise.all(enhancedTransferStepsPromise);

        return {
            transfer: sdkTransaction.FinalizedTransfer.create(transfer),
            enhanced_steps: enhancedTransferSteps,
        };
    }

    public async getEnhancedTransfer(txId: bigint): Promise<EnhancedTransfer> {
        const response = await this.getTransfer(txId);


        const enhancedTransferStepsPromise = (response.transferSteps || [])
            .map((transferStep) => this.enhanceTransferStep(transferStep));

        const enhancedTransferSteps = await Promise.all(enhancedTransferStepsPromise);

        return {
            transfer: sdkTransaction.FinalizedTransfer.create(response),
            enhanced_steps: enhancedTransferSteps,
        };
    }

    public async enhanceTransferStep(
        transferStep: sdkTransaction.TransferStep,
    ): Promise<EnhancedTransferStep> {

        const fromPromise = this.getAccountInfo(transferStep.fromAccountId);
        const toPromise = this.getAccountInfo(transferStep.toAccountId);

        const [from, to] = await Promise.all([fromPromise, toPromise]);

        const fromBankPromise = from.parentAccountId
            ? await this.getAccountInfo(from.parentAccountId)
            : undefined;

        const toBankPromise = (
            to.parentAccountId
                ? await this.getAccountInfo(to.parentAccountId)
                : undefined
        );

        const [fromBank, toBank] = await Promise.all([fromBankPromise, toBankPromise]);

        return {
            from,
            to,
            from_bank: fromBank,
            to_bank: toBank,
        };
    }

    public async observeTransfers(
        properties: PartialMessage<sdkApi.ObserveAccountsRequest>,
        options?: RpcOptions,
    ): Promise<() => ServerStreamingCall<RequestEnvelope, FinalizedTransactions>> {
        const request = sdkApi.ObserveAccountsRequest.create(properties);
        const payload = sdkApi.ObserveAccountsRequest.toBinary(request);

        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });

        return () => this.queryClient.observeTransfers(envelope, options);
    }

    public async getAction(txId: bigint): Promise<Action> {
        const request = sdkTransaction.GetActionRequest.create({ txId });
        const payload = sdkTransaction.GetActionRequest.toBinary(request);

        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });

        const { response } = await this.queryClient.getAction(envelope);

        return response;
    }

    public async listActions(properties: PartialMessage<sdkTransaction.ListActionsRequest>): Promise<sdkTransaction.Actions> {
        const request = sdkTransaction.ListActionsRequest.create(properties);
        const payload = sdkTransaction.ListActionsRequest.toBinary(request);

        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });

        const { response } = await  this.queryClient.listActions(envelope);

        return response;
    }

    public async observeActions(
        properties: PartialMessage<sdkApi.ObserveActionsRequest>,
        options?: RpcOptions,
    ): Promise<() => ServerStreamingCall<RequestEnvelope, FinalizedTransactions>> {
        const request = sdkApi.ObserveActionsRequest.create(properties);
        const payload = sdkApi.ObserveActionsRequest.toBinary(request);

        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });

        return () => this.queryClient.observeActions(envelope, options);
    }

    public async observeMetrics(
        properties: PartialMessage<sdkApi.ObserveAccountsRequest>,
        options?: RpcOptions,
    ): Promise<() => ServerStreamingCall<RequestEnvelope, TransactionMetrics>> {
        const request = sdkApi.ObserveAccountsRequest.create(properties);
        const payload = sdkApi.ObserveAccountsRequest.toBinary(request);

        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });

        return () => this.queryClient.observeMetrics(envelope, options);
    }

    public async listTransactions(properties: PartialMessage<ListTransactionsRequest>): Promise<FinalizedTransactions> {
        const request = sdkApi.ListTransactionsRequest.create(properties);
        const payload = sdkApi.ListTransactionsRequest.toBinary(request);

        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });

        const { response } = await this.queryClient.listTransactions(envelope);

        return response;
    }

    public async groupTransactions(properties?: PartialMessage<GroupTransactionsRequest>): Promise<sdkApi.GroupedFinalizedTransactions> {
        const request = sdkApi.GroupTransactionsRequest.create(properties);
        const payload = sdkApi.GroupTransactionsRequest.toBinary(request);

        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });

        const { response } = await this.queryClient.groupTransactions(envelope);

        return response;
    }

    public async getBank(documentId: Uint8Array): Promise<Bank> {
        const payload = sdkApi.GetBankRequest.toBinary(
            sdkApi.GetBankRequest.create({
                id: documentId,
            }),
        );
        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });
        const { response } = await this.queryClient.getBank(envelope);

        return response;
    }

    public async listBanks(properties?: PartialMessage<sdkApi.ListBanksRequest>): Promise<sdkApi.ListBanksResponse> {
        const request = sdkApi.ListBanksRequest.create(properties);
        const payload = sdkApi.ListBanksRequest.toBinary(request);

        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });

        const { response } = await this.queryClient.listBanks(envelope);

        return response;
    }

    public async getRole(documentId: Uint8Array): Promise<Role> {
        const payload = sdkApi.GetRoleRequest.toBinary(
            sdkApi.GetRoleRequest.create({
                id: documentId,
            }),
        );
        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });
        const { response } = await this.queryClient.getRole(envelope);

        return response;
    }

    public async listRoles(properties: PartialMessage<sdkApi.ListRolesRequest>): Promise<sdkApi.ListRolesResponse> {
        const payload = sdkApi.ListRolesRequest.toBinary(
            sdkApi.ListRolesRequest.create(properties),
        );
        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });
        const { response } = await this.queryClient.listRoles(envelope);

        return response;
    }

    public async getRoleBindings(documentId: Uint8Array): Promise<RoleBinding> {
        const request = sdkApi.GetRoleBindingRequest.create({
            id: documentId,
        });
        const payload = sdkApi.GetRoleBindingRequest.toBinary(request);

        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });

        const { response } = await this.queryClient.getRoleBinding(envelope);

        return response;
    }

    public async listRoleBindings(properties: PartialMessage<sdkApi.ListRoleBindingsRequest>): Promise<sdkApi.ListRoleBindingsResponse> {
        const request = sdkApi.ListRoleBindingsRequest.create(properties);
        const payload = sdkApi.ListRoleBindingsRequest.toBinary(request);

        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });

        const { response } = await this.queryClient.listRoleBindings(envelope);

        return response;
    }

    public async getAccountSet(properties: PartialMessage<sdkApi.GetAccountSetRequest>): Promise<AccountSet> {
        const request = sdkApi.GetAccountSetRequest.create(properties);
        const payload = sdkApi.GetAccountSetRequest.toBinary(request);

        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });

        const { response } = await this.queryClient.getAccountSet(envelope);

        return response;
    }

    public async listAccountSets(properties: PartialMessage<sdkApi.ListAccountSetsRequest>): Promise<sdkApi.ListAccountSetsResponse> {
        const request = sdkApi.ListAccountSetsRequest.create(properties);
        const payload = sdkApi.ListAccountSetsRequest.toBinary(request);

        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });

        const { response } = await this.queryClient.listAccountSets(envelope);

        return response;
    }

    public async getAccountMetadata(documentId: Uint8Array): Promise<AccountMetadata> {
        const request = sdkTransaction.GetAccountRequest.create({ id: documentId });
        const payload = sdkTransaction.GetAccountRequest.toBinary(request);

        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });

        const { response } = await this.queryClient.getAccountMetadata(envelope);

        return response;
    }

    public async listAccountMetadata(properties: PartialMessage<sdkApi.ListAccountMetadataRequest>): Promise<ListAccountMetadataResponse> {
        const request = sdkApi.ListAccountMetadataRequest.create(properties);
        const payload = sdkApi.ListAccountMetadataRequest.toBinary(request);
        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });
        const { response } = await this.queryClient.listAccountMetadata(envelope);

        return response;
    }

    public async getTransaction(
        properties: PartialMessage<sdkApi.GetTransactionRequest>,
    ): Promise<sdkApi.FinalizedTransaction> {
        const request = sdkApi.GetTransactionRequest.create(properties);
        const payload = sdkApi.GetTransactionRequest.toBinary(request);

        const envelope = sdkApi.RequestEnvelope.create({
            payload,
            ...(this.signer && { signature: await this.signer.getSignature(new Uint8Array(payload.buffer)) }),
        });

        const { response } = await this.queryClient.getTransaction(envelope);

        return response;
    }
}
