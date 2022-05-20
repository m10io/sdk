import * as $protobuf from "protobufjs";
/** Namespace m10. */
export namespace m10 {

    /** Namespace sdk. */
    namespace sdk {

        /** Represents a M10TxService */
        class M10TxService extends $protobuf.rpc.Service {

            /**
             * Constructs a new M10TxService service.
             * @param rpcImpl RPC implementation
             * @param [requestDelimited=false] Whether requests are length-delimited
             * @param [responseDelimited=false] Whether responses are length-delimited
             */
            constructor(rpcImpl: $protobuf.RPCImpl, requestDelimited?: boolean, responseDelimited?: boolean);

            /**
             * Creates new M10TxService service using the specified rpc implementation.
             * @param rpcImpl RPC implementation
             * @param [requestDelimited=false] Whether requests are length-delimited
             * @param [responseDelimited=false] Whether responses are length-delimited
             * @returns RPC service. Useful where requests and/or responses are streamed.
             */
            public static create(rpcImpl: $protobuf.RPCImpl, requestDelimited?: boolean, responseDelimited?: boolean): M10TxService;

            /**
             * Calls CreateTransaction.
             * @param request RequestEnvelope message or plain object
             * @param callback Node-style callback called with the error, if any, and TransactionResponse
             */
            public createTransaction(request: m10.sdk.IRequestEnvelope, callback: m10.sdk.M10TxService.CreateTransactionCallback): void;

            /**
             * Calls CreateTransaction.
             * @param request RequestEnvelope message or plain object
             * @returns Promise
             */
            public createTransaction(request: m10.sdk.IRequestEnvelope): Promise<m10.sdk.transaction.TransactionResponse>;
        }

        namespace M10TxService {

            /**
             * Callback as used by {@link m10.sdk.M10TxService#createTransaction}.
             * @param error Error, if any
             * @param [response] TransactionResponse
             */
            type CreateTransactionCallback = (error: (Error|null), response?: m10.sdk.transaction.TransactionResponse) => void;
        }

        /** Represents a M10QueryService */
        class M10QueryService extends $protobuf.rpc.Service {

            /**
             * Constructs a new M10QueryService service.
             * @param rpcImpl RPC implementation
             * @param [requestDelimited=false] Whether requests are length-delimited
             * @param [responseDelimited=false] Whether responses are length-delimited
             */
            constructor(rpcImpl: $protobuf.RPCImpl, requestDelimited?: boolean, responseDelimited?: boolean);

            /**
             * Creates new M10QueryService service using the specified rpc implementation.
             * @param rpcImpl RPC implementation
             * @param [requestDelimited=false] Whether requests are length-delimited
             * @param [responseDelimited=false] Whether responses are length-delimited
             * @returns RPC service. Useful where requests and/or responses are streamed.
             */
            public static create(rpcImpl: $protobuf.RPCImpl, requestDelimited?: boolean, responseDelimited?: boolean): M10QueryService;

            /**
             * Calls GetTransfer.
             * @param request RequestEnvelope message or plain object
             * @param callback Node-style callback called with the error, if any, and FinalizedTransfer
             */
            public getTransfer(request: m10.sdk.IRequestEnvelope, callback: m10.sdk.M10QueryService.GetTransferCallback): void;

            /**
             * Calls GetTransfer.
             * @param request RequestEnvelope message or plain object
             * @returns Promise
             */
            public getTransfer(request: m10.sdk.IRequestEnvelope): Promise<m10.sdk.transaction.FinalizedTransfer>;

            /**
             * Calls ListTransfers.
             * @param request RequestEnvelope message or plain object
             * @param callback Node-style callback called with the error, if any, and FinalizedTransfers
             */
            public listTransfers(request: m10.sdk.IRequestEnvelope, callback: m10.sdk.M10QueryService.ListTransfersCallback): void;

            /**
             * Calls ListTransfers.
             * @param request RequestEnvelope message or plain object
             * @returns Promise
             */
            public listTransfers(request: m10.sdk.IRequestEnvelope): Promise<m10.sdk.transaction.FinalizedTransfers>;

            /**
             * Calls ObserveTransfers.
             * @param request RequestEnvelope message or plain object
             * @param callback Node-style callback called with the error, if any, and FinalizedTransactions
             */
            public observeTransfers(request: m10.sdk.IRequestEnvelope, callback: m10.sdk.M10QueryService.ObserveTransfersCallback): void;

            /**
             * Calls ObserveTransfers.
             * @param request RequestEnvelope message or plain object
             * @returns Promise
             */
            public observeTransfers(request: m10.sdk.IRequestEnvelope): Promise<m10.sdk.FinalizedTransactions>;

            /**
             * Calls GetIndexedAccount.
             * @param request RequestEnvelope message or plain object
             * @param callback Node-style callback called with the error, if any, and IndexedAccount
             */
            public getIndexedAccount(request: m10.sdk.IRequestEnvelope, callback: m10.sdk.M10QueryService.GetIndexedAccountCallback): void;

            /**
             * Calls GetIndexedAccount.
             * @param request RequestEnvelope message or plain object
             * @returns Promise
             */
            public getIndexedAccount(request: m10.sdk.IRequestEnvelope): Promise<m10.sdk.transaction.IndexedAccount>;

            /**
             * Calls GetAccount.
             * @param request RequestEnvelope message or plain object
             * @param callback Node-style callback called with the error, if any, and Account
             */
            public getAccount(request: m10.sdk.IRequestEnvelope, callback: m10.sdk.M10QueryService.GetAccountCallback): void;

            /**
             * Calls GetAccount.
             * @param request RequestEnvelope message or plain object
             * @returns Promise
             */
            public getAccount(request: m10.sdk.IRequestEnvelope): Promise<m10.sdk.model.Account>;

            /**
             * Calls GetAccountInfo.
             * @param request RequestEnvelope message or plain object
             * @param callback Node-style callback called with the error, if any, and AccountInfo
             */
            public getAccountInfo(request: m10.sdk.IRequestEnvelope, callback: m10.sdk.M10QueryService.GetAccountInfoCallback): void;

            /**
             * Calls GetAccountInfo.
             * @param request RequestEnvelope message or plain object
             * @returns Promise
             */
            public getAccountInfo(request: m10.sdk.IRequestEnvelope): Promise<m10.sdk.model.AccountInfo>;

            /**
             * Calls ListAccounts.
             * @param request RequestEnvelope message or plain object
             * @param callback Node-style callback called with the error, if any, and ListAccountsResponse
             */
            public listAccounts(request: m10.sdk.IRequestEnvelope, callback: m10.sdk.M10QueryService.ListAccountsCallback): void;

            /**
             * Calls ListAccounts.
             * @param request RequestEnvelope message or plain object
             * @returns Promise
             */
            public listAccounts(request: m10.sdk.IRequestEnvelope): Promise<m10.sdk.ListAccountsResponse>;

            /**
             * Calls ObserveAccounts.
             * @param request RequestEnvelope message or plain object
             * @param callback Node-style callback called with the error, if any, and FinalizedTransactions
             */
            public observeAccounts(request: m10.sdk.IRequestEnvelope, callback: m10.sdk.M10QueryService.ObserveAccountsCallback): void;

            /**
             * Calls ObserveAccounts.
             * @param request RequestEnvelope message or plain object
             * @returns Promise
             */
            public observeAccounts(request: m10.sdk.IRequestEnvelope): Promise<m10.sdk.FinalizedTransactions>;

            /**
             * Calls GetAction.
             * @param request RequestEnvelope message or plain object
             * @param callback Node-style callback called with the error, if any, and Action
             */
            public getAction(request: m10.sdk.IRequestEnvelope, callback: m10.sdk.M10QueryService.GetActionCallback): void;

            /**
             * Calls GetAction.
             * @param request RequestEnvelope message or plain object
             * @returns Promise
             */
            public getAction(request: m10.sdk.IRequestEnvelope): Promise<m10.sdk.transaction.Action>;

            /**
             * Calls ListActions.
             * @param request RequestEnvelope message or plain object
             * @param callback Node-style callback called with the error, if any, and Actions
             */
            public listActions(request: m10.sdk.IRequestEnvelope, callback: m10.sdk.M10QueryService.ListActionsCallback): void;

            /**
             * Calls ListActions.
             * @param request RequestEnvelope message or plain object
             * @returns Promise
             */
            public listActions(request: m10.sdk.IRequestEnvelope): Promise<m10.sdk.transaction.Actions>;

            /**
             * Calls ObserveActions.
             * @param request RequestEnvelope message or plain object
             * @param callback Node-style callback called with the error, if any, and FinalizedTransactions
             */
            public observeActions(request: m10.sdk.IRequestEnvelope, callback: m10.sdk.M10QueryService.ObserveActionsCallback): void;

            /**
             * Calls ObserveActions.
             * @param request RequestEnvelope message or plain object
             * @returns Promise
             */
            public observeActions(request: m10.sdk.IRequestEnvelope): Promise<m10.sdk.FinalizedTransactions>;

            /**
             * Calls GetChainInfo.
             * @param request Empty message or plain object
             * @param callback Node-style callback called with the error, if any, and ChainInfo
             */
            public getChainInfo(request: google.protobuf.IEmpty, callback: m10.sdk.M10QueryService.GetChainInfoCallback): void;

            /**
             * Calls GetChainInfo.
             * @param request Empty message or plain object
             * @returns Promise
             */
            public getChainInfo(request: google.protobuf.IEmpty): Promise<m10.sdk.ChainInfo>;

            /**
             * Calls GetTransaction.
             * @param request RequestEnvelope message or plain object
             * @param callback Node-style callback called with the error, if any, and FinalizedTransaction
             */
            public getTransaction(request: m10.sdk.IRequestEnvelope, callback: m10.sdk.M10QueryService.GetTransactionCallback): void;

            /**
             * Calls GetTransaction.
             * @param request RequestEnvelope message or plain object
             * @returns Promise
             */
            public getTransaction(request: m10.sdk.IRequestEnvelope): Promise<m10.sdk.FinalizedTransaction>;

            /**
             * Calls ListTransactions.
             * @param request RequestEnvelope message or plain object
             * @param callback Node-style callback called with the error, if any, and FinalizedTransactions
             */
            public listTransactions(request: m10.sdk.IRequestEnvelope, callback: m10.sdk.M10QueryService.ListTransactionsCallback): void;

            /**
             * Calls ListTransactions.
             * @param request RequestEnvelope message or plain object
             * @returns Promise
             */
            public listTransactions(request: m10.sdk.IRequestEnvelope): Promise<m10.sdk.FinalizedTransactions>;

            /**
             * Calls GroupTransactions.
             * @param request RequestEnvelope message or plain object
             * @param callback Node-style callback called with the error, if any, and GroupedFinalizedTransactions
             */
            public groupTransactions(request: m10.sdk.IRequestEnvelope, callback: m10.sdk.M10QueryService.GroupTransactionsCallback): void;

            /**
             * Calls GroupTransactions.
             * @param request RequestEnvelope message or plain object
             * @returns Promise
             */
            public groupTransactions(request: m10.sdk.IRequestEnvelope): Promise<m10.sdk.GroupedFinalizedTransactions>;

            /**
             * Calls GetAccountSet.
             * @param request RequestEnvelope message or plain object
             * @param callback Node-style callback called with the error, if any, and AccountSet
             */
            public getAccountSet(request: m10.sdk.IRequestEnvelope, callback: m10.sdk.M10QueryService.GetAccountSetCallback): void;

            /**
             * Calls GetAccountSet.
             * @param request RequestEnvelope message or plain object
             * @returns Promise
             */
            public getAccountSet(request: m10.sdk.IRequestEnvelope): Promise<m10.sdk.model.AccountSet>;

            /**
             * Calls ListAccountSets.
             * @param request RequestEnvelope message or plain object
             * @param callback Node-style callback called with the error, if any, and ListAccountSetsResponse
             */
            public listAccountSets(request: m10.sdk.IRequestEnvelope, callback: m10.sdk.M10QueryService.ListAccountSetsCallback): void;

            /**
             * Calls ListAccountSets.
             * @param request RequestEnvelope message or plain object
             * @returns Promise
             */
            public listAccountSets(request: m10.sdk.IRequestEnvelope): Promise<m10.sdk.ListAccountSetsResponse>;

            /**
             * Calls GetRoleBinding.
             * @param request RequestEnvelope message or plain object
             * @param callback Node-style callback called with the error, if any, and RoleBinding
             */
            public getRoleBinding(request: m10.sdk.IRequestEnvelope, callback: m10.sdk.M10QueryService.GetRoleBindingCallback): void;

            /**
             * Calls GetRoleBinding.
             * @param request RequestEnvelope message or plain object
             * @returns Promise
             */
            public getRoleBinding(request: m10.sdk.IRequestEnvelope): Promise<m10.sdk.RoleBinding>;

            /**
             * Calls ListRoleBindings.
             * @param request RequestEnvelope message or plain object
             * @param callback Node-style callback called with the error, if any, and ListRoleBindingsResponse
             */
            public listRoleBindings(request: m10.sdk.IRequestEnvelope, callback: m10.sdk.M10QueryService.ListRoleBindingsCallback): void;

            /**
             * Calls ListRoleBindings.
             * @param request RequestEnvelope message or plain object
             * @returns Promise
             */
            public listRoleBindings(request: m10.sdk.IRequestEnvelope): Promise<m10.sdk.ListRoleBindingsResponse>;

            /**
             * Calls GetRole.
             * @param request RequestEnvelope message or plain object
             * @param callback Node-style callback called with the error, if any, and Role
             */
            public getRole(request: m10.sdk.IRequestEnvelope, callback: m10.sdk.M10QueryService.GetRoleCallback): void;

            /**
             * Calls GetRole.
             * @param request RequestEnvelope message or plain object
             * @returns Promise
             */
            public getRole(request: m10.sdk.IRequestEnvelope): Promise<m10.sdk.Role>;

            /**
             * Calls ListRoles.
             * @param request RequestEnvelope message or plain object
             * @param callback Node-style callback called with the error, if any, and ListRolesResponse
             */
            public listRoles(request: m10.sdk.IRequestEnvelope, callback: m10.sdk.M10QueryService.ListRolesCallback): void;

            /**
             * Calls ListRoles.
             * @param request RequestEnvelope message or plain object
             * @returns Promise
             */
            public listRoles(request: m10.sdk.IRequestEnvelope): Promise<m10.sdk.ListRolesResponse>;

            /**
             * Calls ObserveResources.
             * @param request RequestEnvelope message or plain object
             * @param callback Node-style callback called with the error, if any, and FinalizedTransactions
             */
            public observeResources(request: m10.sdk.IRequestEnvelope, callback: m10.sdk.M10QueryService.ObserveResourcesCallback): void;

            /**
             * Calls ObserveResources.
             * @param request RequestEnvelope message or plain object
             * @returns Promise
             */
            public observeResources(request: m10.sdk.IRequestEnvelope): Promise<m10.sdk.FinalizedTransactions>;
        }

        namespace M10QueryService {

            /**
             * Callback as used by {@link m10.sdk.M10QueryService#getTransfer}.
             * @param error Error, if any
             * @param [response] FinalizedTransfer
             */
            type GetTransferCallback = (error: (Error|null), response?: m10.sdk.transaction.FinalizedTransfer) => void;

            /**
             * Callback as used by {@link m10.sdk.M10QueryService#listTransfers}.
             * @param error Error, if any
             * @param [response] FinalizedTransfers
             */
            type ListTransfersCallback = (error: (Error|null), response?: m10.sdk.transaction.FinalizedTransfers) => void;

            /**
             * Callback as used by {@link m10.sdk.M10QueryService#observeTransfers}.
             * @param error Error, if any
             * @param [response] FinalizedTransactions
             */
            type ObserveTransfersCallback = (error: (Error|null), response?: m10.sdk.FinalizedTransactions) => void;

            /**
             * Callback as used by {@link m10.sdk.M10QueryService#getIndexedAccount}.
             * @param error Error, if any
             * @param [response] IndexedAccount
             */
            type GetIndexedAccountCallback = (error: (Error|null), response?: m10.sdk.transaction.IndexedAccount) => void;

            /**
             * Callback as used by {@link m10.sdk.M10QueryService#getAccount}.
             * @param error Error, if any
             * @param [response] Account
             */
            type GetAccountCallback = (error: (Error|null), response?: m10.sdk.model.Account) => void;

            /**
             * Callback as used by {@link m10.sdk.M10QueryService#getAccountInfo}.
             * @param error Error, if any
             * @param [response] AccountInfo
             */
            type GetAccountInfoCallback = (error: (Error|null), response?: m10.sdk.model.AccountInfo) => void;

            /**
             * Callback as used by {@link m10.sdk.M10QueryService#listAccounts}.
             * @param error Error, if any
             * @param [response] ListAccountsResponse
             */
            type ListAccountsCallback = (error: (Error|null), response?: m10.sdk.ListAccountsResponse) => void;

            /**
             * Callback as used by {@link m10.sdk.M10QueryService#observeAccounts}.
             * @param error Error, if any
             * @param [response] FinalizedTransactions
             */
            type ObserveAccountsCallback = (error: (Error|null), response?: m10.sdk.FinalizedTransactions) => void;

            /**
             * Callback as used by {@link m10.sdk.M10QueryService#getAction}.
             * @param error Error, if any
             * @param [response] Action
             */
            type GetActionCallback = (error: (Error|null), response?: m10.sdk.transaction.Action) => void;

            /**
             * Callback as used by {@link m10.sdk.M10QueryService#listActions}.
             * @param error Error, if any
             * @param [response] Actions
             */
            type ListActionsCallback = (error: (Error|null), response?: m10.sdk.transaction.Actions) => void;

            /**
             * Callback as used by {@link m10.sdk.M10QueryService#observeActions}.
             * @param error Error, if any
             * @param [response] FinalizedTransactions
             */
            type ObserveActionsCallback = (error: (Error|null), response?: m10.sdk.FinalizedTransactions) => void;

            /**
             * Callback as used by {@link m10.sdk.M10QueryService#getChainInfo}.
             * @param error Error, if any
             * @param [response] ChainInfo
             */
            type GetChainInfoCallback = (error: (Error|null), response?: m10.sdk.ChainInfo) => void;

            /**
             * Callback as used by {@link m10.sdk.M10QueryService#getTransaction}.
             * @param error Error, if any
             * @param [response] FinalizedTransaction
             */
            type GetTransactionCallback = (error: (Error|null), response?: m10.sdk.FinalizedTransaction) => void;

            /**
             * Callback as used by {@link m10.sdk.M10QueryService#listTransactions}.
             * @param error Error, if any
             * @param [response] FinalizedTransactions
             */
            type ListTransactionsCallback = (error: (Error|null), response?: m10.sdk.FinalizedTransactions) => void;

            /**
             * Callback as used by {@link m10.sdk.M10QueryService#groupTransactions}.
             * @param error Error, if any
             * @param [response] GroupedFinalizedTransactions
             */
            type GroupTransactionsCallback = (error: (Error|null), response?: m10.sdk.GroupedFinalizedTransactions) => void;

            /**
             * Callback as used by {@link m10.sdk.M10QueryService#getAccountSet}.
             * @param error Error, if any
             * @param [response] AccountSet
             */
            type GetAccountSetCallback = (error: (Error|null), response?: m10.sdk.model.AccountSet) => void;

            /**
             * Callback as used by {@link m10.sdk.M10QueryService#listAccountSets}.
             * @param error Error, if any
             * @param [response] ListAccountSetsResponse
             */
            type ListAccountSetsCallback = (error: (Error|null), response?: m10.sdk.ListAccountSetsResponse) => void;

            /**
             * Callback as used by {@link m10.sdk.M10QueryService#getRoleBinding}.
             * @param error Error, if any
             * @param [response] RoleBinding
             */
            type GetRoleBindingCallback = (error: (Error|null), response?: m10.sdk.RoleBinding) => void;

            /**
             * Callback as used by {@link m10.sdk.M10QueryService#listRoleBindings}.
             * @param error Error, if any
             * @param [response] ListRoleBindingsResponse
             */
            type ListRoleBindingsCallback = (error: (Error|null), response?: m10.sdk.ListRoleBindingsResponse) => void;

            /**
             * Callback as used by {@link m10.sdk.M10QueryService#getRole}.
             * @param error Error, if any
             * @param [response] Role
             */
            type GetRoleCallback = (error: (Error|null), response?: m10.sdk.Role) => void;

            /**
             * Callback as used by {@link m10.sdk.M10QueryService#listRoles}.
             * @param error Error, if any
             * @param [response] ListRolesResponse
             */
            type ListRolesCallback = (error: (Error|null), response?: m10.sdk.ListRolesResponse) => void;

            /**
             * Callback as used by {@link m10.sdk.M10QueryService#observeResources}.
             * @param error Error, if any
             * @param [response] FinalizedTransactions
             */
            type ObserveResourcesCallback = (error: (Error|null), response?: m10.sdk.FinalizedTransactions) => void;
        }

        /** Properties of a RequestEnvelope. */
        interface IRequestEnvelope {

            /** RequestEnvelope payload */
            payload?: (Uint8Array|null);

            /** RequestEnvelope signature */
            signature?: (m10.sdk.ISignature|null);
        }

        /** Represents a RequestEnvelope. */
        class RequestEnvelope implements IRequestEnvelope {

            /**
             * Constructs a new RequestEnvelope.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IRequestEnvelope);

            /** RequestEnvelope payload. */
            public payload: Uint8Array;

            /** RequestEnvelope signature. */
            public signature?: (m10.sdk.ISignature|null);

            /**
             * Creates a new RequestEnvelope instance using the specified properties.
             * @param [properties] Properties to set
             * @returns RequestEnvelope instance
             */
            public static create(properties?: m10.sdk.IRequestEnvelope): m10.sdk.RequestEnvelope;

            /**
             * Encodes the specified RequestEnvelope message. Does not implicitly {@link m10.sdk.RequestEnvelope.verify|verify} messages.
             * @param message RequestEnvelope message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IRequestEnvelope, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified RequestEnvelope message, length delimited. Does not implicitly {@link m10.sdk.RequestEnvelope.verify|verify} messages.
             * @param message RequestEnvelope message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IRequestEnvelope, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a RequestEnvelope message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns RequestEnvelope
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.RequestEnvelope;

            /**
             * Decodes a RequestEnvelope message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns RequestEnvelope
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.RequestEnvelope;

            /**
             * Verifies a RequestEnvelope message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a RequestEnvelope message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns RequestEnvelope
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.RequestEnvelope;

            /**
             * Creates a plain object from a RequestEnvelope message. Also converts values to other types if specified.
             * @param message RequestEnvelope
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.RequestEnvelope, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this RequestEnvelope to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a Signature. */
        interface ISignature {

            /** Signature publicKey */
            publicKey?: (Uint8Array|null);

            /** Signature signature */
            signature?: (Uint8Array|null);

            /** Signature algorithm */
            algorithm?: (m10.sdk.Signature.Algorithm|null);
        }

        /** Represents a Signature. */
        class Signature implements ISignature {

            /**
             * Constructs a new Signature.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.ISignature);

            /** Signature publicKey. */
            public publicKey: Uint8Array;

            /** Signature signature. */
            public signature: Uint8Array;

            /** Signature algorithm. */
            public algorithm: m10.sdk.Signature.Algorithm;

            /**
             * Creates a new Signature instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Signature instance
             */
            public static create(properties?: m10.sdk.ISignature): m10.sdk.Signature;

            /**
             * Encodes the specified Signature message. Does not implicitly {@link m10.sdk.Signature.verify|verify} messages.
             * @param message Signature message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.ISignature, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Signature message, length delimited. Does not implicitly {@link m10.sdk.Signature.verify|verify} messages.
             * @param message Signature message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.ISignature, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a Signature message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Signature
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.Signature;

            /**
             * Decodes a Signature message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Signature
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.Signature;

            /**
             * Verifies a Signature message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a Signature message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Signature
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.Signature;

            /**
             * Creates a plain object from a Signature message. Also converts values to other types if specified.
             * @param message Signature
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.Signature, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Signature to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        namespace Signature {

            /** Algorithm enum. */
            enum Algorithm {
                P256_SHA256_ASN1 = 0,
                ED25519 = 1
            }
        }

        /** Properties of a Page. */
        interface IPage {

            /** Page limit */
            limit?: (number|null);

            /** Page lastId */
            lastId?: (Uint8Array|null);
        }

        /** Represents a Page. */
        class Page implements IPage {

            /**
             * Constructs a new Page.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IPage);

            /** Page limit. */
            public limit: number;

            /** Page lastId. */
            public lastId: Uint8Array;

            /**
             * Creates a new Page instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Page instance
             */
            public static create(properties?: m10.sdk.IPage): m10.sdk.Page;

            /**
             * Encodes the specified Page message. Does not implicitly {@link m10.sdk.Page.verify|verify} messages.
             * @param message Page message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IPage, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Page message, length delimited. Does not implicitly {@link m10.sdk.Page.verify|verify} messages.
             * @param message Page message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IPage, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a Page message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Page
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.Page;

            /**
             * Decodes a Page message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Page
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.Page;

            /**
             * Verifies a Page message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a Page message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Page
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.Page;

            /**
             * Creates a plain object from a Page message. Also converts values to other types if specified.
             * @param message Page
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.Page, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Page to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a GetAccountSetRequest. */
        interface IGetAccountSetRequest {

            /** GetAccountSetRequest id */
            id?: (Uint8Array|null);
        }

        /** Represents a GetAccountSetRequest. */
        class GetAccountSetRequest implements IGetAccountSetRequest {

            /**
             * Constructs a new GetAccountSetRequest.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IGetAccountSetRequest);

            /** GetAccountSetRequest id. */
            public id: Uint8Array;

            /**
             * Creates a new GetAccountSetRequest instance using the specified properties.
             * @param [properties] Properties to set
             * @returns GetAccountSetRequest instance
             */
            public static create(properties?: m10.sdk.IGetAccountSetRequest): m10.sdk.GetAccountSetRequest;

            /**
             * Encodes the specified GetAccountSetRequest message. Does not implicitly {@link m10.sdk.GetAccountSetRequest.verify|verify} messages.
             * @param message GetAccountSetRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IGetAccountSetRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified GetAccountSetRequest message, length delimited. Does not implicitly {@link m10.sdk.GetAccountSetRequest.verify|verify} messages.
             * @param message GetAccountSetRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IGetAccountSetRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a GetAccountSetRequest message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns GetAccountSetRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.GetAccountSetRequest;

            /**
             * Decodes a GetAccountSetRequest message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns GetAccountSetRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.GetAccountSetRequest;

            /**
             * Verifies a GetAccountSetRequest message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a GetAccountSetRequest message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns GetAccountSetRequest
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.GetAccountSetRequest;

            /**
             * Creates a plain object from a GetAccountSetRequest message. Also converts values to other types if specified.
             * @param message GetAccountSetRequest
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.GetAccountSetRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this GetAccountSetRequest to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a ListAccountSetsRequest. */
        interface IListAccountSetsRequest {

            /** ListAccountSetsRequest owner */
            owner?: (Uint8Array|null);

            /** ListAccountSetsRequest name */
            name?: (string|null);

            /** ListAccountSetsRequest page */
            page?: (m10.sdk.IPage|null);
        }

        /** Represents a ListAccountSetsRequest. */
        class ListAccountSetsRequest implements IListAccountSetsRequest {

            /**
             * Constructs a new ListAccountSetsRequest.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IListAccountSetsRequest);

            /** ListAccountSetsRequest owner. */
            public owner?: (Uint8Array|null);

            /** ListAccountSetsRequest name. */
            public name?: (string|null);

            /** ListAccountSetsRequest page. */
            public page?: (m10.sdk.IPage|null);

            /** ListAccountSetsRequest filter. */
            public filter?: ("owner"|"name");

            /**
             * Creates a new ListAccountSetsRequest instance using the specified properties.
             * @param [properties] Properties to set
             * @returns ListAccountSetsRequest instance
             */
            public static create(properties?: m10.sdk.IListAccountSetsRequest): m10.sdk.ListAccountSetsRequest;

            /**
             * Encodes the specified ListAccountSetsRequest message. Does not implicitly {@link m10.sdk.ListAccountSetsRequest.verify|verify} messages.
             * @param message ListAccountSetsRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IListAccountSetsRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified ListAccountSetsRequest message, length delimited. Does not implicitly {@link m10.sdk.ListAccountSetsRequest.verify|verify} messages.
             * @param message ListAccountSetsRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IListAccountSetsRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a ListAccountSetsRequest message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns ListAccountSetsRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.ListAccountSetsRequest;

            /**
             * Decodes a ListAccountSetsRequest message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns ListAccountSetsRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.ListAccountSetsRequest;

            /**
             * Verifies a ListAccountSetsRequest message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a ListAccountSetsRequest message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns ListAccountSetsRequest
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.ListAccountSetsRequest;

            /**
             * Creates a plain object from a ListAccountSetsRequest message. Also converts values to other types if specified.
             * @param message ListAccountSetsRequest
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.ListAccountSetsRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this ListAccountSetsRequest to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a ListAccountSetsResponse. */
        interface IListAccountSetsResponse {

            /** ListAccountSetsResponse accountSets */
            accountSets?: (m10.sdk.model.IAccountSet[]|null);

            /** ListAccountSetsResponse nextRequest */
            nextRequest?: (m10.sdk.IListAccountSetsRequest|null);
        }

        /** Represents a ListAccountSetsResponse. */
        class ListAccountSetsResponse implements IListAccountSetsResponse {

            /**
             * Constructs a new ListAccountSetsResponse.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IListAccountSetsResponse);

            /** ListAccountSetsResponse accountSets. */
            public accountSets: m10.sdk.model.IAccountSet[];

            /** ListAccountSetsResponse nextRequest. */
            public nextRequest?: (m10.sdk.IListAccountSetsRequest|null);

            /**
             * Creates a new ListAccountSetsResponse instance using the specified properties.
             * @param [properties] Properties to set
             * @returns ListAccountSetsResponse instance
             */
            public static create(properties?: m10.sdk.IListAccountSetsResponse): m10.sdk.ListAccountSetsResponse;

            /**
             * Encodes the specified ListAccountSetsResponse message. Does not implicitly {@link m10.sdk.ListAccountSetsResponse.verify|verify} messages.
             * @param message ListAccountSetsResponse message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IListAccountSetsResponse, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified ListAccountSetsResponse message, length delimited. Does not implicitly {@link m10.sdk.ListAccountSetsResponse.verify|verify} messages.
             * @param message ListAccountSetsResponse message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IListAccountSetsResponse, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a ListAccountSetsResponse message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns ListAccountSetsResponse
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.ListAccountSetsResponse;

            /**
             * Decodes a ListAccountSetsResponse message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns ListAccountSetsResponse
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.ListAccountSetsResponse;

            /**
             * Verifies a ListAccountSetsResponse message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a ListAccountSetsResponse message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns ListAccountSetsResponse
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.ListAccountSetsResponse;

            /**
             * Creates a plain object from a ListAccountSetsResponse message. Also converts values to other types if specified.
             * @param message ListAccountSetsResponse
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.ListAccountSetsResponse, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this ListAccountSetsResponse to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a ListAccountsRequest. */
        interface IListAccountsRequest {

            /** ListAccountsRequest owner */
            owner?: (Uint8Array|null);

            /** ListAccountsRequest page */
            page?: (m10.sdk.IPage|null);
        }

        /** Represents a ListAccountsRequest. */
        class ListAccountsRequest implements IListAccountsRequest {

            /**
             * Constructs a new ListAccountsRequest.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IListAccountsRequest);

            /** ListAccountsRequest owner. */
            public owner?: (Uint8Array|null);

            /** ListAccountsRequest page. */
            public page?: (m10.sdk.IPage|null);

            /** ListAccountsRequest filter. */
            public filter?: "owner";

            /**
             * Creates a new ListAccountsRequest instance using the specified properties.
             * @param [properties] Properties to set
             * @returns ListAccountsRequest instance
             */
            public static create(properties?: m10.sdk.IListAccountsRequest): m10.sdk.ListAccountsRequest;

            /**
             * Encodes the specified ListAccountsRequest message. Does not implicitly {@link m10.sdk.ListAccountsRequest.verify|verify} messages.
             * @param message ListAccountsRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IListAccountsRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified ListAccountsRequest message, length delimited. Does not implicitly {@link m10.sdk.ListAccountsRequest.verify|verify} messages.
             * @param message ListAccountsRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IListAccountsRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a ListAccountsRequest message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns ListAccountsRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.ListAccountsRequest;

            /**
             * Decodes a ListAccountsRequest message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns ListAccountsRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.ListAccountsRequest;

            /**
             * Verifies a ListAccountsRequest message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a ListAccountsRequest message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns ListAccountsRequest
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.ListAccountsRequest;

            /**
             * Creates a plain object from a ListAccountsRequest message. Also converts values to other types if specified.
             * @param message ListAccountsRequest
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.ListAccountsRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this ListAccountsRequest to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a ListAccountsResponse. */
        interface IListAccountsResponse {

            /** ListAccountsResponse accounts */
            accounts?: (m10.sdk.model.IAccount[]|null);

            /** ListAccountsResponse nextRequest */
            nextRequest?: (m10.sdk.IListAccountsRequest|null);
        }

        /** Represents a ListAccountsResponse. */
        class ListAccountsResponse implements IListAccountsResponse {

            /**
             * Constructs a new ListAccountsResponse.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IListAccountsResponse);

            /** ListAccountsResponse accounts. */
            public accounts: m10.sdk.model.IAccount[];

            /** ListAccountsResponse nextRequest. */
            public nextRequest?: (m10.sdk.IListAccountsRequest|null);

            /**
             * Creates a new ListAccountsResponse instance using the specified properties.
             * @param [properties] Properties to set
             * @returns ListAccountsResponse instance
             */
            public static create(properties?: m10.sdk.IListAccountsResponse): m10.sdk.ListAccountsResponse;

            /**
             * Encodes the specified ListAccountsResponse message. Does not implicitly {@link m10.sdk.ListAccountsResponse.verify|verify} messages.
             * @param message ListAccountsResponse message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IListAccountsResponse, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified ListAccountsResponse message, length delimited. Does not implicitly {@link m10.sdk.ListAccountsResponse.verify|verify} messages.
             * @param message ListAccountsResponse message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IListAccountsResponse, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a ListAccountsResponse message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns ListAccountsResponse
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.ListAccountsResponse;

            /**
             * Decodes a ListAccountsResponse message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns ListAccountsResponse
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.ListAccountsResponse;

            /**
             * Verifies a ListAccountsResponse message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a ListAccountsResponse message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns ListAccountsResponse
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.ListAccountsResponse;

            /**
             * Creates a plain object from a ListAccountsResponse message. Also converts values to other types if specified.
             * @param message ListAccountsResponse
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.ListAccountsResponse, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this ListAccountsResponse to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a GetRoleBindingRequest. */
        interface IGetRoleBindingRequest {

            /** GetRoleBindingRequest id */
            id?: (Uint8Array|null);
        }

        /** Represents a GetRoleBindingRequest. */
        class GetRoleBindingRequest implements IGetRoleBindingRequest {

            /**
             * Constructs a new GetRoleBindingRequest.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IGetRoleBindingRequest);

            /** GetRoleBindingRequest id. */
            public id: Uint8Array;

            /**
             * Creates a new GetRoleBindingRequest instance using the specified properties.
             * @param [properties] Properties to set
             * @returns GetRoleBindingRequest instance
             */
            public static create(properties?: m10.sdk.IGetRoleBindingRequest): m10.sdk.GetRoleBindingRequest;

            /**
             * Encodes the specified GetRoleBindingRequest message. Does not implicitly {@link m10.sdk.GetRoleBindingRequest.verify|verify} messages.
             * @param message GetRoleBindingRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IGetRoleBindingRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified GetRoleBindingRequest message, length delimited. Does not implicitly {@link m10.sdk.GetRoleBindingRequest.verify|verify} messages.
             * @param message GetRoleBindingRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IGetRoleBindingRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a GetRoleBindingRequest message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns GetRoleBindingRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.GetRoleBindingRequest;

            /**
             * Decodes a GetRoleBindingRequest message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns GetRoleBindingRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.GetRoleBindingRequest;

            /**
             * Verifies a GetRoleBindingRequest message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a GetRoleBindingRequest message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns GetRoleBindingRequest
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.GetRoleBindingRequest;

            /**
             * Creates a plain object from a GetRoleBindingRequest message. Also converts values to other types if specified.
             * @param message GetRoleBindingRequest
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.GetRoleBindingRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this GetRoleBindingRequest to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a ListRoleBindingsRequest. */
        interface IListRoleBindingsRequest {

            /** ListRoleBindingsRequest name */
            name?: (string|null);

            /** ListRoleBindingsRequest page */
            page?: (m10.sdk.IPage|null);
        }

        /** Represents a ListRoleBindingsRequest. */
        class ListRoleBindingsRequest implements IListRoleBindingsRequest {

            /**
             * Constructs a new ListRoleBindingsRequest.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IListRoleBindingsRequest);

            /** ListRoleBindingsRequest name. */
            public name?: (string|null);

            /** ListRoleBindingsRequest page. */
            public page?: (m10.sdk.IPage|null);

            /** ListRoleBindingsRequest filter. */
            public filter?: "name";

            /**
             * Creates a new ListRoleBindingsRequest instance using the specified properties.
             * @param [properties] Properties to set
             * @returns ListRoleBindingsRequest instance
             */
            public static create(properties?: m10.sdk.IListRoleBindingsRequest): m10.sdk.ListRoleBindingsRequest;

            /**
             * Encodes the specified ListRoleBindingsRequest message. Does not implicitly {@link m10.sdk.ListRoleBindingsRequest.verify|verify} messages.
             * @param message ListRoleBindingsRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IListRoleBindingsRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified ListRoleBindingsRequest message, length delimited. Does not implicitly {@link m10.sdk.ListRoleBindingsRequest.verify|verify} messages.
             * @param message ListRoleBindingsRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IListRoleBindingsRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a ListRoleBindingsRequest message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns ListRoleBindingsRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.ListRoleBindingsRequest;

            /**
             * Decodes a ListRoleBindingsRequest message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns ListRoleBindingsRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.ListRoleBindingsRequest;

            /**
             * Verifies a ListRoleBindingsRequest message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a ListRoleBindingsRequest message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns ListRoleBindingsRequest
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.ListRoleBindingsRequest;

            /**
             * Creates a plain object from a ListRoleBindingsRequest message. Also converts values to other types if specified.
             * @param message ListRoleBindingsRequest
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.ListRoleBindingsRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this ListRoleBindingsRequest to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a ListRoleBindingsResponse. */
        interface IListRoleBindingsResponse {

            /** ListRoleBindingsResponse roleBindings */
            roleBindings?: (m10.sdk.IRoleBinding[]|null);

            /** ListRoleBindingsResponse nextRequest */
            nextRequest?: (m10.sdk.IListRoleBindingsRequest|null);
        }

        /** Represents a ListRoleBindingsResponse. */
        class ListRoleBindingsResponse implements IListRoleBindingsResponse {

            /**
             * Constructs a new ListRoleBindingsResponse.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IListRoleBindingsResponse);

            /** ListRoleBindingsResponse roleBindings. */
            public roleBindings: m10.sdk.IRoleBinding[];

            /** ListRoleBindingsResponse nextRequest. */
            public nextRequest?: (m10.sdk.IListRoleBindingsRequest|null);

            /**
             * Creates a new ListRoleBindingsResponse instance using the specified properties.
             * @param [properties] Properties to set
             * @returns ListRoleBindingsResponse instance
             */
            public static create(properties?: m10.sdk.IListRoleBindingsResponse): m10.sdk.ListRoleBindingsResponse;

            /**
             * Encodes the specified ListRoleBindingsResponse message. Does not implicitly {@link m10.sdk.ListRoleBindingsResponse.verify|verify} messages.
             * @param message ListRoleBindingsResponse message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IListRoleBindingsResponse, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified ListRoleBindingsResponse message, length delimited. Does not implicitly {@link m10.sdk.ListRoleBindingsResponse.verify|verify} messages.
             * @param message ListRoleBindingsResponse message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IListRoleBindingsResponse, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a ListRoleBindingsResponse message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns ListRoleBindingsResponse
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.ListRoleBindingsResponse;

            /**
             * Decodes a ListRoleBindingsResponse message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns ListRoleBindingsResponse
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.ListRoleBindingsResponse;

            /**
             * Verifies a ListRoleBindingsResponse message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a ListRoleBindingsResponse message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns ListRoleBindingsResponse
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.ListRoleBindingsResponse;

            /**
             * Creates a plain object from a ListRoleBindingsResponse message. Also converts values to other types if specified.
             * @param message ListRoleBindingsResponse
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.ListRoleBindingsResponse, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this ListRoleBindingsResponse to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a GetRoleRequest. */
        interface IGetRoleRequest {

            /** GetRoleRequest id */
            id?: (Uint8Array|null);
        }

        /** Represents a GetRoleRequest. */
        class GetRoleRequest implements IGetRoleRequest {

            /**
             * Constructs a new GetRoleRequest.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IGetRoleRequest);

            /** GetRoleRequest id. */
            public id: Uint8Array;

            /**
             * Creates a new GetRoleRequest instance using the specified properties.
             * @param [properties] Properties to set
             * @returns GetRoleRequest instance
             */
            public static create(properties?: m10.sdk.IGetRoleRequest): m10.sdk.GetRoleRequest;

            /**
             * Encodes the specified GetRoleRequest message. Does not implicitly {@link m10.sdk.GetRoleRequest.verify|verify} messages.
             * @param message GetRoleRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IGetRoleRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified GetRoleRequest message, length delimited. Does not implicitly {@link m10.sdk.GetRoleRequest.verify|verify} messages.
             * @param message GetRoleRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IGetRoleRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a GetRoleRequest message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns GetRoleRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.GetRoleRequest;

            /**
             * Decodes a GetRoleRequest message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns GetRoleRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.GetRoleRequest;

            /**
             * Verifies a GetRoleRequest message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a GetRoleRequest message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns GetRoleRequest
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.GetRoleRequest;

            /**
             * Creates a plain object from a GetRoleRequest message. Also converts values to other types if specified.
             * @param message GetRoleRequest
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.GetRoleRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this GetRoleRequest to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a ListRolesRequest. */
        interface IListRolesRequest {

            /** ListRolesRequest name */
            name?: (string|null);

            /** ListRolesRequest page */
            page?: (m10.sdk.IPage|null);
        }

        /** Represents a ListRolesRequest. */
        class ListRolesRequest implements IListRolesRequest {

            /**
             * Constructs a new ListRolesRequest.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IListRolesRequest);

            /** ListRolesRequest name. */
            public name?: (string|null);

            /** ListRolesRequest page. */
            public page?: (m10.sdk.IPage|null);

            /** ListRolesRequest filter. */
            public filter?: "name";

            /**
             * Creates a new ListRolesRequest instance using the specified properties.
             * @param [properties] Properties to set
             * @returns ListRolesRequest instance
             */
            public static create(properties?: m10.sdk.IListRolesRequest): m10.sdk.ListRolesRequest;

            /**
             * Encodes the specified ListRolesRequest message. Does not implicitly {@link m10.sdk.ListRolesRequest.verify|verify} messages.
             * @param message ListRolesRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IListRolesRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified ListRolesRequest message, length delimited. Does not implicitly {@link m10.sdk.ListRolesRequest.verify|verify} messages.
             * @param message ListRolesRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IListRolesRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a ListRolesRequest message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns ListRolesRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.ListRolesRequest;

            /**
             * Decodes a ListRolesRequest message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns ListRolesRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.ListRolesRequest;

            /**
             * Verifies a ListRolesRequest message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a ListRolesRequest message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns ListRolesRequest
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.ListRolesRequest;

            /**
             * Creates a plain object from a ListRolesRequest message. Also converts values to other types if specified.
             * @param message ListRolesRequest
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.ListRolesRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this ListRolesRequest to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a ListRolesResponse. */
        interface IListRolesResponse {

            /** ListRolesResponse roles */
            roles?: (m10.sdk.IRole[]|null);

            /** ListRolesResponse nextRequest */
            nextRequest?: (m10.sdk.IListRolesRequest|null);
        }

        /** Represents a ListRolesResponse. */
        class ListRolesResponse implements IListRolesResponse {

            /**
             * Constructs a new ListRolesResponse.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IListRolesResponse);

            /** ListRolesResponse roles. */
            public roles: m10.sdk.IRole[];

            /** ListRolesResponse nextRequest. */
            public nextRequest?: (m10.sdk.IListRolesRequest|null);

            /**
             * Creates a new ListRolesResponse instance using the specified properties.
             * @param [properties] Properties to set
             * @returns ListRolesResponse instance
             */
            public static create(properties?: m10.sdk.IListRolesResponse): m10.sdk.ListRolesResponse;

            /**
             * Encodes the specified ListRolesResponse message. Does not implicitly {@link m10.sdk.ListRolesResponse.verify|verify} messages.
             * @param message ListRolesResponse message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IListRolesResponse, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified ListRolesResponse message, length delimited. Does not implicitly {@link m10.sdk.ListRolesResponse.verify|verify} messages.
             * @param message ListRolesResponse message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IListRolesResponse, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a ListRolesResponse message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns ListRolesResponse
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.ListRolesResponse;

            /**
             * Decodes a ListRolesResponse message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns ListRolesResponse
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.ListRolesResponse;

            /**
             * Verifies a ListRolesResponse message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a ListRolesResponse message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns ListRolesResponse
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.ListRolesResponse;

            /**
             * Creates a plain object from a ListRolesResponse message. Also converts values to other types if specified.
             * @param message ListRolesResponse
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.ListRolesResponse, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this ListRolesResponse to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a GetTransactionRequest. */
        interface IGetTransactionRequest {

            /** GetTransactionRequest txId */
            txId?: (number|Long|null);
        }

        /** Represents a GetTransactionRequest. */
        class GetTransactionRequest implements IGetTransactionRequest {

            /**
             * Constructs a new GetTransactionRequest.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IGetTransactionRequest);

            /** GetTransactionRequest txId. */
            public txId: (number|Long);

            /**
             * Creates a new GetTransactionRequest instance using the specified properties.
             * @param [properties] Properties to set
             * @returns GetTransactionRequest instance
             */
            public static create(properties?: m10.sdk.IGetTransactionRequest): m10.sdk.GetTransactionRequest;

            /**
             * Encodes the specified GetTransactionRequest message. Does not implicitly {@link m10.sdk.GetTransactionRequest.verify|verify} messages.
             * @param message GetTransactionRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IGetTransactionRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified GetTransactionRequest message, length delimited. Does not implicitly {@link m10.sdk.GetTransactionRequest.verify|verify} messages.
             * @param message GetTransactionRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IGetTransactionRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a GetTransactionRequest message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns GetTransactionRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.GetTransactionRequest;

            /**
             * Decodes a GetTransactionRequest message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns GetTransactionRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.GetTransactionRequest;

            /**
             * Verifies a GetTransactionRequest message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a GetTransactionRequest message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns GetTransactionRequest
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.GetTransactionRequest;

            /**
             * Creates a plain object from a GetTransactionRequest message. Also converts values to other types if specified.
             * @param message GetTransactionRequest
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.GetTransactionRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this GetTransactionRequest to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a ListTransactionsRequest. */
        interface IListTransactionsRequest {

            /** ListTransactionsRequest contextId */
            contextId?: (Uint8Array|null);

            /** ListTransactionsRequest limit */
            limit?: (number|Long|null);

            /** ListTransactionsRequest minTxId */
            minTxId?: (number|Long|null);

            /** ListTransactionsRequest maxTxId */
            maxTxId?: (number|Long|null);
        }

        /** Represents a ListTransactionsRequest. */
        class ListTransactionsRequest implements IListTransactionsRequest {

            /**
             * Constructs a new ListTransactionsRequest.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IListTransactionsRequest);

            /** ListTransactionsRequest contextId. */
            public contextId: Uint8Array;

            /** ListTransactionsRequest limit. */
            public limit: (number|Long);

            /** ListTransactionsRequest minTxId. */
            public minTxId: (number|Long);

            /** ListTransactionsRequest maxTxId. */
            public maxTxId: (number|Long);

            /**
             * Creates a new ListTransactionsRequest instance using the specified properties.
             * @param [properties] Properties to set
             * @returns ListTransactionsRequest instance
             */
            public static create(properties?: m10.sdk.IListTransactionsRequest): m10.sdk.ListTransactionsRequest;

            /**
             * Encodes the specified ListTransactionsRequest message. Does not implicitly {@link m10.sdk.ListTransactionsRequest.verify|verify} messages.
             * @param message ListTransactionsRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IListTransactionsRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified ListTransactionsRequest message, length delimited. Does not implicitly {@link m10.sdk.ListTransactionsRequest.verify|verify} messages.
             * @param message ListTransactionsRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IListTransactionsRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a ListTransactionsRequest message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns ListTransactionsRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.ListTransactionsRequest;

            /**
             * Decodes a ListTransactionsRequest message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns ListTransactionsRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.ListTransactionsRequest;

            /**
             * Verifies a ListTransactionsRequest message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a ListTransactionsRequest message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns ListTransactionsRequest
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.ListTransactionsRequest;

            /**
             * Creates a plain object from a ListTransactionsRequest message. Also converts values to other types if specified.
             * @param message ListTransactionsRequest
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.ListTransactionsRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this ListTransactionsRequest to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a GroupTransactionsRequest. */
        interface IGroupTransactionsRequest {

            /** GroupTransactionsRequest accountId */
            accountId?: (Uint8Array|null);

            /** GroupTransactionsRequest limitGroups */
            limitGroups?: (number|Long|null);

            /** GroupTransactionsRequest minTxId */
            minTxId?: (number|Long|null);

            /** GroupTransactionsRequest maxTxId */
            maxTxId?: (number|Long|null);
        }

        /** Represents a GroupTransactionsRequest. */
        class GroupTransactionsRequest implements IGroupTransactionsRequest {

            /**
             * Constructs a new GroupTransactionsRequest.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IGroupTransactionsRequest);

            /** GroupTransactionsRequest accountId. */
            public accountId: Uint8Array;

            /** GroupTransactionsRequest limitGroups. */
            public limitGroups: (number|Long);

            /** GroupTransactionsRequest minTxId. */
            public minTxId: (number|Long);

            /** GroupTransactionsRequest maxTxId. */
            public maxTxId: (number|Long);

            /**
             * Creates a new GroupTransactionsRequest instance using the specified properties.
             * @param [properties] Properties to set
             * @returns GroupTransactionsRequest instance
             */
            public static create(properties?: m10.sdk.IGroupTransactionsRequest): m10.sdk.GroupTransactionsRequest;

            /**
             * Encodes the specified GroupTransactionsRequest message. Does not implicitly {@link m10.sdk.GroupTransactionsRequest.verify|verify} messages.
             * @param message GroupTransactionsRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IGroupTransactionsRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified GroupTransactionsRequest message, length delimited. Does not implicitly {@link m10.sdk.GroupTransactionsRequest.verify|verify} messages.
             * @param message GroupTransactionsRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IGroupTransactionsRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a GroupTransactionsRequest message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns GroupTransactionsRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.GroupTransactionsRequest;

            /**
             * Decodes a GroupTransactionsRequest message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns GroupTransactionsRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.GroupTransactionsRequest;

            /**
             * Verifies a GroupTransactionsRequest message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a GroupTransactionsRequest message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns GroupTransactionsRequest
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.GroupTransactionsRequest;

            /**
             * Creates a plain object from a GroupTransactionsRequest message. Also converts values to other types if specified.
             * @param message GroupTransactionsRequest
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.GroupTransactionsRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this GroupTransactionsRequest to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of an ObserveAccountsRequest. */
        interface IObserveAccountsRequest {

            /** ObserveAccountsRequest startingFrom */
            startingFrom?: (m10.sdk.ITxId|null);

            /** ObserveAccountsRequest involvedAccounts */
            involvedAccounts?: (Uint8Array[]|null);
        }

        /** Represents an ObserveAccountsRequest. */
        class ObserveAccountsRequest implements IObserveAccountsRequest {

            /**
             * Constructs a new ObserveAccountsRequest.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IObserveAccountsRequest);

            /** ObserveAccountsRequest startingFrom. */
            public startingFrom?: (m10.sdk.ITxId|null);

            /** ObserveAccountsRequest involvedAccounts. */
            public involvedAccounts: Uint8Array[];

            /**
             * Creates a new ObserveAccountsRequest instance using the specified properties.
             * @param [properties] Properties to set
             * @returns ObserveAccountsRequest instance
             */
            public static create(properties?: m10.sdk.IObserveAccountsRequest): m10.sdk.ObserveAccountsRequest;

            /**
             * Encodes the specified ObserveAccountsRequest message. Does not implicitly {@link m10.sdk.ObserveAccountsRequest.verify|verify} messages.
             * @param message ObserveAccountsRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IObserveAccountsRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified ObserveAccountsRequest message, length delimited. Does not implicitly {@link m10.sdk.ObserveAccountsRequest.verify|verify} messages.
             * @param message ObserveAccountsRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IObserveAccountsRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an ObserveAccountsRequest message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns ObserveAccountsRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.ObserveAccountsRequest;

            /**
             * Decodes an ObserveAccountsRequest message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns ObserveAccountsRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.ObserveAccountsRequest;

            /**
             * Verifies an ObserveAccountsRequest message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an ObserveAccountsRequest message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns ObserveAccountsRequest
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.ObserveAccountsRequest;

            /**
             * Creates a plain object from an ObserveAccountsRequest message. Also converts values to other types if specified.
             * @param message ObserveAccountsRequest
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.ObserveAccountsRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this ObserveAccountsRequest to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of an ObserveResourcesRequest. */
        interface IObserveResourcesRequest {

            /** ObserveResourcesRequest expression */
            expression?: (m10.sdk.IExp|null);

            /** ObserveResourcesRequest collection */
            collection?: (string|null);

            /** ObserveResourcesRequest startingFrom */
            startingFrom?: (m10.sdk.ITxId|null);
        }

        /** Represents an ObserveResourcesRequest. */
        class ObserveResourcesRequest implements IObserveResourcesRequest {

            /**
             * Constructs a new ObserveResourcesRequest.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IObserveResourcesRequest);

            /** ObserveResourcesRequest expression. */
            public expression?: (m10.sdk.IExp|null);

            /** ObserveResourcesRequest collection. */
            public collection: string;

            /** ObserveResourcesRequest startingFrom. */
            public startingFrom?: (m10.sdk.ITxId|null);

            /**
             * Creates a new ObserveResourcesRequest instance using the specified properties.
             * @param [properties] Properties to set
             * @returns ObserveResourcesRequest instance
             */
            public static create(properties?: m10.sdk.IObserveResourcesRequest): m10.sdk.ObserveResourcesRequest;

            /**
             * Encodes the specified ObserveResourcesRequest message. Does not implicitly {@link m10.sdk.ObserveResourcesRequest.verify|verify} messages.
             * @param message ObserveResourcesRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IObserveResourcesRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified ObserveResourcesRequest message, length delimited. Does not implicitly {@link m10.sdk.ObserveResourcesRequest.verify|verify} messages.
             * @param message ObserveResourcesRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IObserveResourcesRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an ObserveResourcesRequest message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns ObserveResourcesRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.ObserveResourcesRequest;

            /**
             * Decodes an ObserveResourcesRequest message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns ObserveResourcesRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.ObserveResourcesRequest;

            /**
             * Verifies an ObserveResourcesRequest message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an ObserveResourcesRequest message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns ObserveResourcesRequest
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.ObserveResourcesRequest;

            /**
             * Creates a plain object from an ObserveResourcesRequest message. Also converts values to other types if specified.
             * @param message ObserveResourcesRequest
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.ObserveResourcesRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this ObserveResourcesRequest to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a TxId. */
        interface ITxId {

            /** TxId txId */
            txId?: (number|Long|null);
        }

        /** Represents a TxId. */
        class TxId implements ITxId {

            /**
             * Constructs a new TxId.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.ITxId);

            /** TxId txId. */
            public txId: (number|Long);

            /**
             * Creates a new TxId instance using the specified properties.
             * @param [properties] Properties to set
             * @returns TxId instance
             */
            public static create(properties?: m10.sdk.ITxId): m10.sdk.TxId;

            /**
             * Encodes the specified TxId message. Does not implicitly {@link m10.sdk.TxId.verify|verify} messages.
             * @param message TxId message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.ITxId, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified TxId message, length delimited. Does not implicitly {@link m10.sdk.TxId.verify|verify} messages.
             * @param message TxId message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.ITxId, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a TxId message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns TxId
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.TxId;

            /**
             * Decodes a TxId message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns TxId
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.TxId;

            /**
             * Verifies a TxId message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a TxId message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns TxId
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.TxId;

            /**
             * Creates a plain object from a TxId message. Also converts values to other types if specified.
             * @param message TxId
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.TxId, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this TxId to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of an ObserveActionsRequest. */
        interface IObserveActionsRequest {

            /** ObserveActionsRequest startingFrom */
            startingFrom?: (m10.sdk.ITxId|null);

            /** ObserveActionsRequest name */
            name?: (string|null);

            /** ObserveActionsRequest involvesAccounts */
            involvesAccounts?: (Uint8Array[]|null);
        }

        /** Represents an ObserveActionsRequest. */
        class ObserveActionsRequest implements IObserveActionsRequest {

            /**
             * Constructs a new ObserveActionsRequest.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IObserveActionsRequest);

            /** ObserveActionsRequest startingFrom. */
            public startingFrom?: (m10.sdk.ITxId|null);

            /** ObserveActionsRequest name. */
            public name: string;

            /** ObserveActionsRequest involvesAccounts. */
            public involvesAccounts: Uint8Array[];

            /**
             * Creates a new ObserveActionsRequest instance using the specified properties.
             * @param [properties] Properties to set
             * @returns ObserveActionsRequest instance
             */
            public static create(properties?: m10.sdk.IObserveActionsRequest): m10.sdk.ObserveActionsRequest;

            /**
             * Encodes the specified ObserveActionsRequest message. Does not implicitly {@link m10.sdk.ObserveActionsRequest.verify|verify} messages.
             * @param message ObserveActionsRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IObserveActionsRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified ObserveActionsRequest message, length delimited. Does not implicitly {@link m10.sdk.ObserveActionsRequest.verify|verify} messages.
             * @param message ObserveActionsRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IObserveActionsRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an ObserveActionsRequest message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns ObserveActionsRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.ObserveActionsRequest;

            /**
             * Decodes an ObserveActionsRequest message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns ObserveActionsRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.ObserveActionsRequest;

            /**
             * Verifies an ObserveActionsRequest message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an ObserveActionsRequest message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns ObserveActionsRequest
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.ObserveActionsRequest;

            /**
             * Creates a plain object from an ObserveActionsRequest message. Also converts values to other types if specified.
             * @param message ObserveActionsRequest
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.ObserveActionsRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this ObserveActionsRequest to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a FinalizedTransaction. */
        interface IFinalizedTransaction {

            /** FinalizedTransaction request */
            request?: (m10.sdk.transaction.ITransactionRequestPayload|null);

            /** FinalizedTransaction response */
            response?: (m10.sdk.transaction.ITransactionResponse|null);
        }

        /** Represents a FinalizedTransaction. */
        class FinalizedTransaction implements IFinalizedTransaction {

            /**
             * Constructs a new FinalizedTransaction.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IFinalizedTransaction);

            /** FinalizedTransaction request. */
            public request?: (m10.sdk.transaction.ITransactionRequestPayload|null);

            /** FinalizedTransaction response. */
            public response?: (m10.sdk.transaction.ITransactionResponse|null);

            /**
             * Creates a new FinalizedTransaction instance using the specified properties.
             * @param [properties] Properties to set
             * @returns FinalizedTransaction instance
             */
            public static create(properties?: m10.sdk.IFinalizedTransaction): m10.sdk.FinalizedTransaction;

            /**
             * Encodes the specified FinalizedTransaction message. Does not implicitly {@link m10.sdk.FinalizedTransaction.verify|verify} messages.
             * @param message FinalizedTransaction message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IFinalizedTransaction, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified FinalizedTransaction message, length delimited. Does not implicitly {@link m10.sdk.FinalizedTransaction.verify|verify} messages.
             * @param message FinalizedTransaction message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IFinalizedTransaction, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a FinalizedTransaction message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns FinalizedTransaction
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.FinalizedTransaction;

            /**
             * Decodes a FinalizedTransaction message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns FinalizedTransaction
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.FinalizedTransaction;

            /**
             * Verifies a FinalizedTransaction message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a FinalizedTransaction message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns FinalizedTransaction
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.FinalizedTransaction;

            /**
             * Creates a plain object from a FinalizedTransaction message. Also converts values to other types if specified.
             * @param message FinalizedTransaction
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.FinalizedTransaction, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this FinalizedTransaction to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a FinalizedTransactions. */
        interface IFinalizedTransactions {

            /** FinalizedTransactions transactions */
            transactions?: (m10.sdk.IFinalizedTransaction[]|null);
        }

        /** Represents a FinalizedTransactions. */
        class FinalizedTransactions implements IFinalizedTransactions {

            /**
             * Constructs a new FinalizedTransactions.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IFinalizedTransactions);

            /** FinalizedTransactions transactions. */
            public transactions: m10.sdk.IFinalizedTransaction[];

            /**
             * Creates a new FinalizedTransactions instance using the specified properties.
             * @param [properties] Properties to set
             * @returns FinalizedTransactions instance
             */
            public static create(properties?: m10.sdk.IFinalizedTransactions): m10.sdk.FinalizedTransactions;

            /**
             * Encodes the specified FinalizedTransactions message. Does not implicitly {@link m10.sdk.FinalizedTransactions.verify|verify} messages.
             * @param message FinalizedTransactions message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IFinalizedTransactions, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified FinalizedTransactions message, length delimited. Does not implicitly {@link m10.sdk.FinalizedTransactions.verify|verify} messages.
             * @param message FinalizedTransactions message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IFinalizedTransactions, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a FinalizedTransactions message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns FinalizedTransactions
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.FinalizedTransactions;

            /**
             * Decodes a FinalizedTransactions message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns FinalizedTransactions
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.FinalizedTransactions;

            /**
             * Verifies a FinalizedTransactions message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a FinalizedTransactions message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns FinalizedTransactions
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.FinalizedTransactions;

            /**
             * Creates a plain object from a FinalizedTransactions message. Also converts values to other types if specified.
             * @param message FinalizedTransactions
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.FinalizedTransactions, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this FinalizedTransactions to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a GroupedFinalizedTransactions. */
        interface IGroupedFinalizedTransactions {

            /** GroupedFinalizedTransactions groups */
            groups?: (m10.sdk.IFinalizedTransactions[]|null);
        }

        /** Represents a GroupedFinalizedTransactions. */
        class GroupedFinalizedTransactions implements IGroupedFinalizedTransactions {

            /**
             * Constructs a new GroupedFinalizedTransactions.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IGroupedFinalizedTransactions);

            /** GroupedFinalizedTransactions groups. */
            public groups: m10.sdk.IFinalizedTransactions[];

            /**
             * Creates a new GroupedFinalizedTransactions instance using the specified properties.
             * @param [properties] Properties to set
             * @returns GroupedFinalizedTransactions instance
             */
            public static create(properties?: m10.sdk.IGroupedFinalizedTransactions): m10.sdk.GroupedFinalizedTransactions;

            /**
             * Encodes the specified GroupedFinalizedTransactions message. Does not implicitly {@link m10.sdk.GroupedFinalizedTransactions.verify|verify} messages.
             * @param message GroupedFinalizedTransactions message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IGroupedFinalizedTransactions, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified GroupedFinalizedTransactions message, length delimited. Does not implicitly {@link m10.sdk.GroupedFinalizedTransactions.verify|verify} messages.
             * @param message GroupedFinalizedTransactions message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IGroupedFinalizedTransactions, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a GroupedFinalizedTransactions message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns GroupedFinalizedTransactions
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.GroupedFinalizedTransactions;

            /**
             * Decodes a GroupedFinalizedTransactions message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns GroupedFinalizedTransactions
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.GroupedFinalizedTransactions;

            /**
             * Verifies a GroupedFinalizedTransactions message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a GroupedFinalizedTransactions message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns GroupedFinalizedTransactions
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.GroupedFinalizedTransactions;

            /**
             * Creates a plain object from a GroupedFinalizedTransactions message. Also converts values to other types if specified.
             * @param message GroupedFinalizedTransactions
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.GroupedFinalizedTransactions, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this GroupedFinalizedTransactions to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a ChainInfo. */
        interface IChainInfo {

            /** ChainInfo blockHeight */
            blockHeight?: (number|Long|null);
        }

        /** Represents a ChainInfo. */
        class ChainInfo implements IChainInfo {

            /**
             * Constructs a new ChainInfo.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IChainInfo);

            /** ChainInfo blockHeight. */
            public blockHeight: (number|Long);

            /**
             * Creates a new ChainInfo instance using the specified properties.
             * @param [properties] Properties to set
             * @returns ChainInfo instance
             */
            public static create(properties?: m10.sdk.IChainInfo): m10.sdk.ChainInfo;

            /**
             * Encodes the specified ChainInfo message. Does not implicitly {@link m10.sdk.ChainInfo.verify|verify} messages.
             * @param message ChainInfo message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IChainInfo, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified ChainInfo message, length delimited. Does not implicitly {@link m10.sdk.ChainInfo.verify|verify} messages.
             * @param message ChainInfo message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IChainInfo, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a ChainInfo message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns ChainInfo
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.ChainInfo;

            /**
             * Decodes a ChainInfo message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns ChainInfo
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.ChainInfo;

            /**
             * Verifies a ChainInfo message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a ChainInfo message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns ChainInfo
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.ChainInfo;

            /**
             * Creates a plain object from a ChainInfo message. Also converts values to other types if specified.
             * @param message ChainInfo
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.ChainInfo, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this ChainInfo to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a DocumentOperations. */
        interface IDocumentOperations {

            /** DocumentOperations operations */
            operations?: (m10.sdk.IOperation[]|null);
        }

        /** Represents a DocumentOperations. */
        class DocumentOperations implements IDocumentOperations {

            /**
             * Constructs a new DocumentOperations.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IDocumentOperations);

            /** DocumentOperations operations. */
            public operations: m10.sdk.IOperation[];

            /**
             * Creates a new DocumentOperations instance using the specified properties.
             * @param [properties] Properties to set
             * @returns DocumentOperations instance
             */
            public static create(properties?: m10.sdk.IDocumentOperations): m10.sdk.DocumentOperations;

            /**
             * Encodes the specified DocumentOperations message. Does not implicitly {@link m10.sdk.DocumentOperations.verify|verify} messages.
             * @param message DocumentOperations message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IDocumentOperations, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified DocumentOperations message, length delimited. Does not implicitly {@link m10.sdk.DocumentOperations.verify|verify} messages.
             * @param message DocumentOperations message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IDocumentOperations, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a DocumentOperations message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns DocumentOperations
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.DocumentOperations;

            /**
             * Decodes a DocumentOperations message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns DocumentOperations
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.DocumentOperations;

            /**
             * Verifies a DocumentOperations message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a DocumentOperations message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns DocumentOperations
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.DocumentOperations;

            /**
             * Creates a plain object from a DocumentOperations message. Also converts values to other types if specified.
             * @param message DocumentOperations
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.DocumentOperations, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this DocumentOperations to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of an Operation. */
        interface IOperation {

            /** Operation insertDocument */
            insertDocument?: (m10.sdk.Operation.IInsertDocument|null);

            /** Operation updateDocument */
            updateDocument?: (m10.sdk.Operation.IUpdateDocument|null);

            /** Operation deleteDocument */
            deleteDocument?: (m10.sdk.Operation.IDeleteDocument|null);

            /** Operation insertCollection */
            insertCollection?: (m10.sdk.ICollectionMetadata|null);

            /** Operation insertIndex */
            insertIndex?: (m10.sdk.Operation.IInsertIndex|null);
        }

        /** Represents an Operation. */
        class Operation implements IOperation {

            /**
             * Constructs a new Operation.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IOperation);

            /** Operation insertDocument. */
            public insertDocument?: (m10.sdk.Operation.IInsertDocument|null);

            /** Operation updateDocument. */
            public updateDocument?: (m10.sdk.Operation.IUpdateDocument|null);

            /** Operation deleteDocument. */
            public deleteDocument?: (m10.sdk.Operation.IDeleteDocument|null);

            /** Operation insertCollection. */
            public insertCollection?: (m10.sdk.ICollectionMetadata|null);

            /** Operation insertIndex. */
            public insertIndex?: (m10.sdk.Operation.IInsertIndex|null);

            /** Operation operation. */
            public operation?: ("insertDocument"|"updateDocument"|"deleteDocument"|"insertCollection"|"insertIndex");

            /**
             * Creates a new Operation instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Operation instance
             */
            public static create(properties?: m10.sdk.IOperation): m10.sdk.Operation;

            /**
             * Encodes the specified Operation message. Does not implicitly {@link m10.sdk.Operation.verify|verify} messages.
             * @param message Operation message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IOperation, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Operation message, length delimited. Does not implicitly {@link m10.sdk.Operation.verify|verify} messages.
             * @param message Operation message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IOperation, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an Operation message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Operation
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.Operation;

            /**
             * Decodes an Operation message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Operation
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.Operation;

            /**
             * Verifies an Operation message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an Operation message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Operation
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.Operation;

            /**
             * Creates a plain object from an Operation message. Also converts values to other types if specified.
             * @param message Operation
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.Operation, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Operation to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        namespace Operation {

            /** Properties of an InsertDocument. */
            interface IInsertDocument {

                /** InsertDocument collection */
                collection?: (string|null);

                /** InsertDocument document */
                document?: (Uint8Array|null);
            }

            /** Represents an InsertDocument. */
            class InsertDocument implements IInsertDocument {

                /**
                 * Constructs a new InsertDocument.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.Operation.IInsertDocument);

                /** InsertDocument collection. */
                public collection: string;

                /** InsertDocument document. */
                public document: Uint8Array;

                /**
                 * Creates a new InsertDocument instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns InsertDocument instance
                 */
                public static create(properties?: m10.sdk.Operation.IInsertDocument): m10.sdk.Operation.InsertDocument;

                /**
                 * Encodes the specified InsertDocument message. Does not implicitly {@link m10.sdk.Operation.InsertDocument.verify|verify} messages.
                 * @param message InsertDocument message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.Operation.IInsertDocument, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified InsertDocument message, length delimited. Does not implicitly {@link m10.sdk.Operation.InsertDocument.verify|verify} messages.
                 * @param message InsertDocument message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.Operation.IInsertDocument, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes an InsertDocument message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns InsertDocument
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.Operation.InsertDocument;

                /**
                 * Decodes an InsertDocument message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns InsertDocument
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.Operation.InsertDocument;

                /**
                 * Verifies an InsertDocument message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates an InsertDocument message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns InsertDocument
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.Operation.InsertDocument;

                /**
                 * Creates a plain object from an InsertDocument message. Also converts values to other types if specified.
                 * @param message InsertDocument
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.Operation.InsertDocument, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this InsertDocument to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of an UpdateDocument. */
            interface IUpdateDocument {

                /** UpdateDocument collection */
                collection?: (string|null);

                /** UpdateDocument primaryKey */
                primaryKey?: (m10.sdk.IValue|null);

                /** UpdateDocument document */
                document?: (Uint8Array|null);

                /** UpdateDocument fieldMask */
                fieldMask?: (google.protobuf.IFieldMask|null);

                /** UpdateDocument mergeRepeated */
                mergeRepeated?: (boolean|null);
            }

            /** Represents an UpdateDocument. */
            class UpdateDocument implements IUpdateDocument {

                /**
                 * Constructs a new UpdateDocument.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.Operation.IUpdateDocument);

                /** UpdateDocument collection. */
                public collection: string;

                /** UpdateDocument primaryKey. */
                public primaryKey?: (m10.sdk.IValue|null);

                /** UpdateDocument document. */
                public document: Uint8Array;

                /** UpdateDocument fieldMask. */
                public fieldMask?: (google.protobuf.IFieldMask|null);

                /** UpdateDocument mergeRepeated. */
                public mergeRepeated: boolean;

                /**
                 * Creates a new UpdateDocument instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns UpdateDocument instance
                 */
                public static create(properties?: m10.sdk.Operation.IUpdateDocument): m10.sdk.Operation.UpdateDocument;

                /**
                 * Encodes the specified UpdateDocument message. Does not implicitly {@link m10.sdk.Operation.UpdateDocument.verify|verify} messages.
                 * @param message UpdateDocument message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.Operation.IUpdateDocument, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified UpdateDocument message, length delimited. Does not implicitly {@link m10.sdk.Operation.UpdateDocument.verify|verify} messages.
                 * @param message UpdateDocument message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.Operation.IUpdateDocument, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes an UpdateDocument message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns UpdateDocument
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.Operation.UpdateDocument;

                /**
                 * Decodes an UpdateDocument message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns UpdateDocument
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.Operation.UpdateDocument;

                /**
                 * Verifies an UpdateDocument message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates an UpdateDocument message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns UpdateDocument
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.Operation.UpdateDocument;

                /**
                 * Creates a plain object from an UpdateDocument message. Also converts values to other types if specified.
                 * @param message UpdateDocument
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.Operation.UpdateDocument, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this UpdateDocument to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a DeleteDocument. */
            interface IDeleteDocument {

                /** DeleteDocument collection */
                collection?: (string|null);

                /** DeleteDocument primaryKey */
                primaryKey?: (m10.sdk.IValue|null);
            }

            /** Represents a DeleteDocument. */
            class DeleteDocument implements IDeleteDocument {

                /**
                 * Constructs a new DeleteDocument.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.Operation.IDeleteDocument);

                /** DeleteDocument collection. */
                public collection: string;

                /** DeleteDocument primaryKey. */
                public primaryKey?: (m10.sdk.IValue|null);

                /**
                 * Creates a new DeleteDocument instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns DeleteDocument instance
                 */
                public static create(properties?: m10.sdk.Operation.IDeleteDocument): m10.sdk.Operation.DeleteDocument;

                /**
                 * Encodes the specified DeleteDocument message. Does not implicitly {@link m10.sdk.Operation.DeleteDocument.verify|verify} messages.
                 * @param message DeleteDocument message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.Operation.IDeleteDocument, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified DeleteDocument message, length delimited. Does not implicitly {@link m10.sdk.Operation.DeleteDocument.verify|verify} messages.
                 * @param message DeleteDocument message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.Operation.IDeleteDocument, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a DeleteDocument message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns DeleteDocument
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.Operation.DeleteDocument;

                /**
                 * Decodes a DeleteDocument message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns DeleteDocument
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.Operation.DeleteDocument;

                /**
                 * Verifies a DeleteDocument message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a DeleteDocument message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns DeleteDocument
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.Operation.DeleteDocument;

                /**
                 * Creates a plain object from a DeleteDocument message. Also converts values to other types if specified.
                 * @param message DeleteDocument
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.Operation.DeleteDocument, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this DeleteDocument to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of an InsertIndex. */
            interface IInsertIndex {

                /** InsertIndex collection */
                collection?: (string|null);

                /** InsertIndex path */
                path?: (string|null);
            }

            /** Represents an InsertIndex. */
            class InsertIndex implements IInsertIndex {

                /**
                 * Constructs a new InsertIndex.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.Operation.IInsertIndex);

                /** InsertIndex collection. */
                public collection: string;

                /** InsertIndex path. */
                public path: string;

                /**
                 * Creates a new InsertIndex instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns InsertIndex instance
                 */
                public static create(properties?: m10.sdk.Operation.IInsertIndex): m10.sdk.Operation.InsertIndex;

                /**
                 * Encodes the specified InsertIndex message. Does not implicitly {@link m10.sdk.Operation.InsertIndex.verify|verify} messages.
                 * @param message InsertIndex message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.Operation.IInsertIndex, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified InsertIndex message, length delimited. Does not implicitly {@link m10.sdk.Operation.InsertIndex.verify|verify} messages.
                 * @param message InsertIndex message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.Operation.IInsertIndex, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes an InsertIndex message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns InsertIndex
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.Operation.InsertIndex;

                /**
                 * Decodes an InsertIndex message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns InsertIndex
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.Operation.InsertIndex;

                /**
                 * Verifies an InsertIndex message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates an InsertIndex message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns InsertIndex
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.Operation.InsertIndex;

                /**
                 * Creates a plain object from an InsertIndex message. Also converts values to other types if specified.
                 * @param message InsertIndex
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.Operation.InsertIndex, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this InsertIndex to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }
        }

        /** Properties of a CollectionMetadata. */
        interface ICollectionMetadata {

            /** CollectionMetadata name */
            name?: (string|null);

            /** CollectionMetadata descriptorName */
            descriptorName?: (string|null);

            /** CollectionMetadata fileDescriptorSet */
            fileDescriptorSet?: (google.protobuf.IFileDescriptorSet|null);

            /** CollectionMetadata indexMetadata */
            indexMetadata?: (m10.sdk.IIndexMetadata[]|null);

            /** CollectionMetadata primaryKeyPath */
            primaryKeyPath?: (string|null);
        }

        /** Represents a CollectionMetadata. */
        class CollectionMetadata implements ICollectionMetadata {

            /**
             * Constructs a new CollectionMetadata.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.ICollectionMetadata);

            /** CollectionMetadata name. */
            public name: string;

            /** CollectionMetadata descriptorName. */
            public descriptorName: string;

            /** CollectionMetadata fileDescriptorSet. */
            public fileDescriptorSet?: (google.protobuf.IFileDescriptorSet|null);

            /** CollectionMetadata indexMetadata. */
            public indexMetadata: m10.sdk.IIndexMetadata[];

            /** CollectionMetadata primaryKeyPath. */
            public primaryKeyPath: string;

            /**
             * Creates a new CollectionMetadata instance using the specified properties.
             * @param [properties] Properties to set
             * @returns CollectionMetadata instance
             */
            public static create(properties?: m10.sdk.ICollectionMetadata): m10.sdk.CollectionMetadata;

            /**
             * Encodes the specified CollectionMetadata message. Does not implicitly {@link m10.sdk.CollectionMetadata.verify|verify} messages.
             * @param message CollectionMetadata message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.ICollectionMetadata, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified CollectionMetadata message, length delimited. Does not implicitly {@link m10.sdk.CollectionMetadata.verify|verify} messages.
             * @param message CollectionMetadata message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.ICollectionMetadata, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a CollectionMetadata message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns CollectionMetadata
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.CollectionMetadata;

            /**
             * Decodes a CollectionMetadata message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns CollectionMetadata
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.CollectionMetadata;

            /**
             * Verifies a CollectionMetadata message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a CollectionMetadata message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns CollectionMetadata
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.CollectionMetadata;

            /**
             * Creates a plain object from a CollectionMetadata message. Also converts values to other types if specified.
             * @param message CollectionMetadata
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.CollectionMetadata, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this CollectionMetadata to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of an IndexMetadata. */
        interface IIndexMetadata {

            /** IndexMetadata path */
            path?: (string[]|null);
        }

        /** Represents an IndexMetadata. */
        class IndexMetadata implements IIndexMetadata {

            /**
             * Constructs a new IndexMetadata.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IIndexMetadata);

            /** IndexMetadata path. */
            public path: string[];

            /**
             * Creates a new IndexMetadata instance using the specified properties.
             * @param [properties] Properties to set
             * @returns IndexMetadata instance
             */
            public static create(properties?: m10.sdk.IIndexMetadata): m10.sdk.IndexMetadata;

            /**
             * Encodes the specified IndexMetadata message. Does not implicitly {@link m10.sdk.IndexMetadata.verify|verify} messages.
             * @param message IndexMetadata message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IIndexMetadata, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified IndexMetadata message, length delimited. Does not implicitly {@link m10.sdk.IndexMetadata.verify|verify} messages.
             * @param message IndexMetadata message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IIndexMetadata, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an IndexMetadata message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns IndexMetadata
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.IndexMetadata;

            /**
             * Decodes an IndexMetadata message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns IndexMetadata
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.IndexMetadata;

            /**
             * Verifies an IndexMetadata message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an IndexMetadata message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns IndexMetadata
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.IndexMetadata;

            /**
             * Creates a plain object from an IndexMetadata message. Also converts values to other types if specified.
             * @param message IndexMetadata
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.IndexMetadata, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this IndexMetadata to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of an Exp. */
        interface IExp {

            /** Exp exp */
            exp?: (string|null);

            /** Exp vars */
            vars?: ({ [k: string]: m10.sdk.IValue }|null);
        }

        /** Represents an Exp. */
        class Exp implements IExp {

            /**
             * Constructs a new Exp.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IExp);

            /** Exp exp. */
            public exp: string;

            /** Exp vars. */
            public vars: { [k: string]: m10.sdk.IValue };

            /**
             * Creates a new Exp instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Exp instance
             */
            public static create(properties?: m10.sdk.IExp): m10.sdk.Exp;

            /**
             * Encodes the specified Exp message. Does not implicitly {@link m10.sdk.Exp.verify|verify} messages.
             * @param message Exp message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IExp, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Exp message, length delimited. Does not implicitly {@link m10.sdk.Exp.verify|verify} messages.
             * @param message Exp message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IExp, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an Exp message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Exp
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.Exp;

            /**
             * Decodes an Exp message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Exp
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.Exp;

            /**
             * Verifies an Exp message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an Exp message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Exp
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.Exp;

            /**
             * Creates a plain object from an Exp message. Also converts values to other types if specified.
             * @param message Exp
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.Exp, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Exp to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a Value. */
        interface IValue {

            /** Value stringValue */
            stringValue?: (string|null);

            /** Value int8Value */
            int8Value?: (number|null);

            /** Value int16Value */
            int16Value?: (number|null);

            /** Value int32Value */
            int32Value?: (number|null);

            /** Value int64Value */
            int64Value?: (number|Long|null);

            /** Value uint8Value */
            uint8Value?: (number|null);

            /** Value uint16Value */
            uint16Value?: (number|null);

            /** Value uint32Value */
            uint32Value?: (number|null);

            /** Value uint64Value */
            uint64Value?: (number|Long|null);

            /** Value doubleValue */
            doubleValue?: (number|null);

            /** Value floatValue */
            floatValue?: (number|null);

            /** Value boolValue */
            boolValue?: (boolean|null);

            /** Value bytesValue */
            bytesValue?: (Uint8Array|null);
        }

        /** Represents a Value. */
        class Value implements IValue {

            /**
             * Constructs a new Value.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IValue);

            /** Value stringValue. */
            public stringValue?: (string|null);

            /** Value int8Value. */
            public int8Value?: (number|null);

            /** Value int16Value. */
            public int16Value?: (number|null);

            /** Value int32Value. */
            public int32Value?: (number|null);

            /** Value int64Value. */
            public int64Value?: (number|Long|null);

            /** Value uint8Value. */
            public uint8Value?: (number|null);

            /** Value uint16Value. */
            public uint16Value?: (number|null);

            /** Value uint32Value. */
            public uint32Value?: (number|null);

            /** Value uint64Value. */
            public uint64Value?: (number|Long|null);

            /** Value doubleValue. */
            public doubleValue?: (number|null);

            /** Value floatValue. */
            public floatValue?: (number|null);

            /** Value boolValue. */
            public boolValue?: (boolean|null);

            /** Value bytesValue. */
            public bytesValue?: (Uint8Array|null);

            /** Value value. */
            public value?: ("stringValue"|"int8Value"|"int16Value"|"int32Value"|"int64Value"|"uint8Value"|"uint16Value"|"uint32Value"|"uint64Value"|"doubleValue"|"floatValue"|"boolValue"|"bytesValue");

            /**
             * Creates a new Value instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Value instance
             */
            public static create(properties?: m10.sdk.IValue): m10.sdk.Value;

            /**
             * Encodes the specified Value message. Does not implicitly {@link m10.sdk.Value.verify|verify} messages.
             * @param message Value message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IValue, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Value message, length delimited. Does not implicitly {@link m10.sdk.Value.verify|verify} messages.
             * @param message Value message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IValue, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a Value message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Value
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.Value;

            /**
             * Decodes a Value message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Value
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.Value;

            /**
             * Verifies a Value message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a Value message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Value
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.Value;

            /**
             * Creates a plain object from a Value message. Also converts values to other types if specified.
             * @param message Value
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.Value, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Value to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a QueryRequest. */
        interface IQueryRequest {

            /** QueryRequest collection */
            collection?: (string|null);

            /** QueryRequest expression */
            expression?: (m10.sdk.IExp|null);

            /** QueryRequest publicKey */
            publicKey?: (Uint8Array|null);
        }

        /** Represents a QueryRequest. */
        class QueryRequest implements IQueryRequest {

            /**
             * Constructs a new QueryRequest.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IQueryRequest);

            /** QueryRequest collection. */
            public collection: string;

            /** QueryRequest expression. */
            public expression?: (m10.sdk.IExp|null);

            /** QueryRequest publicKey. */
            public publicKey?: (Uint8Array|null);

            /** QueryRequest _publicKey. */
            public _publicKey?: "publicKey";

            /**
             * Creates a new QueryRequest instance using the specified properties.
             * @param [properties] Properties to set
             * @returns QueryRequest instance
             */
            public static create(properties?: m10.sdk.IQueryRequest): m10.sdk.QueryRequest;

            /**
             * Encodes the specified QueryRequest message. Does not implicitly {@link m10.sdk.QueryRequest.verify|verify} messages.
             * @param message QueryRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IQueryRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified QueryRequest message, length delimited. Does not implicitly {@link m10.sdk.QueryRequest.verify|verify} messages.
             * @param message QueryRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IQueryRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a QueryRequest message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns QueryRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.QueryRequest;

            /**
             * Decodes a QueryRequest message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns QueryRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.QueryRequest;

            /**
             * Verifies a QueryRequest message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a QueryRequest message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns QueryRequest
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.QueryRequest;

            /**
             * Creates a plain object from a QueryRequest message. Also converts values to other types if specified.
             * @param message QueryRequest
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.QueryRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this QueryRequest to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Namespace metadata. */
        namespace metadata {

            /** Properties of an Attachment. */
            interface IAttachment {

                /** Attachment objectId */
                objectId?: (string|null);

                /** Attachment type */
                type?: (m10.sdk.metadata.Attachment.AttachmentType|null);
            }

            /** Represents an Attachment. */
            class Attachment implements IAttachment {

                /**
                 * Constructs a new Attachment.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.metadata.IAttachment);

                /** Attachment objectId. */
                public objectId: string;

                /** Attachment type. */
                public type: m10.sdk.metadata.Attachment.AttachmentType;

                /**
                 * Creates a new Attachment instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Attachment instance
                 */
                public static create(properties?: m10.sdk.metadata.IAttachment): m10.sdk.metadata.Attachment;

                /**
                 * Encodes the specified Attachment message. Does not implicitly {@link m10.sdk.metadata.Attachment.verify|verify} messages.
                 * @param message Attachment message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.metadata.IAttachment, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Attachment message, length delimited. Does not implicitly {@link m10.sdk.metadata.Attachment.verify|verify} messages.
                 * @param message Attachment message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.metadata.IAttachment, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes an Attachment message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Attachment
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.metadata.Attachment;

                /**
                 * Decodes an Attachment message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Attachment
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.metadata.Attachment;

                /**
                 * Verifies an Attachment message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates an Attachment message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Attachment
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.metadata.Attachment;

                /**
                 * Creates a plain object from an Attachment message. Also converts values to other types if specified.
                 * @param message Attachment
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.metadata.Attachment, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Attachment to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            namespace Attachment {

                /** AttachmentType enum. */
                enum AttachmentType {
                    OBJECT = 0,
                    IMAGE = 1
                }
            }

            /** Properties of a Memo. */
            interface IMemo {

                /** Memo plaintext */
                plaintext?: (string|null);
            }

            /** Represents a Memo. */
            class Memo implements IMemo {

                /**
                 * Constructs a new Memo.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.metadata.IMemo);

                /** Memo plaintext. */
                public plaintext: string;

                /**
                 * Creates a new Memo instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Memo instance
                 */
                public static create(properties?: m10.sdk.metadata.IMemo): m10.sdk.metadata.Memo;

                /**
                 * Encodes the specified Memo message. Does not implicitly {@link m10.sdk.metadata.Memo.verify|verify} messages.
                 * @param message Memo message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.metadata.IMemo, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Memo message, length delimited. Does not implicitly {@link m10.sdk.metadata.Memo.verify|verify} messages.
                 * @param message Memo message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.metadata.IMemo, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Memo message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Memo
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.metadata.Memo;

                /**
                 * Decodes a Memo message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Memo
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.metadata.Memo;

                /**
                 * Verifies a Memo message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Memo message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Memo
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.metadata.Memo;

                /**
                 * Creates a plain object from a Memo message. Also converts values to other types if specified.
                 * @param message Memo
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.metadata.Memo, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Memo to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a Fee. */
            interface IFee {
            }

            /** Represents a Fee. */
            class Fee implements IFee {

                /**
                 * Constructs a new Fee.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.metadata.IFee);

                /**
                 * Creates a new Fee instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Fee instance
                 */
                public static create(properties?: m10.sdk.metadata.IFee): m10.sdk.metadata.Fee;

                /**
                 * Encodes the specified Fee message. Does not implicitly {@link m10.sdk.metadata.Fee.verify|verify} messages.
                 * @param message Fee message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.metadata.IFee, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Fee message, length delimited. Does not implicitly {@link m10.sdk.metadata.Fee.verify|verify} messages.
                 * @param message Fee message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.metadata.IFee, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Fee message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Fee
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.metadata.Fee;

                /**
                 * Decodes a Fee message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Fee
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.metadata.Fee;

                /**
                 * Verifies a Fee message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Fee message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Fee
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.metadata.Fee;

                /**
                 * Creates a plain object from a Fee message. Also converts values to other types if specified.
                 * @param message Fee
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.metadata.Fee, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Fee to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a Withdraw. */
            interface IWithdraw {

                /** Withdraw bankAccountId */
                bankAccountId?: (string|null);
            }

            /** Represents a Withdraw. */
            class Withdraw implements IWithdraw {

                /**
                 * Constructs a new Withdraw.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.metadata.IWithdraw);

                /** Withdraw bankAccountId. */
                public bankAccountId: string;

                /**
                 * Creates a new Withdraw instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Withdraw instance
                 */
                public static create(properties?: m10.sdk.metadata.IWithdraw): m10.sdk.metadata.Withdraw;

                /**
                 * Encodes the specified Withdraw message. Does not implicitly {@link m10.sdk.metadata.Withdraw.verify|verify} messages.
                 * @param message Withdraw message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.metadata.IWithdraw, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Withdraw message, length delimited. Does not implicitly {@link m10.sdk.metadata.Withdraw.verify|verify} messages.
                 * @param message Withdraw message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.metadata.IWithdraw, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Withdraw message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Withdraw
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.metadata.Withdraw;

                /**
                 * Decodes a Withdraw message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Withdraw
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.metadata.Withdraw;

                /**
                 * Verifies a Withdraw message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Withdraw message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Withdraw
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.metadata.Withdraw;

                /**
                 * Creates a plain object from a Withdraw message. Also converts values to other types if specified.
                 * @param message Withdraw
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.metadata.Withdraw, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Withdraw to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a Deposit. */
            interface IDeposit {

                /** Deposit bankAccountId */
                bankAccountId?: (string|null);
            }

            /** Represents a Deposit. */
            class Deposit implements IDeposit {

                /**
                 * Constructs a new Deposit.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.metadata.IDeposit);

                /** Deposit bankAccountId. */
                public bankAccountId: string;

                /**
                 * Creates a new Deposit instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Deposit instance
                 */
                public static create(properties?: m10.sdk.metadata.IDeposit): m10.sdk.metadata.Deposit;

                /**
                 * Encodes the specified Deposit message. Does not implicitly {@link m10.sdk.metadata.Deposit.verify|verify} messages.
                 * @param message Deposit message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.metadata.IDeposit, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Deposit message, length delimited. Does not implicitly {@link m10.sdk.metadata.Deposit.verify|verify} messages.
                 * @param message Deposit message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.metadata.IDeposit, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Deposit message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Deposit
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.metadata.Deposit;

                /**
                 * Decodes a Deposit message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Deposit
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.metadata.Deposit;

                /**
                 * Verifies a Deposit message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Deposit message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Deposit
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.metadata.Deposit;

                /**
                 * Creates a plain object from a Deposit message. Also converts values to other types if specified.
                 * @param message Deposit
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.metadata.Deposit, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Deposit to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a Contract. */
            interface IContract {

                /** Contract transactions */
                transactions?: (Uint8Array|null);

                /** Contract endorsements */
                endorsements?: (m10.sdk.metadata.IEndorsement[]|null);
            }

            /** Represents a Contract. */
            class Contract implements IContract {

                /**
                 * Constructs a new Contract.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.metadata.IContract);

                /** Contract transactions. */
                public transactions: Uint8Array;

                /** Contract endorsements. */
                public endorsements: m10.sdk.metadata.IEndorsement[];

                /**
                 * Creates a new Contract instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Contract instance
                 */
                public static create(properties?: m10.sdk.metadata.IContract): m10.sdk.metadata.Contract;

                /**
                 * Encodes the specified Contract message. Does not implicitly {@link m10.sdk.metadata.Contract.verify|verify} messages.
                 * @param message Contract message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.metadata.IContract, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Contract message, length delimited. Does not implicitly {@link m10.sdk.metadata.Contract.verify|verify} messages.
                 * @param message Contract message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.metadata.IContract, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Contract message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Contract
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.metadata.Contract;

                /**
                 * Decodes a Contract message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Contract
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.metadata.Contract;

                /**
                 * Verifies a Contract message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Contract message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Contract
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.metadata.Contract;

                /**
                 * Creates a plain object from a Contract message. Also converts values to other types if specified.
                 * @param message Contract
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.metadata.Contract, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Contract to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of an Endorsement. */
            interface IEndorsement {

                /** Endorsement ledgerId */
                ledgerId?: (string|null);

                /** Endorsement signature */
                signature?: (m10.sdk.ISignature|null);
            }

            /** Represents an Endorsement. */
            class Endorsement implements IEndorsement {

                /**
                 * Constructs a new Endorsement.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.metadata.IEndorsement);

                /** Endorsement ledgerId. */
                public ledgerId: string;

                /** Endorsement signature. */
                public signature?: (m10.sdk.ISignature|null);

                /**
                 * Creates a new Endorsement instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Endorsement instance
                 */
                public static create(properties?: m10.sdk.metadata.IEndorsement): m10.sdk.metadata.Endorsement;

                /**
                 * Encodes the specified Endorsement message. Does not implicitly {@link m10.sdk.metadata.Endorsement.verify|verify} messages.
                 * @param message Endorsement message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.metadata.IEndorsement, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Endorsement message, length delimited. Does not implicitly {@link m10.sdk.metadata.Endorsement.verify|verify} messages.
                 * @param message Endorsement message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.metadata.IEndorsement, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes an Endorsement message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Endorsement
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.metadata.Endorsement;

                /**
                 * Decodes an Endorsement message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Endorsement
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.metadata.Endorsement;

                /**
                 * Verifies an Endorsement message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates an Endorsement message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Endorsement
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.metadata.Endorsement;

                /**
                 * Creates a plain object from an Endorsement message. Also converts values to other types if specified.
                 * @param message Endorsement
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.metadata.Endorsement, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Endorsement to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a PaymentRequest. */
            interface IPaymentRequest {

                /** PaymentRequest transfer */
                transfer?: (m10.sdk.transaction.ICreateTransfer|null);

                /** PaymentRequest status */
                status?: (m10.sdk.metadata.PaymentRequest.PaymentRequestStatus|null);
            }

            /** Represents a PaymentRequest. */
            class PaymentRequest implements IPaymentRequest {

                /**
                 * Constructs a new PaymentRequest.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.metadata.IPaymentRequest);

                /** PaymentRequest transfer. */
                public transfer?: (m10.sdk.transaction.ICreateTransfer|null);

                /** PaymentRequest status. */
                public status: m10.sdk.metadata.PaymentRequest.PaymentRequestStatus;

                /**
                 * Creates a new PaymentRequest instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns PaymentRequest instance
                 */
                public static create(properties?: m10.sdk.metadata.IPaymentRequest): m10.sdk.metadata.PaymentRequest;

                /**
                 * Encodes the specified PaymentRequest message. Does not implicitly {@link m10.sdk.metadata.PaymentRequest.verify|verify} messages.
                 * @param message PaymentRequest message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.metadata.IPaymentRequest, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified PaymentRequest message, length delimited. Does not implicitly {@link m10.sdk.metadata.PaymentRequest.verify|verify} messages.
                 * @param message PaymentRequest message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.metadata.IPaymentRequest, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a PaymentRequest message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns PaymentRequest
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.metadata.PaymentRequest;

                /**
                 * Decodes a PaymentRequest message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns PaymentRequest
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.metadata.PaymentRequest;

                /**
                 * Verifies a PaymentRequest message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a PaymentRequest message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns PaymentRequest
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.metadata.PaymentRequest;

                /**
                 * Creates a plain object from a PaymentRequest message. Also converts values to other types if specified.
                 * @param message PaymentRequest
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.metadata.PaymentRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this PaymentRequest to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            namespace PaymentRequest {

                /** PaymentRequestStatus enum. */
                enum PaymentRequestStatus {
                    PENDING = 0,
                    DECLINED = 1,
                    CANCELED = 2,
                    IN_PROGRESS = 3
                }
            }
        }

        /** Namespace model. */
        namespace model {

            /** Properties of an Account. */
            interface IAccount {

                /** Account owner */
                owner?: (Uint8Array|null);

                /** Account profileImageUrl */
                profileImageUrl?: (string|null);

                /** Account name */
                name?: (string|null);

                /** Account publicName */
                publicName?: (string|null);

                /** Account id */
                id?: (Uint8Array|null);
            }

            /** Represents an Account. */
            class Account implements IAccount {

                /**
                 * Constructs a new Account.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.model.IAccount);

                /** Account owner. */
                public owner: Uint8Array;

                /** Account profileImageUrl. */
                public profileImageUrl: string;

                /** Account name. */
                public name: string;

                /** Account publicName. */
                public publicName: string;

                /** Account id. */
                public id: Uint8Array;

                /**
                 * Creates a new Account instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Account instance
                 */
                public static create(properties?: m10.sdk.model.IAccount): m10.sdk.model.Account;

                /**
                 * Encodes the specified Account message. Does not implicitly {@link m10.sdk.model.Account.verify|verify} messages.
                 * @param message Account message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.model.IAccount, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Account message, length delimited. Does not implicitly {@link m10.sdk.model.Account.verify|verify} messages.
                 * @param message Account message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.model.IAccount, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes an Account message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Account
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.model.Account;

                /**
                 * Decodes an Account message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Account
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.model.Account;

                /**
                 * Verifies an Account message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates an Account message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Account
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.model.Account;

                /**
                 * Creates a plain object from an Account message. Also converts values to other types if specified.
                 * @param message Account
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.model.Account, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Account to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of an AccountRef. */
            interface IAccountRef {

                /** AccountRef ledgerId */
                ledgerId?: (string|null);

                /** AccountRef accountId */
                accountId?: (Uint8Array|null);
            }

            /** Represents an AccountRef. */
            class AccountRef implements IAccountRef {

                /**
                 * Constructs a new AccountRef.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.model.IAccountRef);

                /** AccountRef ledgerId. */
                public ledgerId: string;

                /** AccountRef accountId. */
                public accountId: Uint8Array;

                /**
                 * Creates a new AccountRef instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns AccountRef instance
                 */
                public static create(properties?: m10.sdk.model.IAccountRef): m10.sdk.model.AccountRef;

                /**
                 * Encodes the specified AccountRef message. Does not implicitly {@link m10.sdk.model.AccountRef.verify|verify} messages.
                 * @param message AccountRef message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.model.IAccountRef, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified AccountRef message, length delimited. Does not implicitly {@link m10.sdk.model.AccountRef.verify|verify} messages.
                 * @param message AccountRef message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.model.IAccountRef, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes an AccountRef message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns AccountRef
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.model.AccountRef;

                /**
                 * Decodes an AccountRef message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns AccountRef
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.model.AccountRef;

                /**
                 * Verifies an AccountRef message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates an AccountRef message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns AccountRef
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.model.AccountRef;

                /**
                 * Creates a plain object from an AccountRef message. Also converts values to other types if specified.
                 * @param message AccountRef
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.model.AccountRef, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this AccountRef to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of an AccountSet. */
            interface IAccountSet {

                /** AccountSet owner */
                owner?: (Uint8Array|null);

                /** AccountSet accounts */
                accounts?: (m10.sdk.model.IAccountRef[]|null);

                /** AccountSet id */
                id?: (Uint8Array|null);
            }

            /** Represents an AccountSet. */
            class AccountSet implements IAccountSet {

                /**
                 * Constructs a new AccountSet.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.model.IAccountSet);

                /** AccountSet owner. */
                public owner: Uint8Array;

                /** AccountSet accounts. */
                public accounts: m10.sdk.model.IAccountRef[];

                /** AccountSet id. */
                public id: Uint8Array;

                /**
                 * Creates a new AccountSet instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns AccountSet instance
                 */
                public static create(properties?: m10.sdk.model.IAccountSet): m10.sdk.model.AccountSet;

                /**
                 * Encodes the specified AccountSet message. Does not implicitly {@link m10.sdk.model.AccountSet.verify|verify} messages.
                 * @param message AccountSet message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.model.IAccountSet, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified AccountSet message, length delimited. Does not implicitly {@link m10.sdk.model.AccountSet.verify|verify} messages.
                 * @param message AccountSet message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.model.IAccountSet, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes an AccountSet message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns AccountSet
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.model.AccountSet;

                /**
                 * Decodes an AccountSet message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns AccountSet
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.model.AccountSet;

                /**
                 * Verifies an AccountSet message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates an AccountSet message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns AccountSet
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.model.AccountSet;

                /**
                 * Creates a plain object from an AccountSet message. Also converts values to other types if specified.
                 * @param message AccountSet
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.model.AccountSet, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this AccountSet to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of an AccountInfo. */
            interface IAccountInfo {

                /** AccountInfo accountId */
                accountId?: (Uint8Array|null);

                /** AccountInfo parentAccountId */
                parentAccountId?: (Uint8Array|null);

                /** AccountInfo publicName */
                publicName?: (string|null);

                /** AccountInfo profileImageUrl */
                profileImageUrl?: (string|null);

                /** AccountInfo code */
                code?: (string|null);

                /** AccountInfo decimalPlaces */
                decimalPlaces?: (number|null);
            }

            /** Represents an AccountInfo. */
            class AccountInfo implements IAccountInfo {

                /**
                 * Constructs a new AccountInfo.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.model.IAccountInfo);

                /** AccountInfo accountId. */
                public accountId: Uint8Array;

                /** AccountInfo parentAccountId. */
                public parentAccountId: Uint8Array;

                /** AccountInfo publicName. */
                public publicName: string;

                /** AccountInfo profileImageUrl. */
                public profileImageUrl: string;

                /** AccountInfo code. */
                public code: string;

                /** AccountInfo decimalPlaces. */
                public decimalPlaces: number;

                /**
                 * Creates a new AccountInfo instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns AccountInfo instance
                 */
                public static create(properties?: m10.sdk.model.IAccountInfo): m10.sdk.model.AccountInfo;

                /**
                 * Encodes the specified AccountInfo message. Does not implicitly {@link m10.sdk.model.AccountInfo.verify|verify} messages.
                 * @param message AccountInfo message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.model.IAccountInfo, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified AccountInfo message, length delimited. Does not implicitly {@link m10.sdk.model.AccountInfo.verify|verify} messages.
                 * @param message AccountInfo message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.model.IAccountInfo, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes an AccountInfo message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns AccountInfo
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.model.AccountInfo;

                /**
                 * Decodes an AccountInfo message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns AccountInfo
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.model.AccountInfo;

                /**
                 * Verifies an AccountInfo message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates an AccountInfo message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns AccountInfo
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.model.AccountInfo;

                /**
                 * Creates a plain object from an AccountInfo message. Also converts values to other types if specified.
                 * @param message AccountInfo
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.model.AccountInfo, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this AccountInfo to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }
        }

        /** Properties of a RoleBinding. */
        interface IRoleBinding {

            /** RoleBinding id */
            id?: (Uint8Array|null);

            /** RoleBinding name */
            name?: (string|null);

            /** RoleBinding role */
            role?: (Uint8Array|null);

            /** RoleBinding subjects */
            subjects?: (Uint8Array[]|null);

            /** RoleBinding expressions */
            expressions?: (m10.sdk.IExpression[]|null);

            /** RoleBinding isUniversal */
            isUniversal?: (boolean|null);

            /** RoleBinding owner */
            owner?: (Uint8Array|null);
        }

        /** Represents a RoleBinding. */
        class RoleBinding implements IRoleBinding {

            /**
             * Constructs a new RoleBinding.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IRoleBinding);

            /** RoleBinding id. */
            public id: Uint8Array;

            /** RoleBinding name. */
            public name: string;

            /** RoleBinding role. */
            public role: Uint8Array;

            /** RoleBinding subjects. */
            public subjects: Uint8Array[];

            /** RoleBinding expressions. */
            public expressions: m10.sdk.IExpression[];

            /** RoleBinding isUniversal. */
            public isUniversal: boolean;

            /** RoleBinding owner. */
            public owner: Uint8Array;

            /**
             * Creates a new RoleBinding instance using the specified properties.
             * @param [properties] Properties to set
             * @returns RoleBinding instance
             */
            public static create(properties?: m10.sdk.IRoleBinding): m10.sdk.RoleBinding;

            /**
             * Encodes the specified RoleBinding message. Does not implicitly {@link m10.sdk.RoleBinding.verify|verify} messages.
             * @param message RoleBinding message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IRoleBinding, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified RoleBinding message, length delimited. Does not implicitly {@link m10.sdk.RoleBinding.verify|verify} messages.
             * @param message RoleBinding message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IRoleBinding, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a RoleBinding message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns RoleBinding
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.RoleBinding;

            /**
             * Decodes a RoleBinding message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns RoleBinding
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.RoleBinding;

            /**
             * Verifies a RoleBinding message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a RoleBinding message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns RoleBinding
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.RoleBinding;

            /**
             * Creates a plain object from a RoleBinding message. Also converts values to other types if specified.
             * @param message RoleBinding
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.RoleBinding, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this RoleBinding to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of an Expression. */
        interface IExpression {

            /** Expression collection */
            collection?: (string|null);

            /** Expression expression */
            expression?: (string|null);
        }

        /** Represents an Expression. */
        class Expression implements IExpression {

            /**
             * Constructs a new Expression.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IExpression);

            /** Expression collection. */
            public collection: string;

            /** Expression expression. */
            public expression: string;

            /**
             * Creates a new Expression instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Expression instance
             */
            public static create(properties?: m10.sdk.IExpression): m10.sdk.Expression;

            /**
             * Encodes the specified Expression message. Does not implicitly {@link m10.sdk.Expression.verify|verify} messages.
             * @param message Expression message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IExpression, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Expression message, length delimited. Does not implicitly {@link m10.sdk.Expression.verify|verify} messages.
             * @param message Expression message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IExpression, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an Expression message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Expression
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.Expression;

            /**
             * Decodes an Expression message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Expression
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.Expression;

            /**
             * Verifies an Expression message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an Expression message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Expression
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.Expression;

            /**
             * Creates a plain object from an Expression message. Also converts values to other types if specified.
             * @param message Expression
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.Expression, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Expression to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a Role. */
        interface IRole {

            /** Role id */
            id?: (Uint8Array|null);

            /** Role owner */
            owner?: (Uint8Array|null);

            /** Role name */
            name?: (string|null);

            /** Role rules */
            rules?: (m10.sdk.IRule[]|null);
        }

        /** Represents a Role. */
        class Role implements IRole {

            /**
             * Constructs a new Role.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IRole);

            /** Role id. */
            public id: Uint8Array;

            /** Role owner. */
            public owner: Uint8Array;

            /** Role name. */
            public name: string;

            /** Role rules. */
            public rules: m10.sdk.IRule[];

            /**
             * Creates a new Role instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Role instance
             */
            public static create(properties?: m10.sdk.IRole): m10.sdk.Role;

            /**
             * Encodes the specified Role message. Does not implicitly {@link m10.sdk.Role.verify|verify} messages.
             * @param message Role message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IRole, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Role message, length delimited. Does not implicitly {@link m10.sdk.Role.verify|verify} messages.
             * @param message Role message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IRole, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a Role message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Role
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.Role;

            /**
             * Decodes a Role message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Role
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.Role;

            /**
             * Verifies a Role message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a Role message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Role
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.Role;

            /**
             * Creates a plain object from a Role message. Also converts values to other types if specified.
             * @param message Role
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.Role, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Role to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a Rule. */
        interface IRule {

            /** Rule collection */
            collection?: (string|null);

            /** Rule instanceKeys */
            instanceKeys?: (m10.sdk.IValue[]|null);

            /** Rule verbs */
            verbs?: (m10.sdk.Rule.Verb[]|null);
        }

        /** Represents a Rule. */
        class Rule implements IRule {

            /**
             * Constructs a new Rule.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.sdk.IRule);

            /** Rule collection. */
            public collection: string;

            /** Rule instanceKeys. */
            public instanceKeys: m10.sdk.IValue[];

            /** Rule verbs. */
            public verbs: m10.sdk.Rule.Verb[];

            /**
             * Creates a new Rule instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Rule instance
             */
            public static create(properties?: m10.sdk.IRule): m10.sdk.Rule;

            /**
             * Encodes the specified Rule message. Does not implicitly {@link m10.sdk.Rule.verify|verify} messages.
             * @param message Rule message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.sdk.IRule, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Rule message, length delimited. Does not implicitly {@link m10.sdk.Rule.verify|verify} messages.
             * @param message Rule message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.sdk.IRule, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a Rule message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Rule
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.Rule;

            /**
             * Decodes a Rule message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Rule
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.Rule;

            /**
             * Verifies a Rule message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a Rule message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Rule
             */
            public static fromObject(object: { [k: string]: any }): m10.sdk.Rule;

            /**
             * Creates a plain object from a Rule message. Also converts values to other types if specified.
             * @param message Rule
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.sdk.Rule, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Rule to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        namespace Rule {

            /** Verb enum. */
            enum Verb {
                READ = 0,
                CREATE = 1,
                UPDATE = 2,
                DELETE = 3,
                TRANSACT = 4,
                INITIATE = 5,
                COMMIT = 6
            }
        }

        /** Namespace transaction. */
        namespace transaction {

            /** Properties of a TransactionRequestPayload. */
            interface ITransactionRequestPayload {

                /** TransactionRequestPayload nonce */
                nonce?: (number|Long|null);

                /** TransactionRequestPayload timestamp */
                timestamp?: (number|Long|null);

                /** TransactionRequestPayload contextId */
                contextId?: (Uint8Array|null);

                /** TransactionRequestPayload data */
                data?: (m10.sdk.transaction.ITransactionData|null);
            }

            /** Represents a TransactionRequestPayload. */
            class TransactionRequestPayload implements ITransactionRequestPayload {

                /**
                 * Constructs a new TransactionRequestPayload.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.ITransactionRequestPayload);

                /** TransactionRequestPayload nonce. */
                public nonce: (number|Long);

                /** TransactionRequestPayload timestamp. */
                public timestamp: (number|Long);

                /** TransactionRequestPayload contextId. */
                public contextId: Uint8Array;

                /** TransactionRequestPayload data. */
                public data?: (m10.sdk.transaction.ITransactionData|null);

                /**
                 * Creates a new TransactionRequestPayload instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns TransactionRequestPayload instance
                 */
                public static create(properties?: m10.sdk.transaction.ITransactionRequestPayload): m10.sdk.transaction.TransactionRequestPayload;

                /**
                 * Encodes the specified TransactionRequestPayload message. Does not implicitly {@link m10.sdk.transaction.TransactionRequestPayload.verify|verify} messages.
                 * @param message TransactionRequestPayload message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.ITransactionRequestPayload, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified TransactionRequestPayload message, length delimited. Does not implicitly {@link m10.sdk.transaction.TransactionRequestPayload.verify|verify} messages.
                 * @param message TransactionRequestPayload message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.ITransactionRequestPayload, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a TransactionRequestPayload message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns TransactionRequestPayload
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.TransactionRequestPayload;

                /**
                 * Decodes a TransactionRequestPayload message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns TransactionRequestPayload
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.TransactionRequestPayload;

                /**
                 * Verifies a TransactionRequestPayload message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a TransactionRequestPayload message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns TransactionRequestPayload
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.TransactionRequestPayload;

                /**
                 * Creates a plain object from a TransactionRequestPayload message. Also converts values to other types if specified.
                 * @param message TransactionRequestPayload
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.TransactionRequestPayload, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this TransactionRequestPayload to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a TransactionData. */
            interface ITransactionData {

                /** TransactionData transfer */
                transfer?: (m10.sdk.transaction.ICreateTransfer|null);

                /** TransactionData createLedgerAccount */
                createLedgerAccount?: (m10.sdk.transaction.ICreateLedgerAccount|null);

                /** TransactionData setFreezeState */
                setFreezeState?: (m10.sdk.transaction.ISetFreezeState|null);

                /** TransactionData documentOperations */
                documentOperations?: (m10.sdk.IDocumentOperations|null);

                /** TransactionData invokeAction */
                invokeAction?: (m10.sdk.transaction.IInvokeAction|null);

                /** TransactionData initiateTransfer */
                initiateTransfer?: (m10.sdk.transaction.ICreateTransfer|null);

                /** TransactionData commitTransfer */
                commitTransfer?: (m10.sdk.transaction.ICommitTransfer|null);

                /** TransactionData setInstrument */
                setInstrument?: (m10.sdk.transaction.ISetInstrument|null);
            }

            /** Represents a TransactionData. */
            class TransactionData implements ITransactionData {

                /**
                 * Constructs a new TransactionData.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.ITransactionData);

                /** TransactionData transfer. */
                public transfer?: (m10.sdk.transaction.ICreateTransfer|null);

                /** TransactionData createLedgerAccount. */
                public createLedgerAccount?: (m10.sdk.transaction.ICreateLedgerAccount|null);

                /** TransactionData setFreezeState. */
                public setFreezeState?: (m10.sdk.transaction.ISetFreezeState|null);

                /** TransactionData documentOperations. */
                public documentOperations?: (m10.sdk.IDocumentOperations|null);

                /** TransactionData invokeAction. */
                public invokeAction?: (m10.sdk.transaction.IInvokeAction|null);

                /** TransactionData initiateTransfer. */
                public initiateTransfer?: (m10.sdk.transaction.ICreateTransfer|null);

                /** TransactionData commitTransfer. */
                public commitTransfer?: (m10.sdk.transaction.ICommitTransfer|null);

                /** TransactionData setInstrument. */
                public setInstrument?: (m10.sdk.transaction.ISetInstrument|null);

                /** TransactionData data. */
                public data?: ("transfer"|"createLedgerAccount"|"setFreezeState"|"documentOperations"|"invokeAction"|"initiateTransfer"|"commitTransfer"|"setInstrument");

                /**
                 * Creates a new TransactionData instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns TransactionData instance
                 */
                public static create(properties?: m10.sdk.transaction.ITransactionData): m10.sdk.transaction.TransactionData;

                /**
                 * Encodes the specified TransactionData message. Does not implicitly {@link m10.sdk.transaction.TransactionData.verify|verify} messages.
                 * @param message TransactionData message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.ITransactionData, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified TransactionData message, length delimited. Does not implicitly {@link m10.sdk.transaction.TransactionData.verify|verify} messages.
                 * @param message TransactionData message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.ITransactionData, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a TransactionData message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns TransactionData
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.TransactionData;

                /**
                 * Decodes a TransactionData message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns TransactionData
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.TransactionData;

                /**
                 * Verifies a TransactionData message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a TransactionData message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns TransactionData
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.TransactionData;

                /**
                 * Creates a plain object from a TransactionData message. Also converts values to other types if specified.
                 * @param message TransactionData
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.TransactionData, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this TransactionData to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a TransactionResponse. */
            interface ITransactionResponse {

                /** TransactionResponse txId */
                txId?: (number|Long|null);

                /** TransactionResponse error */
                error?: (m10.sdk.transaction.ITransactionError|null);

                /** TransactionResponse timestamp */
                timestamp?: (number|Long|null);

                /** TransactionResponse accountCreated */
                accountCreated?: (Uint8Array|null);

                /** TransactionResponse transferCommitted */
                transferCommitted?: (m10.sdk.transaction.ICreateTransfer|null);
            }

            /** Represents a TransactionResponse. */
            class TransactionResponse implements ITransactionResponse {

                /**
                 * Constructs a new TransactionResponse.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.ITransactionResponse);

                /** TransactionResponse txId. */
                public txId: (number|Long);

                /** TransactionResponse error. */
                public error?: (m10.sdk.transaction.ITransactionError|null);

                /** TransactionResponse timestamp. */
                public timestamp: (number|Long);

                /** TransactionResponse accountCreated. */
                public accountCreated: Uint8Array;

                /** TransactionResponse transferCommitted. */
                public transferCommitted?: (m10.sdk.transaction.ICreateTransfer|null);

                /**
                 * Creates a new TransactionResponse instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns TransactionResponse instance
                 */
                public static create(properties?: m10.sdk.transaction.ITransactionResponse): m10.sdk.transaction.TransactionResponse;

                /**
                 * Encodes the specified TransactionResponse message. Does not implicitly {@link m10.sdk.transaction.TransactionResponse.verify|verify} messages.
                 * @param message TransactionResponse message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.ITransactionResponse, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified TransactionResponse message, length delimited. Does not implicitly {@link m10.sdk.transaction.TransactionResponse.verify|verify} messages.
                 * @param message TransactionResponse message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.ITransactionResponse, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a TransactionResponse message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns TransactionResponse
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.TransactionResponse;

                /**
                 * Decodes a TransactionResponse message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns TransactionResponse
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.TransactionResponse;

                /**
                 * Verifies a TransactionResponse message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a TransactionResponse message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns TransactionResponse
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.TransactionResponse;

                /**
                 * Creates a plain object from a TransactionResponse message. Also converts values to other types if specified.
                 * @param message TransactionResponse
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.TransactionResponse, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this TransactionResponse to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a TransactionError. */
            interface ITransactionError {

                /** TransactionError code */
                code?: (m10.sdk.transaction.TransactionError.Code|null);

                /** TransactionError message */
                message?: (string|null);
            }

            /** Represents a TransactionError. */
            class TransactionError implements ITransactionError {

                /**
                 * Constructs a new TransactionError.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.ITransactionError);

                /** TransactionError code. */
                public code: m10.sdk.transaction.TransactionError.Code;

                /** TransactionError message. */
                public message: string;

                /**
                 * Creates a new TransactionError instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns TransactionError instance
                 */
                public static create(properties?: m10.sdk.transaction.ITransactionError): m10.sdk.transaction.TransactionError;

                /**
                 * Encodes the specified TransactionError message. Does not implicitly {@link m10.sdk.transaction.TransactionError.verify|verify} messages.
                 * @param message TransactionError message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.ITransactionError, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified TransactionError message, length delimited. Does not implicitly {@link m10.sdk.transaction.TransactionError.verify|verify} messages.
                 * @param message TransactionError message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.ITransactionError, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a TransactionError message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns TransactionError
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.TransactionError;

                /**
                 * Decodes a TransactionError message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns TransactionError
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.TransactionError;

                /**
                 * Verifies a TransactionError message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a TransactionError message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns TransactionError
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.TransactionError;

                /**
                 * Creates a plain object from a TransactionError message. Also converts values to other types if specified.
                 * @param message TransactionError
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.TransactionError, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this TransactionError to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            namespace TransactionError {

                /** Code enum. */
                enum Code {
                    UNKNOWN = 0,
                    UNIMPLEMENTED = 1,
                    NOT_FOUND = 2,
                    ALREADY_EXISTS = 3,
                    UNAUTHORIZED = 4,
                    BAD_REQUEST = 5,
                    INVALID_REQUEST_TYPE = 6,
                    INVALID_ACCOUNT_ID = 7,
                    INVALID_TRANSFER = 8,
                    MESSAGE_TOO_LARGE = 10,
                    INVALID_SIGNATURE = 11,
                    VERIFICATION_FAILED = 12,
                    REPLAY_PROTECTION = 20,
                    INVALID_EXPRESSION = 21,
                    INCORRECT_TYPE = 22,
                    ACCOUNT_FROZEN = 23,
                    UNMODIFIED_STATE = 24,
                    INSUFFICIENT_BALANCE = 25,
                    BALANCE_OVERFLOW = 26,
                    ACCOUNT_DEPTH_EXCEEDED = 27,
                    INVALID_TARGET = 30
                }
            }

            /** Properties of a CreateLedgerTransfer. */
            interface ICreateLedgerTransfer {

                /** CreateLedgerTransfer ledgerId */
                ledgerId?: (string|null);

                /** CreateLedgerTransfer nonce */
                nonce?: (number|Long|null);

                /** CreateLedgerTransfer transfer */
                transfer?: (m10.sdk.transaction.ICreateTransfer|null);
            }

            /** Represents a CreateLedgerTransfer. */
            class CreateLedgerTransfer implements ICreateLedgerTransfer {

                /**
                 * Constructs a new CreateLedgerTransfer.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.ICreateLedgerTransfer);

                /** CreateLedgerTransfer ledgerId. */
                public ledgerId: string;

                /** CreateLedgerTransfer nonce. */
                public nonce: (number|Long);

                /** CreateLedgerTransfer transfer. */
                public transfer?: (m10.sdk.transaction.ICreateTransfer|null);

                /**
                 * Creates a new CreateLedgerTransfer instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns CreateLedgerTransfer instance
                 */
                public static create(properties?: m10.sdk.transaction.ICreateLedgerTransfer): m10.sdk.transaction.CreateLedgerTransfer;

                /**
                 * Encodes the specified CreateLedgerTransfer message. Does not implicitly {@link m10.sdk.transaction.CreateLedgerTransfer.verify|verify} messages.
                 * @param message CreateLedgerTransfer message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.ICreateLedgerTransfer, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified CreateLedgerTransfer message, length delimited. Does not implicitly {@link m10.sdk.transaction.CreateLedgerTransfer.verify|verify} messages.
                 * @param message CreateLedgerTransfer message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.ICreateLedgerTransfer, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a CreateLedgerTransfer message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns CreateLedgerTransfer
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.CreateLedgerTransfer;

                /**
                 * Decodes a CreateLedgerTransfer message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns CreateLedgerTransfer
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.CreateLedgerTransfer;

                /**
                 * Verifies a CreateLedgerTransfer message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a CreateLedgerTransfer message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns CreateLedgerTransfer
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.CreateLedgerTransfer;

                /**
                 * Creates a plain object from a CreateLedgerTransfer message. Also converts values to other types if specified.
                 * @param message CreateLedgerTransfer
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.CreateLedgerTransfer, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this CreateLedgerTransfer to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a CreateLedgerTransfers. */
            interface ICreateLedgerTransfers {

                /** CreateLedgerTransfers transfers */
                transfers?: (m10.sdk.transaction.ICreateLedgerTransfer[]|null);

                /** CreateLedgerTransfers validUntil */
                validUntil?: (number|Long|null);
            }

            /** Represents a CreateLedgerTransfers. */
            class CreateLedgerTransfers implements ICreateLedgerTransfers {

                /**
                 * Constructs a new CreateLedgerTransfers.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.ICreateLedgerTransfers);

                /** CreateLedgerTransfers transfers. */
                public transfers: m10.sdk.transaction.ICreateLedgerTransfer[];

                /** CreateLedgerTransfers validUntil. */
                public validUntil: (number|Long);

                /**
                 * Creates a new CreateLedgerTransfers instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns CreateLedgerTransfers instance
                 */
                public static create(properties?: m10.sdk.transaction.ICreateLedgerTransfers): m10.sdk.transaction.CreateLedgerTransfers;

                /**
                 * Encodes the specified CreateLedgerTransfers message. Does not implicitly {@link m10.sdk.transaction.CreateLedgerTransfers.verify|verify} messages.
                 * @param message CreateLedgerTransfers message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.ICreateLedgerTransfers, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified CreateLedgerTransfers message, length delimited. Does not implicitly {@link m10.sdk.transaction.CreateLedgerTransfers.verify|verify} messages.
                 * @param message CreateLedgerTransfers message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.ICreateLedgerTransfers, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a CreateLedgerTransfers message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns CreateLedgerTransfers
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.CreateLedgerTransfers;

                /**
                 * Decodes a CreateLedgerTransfers message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns CreateLedgerTransfers
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.CreateLedgerTransfers;

                /**
                 * Verifies a CreateLedgerTransfers message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a CreateLedgerTransfers message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns CreateLedgerTransfers
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.CreateLedgerTransfers;

                /**
                 * Creates a plain object from a CreateLedgerTransfers message. Also converts values to other types if specified.
                 * @param message CreateLedgerTransfers
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.CreateLedgerTransfers, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this CreateLedgerTransfers to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a GetTransferRequest. */
            interface IGetTransferRequest {

                /** GetTransferRequest txId */
                txId?: (number|Long|null);
            }

            /** Represents a GetTransferRequest. */
            class GetTransferRequest implements IGetTransferRequest {

                /**
                 * Constructs a new GetTransferRequest.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.IGetTransferRequest);

                /** GetTransferRequest txId. */
                public txId: (number|Long);

                /**
                 * Creates a new GetTransferRequest instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns GetTransferRequest instance
                 */
                public static create(properties?: m10.sdk.transaction.IGetTransferRequest): m10.sdk.transaction.GetTransferRequest;

                /**
                 * Encodes the specified GetTransferRequest message. Does not implicitly {@link m10.sdk.transaction.GetTransferRequest.verify|verify} messages.
                 * @param message GetTransferRequest message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.IGetTransferRequest, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified GetTransferRequest message, length delimited. Does not implicitly {@link m10.sdk.transaction.GetTransferRequest.verify|verify} messages.
                 * @param message GetTransferRequest message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.IGetTransferRequest, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a GetTransferRequest message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns GetTransferRequest
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.GetTransferRequest;

                /**
                 * Decodes a GetTransferRequest message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns GetTransferRequest
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.GetTransferRequest;

                /**
                 * Verifies a GetTransferRequest message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a GetTransferRequest message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns GetTransferRequest
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.GetTransferRequest;

                /**
                 * Creates a plain object from a GetTransferRequest message. Also converts values to other types if specified.
                 * @param message GetTransferRequest
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.GetTransferRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this GetTransferRequest to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a ListTransferRequest. */
            interface IListTransferRequest {

                /** ListTransferRequest accountId */
                accountId?: (Uint8Array|null);

                /** ListTransferRequest contextId */
                contextId?: (Uint8Array|null);

                /** ListTransferRequest limit */
                limit?: (number|Long|null);

                /** ListTransferRequest includeChildAccounts */
                includeChildAccounts?: (boolean|null);

                /** ListTransferRequest minTxId */
                minTxId?: (number|Long|null);

                /** ListTransferRequest maxTxId */
                maxTxId?: (number|Long|null);
            }

            /** Represents a ListTransferRequest. */
            class ListTransferRequest implements IListTransferRequest {

                /**
                 * Constructs a new ListTransferRequest.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.IListTransferRequest);

                /** ListTransferRequest accountId. */
                public accountId?: (Uint8Array|null);

                /** ListTransferRequest contextId. */
                public contextId?: (Uint8Array|null);

                /** ListTransferRequest limit. */
                public limit: (number|Long);

                /** ListTransferRequest includeChildAccounts. */
                public includeChildAccounts: boolean;

                /** ListTransferRequest minTxId. */
                public minTxId: (number|Long);

                /** ListTransferRequest maxTxId. */
                public maxTxId: (number|Long);

                /** ListTransferRequest filter. */
                public filter?: ("accountId"|"contextId");

                /**
                 * Creates a new ListTransferRequest instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns ListTransferRequest instance
                 */
                public static create(properties?: m10.sdk.transaction.IListTransferRequest): m10.sdk.transaction.ListTransferRequest;

                /**
                 * Encodes the specified ListTransferRequest message. Does not implicitly {@link m10.sdk.transaction.ListTransferRequest.verify|verify} messages.
                 * @param message ListTransferRequest message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.IListTransferRequest, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified ListTransferRequest message, length delimited. Does not implicitly {@link m10.sdk.transaction.ListTransferRequest.verify|verify} messages.
                 * @param message ListTransferRequest message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.IListTransferRequest, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a ListTransferRequest message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns ListTransferRequest
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.ListTransferRequest;

                /**
                 * Decodes a ListTransferRequest message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns ListTransferRequest
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.ListTransferRequest;

                /**
                 * Verifies a ListTransferRequest message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a ListTransferRequest message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns ListTransferRequest
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.ListTransferRequest;

                /**
                 * Creates a plain object from a ListTransferRequest message. Also converts values to other types if specified.
                 * @param message ListTransferRequest
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.ListTransferRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this ListTransferRequest to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a CreateTransfer. */
            interface ICreateTransfer {

                /** CreateTransfer transferSteps */
                transferSteps?: (m10.sdk.transaction.ITransferStep[]|null);
            }

            /** Represents a CreateTransfer. */
            class CreateTransfer implements ICreateTransfer {

                /**
                 * Constructs a new CreateTransfer.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.ICreateTransfer);

                /** CreateTransfer transferSteps. */
                public transferSteps: m10.sdk.transaction.ITransferStep[];

                /**
                 * Creates a new CreateTransfer instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns CreateTransfer instance
                 */
                public static create(properties?: m10.sdk.transaction.ICreateTransfer): m10.sdk.transaction.CreateTransfer;

                /**
                 * Encodes the specified CreateTransfer message. Does not implicitly {@link m10.sdk.transaction.CreateTransfer.verify|verify} messages.
                 * @param message CreateTransfer message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.ICreateTransfer, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified CreateTransfer message, length delimited. Does not implicitly {@link m10.sdk.transaction.CreateTransfer.verify|verify} messages.
                 * @param message CreateTransfer message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.ICreateTransfer, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a CreateTransfer message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns CreateTransfer
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.CreateTransfer;

                /**
                 * Decodes a CreateTransfer message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns CreateTransfer
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.CreateTransfer;

                /**
                 * Verifies a CreateTransfer message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a CreateTransfer message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns CreateTransfer
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.CreateTransfer;

                /**
                 * Creates a plain object from a CreateTransfer message. Also converts values to other types if specified.
                 * @param message CreateTransfer
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.CreateTransfer, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this CreateTransfer to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a TransferStep. */
            interface ITransferStep {

                /** TransferStep fromAccountId */
                fromAccountId?: (Uint8Array|null);

                /** TransferStep toAccountId */
                toAccountId?: (Uint8Array|null);

                /** TransferStep amount */
                amount?: (number|Long|null);

                /** TransferStep metadata */
                metadata?: (google.protobuf.IAny[]|null);
            }

            /** Represents a TransferStep. */
            class TransferStep implements ITransferStep {

                /**
                 * Constructs a new TransferStep.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.ITransferStep);

                /** TransferStep fromAccountId. */
                public fromAccountId: Uint8Array;

                /** TransferStep toAccountId. */
                public toAccountId: Uint8Array;

                /** TransferStep amount. */
                public amount: (number|Long);

                /** TransferStep metadata. */
                public metadata: google.protobuf.IAny[];

                /**
                 * Creates a new TransferStep instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns TransferStep instance
                 */
                public static create(properties?: m10.sdk.transaction.ITransferStep): m10.sdk.transaction.TransferStep;

                /**
                 * Encodes the specified TransferStep message. Does not implicitly {@link m10.sdk.transaction.TransferStep.verify|verify} messages.
                 * @param message TransferStep message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.ITransferStep, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified TransferStep message, length delimited. Does not implicitly {@link m10.sdk.transaction.TransferStep.verify|verify} messages.
                 * @param message TransferStep message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.ITransferStep, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a TransferStep message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns TransferStep
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.TransferStep;

                /**
                 * Decodes a TransferStep message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns TransferStep
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.TransferStep;

                /**
                 * Verifies a TransferStep message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a TransferStep message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns TransferStep
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.TransferStep;

                /**
                 * Creates a plain object from a TransferStep message. Also converts values to other types if specified.
                 * @param message TransferStep
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.TransferStep, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this TransferStep to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a FinalizedTransfer. */
            interface IFinalizedTransfer {

                /** FinalizedTransfer txId */
                txId?: (number|Long|null);

                /** FinalizedTransfer contextId */
                contextId?: (Uint8Array|null);

                /** FinalizedTransfer transferSteps */
                transferSteps?: (m10.sdk.transaction.ITransferStep[]|null);

                /** FinalizedTransfer error */
                error?: (m10.sdk.transaction.ITransactionError|null);

                /** FinalizedTransfer timestamp */
                timestamp?: (number|Long|null);

                /** FinalizedTransfer state */
                state?: (m10.sdk.transaction.FinalizedTransfer.TransferState|null);
            }

            /** Represents a FinalizedTransfer. */
            class FinalizedTransfer implements IFinalizedTransfer {

                /**
                 * Constructs a new FinalizedTransfer.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.IFinalizedTransfer);

                /** FinalizedTransfer txId. */
                public txId: (number|Long);

                /** FinalizedTransfer contextId. */
                public contextId: Uint8Array;

                /** FinalizedTransfer transferSteps. */
                public transferSteps: m10.sdk.transaction.ITransferStep[];

                /** FinalizedTransfer error. */
                public error?: (m10.sdk.transaction.ITransactionError|null);

                /** FinalizedTransfer timestamp. */
                public timestamp: (number|Long);

                /** FinalizedTransfer state. */
                public state: m10.sdk.transaction.FinalizedTransfer.TransferState;

                /**
                 * Creates a new FinalizedTransfer instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns FinalizedTransfer instance
                 */
                public static create(properties?: m10.sdk.transaction.IFinalizedTransfer): m10.sdk.transaction.FinalizedTransfer;

                /**
                 * Encodes the specified FinalizedTransfer message. Does not implicitly {@link m10.sdk.transaction.FinalizedTransfer.verify|verify} messages.
                 * @param message FinalizedTransfer message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.IFinalizedTransfer, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified FinalizedTransfer message, length delimited. Does not implicitly {@link m10.sdk.transaction.FinalizedTransfer.verify|verify} messages.
                 * @param message FinalizedTransfer message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.IFinalizedTransfer, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a FinalizedTransfer message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns FinalizedTransfer
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.FinalizedTransfer;

                /**
                 * Decodes a FinalizedTransfer message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns FinalizedTransfer
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.FinalizedTransfer;

                /**
                 * Verifies a FinalizedTransfer message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a FinalizedTransfer message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns FinalizedTransfer
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.FinalizedTransfer;

                /**
                 * Creates a plain object from a FinalizedTransfer message. Also converts values to other types if specified.
                 * @param message FinalizedTransfer
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.FinalizedTransfer, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this FinalizedTransfer to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            namespace FinalizedTransfer {

                /** TransferState enum. */
                enum TransferState {
                    ACCEPTED = 0,
                    REJECTED = 1,
                    PENDING = 2,
                    EXPIRED = 3
                }
            }

            /** Properties of a FinalizedTransfers. */
            interface IFinalizedTransfers {

                /** FinalizedTransfers transfers */
                transfers?: (m10.sdk.transaction.IFinalizedTransfer[]|null);
            }

            /** Represents a FinalizedTransfers. */
            class FinalizedTransfers implements IFinalizedTransfers {

                /**
                 * Constructs a new FinalizedTransfers.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.IFinalizedTransfers);

                /** FinalizedTransfers transfers. */
                public transfers: m10.sdk.transaction.IFinalizedTransfer[];

                /**
                 * Creates a new FinalizedTransfers instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns FinalizedTransfers instance
                 */
                public static create(properties?: m10.sdk.transaction.IFinalizedTransfers): m10.sdk.transaction.FinalizedTransfers;

                /**
                 * Encodes the specified FinalizedTransfers message. Does not implicitly {@link m10.sdk.transaction.FinalizedTransfers.verify|verify} messages.
                 * @param message FinalizedTransfers message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.IFinalizedTransfers, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified FinalizedTransfers message, length delimited. Does not implicitly {@link m10.sdk.transaction.FinalizedTransfers.verify|verify} messages.
                 * @param message FinalizedTransfers message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.IFinalizedTransfers, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a FinalizedTransfers message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns FinalizedTransfers
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.FinalizedTransfers;

                /**
                 * Decodes a FinalizedTransfers message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns FinalizedTransfers
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.FinalizedTransfers;

                /**
                 * Verifies a FinalizedTransfers message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a FinalizedTransfers message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns FinalizedTransfers
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.FinalizedTransfers;

                /**
                 * Creates a plain object from a FinalizedTransfers message. Also converts values to other types if specified.
                 * @param message FinalizedTransfers
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.FinalizedTransfers, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this FinalizedTransfers to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of an Instrument. */
            interface IInstrument {

                /** Instrument code */
                code?: (string|null);

                /** Instrument decimalPlaces */
                decimalPlaces?: (number|null);

                /** Instrument description */
                description?: (string|null);
            }

            /** Represents an Instrument. */
            class Instrument implements IInstrument {

                /**
                 * Constructs a new Instrument.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.IInstrument);

                /** Instrument code. */
                public code: string;

                /** Instrument decimalPlaces. */
                public decimalPlaces: number;

                /** Instrument description. */
                public description: string;

                /**
                 * Creates a new Instrument instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Instrument instance
                 */
                public static create(properties?: m10.sdk.transaction.IInstrument): m10.sdk.transaction.Instrument;

                /**
                 * Encodes the specified Instrument message. Does not implicitly {@link m10.sdk.transaction.Instrument.verify|verify} messages.
                 * @param message Instrument message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.IInstrument, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Instrument message, length delimited. Does not implicitly {@link m10.sdk.transaction.Instrument.verify|verify} messages.
                 * @param message Instrument message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.IInstrument, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes an Instrument message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Instrument
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.Instrument;

                /**
                 * Decodes an Instrument message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Instrument
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.Instrument;

                /**
                 * Verifies an Instrument message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates an Instrument message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Instrument
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.Instrument;

                /**
                 * Creates a plain object from an Instrument message. Also converts values to other types if specified.
                 * @param message Instrument
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.Instrument, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Instrument to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a CreateLedgerAccount. */
            interface ICreateLedgerAccount {

                /** CreateLedgerAccount parentId */
                parentId?: (Uint8Array|null);

                /** CreateLedgerAccount issuance */
                issuance?: (boolean|null);

                /** CreateLedgerAccount frozen */
                frozen?: (boolean|null);

                /** CreateLedgerAccount instrument */
                instrument?: (m10.sdk.transaction.IInstrument|null);
            }

            /** Represents a CreateLedgerAccount. */
            class CreateLedgerAccount implements ICreateLedgerAccount {

                /**
                 * Constructs a new CreateLedgerAccount.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.ICreateLedgerAccount);

                /** CreateLedgerAccount parentId. */
                public parentId: Uint8Array;

                /** CreateLedgerAccount issuance. */
                public issuance: boolean;

                /** CreateLedgerAccount frozen. */
                public frozen: boolean;

                /** CreateLedgerAccount instrument. */
                public instrument?: (m10.sdk.transaction.IInstrument|null);

                /**
                 * Creates a new CreateLedgerAccount instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns CreateLedgerAccount instance
                 */
                public static create(properties?: m10.sdk.transaction.ICreateLedgerAccount): m10.sdk.transaction.CreateLedgerAccount;

                /**
                 * Encodes the specified CreateLedgerAccount message. Does not implicitly {@link m10.sdk.transaction.CreateLedgerAccount.verify|verify} messages.
                 * @param message CreateLedgerAccount message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.ICreateLedgerAccount, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified CreateLedgerAccount message, length delimited. Does not implicitly {@link m10.sdk.transaction.CreateLedgerAccount.verify|verify} messages.
                 * @param message CreateLedgerAccount message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.ICreateLedgerAccount, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a CreateLedgerAccount message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns CreateLedgerAccount
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.CreateLedgerAccount;

                /**
                 * Decodes a CreateLedgerAccount message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns CreateLedgerAccount
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.CreateLedgerAccount;

                /**
                 * Verifies a CreateLedgerAccount message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a CreateLedgerAccount message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns CreateLedgerAccount
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.CreateLedgerAccount;

                /**
                 * Creates a plain object from a CreateLedgerAccount message. Also converts values to other types if specified.
                 * @param message CreateLedgerAccount
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.CreateLedgerAccount, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this CreateLedgerAccount to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a SetFreezeState. */
            interface ISetFreezeState {

                /** SetFreezeState accountId */
                accountId?: (Uint8Array|null);

                /** SetFreezeState frozen */
                frozen?: (boolean|null);
            }

            /** Represents a SetFreezeState. */
            class SetFreezeState implements ISetFreezeState {

                /**
                 * Constructs a new SetFreezeState.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.ISetFreezeState);

                /** SetFreezeState accountId. */
                public accountId: Uint8Array;

                /** SetFreezeState frozen. */
                public frozen: boolean;

                /**
                 * Creates a new SetFreezeState instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns SetFreezeState instance
                 */
                public static create(properties?: m10.sdk.transaction.ISetFreezeState): m10.sdk.transaction.SetFreezeState;

                /**
                 * Encodes the specified SetFreezeState message. Does not implicitly {@link m10.sdk.transaction.SetFreezeState.verify|verify} messages.
                 * @param message SetFreezeState message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.ISetFreezeState, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified SetFreezeState message, length delimited. Does not implicitly {@link m10.sdk.transaction.SetFreezeState.verify|verify} messages.
                 * @param message SetFreezeState message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.ISetFreezeState, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a SetFreezeState message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns SetFreezeState
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.SetFreezeState;

                /**
                 * Decodes a SetFreezeState message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns SetFreezeState
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.SetFreezeState;

                /**
                 * Verifies a SetFreezeState message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a SetFreezeState message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns SetFreezeState
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.SetFreezeState;

                /**
                 * Creates a plain object from a SetFreezeState message. Also converts values to other types if specified.
                 * @param message SetFreezeState
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.SetFreezeState, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this SetFreezeState to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a SetInstrument. */
            interface ISetInstrument {

                /** SetInstrument accountId */
                accountId?: (Uint8Array|null);

                /** SetInstrument code */
                code?: (string|null);

                /** SetInstrument decimalPlaces */
                decimalPlaces?: (number|null);

                /** SetInstrument description */
                description?: (string|null);
            }

            /** Represents a SetInstrument. */
            class SetInstrument implements ISetInstrument {

                /**
                 * Constructs a new SetInstrument.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.ISetInstrument);

                /** SetInstrument accountId. */
                public accountId: Uint8Array;

                /** SetInstrument code. */
                public code: string;

                /** SetInstrument decimalPlaces. */
                public decimalPlaces: number;

                /** SetInstrument description. */
                public description: string;

                /**
                 * Creates a new SetInstrument instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns SetInstrument instance
                 */
                public static create(properties?: m10.sdk.transaction.ISetInstrument): m10.sdk.transaction.SetInstrument;

                /**
                 * Encodes the specified SetInstrument message. Does not implicitly {@link m10.sdk.transaction.SetInstrument.verify|verify} messages.
                 * @param message SetInstrument message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.ISetInstrument, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified SetInstrument message, length delimited. Does not implicitly {@link m10.sdk.transaction.SetInstrument.verify|verify} messages.
                 * @param message SetInstrument message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.ISetInstrument, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a SetInstrument message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns SetInstrument
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.SetInstrument;

                /**
                 * Decodes a SetInstrument message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns SetInstrument
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.SetInstrument;

                /**
                 * Verifies a SetInstrument message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a SetInstrument message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns SetInstrument
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.SetInstrument;

                /**
                 * Creates a plain object from a SetInstrument message. Also converts values to other types if specified.
                 * @param message SetInstrument
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.SetInstrument, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this SetInstrument to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a GetAccountRequest. */
            interface IGetAccountRequest {

                /** GetAccountRequest id */
                id?: (Uint8Array|null);
            }

            /** Represents a GetAccountRequest. */
            class GetAccountRequest implements IGetAccountRequest {

                /**
                 * Constructs a new GetAccountRequest.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.IGetAccountRequest);

                /** GetAccountRequest id. */
                public id: Uint8Array;

                /**
                 * Creates a new GetAccountRequest instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns GetAccountRequest instance
                 */
                public static create(properties?: m10.sdk.transaction.IGetAccountRequest): m10.sdk.transaction.GetAccountRequest;

                /**
                 * Encodes the specified GetAccountRequest message. Does not implicitly {@link m10.sdk.transaction.GetAccountRequest.verify|verify} messages.
                 * @param message GetAccountRequest message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.IGetAccountRequest, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified GetAccountRequest message, length delimited. Does not implicitly {@link m10.sdk.transaction.GetAccountRequest.verify|verify} messages.
                 * @param message GetAccountRequest message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.IGetAccountRequest, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a GetAccountRequest message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns GetAccountRequest
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.GetAccountRequest;

                /**
                 * Decodes a GetAccountRequest message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns GetAccountRequest
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.GetAccountRequest;

                /**
                 * Verifies a GetAccountRequest message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a GetAccountRequest message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns GetAccountRequest
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.GetAccountRequest;

                /**
                 * Creates a plain object from a GetAccountRequest message. Also converts values to other types if specified.
                 * @param message GetAccountRequest
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.GetAccountRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this GetAccountRequest to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of an IndexedAccount. */
            interface IIndexedAccount {

                /** IndexedAccount id */
                id?: (Uint8Array|null);

                /** IndexedAccount issuance */
                issuance?: (m10.sdk.transaction.IndexedAccount.IIssuance|null);

                /** IndexedAccount balance */
                balance?: (number|Long|null);

                /** IndexedAccount frozen */
                frozen?: (boolean|null);

                /** IndexedAccount instrument */
                instrument?: (m10.sdk.transaction.IInstrument|null);
            }

            /** Represents an IndexedAccount. */
            class IndexedAccount implements IIndexedAccount {

                /**
                 * Constructs a new IndexedAccount.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.IIndexedAccount);

                /** IndexedAccount id. */
                public id: Uint8Array;

                /** IndexedAccount issuance. */
                public issuance?: (m10.sdk.transaction.IndexedAccount.IIssuance|null);

                /** IndexedAccount balance. */
                public balance: (number|Long);

                /** IndexedAccount frozen. */
                public frozen: boolean;

                /** IndexedAccount instrument. */
                public instrument?: (m10.sdk.transaction.IInstrument|null);

                /**
                 * Creates a new IndexedAccount instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns IndexedAccount instance
                 */
                public static create(properties?: m10.sdk.transaction.IIndexedAccount): m10.sdk.transaction.IndexedAccount;

                /**
                 * Encodes the specified IndexedAccount message. Does not implicitly {@link m10.sdk.transaction.IndexedAccount.verify|verify} messages.
                 * @param message IndexedAccount message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.IIndexedAccount, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified IndexedAccount message, length delimited. Does not implicitly {@link m10.sdk.transaction.IndexedAccount.verify|verify} messages.
                 * @param message IndexedAccount message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.IIndexedAccount, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes an IndexedAccount message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns IndexedAccount
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.IndexedAccount;

                /**
                 * Decodes an IndexedAccount message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns IndexedAccount
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.IndexedAccount;

                /**
                 * Verifies an IndexedAccount message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates an IndexedAccount message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns IndexedAccount
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.IndexedAccount;

                /**
                 * Creates a plain object from an IndexedAccount message. Also converts values to other types if specified.
                 * @param message IndexedAccount
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.IndexedAccount, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this IndexedAccount to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            namespace IndexedAccount {

                /** Properties of an Issuance. */
                interface IIssuance {

                    /** Issuance issuedBalance */
                    issuedBalance?: (number|Long|null);

                    /** Issuance nonLeafChildren */
                    nonLeafChildren?: (number|Long|null);

                    /** Issuance leafChildren */
                    leafChildren?: (number|Long|null);
                }

                /** Represents an Issuance. */
                class Issuance implements IIssuance {

                    /**
                     * Constructs a new Issuance.
                     * @param [properties] Properties to set
                     */
                    constructor(properties?: m10.sdk.transaction.IndexedAccount.IIssuance);

                    /** Issuance issuedBalance. */
                    public issuedBalance: (number|Long);

                    /** Issuance nonLeafChildren. */
                    public nonLeafChildren: (number|Long);

                    /** Issuance leafChildren. */
                    public leafChildren: (number|Long);

                    /**
                     * Creates a new Issuance instance using the specified properties.
                     * @param [properties] Properties to set
                     * @returns Issuance instance
                     */
                    public static create(properties?: m10.sdk.transaction.IndexedAccount.IIssuance): m10.sdk.transaction.IndexedAccount.Issuance;

                    /**
                     * Encodes the specified Issuance message. Does not implicitly {@link m10.sdk.transaction.IndexedAccount.Issuance.verify|verify} messages.
                     * @param message Issuance message or plain object to encode
                     * @param [writer] Writer to encode to
                     * @returns Writer
                     */
                    public static encode(message: m10.sdk.transaction.IndexedAccount.IIssuance, writer?: $protobuf.Writer): $protobuf.Writer;

                    /**
                     * Encodes the specified Issuance message, length delimited. Does not implicitly {@link m10.sdk.transaction.IndexedAccount.Issuance.verify|verify} messages.
                     * @param message Issuance message or plain object to encode
                     * @param [writer] Writer to encode to
                     * @returns Writer
                     */
                    public static encodeDelimited(message: m10.sdk.transaction.IndexedAccount.IIssuance, writer?: $protobuf.Writer): $protobuf.Writer;

                    /**
                     * Decodes an Issuance message from the specified reader or buffer.
                     * @param reader Reader or buffer to decode from
                     * @param [length] Message length if known beforehand
                     * @returns Issuance
                     * @throws {Error} If the payload is not a reader or valid buffer
                     * @throws {$protobuf.util.ProtocolError} If required fields are missing
                     */
                    public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.IndexedAccount.Issuance;

                    /**
                     * Decodes an Issuance message from the specified reader or buffer, length delimited.
                     * @param reader Reader or buffer to decode from
                     * @returns Issuance
                     * @throws {Error} If the payload is not a reader or valid buffer
                     * @throws {$protobuf.util.ProtocolError} If required fields are missing
                     */
                    public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.IndexedAccount.Issuance;

                    /**
                     * Verifies an Issuance message.
                     * @param message Plain object to verify
                     * @returns `null` if valid, otherwise the reason why it is not
                     */
                    public static verify(message: { [k: string]: any }): (string|null);

                    /**
                     * Creates an Issuance message from a plain object. Also converts values to their respective internal types.
                     * @param object Plain object
                     * @returns Issuance
                     */
                    public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.IndexedAccount.Issuance;

                    /**
                     * Creates a plain object from an Issuance message. Also converts values to other types if specified.
                     * @param message Issuance
                     * @param [options] Conversion options
                     * @returns Plain object
                     */
                    public static toObject(message: m10.sdk.transaction.IndexedAccount.Issuance, options?: $protobuf.IConversionOptions): { [k: string]: any };

                    /**
                     * Converts this Issuance to JSON.
                     * @returns JSON object
                     */
                    public toJSON(): { [k: string]: any };
                }
            }

            /** Properties of an InvokeAction. */
            interface IInvokeAction {

                /** InvokeAction name */
                name?: (string|null);

                /** InvokeAction fromAccount */
                fromAccount?: (Uint8Array|null);

                /** InvokeAction target */
                target?: (m10.sdk.transaction.ITarget|null);

                /** InvokeAction payload */
                payload?: (Uint8Array|null);
            }

            /** Represents an InvokeAction. */
            class InvokeAction implements IInvokeAction {

                /**
                 * Constructs a new InvokeAction.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.IInvokeAction);

                /** InvokeAction name. */
                public name: string;

                /** InvokeAction fromAccount. */
                public fromAccount: Uint8Array;

                /** InvokeAction target. */
                public target?: (m10.sdk.transaction.ITarget|null);

                /** InvokeAction payload. */
                public payload: Uint8Array;

                /**
                 * Creates a new InvokeAction instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns InvokeAction instance
                 */
                public static create(properties?: m10.sdk.transaction.IInvokeAction): m10.sdk.transaction.InvokeAction;

                /**
                 * Encodes the specified InvokeAction message. Does not implicitly {@link m10.sdk.transaction.InvokeAction.verify|verify} messages.
                 * @param message InvokeAction message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.IInvokeAction, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified InvokeAction message, length delimited. Does not implicitly {@link m10.sdk.transaction.InvokeAction.verify|verify} messages.
                 * @param message InvokeAction message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.IInvokeAction, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes an InvokeAction message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns InvokeAction
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.InvokeAction;

                /**
                 * Decodes an InvokeAction message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns InvokeAction
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.InvokeAction;

                /**
                 * Verifies an InvokeAction message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates an InvokeAction message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns InvokeAction
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.InvokeAction;

                /**
                 * Creates a plain object from an InvokeAction message. Also converts values to other types if specified.
                 * @param message InvokeAction
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.InvokeAction, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this InvokeAction to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a Target. */
            interface ITarget {

                /** Target accountId */
                accountId?: (Uint8Array|null);
            }

            /** Represents a Target. */
            class Target implements ITarget {

                /**
                 * Constructs a new Target.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.ITarget);

                /** Target accountId. */
                public accountId?: (Uint8Array|null);

                /** Target target. */
                public target?: "accountId";

                /**
                 * Creates a new Target instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Target instance
                 */
                public static create(properties?: m10.sdk.transaction.ITarget): m10.sdk.transaction.Target;

                /**
                 * Encodes the specified Target message. Does not implicitly {@link m10.sdk.transaction.Target.verify|verify} messages.
                 * @param message Target message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.ITarget, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Target message, length delimited. Does not implicitly {@link m10.sdk.transaction.Target.verify|verify} messages.
                 * @param message Target message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.ITarget, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Target message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Target
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.Target;

                /**
                 * Decodes a Target message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Target
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.Target;

                /**
                 * Verifies a Target message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Target message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Target
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.Target;

                /**
                 * Creates a plain object from a Target message. Also converts values to other types if specified.
                 * @param message Target
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.Target, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Target to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of an Action. */
            interface IAction {

                /** Action txId */
                txId?: (number|Long|null);

                /** Action name */
                name?: (string|null);

                /** Action contextId */
                contextId?: (Uint8Array|null);

                /** Action fromAccount */
                fromAccount?: (Uint8Array|null);

                /** Action target */
                target?: (m10.sdk.transaction.ITarget|null);

                /** Action payload */
                payload?: (Uint8Array|null);
            }

            /** Represents an Action. */
            class Action implements IAction {

                /**
                 * Constructs a new Action.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.IAction);

                /** Action txId. */
                public txId: (number|Long);

                /** Action name. */
                public name: string;

                /** Action contextId. */
                public contextId: Uint8Array;

                /** Action fromAccount. */
                public fromAccount: Uint8Array;

                /** Action target. */
                public target?: (m10.sdk.transaction.ITarget|null);

                /** Action payload. */
                public payload: Uint8Array;

                /**
                 * Creates a new Action instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Action instance
                 */
                public static create(properties?: m10.sdk.transaction.IAction): m10.sdk.transaction.Action;

                /**
                 * Encodes the specified Action message. Does not implicitly {@link m10.sdk.transaction.Action.verify|verify} messages.
                 * @param message Action message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.IAction, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Action message, length delimited. Does not implicitly {@link m10.sdk.transaction.Action.verify|verify} messages.
                 * @param message Action message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.IAction, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes an Action message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Action
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.Action;

                /**
                 * Decodes an Action message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Action
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.Action;

                /**
                 * Verifies an Action message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates an Action message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Action
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.Action;

                /**
                 * Creates a plain object from an Action message. Also converts values to other types if specified.
                 * @param message Action
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.Action, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Action to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of an Actions. */
            interface IActions {

                /** Actions actions */
                actions?: (m10.sdk.transaction.IAction[]|null);
            }

            /** Represents an Actions. */
            class Actions implements IActions {

                /**
                 * Constructs a new Actions.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.IActions);

                /** Actions actions. */
                public actions: m10.sdk.transaction.IAction[];

                /**
                 * Creates a new Actions instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Actions instance
                 */
                public static create(properties?: m10.sdk.transaction.IActions): m10.sdk.transaction.Actions;

                /**
                 * Encodes the specified Actions message. Does not implicitly {@link m10.sdk.transaction.Actions.verify|verify} messages.
                 * @param message Actions message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.IActions, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Actions message, length delimited. Does not implicitly {@link m10.sdk.transaction.Actions.verify|verify} messages.
                 * @param message Actions message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.IActions, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes an Actions message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Actions
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.Actions;

                /**
                 * Decodes an Actions message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Actions
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.Actions;

                /**
                 * Verifies an Actions message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates an Actions message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Actions
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.Actions;

                /**
                 * Creates a plain object from an Actions message. Also converts values to other types if specified.
                 * @param message Actions
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.Actions, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Actions to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a GetActionRequest. */
            interface IGetActionRequest {

                /** GetActionRequest txId */
                txId?: (number|Long|null);
            }

            /** Represents a GetActionRequest. */
            class GetActionRequest implements IGetActionRequest {

                /**
                 * Constructs a new GetActionRequest.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.IGetActionRequest);

                /** GetActionRequest txId. */
                public txId: (number|Long);

                /**
                 * Creates a new GetActionRequest instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns GetActionRequest instance
                 */
                public static create(properties?: m10.sdk.transaction.IGetActionRequest): m10.sdk.transaction.GetActionRequest;

                /**
                 * Encodes the specified GetActionRequest message. Does not implicitly {@link m10.sdk.transaction.GetActionRequest.verify|verify} messages.
                 * @param message GetActionRequest message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.IGetActionRequest, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified GetActionRequest message, length delimited. Does not implicitly {@link m10.sdk.transaction.GetActionRequest.verify|verify} messages.
                 * @param message GetActionRequest message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.IGetActionRequest, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a GetActionRequest message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns GetActionRequest
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.GetActionRequest;

                /**
                 * Decodes a GetActionRequest message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns GetActionRequest
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.GetActionRequest;

                /**
                 * Verifies a GetActionRequest message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a GetActionRequest message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns GetActionRequest
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.GetActionRequest;

                /**
                 * Creates a plain object from a GetActionRequest message. Also converts values to other types if specified.
                 * @param message GetActionRequest
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.GetActionRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this GetActionRequest to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a ListActionsRequest. */
            interface IListActionsRequest {

                /** ListActionsRequest name */
                name?: (string|null);

                /** ListActionsRequest accountId */
                accountId?: (Uint8Array|null);

                /** ListActionsRequest contextId */
                contextId?: (Uint8Array|null);

                /** ListActionsRequest limit */
                limit?: (number|Long|null);

                /** ListActionsRequest minTxId */
                minTxId?: (number|Long|null);

                /** ListActionsRequest maxTxId */
                maxTxId?: (number|Long|null);
            }

            /** Represents a ListActionsRequest. */
            class ListActionsRequest implements IListActionsRequest {

                /**
                 * Constructs a new ListActionsRequest.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.IListActionsRequest);

                /** ListActionsRequest name. */
                public name: string;

                /** ListActionsRequest accountId. */
                public accountId?: (Uint8Array|null);

                /** ListActionsRequest contextId. */
                public contextId?: (Uint8Array|null);

                /** ListActionsRequest limit. */
                public limit: (number|Long);

                /** ListActionsRequest minTxId. */
                public minTxId: (number|Long);

                /** ListActionsRequest maxTxId. */
                public maxTxId: (number|Long);

                /** ListActionsRequest filter. */
                public filter?: ("accountId"|"contextId");

                /**
                 * Creates a new ListActionsRequest instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns ListActionsRequest instance
                 */
                public static create(properties?: m10.sdk.transaction.IListActionsRequest): m10.sdk.transaction.ListActionsRequest;

                /**
                 * Encodes the specified ListActionsRequest message. Does not implicitly {@link m10.sdk.transaction.ListActionsRequest.verify|verify} messages.
                 * @param message ListActionsRequest message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.IListActionsRequest, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified ListActionsRequest message, length delimited. Does not implicitly {@link m10.sdk.transaction.ListActionsRequest.verify|verify} messages.
                 * @param message ListActionsRequest message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.IListActionsRequest, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a ListActionsRequest message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns ListActionsRequest
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.ListActionsRequest;

                /**
                 * Decodes a ListActionsRequest message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns ListActionsRequest
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.ListActionsRequest;

                /**
                 * Verifies a ListActionsRequest message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a ListActionsRequest message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns ListActionsRequest
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.ListActionsRequest;

                /**
                 * Creates a plain object from a ListActionsRequest message. Also converts values to other types if specified.
                 * @param message ListActionsRequest
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.ListActionsRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this ListActionsRequest to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a CommitTransfer. */
            interface ICommitTransfer {

                /** CommitTransfer pendingTxId */
                pendingTxId?: (number|Long|null);

                /** CommitTransfer newState */
                newState?: (m10.sdk.transaction.CommitTransfer.TransferState|null);
            }

            /** Represents a CommitTransfer. */
            class CommitTransfer implements ICommitTransfer {

                /**
                 * Constructs a new CommitTransfer.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: m10.sdk.transaction.ICommitTransfer);

                /** CommitTransfer pendingTxId. */
                public pendingTxId: (number|Long);

                /** CommitTransfer newState. */
                public newState: m10.sdk.transaction.CommitTransfer.TransferState;

                /**
                 * Creates a new CommitTransfer instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns CommitTransfer instance
                 */
                public static create(properties?: m10.sdk.transaction.ICommitTransfer): m10.sdk.transaction.CommitTransfer;

                /**
                 * Encodes the specified CommitTransfer message. Does not implicitly {@link m10.sdk.transaction.CommitTransfer.verify|verify} messages.
                 * @param message CommitTransfer message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: m10.sdk.transaction.ICommitTransfer, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified CommitTransfer message, length delimited. Does not implicitly {@link m10.sdk.transaction.CommitTransfer.verify|verify} messages.
                 * @param message CommitTransfer message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: m10.sdk.transaction.ICommitTransfer, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a CommitTransfer message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns CommitTransfer
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.sdk.transaction.CommitTransfer;

                /**
                 * Decodes a CommitTransfer message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns CommitTransfer
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.sdk.transaction.CommitTransfer;

                /**
                 * Verifies a CommitTransfer message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a CommitTransfer message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns CommitTransfer
                 */
                public static fromObject(object: { [k: string]: any }): m10.sdk.transaction.CommitTransfer;

                /**
                 * Creates a plain object from a CommitTransfer message. Also converts values to other types if specified.
                 * @param message CommitTransfer
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: m10.sdk.transaction.CommitTransfer, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this CommitTransfer to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            namespace CommitTransfer {

                /** TransferState enum. */
                enum TransferState {
                    ACCEPTED = 0,
                    REJECTED = 1
                }
            }
        }
    }

    /** Namespace directory. */
    namespace directory {

        /** Represents a DirectoryService */
        class DirectoryService extends $protobuf.rpc.Service {

            /**
             * Constructs a new DirectoryService service.
             * @param rpcImpl RPC implementation
             * @param [requestDelimited=false] Whether requests are length-delimited
             * @param [responseDelimited=false] Whether responses are length-delimited
             */
            constructor(rpcImpl: $protobuf.RPCImpl, requestDelimited?: boolean, responseDelimited?: boolean);

            /**
             * Creates new DirectoryService service using the specified rpc implementation.
             * @param rpcImpl RPC implementation
             * @param [requestDelimited=false] Whether requests are length-delimited
             * @param [responseDelimited=false] Whether responses are length-delimited
             * @returns RPC service. Useful where requests and/or responses are streamed.
             */
            public static create(rpcImpl: $protobuf.RPCImpl, requestDelimited?: boolean, responseDelimited?: boolean): DirectoryService;

            /**
             * Calls CreateLedger.
             * @param request Ledger message or plain object
             * @param callback Node-style callback called with the error, if any, and Empty
             */
            public createLedger(request: m10.directory.ILedger, callback: m10.directory.DirectoryService.CreateLedgerCallback): void;

            /**
             * Calls CreateLedger.
             * @param request Ledger message or plain object
             * @returns Promise
             */
            public createLedger(request: m10.directory.ILedger): Promise<google.protobuf.Empty>;

            /**
             * Calls ListLedgers.
             * @param request Empty message or plain object
             * @param callback Node-style callback called with the error, if any, and ListLedgersResponse
             */
            public listLedgers(request: google.protobuf.IEmpty, callback: m10.directory.DirectoryService.ListLedgersCallback): void;

            /**
             * Calls ListLedgers.
             * @param request Empty message or plain object
             * @returns Promise
             */
            public listLedgers(request: google.protobuf.IEmpty): Promise<m10.directory.ListLedgersResponse>;

            /**
             * Calls CheckAlias.
             * @param request CheckAliasRequest message or plain object
             * @param callback Node-style callback called with the error, if any, and Empty
             */
            public checkAlias(request: m10.directory.ICheckAliasRequest, callback: m10.directory.DirectoryService.CheckAliasCallback): void;

            /**
             * Calls CheckAlias.
             * @param request CheckAliasRequest message or plain object
             * @returns Promise
             */
            public checkAlias(request: m10.directory.ICheckAliasRequest): Promise<google.protobuf.Empty>;

            /**
             * Calls CreateAlias.
             * @param request Alias message or plain object
             * @param callback Node-style callback called with the error, if any, and Empty
             */
            public createAlias(request: m10.directory.IAlias, callback: m10.directory.DirectoryService.CreateAliasCallback): void;

            /**
             * Calls CreateAlias.
             * @param request Alias message or plain object
             * @returns Promise
             */
            public createAlias(request: m10.directory.IAlias): Promise<google.protobuf.Empty>;

            /**
             * Calls SearchAliases.
             * @param request SearchAliasesRequest message or plain object
             * @param callback Node-style callback called with the error, if any, and SearchAliasesResponse
             */
            public searchAliases(request: m10.directory.ISearchAliasesRequest, callback: m10.directory.DirectoryService.SearchAliasesCallback): void;

            /**
             * Calls SearchAliases.
             * @param request SearchAliasesRequest message or plain object
             * @returns Promise
             */
            public searchAliases(request: m10.directory.ISearchAliasesRequest): Promise<m10.directory.SearchAliasesResponse>;

            /**
             * Calls CreateObjectUrl.
             * @param request Empty message or plain object
             * @param callback Node-style callback called with the error, if any, and ObjectUrlResponse
             */
            public createObjectUrl(request: google.protobuf.IEmpty, callback: m10.directory.DirectoryService.CreateObjectUrlCallback): void;

            /**
             * Calls CreateObjectUrl.
             * @param request Empty message or plain object
             * @returns Promise
             */
            public createObjectUrl(request: google.protobuf.IEmpty): Promise<m10.directory.ObjectUrlResponse>;

            /**
             * Calls GetObjectUrl.
             * @param request GetObjectUrlRequest message or plain object
             * @param callback Node-style callback called with the error, if any, and ObjectUrlResponse
             */
            public getObjectUrl(request: m10.directory.IGetObjectUrlRequest, callback: m10.directory.DirectoryService.GetObjectUrlCallback): void;

            /**
             * Calls GetObjectUrl.
             * @param request GetObjectUrlRequest message or plain object
             * @returns Promise
             */
            public getObjectUrl(request: m10.directory.IGetObjectUrlRequest): Promise<m10.directory.ObjectUrlResponse>;

            /**
             * Calls CreateImageUrl.
             * @param request CreateImageUrlRequest message or plain object
             * @param callback Node-style callback called with the error, if any, and ObjectUrlResponse
             */
            public createImageUrl(request: m10.directory.ICreateImageUrlRequest, callback: m10.directory.DirectoryService.CreateImageUrlCallback): void;

            /**
             * Calls CreateImageUrl.
             * @param request CreateImageUrlRequest message or plain object
             * @returns Promise
             */
            public createImageUrl(request: m10.directory.ICreateImageUrlRequest): Promise<m10.directory.ObjectUrlResponse>;
        }

        namespace DirectoryService {

            /**
             * Callback as used by {@link m10.directory.DirectoryService#createLedger}.
             * @param error Error, if any
             * @param [response] Empty
             */
            type CreateLedgerCallback = (error: (Error|null), response?: google.protobuf.Empty) => void;

            /**
             * Callback as used by {@link m10.directory.DirectoryService#listLedgers}.
             * @param error Error, if any
             * @param [response] ListLedgersResponse
             */
            type ListLedgersCallback = (error: (Error|null), response?: m10.directory.ListLedgersResponse) => void;

            /**
             * Callback as used by {@link m10.directory.DirectoryService#checkAlias}.
             * @param error Error, if any
             * @param [response] Empty
             */
            type CheckAliasCallback = (error: (Error|null), response?: google.protobuf.Empty) => void;

            /**
             * Callback as used by {@link m10.directory.DirectoryService#createAlias}.
             * @param error Error, if any
             * @param [response] Empty
             */
            type CreateAliasCallback = (error: (Error|null), response?: google.protobuf.Empty) => void;

            /**
             * Callback as used by {@link m10.directory.DirectoryService#searchAliases}.
             * @param error Error, if any
             * @param [response] SearchAliasesResponse
             */
            type SearchAliasesCallback = (error: (Error|null), response?: m10.directory.SearchAliasesResponse) => void;

            /**
             * Callback as used by {@link m10.directory.DirectoryService#createObjectUrl}.
             * @param error Error, if any
             * @param [response] ObjectUrlResponse
             */
            type CreateObjectUrlCallback = (error: (Error|null), response?: m10.directory.ObjectUrlResponse) => void;

            /**
             * Callback as used by {@link m10.directory.DirectoryService#getObjectUrl}.
             * @param error Error, if any
             * @param [response] ObjectUrlResponse
             */
            type GetObjectUrlCallback = (error: (Error|null), response?: m10.directory.ObjectUrlResponse) => void;

            /**
             * Callback as used by {@link m10.directory.DirectoryService#createImageUrl}.
             * @param error Error, if any
             * @param [response] ObjectUrlResponse
             */
            type CreateImageUrlCallback = (error: (Error|null), response?: m10.directory.ObjectUrlResponse) => void;
        }

        /** Properties of a Ledger. */
        interface ILedger {

            /** Ledger operator */
            operator?: (string|null);

            /** Ledger url */
            url?: (string|null);

            /** Ledger code */
            code?: (string|null);

            /** Ledger decimals */
            decimals?: (number|null);
        }

        /** Represents a Ledger. */
        class Ledger implements ILedger {

            /**
             * Constructs a new Ledger.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.directory.ILedger);

            /** Ledger operator. */
            public operator: string;

            /** Ledger url. */
            public url: string;

            /** Ledger code. */
            public code: string;

            /** Ledger decimals. */
            public decimals: number;

            /**
             * Creates a new Ledger instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Ledger instance
             */
            public static create(properties?: m10.directory.ILedger): m10.directory.Ledger;

            /**
             * Encodes the specified Ledger message. Does not implicitly {@link m10.directory.Ledger.verify|verify} messages.
             * @param message Ledger message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.directory.ILedger, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Ledger message, length delimited. Does not implicitly {@link m10.directory.Ledger.verify|verify} messages.
             * @param message Ledger message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.directory.ILedger, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a Ledger message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Ledger
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.directory.Ledger;

            /**
             * Decodes a Ledger message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Ledger
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.directory.Ledger;

            /**
             * Verifies a Ledger message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a Ledger message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Ledger
             */
            public static fromObject(object: { [k: string]: any }): m10.directory.Ledger;

            /**
             * Creates a plain object from a Ledger message. Also converts values to other types if specified.
             * @param message Ledger
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.directory.Ledger, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Ledger to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a ListLedgersResponse. */
        interface IListLedgersResponse {

            /** ListLedgersResponse ledgers */
            ledgers?: (m10.directory.ILedger[]|null);
        }

        /** Represents a ListLedgersResponse. */
        class ListLedgersResponse implements IListLedgersResponse {

            /**
             * Constructs a new ListLedgersResponse.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.directory.IListLedgersResponse);

            /** ListLedgersResponse ledgers. */
            public ledgers: m10.directory.ILedger[];

            /**
             * Creates a new ListLedgersResponse instance using the specified properties.
             * @param [properties] Properties to set
             * @returns ListLedgersResponse instance
             */
            public static create(properties?: m10.directory.IListLedgersResponse): m10.directory.ListLedgersResponse;

            /**
             * Encodes the specified ListLedgersResponse message. Does not implicitly {@link m10.directory.ListLedgersResponse.verify|verify} messages.
             * @param message ListLedgersResponse message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.directory.IListLedgersResponse, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified ListLedgersResponse message, length delimited. Does not implicitly {@link m10.directory.ListLedgersResponse.verify|verify} messages.
             * @param message ListLedgersResponse message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.directory.IListLedgersResponse, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a ListLedgersResponse message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns ListLedgersResponse
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.directory.ListLedgersResponse;

            /**
             * Decodes a ListLedgersResponse message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns ListLedgersResponse
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.directory.ListLedgersResponse;

            /**
             * Verifies a ListLedgersResponse message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a ListLedgersResponse message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns ListLedgersResponse
             */
            public static fromObject(object: { [k: string]: any }): m10.directory.ListLedgersResponse;

            /**
             * Creates a plain object from a ListLedgersResponse message. Also converts values to other types if specified.
             * @param message ListLedgersResponse
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.directory.ListLedgersResponse, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this ListLedgersResponse to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of an Alias. */
        interface IAlias {

            /** Alias handle */
            handle?: (string|null);

            /** Alias displayName */
            displayName?: (string|null);

            /** Alias accountSetId */
            accountSetId?: (Uint8Array|null);

            /** Alias operator */
            operator?: (string|null);

            /** Alias code */
            code?: (string|null);

            /** Alias aliasType */
            aliasType?: (m10.directory.Alias.Type|null);
        }

        /** Represents an Alias. */
        class Alias implements IAlias {

            /**
             * Constructs a new Alias.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.directory.IAlias);

            /** Alias handle. */
            public handle: string;

            /** Alias displayName. */
            public displayName: string;

            /** Alias accountSetId. */
            public accountSetId: Uint8Array;

            /** Alias operator. */
            public operator: string;

            /** Alias code. */
            public code: string;

            /** Alias aliasType. */
            public aliasType: m10.directory.Alias.Type;

            /**
             * Creates a new Alias instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Alias instance
             */
            public static create(properties?: m10.directory.IAlias): m10.directory.Alias;

            /**
             * Encodes the specified Alias message. Does not implicitly {@link m10.directory.Alias.verify|verify} messages.
             * @param message Alias message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.directory.IAlias, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Alias message, length delimited. Does not implicitly {@link m10.directory.Alias.verify|verify} messages.
             * @param message Alias message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.directory.IAlias, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an Alias message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Alias
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.directory.Alias;

            /**
             * Decodes an Alias message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Alias
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.directory.Alias;

            /**
             * Verifies an Alias message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an Alias message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Alias
             */
            public static fromObject(object: { [k: string]: any }): m10.directory.Alias;

            /**
             * Creates a plain object from an Alias message. Also converts values to other types if specified.
             * @param message Alias
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.directory.Alias, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Alias to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        namespace Alias {

            /** Type enum. */
            enum Type {
                HANDLE = 0,
                EMAIL = 1,
                PHONE = 2
            }
        }

        /** Properties of a CheckAliasRequest. */
        interface ICheckAliasRequest {

            /** CheckAliasRequest handle */
            handle?: (string|null);
        }

        /** Represents a CheckAliasRequest. */
        class CheckAliasRequest implements ICheckAliasRequest {

            /**
             * Constructs a new CheckAliasRequest.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.directory.ICheckAliasRequest);

            /** CheckAliasRequest handle. */
            public handle: string;

            /**
             * Creates a new CheckAliasRequest instance using the specified properties.
             * @param [properties] Properties to set
             * @returns CheckAliasRequest instance
             */
            public static create(properties?: m10.directory.ICheckAliasRequest): m10.directory.CheckAliasRequest;

            /**
             * Encodes the specified CheckAliasRequest message. Does not implicitly {@link m10.directory.CheckAliasRequest.verify|verify} messages.
             * @param message CheckAliasRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.directory.ICheckAliasRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified CheckAliasRequest message, length delimited. Does not implicitly {@link m10.directory.CheckAliasRequest.verify|verify} messages.
             * @param message CheckAliasRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.directory.ICheckAliasRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a CheckAliasRequest message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns CheckAliasRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.directory.CheckAliasRequest;

            /**
             * Decodes a CheckAliasRequest message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns CheckAliasRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.directory.CheckAliasRequest;

            /**
             * Verifies a CheckAliasRequest message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a CheckAliasRequest message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns CheckAliasRequest
             */
            public static fromObject(object: { [k: string]: any }): m10.directory.CheckAliasRequest;

            /**
             * Creates a plain object from a CheckAliasRequest message. Also converts values to other types if specified.
             * @param message CheckAliasRequest
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.directory.CheckAliasRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this CheckAliasRequest to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a SearchAliasesRequest. */
        interface ISearchAliasesRequest {

            /** SearchAliasesRequest handlePrefix */
            handlePrefix?: (string|null);

            /** SearchAliasesRequest pageSize */
            pageSize?: (number|null);

            /** SearchAliasesRequest pageToken */
            pageToken?: (string|null);

            /** SearchAliasesRequest subject */
            subject?: (string|null);
        }

        /** Represents a SearchAliasesRequest. */
        class SearchAliasesRequest implements ISearchAliasesRequest {

            /**
             * Constructs a new SearchAliasesRequest.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.directory.ISearchAliasesRequest);

            /** SearchAliasesRequest handlePrefix. */
            public handlePrefix: string;

            /** SearchAliasesRequest pageSize. */
            public pageSize: number;

            /** SearchAliasesRequest pageToken. */
            public pageToken: string;

            /** SearchAliasesRequest subject. */
            public subject: string;

            /**
             * Creates a new SearchAliasesRequest instance using the specified properties.
             * @param [properties] Properties to set
             * @returns SearchAliasesRequest instance
             */
            public static create(properties?: m10.directory.ISearchAliasesRequest): m10.directory.SearchAliasesRequest;

            /**
             * Encodes the specified SearchAliasesRequest message. Does not implicitly {@link m10.directory.SearchAliasesRequest.verify|verify} messages.
             * @param message SearchAliasesRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.directory.ISearchAliasesRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified SearchAliasesRequest message, length delimited. Does not implicitly {@link m10.directory.SearchAliasesRequest.verify|verify} messages.
             * @param message SearchAliasesRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.directory.ISearchAliasesRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a SearchAliasesRequest message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns SearchAliasesRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.directory.SearchAliasesRequest;

            /**
             * Decodes a SearchAliasesRequest message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns SearchAliasesRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.directory.SearchAliasesRequest;

            /**
             * Verifies a SearchAliasesRequest message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a SearchAliasesRequest message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns SearchAliasesRequest
             */
            public static fromObject(object: { [k: string]: any }): m10.directory.SearchAliasesRequest;

            /**
             * Creates a plain object from a SearchAliasesRequest message. Also converts values to other types if specified.
             * @param message SearchAliasesRequest
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.directory.SearchAliasesRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this SearchAliasesRequest to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a SearchAliasesResponse. */
        interface ISearchAliasesResponse {

            /** SearchAliasesResponse aliases */
            aliases?: (m10.directory.IAlias[]|null);

            /** SearchAliasesResponse nextPageToken */
            nextPageToken?: (string|null);
        }

        /** Represents a SearchAliasesResponse. */
        class SearchAliasesResponse implements ISearchAliasesResponse {

            /**
             * Constructs a new SearchAliasesResponse.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.directory.ISearchAliasesResponse);

            /** SearchAliasesResponse aliases. */
            public aliases: m10.directory.IAlias[];

            /** SearchAliasesResponse nextPageToken. */
            public nextPageToken: string;

            /**
             * Creates a new SearchAliasesResponse instance using the specified properties.
             * @param [properties] Properties to set
             * @returns SearchAliasesResponse instance
             */
            public static create(properties?: m10.directory.ISearchAliasesResponse): m10.directory.SearchAliasesResponse;

            /**
             * Encodes the specified SearchAliasesResponse message. Does not implicitly {@link m10.directory.SearchAliasesResponse.verify|verify} messages.
             * @param message SearchAliasesResponse message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.directory.ISearchAliasesResponse, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified SearchAliasesResponse message, length delimited. Does not implicitly {@link m10.directory.SearchAliasesResponse.verify|verify} messages.
             * @param message SearchAliasesResponse message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.directory.ISearchAliasesResponse, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a SearchAliasesResponse message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns SearchAliasesResponse
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.directory.SearchAliasesResponse;

            /**
             * Decodes a SearchAliasesResponse message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns SearchAliasesResponse
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.directory.SearchAliasesResponse;

            /**
             * Verifies a SearchAliasesResponse message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a SearchAliasesResponse message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns SearchAliasesResponse
             */
            public static fromObject(object: { [k: string]: any }): m10.directory.SearchAliasesResponse;

            /**
             * Creates a plain object from a SearchAliasesResponse message. Also converts values to other types if specified.
             * @param message SearchAliasesResponse
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.directory.SearchAliasesResponse, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this SearchAliasesResponse to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a GetObjectUrlRequest. */
        interface IGetObjectUrlRequest {

            /** GetObjectUrlRequest objectId */
            objectId?: (string|null);
        }

        /** Represents a GetObjectUrlRequest. */
        class GetObjectUrlRequest implements IGetObjectUrlRequest {

            /**
             * Constructs a new GetObjectUrlRequest.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.directory.IGetObjectUrlRequest);

            /** GetObjectUrlRequest objectId. */
            public objectId: string;

            /**
             * Creates a new GetObjectUrlRequest instance using the specified properties.
             * @param [properties] Properties to set
             * @returns GetObjectUrlRequest instance
             */
            public static create(properties?: m10.directory.IGetObjectUrlRequest): m10.directory.GetObjectUrlRequest;

            /**
             * Encodes the specified GetObjectUrlRequest message. Does not implicitly {@link m10.directory.GetObjectUrlRequest.verify|verify} messages.
             * @param message GetObjectUrlRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.directory.IGetObjectUrlRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified GetObjectUrlRequest message, length delimited. Does not implicitly {@link m10.directory.GetObjectUrlRequest.verify|verify} messages.
             * @param message GetObjectUrlRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.directory.IGetObjectUrlRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a GetObjectUrlRequest message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns GetObjectUrlRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.directory.GetObjectUrlRequest;

            /**
             * Decodes a GetObjectUrlRequest message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns GetObjectUrlRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.directory.GetObjectUrlRequest;

            /**
             * Verifies a GetObjectUrlRequest message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a GetObjectUrlRequest message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns GetObjectUrlRequest
             */
            public static fromObject(object: { [k: string]: any }): m10.directory.GetObjectUrlRequest;

            /**
             * Creates a plain object from a GetObjectUrlRequest message. Also converts values to other types if specified.
             * @param message GetObjectUrlRequest
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.directory.GetObjectUrlRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this GetObjectUrlRequest to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of an ObjectUrlResponse. */
        interface IObjectUrlResponse {

            /** ObjectUrlResponse presignedUrl */
            presignedUrl?: (string|null);

            /** ObjectUrlResponse objectId */
            objectId?: (string|null);

            /** ObjectUrlResponse url */
            url?: (string|null);
        }

        /** Represents an ObjectUrlResponse. */
        class ObjectUrlResponse implements IObjectUrlResponse {

            /**
             * Constructs a new ObjectUrlResponse.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.directory.IObjectUrlResponse);

            /** ObjectUrlResponse presignedUrl. */
            public presignedUrl: string;

            /** ObjectUrlResponse objectId. */
            public objectId: string;

            /** ObjectUrlResponse url. */
            public url: string;

            /**
             * Creates a new ObjectUrlResponse instance using the specified properties.
             * @param [properties] Properties to set
             * @returns ObjectUrlResponse instance
             */
            public static create(properties?: m10.directory.IObjectUrlResponse): m10.directory.ObjectUrlResponse;

            /**
             * Encodes the specified ObjectUrlResponse message. Does not implicitly {@link m10.directory.ObjectUrlResponse.verify|verify} messages.
             * @param message ObjectUrlResponse message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.directory.IObjectUrlResponse, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified ObjectUrlResponse message, length delimited. Does not implicitly {@link m10.directory.ObjectUrlResponse.verify|verify} messages.
             * @param message ObjectUrlResponse message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.directory.IObjectUrlResponse, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an ObjectUrlResponse message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns ObjectUrlResponse
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.directory.ObjectUrlResponse;

            /**
             * Decodes an ObjectUrlResponse message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns ObjectUrlResponse
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.directory.ObjectUrlResponse;

            /**
             * Verifies an ObjectUrlResponse message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an ObjectUrlResponse message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns ObjectUrlResponse
             */
            public static fromObject(object: { [k: string]: any }): m10.directory.ObjectUrlResponse;

            /**
             * Creates a plain object from an ObjectUrlResponse message. Also converts values to other types if specified.
             * @param message ObjectUrlResponse
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.directory.ObjectUrlResponse, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this ObjectUrlResponse to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a CreateImageUrlRequest. */
        interface ICreateImageUrlRequest {

            /** CreateImageUrlRequest mimeType */
            mimeType?: (string|null);
        }

        /** Represents a CreateImageUrlRequest. */
        class CreateImageUrlRequest implements ICreateImageUrlRequest {

            /**
             * Constructs a new CreateImageUrlRequest.
             * @param [properties] Properties to set
             */
            constructor(properties?: m10.directory.ICreateImageUrlRequest);

            /** CreateImageUrlRequest mimeType. */
            public mimeType?: (string|null);

            /** CreateImageUrlRequest _mimeType. */
            public _mimeType?: "mimeType";

            /**
             * Creates a new CreateImageUrlRequest instance using the specified properties.
             * @param [properties] Properties to set
             * @returns CreateImageUrlRequest instance
             */
            public static create(properties?: m10.directory.ICreateImageUrlRequest): m10.directory.CreateImageUrlRequest;

            /**
             * Encodes the specified CreateImageUrlRequest message. Does not implicitly {@link m10.directory.CreateImageUrlRequest.verify|verify} messages.
             * @param message CreateImageUrlRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: m10.directory.ICreateImageUrlRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified CreateImageUrlRequest message, length delimited. Does not implicitly {@link m10.directory.CreateImageUrlRequest.verify|verify} messages.
             * @param message CreateImageUrlRequest message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: m10.directory.ICreateImageUrlRequest, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a CreateImageUrlRequest message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns CreateImageUrlRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): m10.directory.CreateImageUrlRequest;

            /**
             * Decodes a CreateImageUrlRequest message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns CreateImageUrlRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): m10.directory.CreateImageUrlRequest;

            /**
             * Verifies a CreateImageUrlRequest message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a CreateImageUrlRequest message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns CreateImageUrlRequest
             */
            public static fromObject(object: { [k: string]: any }): m10.directory.CreateImageUrlRequest;

            /**
             * Creates a plain object from a CreateImageUrlRequest message. Also converts values to other types if specified.
             * @param message CreateImageUrlRequest
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: m10.directory.CreateImageUrlRequest, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this CreateImageUrlRequest to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }
    }
}

/** Namespace google. */
export namespace google {

    /** Namespace protobuf. */
    namespace protobuf {

        /** Properties of an Empty. */
        interface IEmpty {
        }

        /** Represents an Empty. */
        class Empty implements IEmpty {

            /**
             * Constructs a new Empty.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.IEmpty);

            /**
             * Creates a new Empty instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Empty instance
             */
            public static create(properties?: google.protobuf.IEmpty): google.protobuf.Empty;

            /**
             * Encodes the specified Empty message. Does not implicitly {@link google.protobuf.Empty.verify|verify} messages.
             * @param message Empty message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.IEmpty, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Empty message, length delimited. Does not implicitly {@link google.protobuf.Empty.verify|verify} messages.
             * @param message Empty message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.IEmpty, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an Empty message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Empty
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.Empty;

            /**
             * Decodes an Empty message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Empty
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.Empty;

            /**
             * Verifies an Empty message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an Empty message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Empty
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.Empty;

            /**
             * Creates a plain object from an Empty message. Also converts values to other types if specified.
             * @param message Empty
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.Empty, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Empty to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a FileDescriptorSet. */
        interface IFileDescriptorSet {

            /** FileDescriptorSet file */
            file?: (google.protobuf.IFileDescriptorProto[]|null);
        }

        /** Represents a FileDescriptorSet. */
        class FileDescriptorSet implements IFileDescriptorSet {

            /**
             * Constructs a new FileDescriptorSet.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.IFileDescriptorSet);

            /** FileDescriptorSet file. */
            public file: google.protobuf.IFileDescriptorProto[];

            /**
             * Creates a new FileDescriptorSet instance using the specified properties.
             * @param [properties] Properties to set
             * @returns FileDescriptorSet instance
             */
            public static create(properties?: google.protobuf.IFileDescriptorSet): google.protobuf.FileDescriptorSet;

            /**
             * Encodes the specified FileDescriptorSet message. Does not implicitly {@link google.protobuf.FileDescriptorSet.verify|verify} messages.
             * @param message FileDescriptorSet message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.IFileDescriptorSet, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified FileDescriptorSet message, length delimited. Does not implicitly {@link google.protobuf.FileDescriptorSet.verify|verify} messages.
             * @param message FileDescriptorSet message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.IFileDescriptorSet, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a FileDescriptorSet message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns FileDescriptorSet
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.FileDescriptorSet;

            /**
             * Decodes a FileDescriptorSet message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns FileDescriptorSet
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.FileDescriptorSet;

            /**
             * Verifies a FileDescriptorSet message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a FileDescriptorSet message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns FileDescriptorSet
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.FileDescriptorSet;

            /**
             * Creates a plain object from a FileDescriptorSet message. Also converts values to other types if specified.
             * @param message FileDescriptorSet
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.FileDescriptorSet, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this FileDescriptorSet to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a FileDescriptorProto. */
        interface IFileDescriptorProto {

            /** FileDescriptorProto name */
            name?: (string|null);

            /** FileDescriptorProto package */
            "package"?: (string|null);

            /** FileDescriptorProto dependency */
            dependency?: (string[]|null);

            /** FileDescriptorProto publicDependency */
            publicDependency?: (number[]|null);

            /** FileDescriptorProto weakDependency */
            weakDependency?: (number[]|null);

            /** FileDescriptorProto messageType */
            messageType?: (google.protobuf.IDescriptorProto[]|null);

            /** FileDescriptorProto enumType */
            enumType?: (google.protobuf.IEnumDescriptorProto[]|null);

            /** FileDescriptorProto service */
            service?: (google.protobuf.IServiceDescriptorProto[]|null);

            /** FileDescriptorProto extension */
            extension?: (google.protobuf.IFieldDescriptorProto[]|null);

            /** FileDescriptorProto options */
            options?: (google.protobuf.IFileOptions|null);

            /** FileDescriptorProto sourceCodeInfo */
            sourceCodeInfo?: (google.protobuf.ISourceCodeInfo|null);

            /** FileDescriptorProto syntax */
            syntax?: (string|null);
        }

        /** Represents a FileDescriptorProto. */
        class FileDescriptorProto implements IFileDescriptorProto {

            /**
             * Constructs a new FileDescriptorProto.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.IFileDescriptorProto);

            /** FileDescriptorProto name. */
            public name: string;

            /** FileDescriptorProto package. */
            public package: string;

            /** FileDescriptorProto dependency. */
            public dependency: string[];

            /** FileDescriptorProto publicDependency. */
            public publicDependency: number[];

            /** FileDescriptorProto weakDependency. */
            public weakDependency: number[];

            /** FileDescriptorProto messageType. */
            public messageType: google.protobuf.IDescriptorProto[];

            /** FileDescriptorProto enumType. */
            public enumType: google.protobuf.IEnumDescriptorProto[];

            /** FileDescriptorProto service. */
            public service: google.protobuf.IServiceDescriptorProto[];

            /** FileDescriptorProto extension. */
            public extension: google.protobuf.IFieldDescriptorProto[];

            /** FileDescriptorProto options. */
            public options?: (google.protobuf.IFileOptions|null);

            /** FileDescriptorProto sourceCodeInfo. */
            public sourceCodeInfo?: (google.protobuf.ISourceCodeInfo|null);

            /** FileDescriptorProto syntax. */
            public syntax: string;

            /**
             * Creates a new FileDescriptorProto instance using the specified properties.
             * @param [properties] Properties to set
             * @returns FileDescriptorProto instance
             */
            public static create(properties?: google.protobuf.IFileDescriptorProto): google.protobuf.FileDescriptorProto;

            /**
             * Encodes the specified FileDescriptorProto message. Does not implicitly {@link google.protobuf.FileDescriptorProto.verify|verify} messages.
             * @param message FileDescriptorProto message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.IFileDescriptorProto, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified FileDescriptorProto message, length delimited. Does not implicitly {@link google.protobuf.FileDescriptorProto.verify|verify} messages.
             * @param message FileDescriptorProto message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.IFileDescriptorProto, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a FileDescriptorProto message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns FileDescriptorProto
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.FileDescriptorProto;

            /**
             * Decodes a FileDescriptorProto message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns FileDescriptorProto
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.FileDescriptorProto;

            /**
             * Verifies a FileDescriptorProto message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a FileDescriptorProto message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns FileDescriptorProto
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.FileDescriptorProto;

            /**
             * Creates a plain object from a FileDescriptorProto message. Also converts values to other types if specified.
             * @param message FileDescriptorProto
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.FileDescriptorProto, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this FileDescriptorProto to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a DescriptorProto. */
        interface IDescriptorProto {

            /** DescriptorProto name */
            name?: (string|null);

            /** DescriptorProto field */
            field?: (google.protobuf.IFieldDescriptorProto[]|null);

            /** DescriptorProto extension */
            extension?: (google.protobuf.IFieldDescriptorProto[]|null);

            /** DescriptorProto nestedType */
            nestedType?: (google.protobuf.IDescriptorProto[]|null);

            /** DescriptorProto enumType */
            enumType?: (google.protobuf.IEnumDescriptorProto[]|null);

            /** DescriptorProto extensionRange */
            extensionRange?: (google.protobuf.DescriptorProto.IExtensionRange[]|null);

            /** DescriptorProto oneofDecl */
            oneofDecl?: (google.protobuf.IOneofDescriptorProto[]|null);

            /** DescriptorProto options */
            options?: (google.protobuf.IMessageOptions|null);

            /** DescriptorProto reservedRange */
            reservedRange?: (google.protobuf.DescriptorProto.IReservedRange[]|null);

            /** DescriptorProto reservedName */
            reservedName?: (string[]|null);
        }

        /** Represents a DescriptorProto. */
        class DescriptorProto implements IDescriptorProto {

            /**
             * Constructs a new DescriptorProto.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.IDescriptorProto);

            /** DescriptorProto name. */
            public name: string;

            /** DescriptorProto field. */
            public field: google.protobuf.IFieldDescriptorProto[];

            /** DescriptorProto extension. */
            public extension: google.protobuf.IFieldDescriptorProto[];

            /** DescriptorProto nestedType. */
            public nestedType: google.protobuf.IDescriptorProto[];

            /** DescriptorProto enumType. */
            public enumType: google.protobuf.IEnumDescriptorProto[];

            /** DescriptorProto extensionRange. */
            public extensionRange: google.protobuf.DescriptorProto.IExtensionRange[];

            /** DescriptorProto oneofDecl. */
            public oneofDecl: google.protobuf.IOneofDescriptorProto[];

            /** DescriptorProto options. */
            public options?: (google.protobuf.IMessageOptions|null);

            /** DescriptorProto reservedRange. */
            public reservedRange: google.protobuf.DescriptorProto.IReservedRange[];

            /** DescriptorProto reservedName. */
            public reservedName: string[];

            /**
             * Creates a new DescriptorProto instance using the specified properties.
             * @param [properties] Properties to set
             * @returns DescriptorProto instance
             */
            public static create(properties?: google.protobuf.IDescriptorProto): google.protobuf.DescriptorProto;

            /**
             * Encodes the specified DescriptorProto message. Does not implicitly {@link google.protobuf.DescriptorProto.verify|verify} messages.
             * @param message DescriptorProto message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.IDescriptorProto, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified DescriptorProto message, length delimited. Does not implicitly {@link google.protobuf.DescriptorProto.verify|verify} messages.
             * @param message DescriptorProto message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.IDescriptorProto, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a DescriptorProto message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns DescriptorProto
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.DescriptorProto;

            /**
             * Decodes a DescriptorProto message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns DescriptorProto
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.DescriptorProto;

            /**
             * Verifies a DescriptorProto message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a DescriptorProto message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns DescriptorProto
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.DescriptorProto;

            /**
             * Creates a plain object from a DescriptorProto message. Also converts values to other types if specified.
             * @param message DescriptorProto
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.DescriptorProto, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this DescriptorProto to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        namespace DescriptorProto {

            /** Properties of an ExtensionRange. */
            interface IExtensionRange {

                /** ExtensionRange start */
                start?: (number|null);

                /** ExtensionRange end */
                end?: (number|null);
            }

            /** Represents an ExtensionRange. */
            class ExtensionRange implements IExtensionRange {

                /**
                 * Constructs a new ExtensionRange.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: google.protobuf.DescriptorProto.IExtensionRange);

                /** ExtensionRange start. */
                public start: number;

                /** ExtensionRange end. */
                public end: number;

                /**
                 * Creates a new ExtensionRange instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns ExtensionRange instance
                 */
                public static create(properties?: google.protobuf.DescriptorProto.IExtensionRange): google.protobuf.DescriptorProto.ExtensionRange;

                /**
                 * Encodes the specified ExtensionRange message. Does not implicitly {@link google.protobuf.DescriptorProto.ExtensionRange.verify|verify} messages.
                 * @param message ExtensionRange message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: google.protobuf.DescriptorProto.IExtensionRange, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified ExtensionRange message, length delimited. Does not implicitly {@link google.protobuf.DescriptorProto.ExtensionRange.verify|verify} messages.
                 * @param message ExtensionRange message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: google.protobuf.DescriptorProto.IExtensionRange, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes an ExtensionRange message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns ExtensionRange
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.DescriptorProto.ExtensionRange;

                /**
                 * Decodes an ExtensionRange message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns ExtensionRange
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.DescriptorProto.ExtensionRange;

                /**
                 * Verifies an ExtensionRange message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates an ExtensionRange message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns ExtensionRange
                 */
                public static fromObject(object: { [k: string]: any }): google.protobuf.DescriptorProto.ExtensionRange;

                /**
                 * Creates a plain object from an ExtensionRange message. Also converts values to other types if specified.
                 * @param message ExtensionRange
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: google.protobuf.DescriptorProto.ExtensionRange, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this ExtensionRange to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }

            /** Properties of a ReservedRange. */
            interface IReservedRange {

                /** ReservedRange start */
                start?: (number|null);

                /** ReservedRange end */
                end?: (number|null);
            }

            /** Represents a ReservedRange. */
            class ReservedRange implements IReservedRange {

                /**
                 * Constructs a new ReservedRange.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: google.protobuf.DescriptorProto.IReservedRange);

                /** ReservedRange start. */
                public start: number;

                /** ReservedRange end. */
                public end: number;

                /**
                 * Creates a new ReservedRange instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns ReservedRange instance
                 */
                public static create(properties?: google.protobuf.DescriptorProto.IReservedRange): google.protobuf.DescriptorProto.ReservedRange;

                /**
                 * Encodes the specified ReservedRange message. Does not implicitly {@link google.protobuf.DescriptorProto.ReservedRange.verify|verify} messages.
                 * @param message ReservedRange message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: google.protobuf.DescriptorProto.IReservedRange, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified ReservedRange message, length delimited. Does not implicitly {@link google.protobuf.DescriptorProto.ReservedRange.verify|verify} messages.
                 * @param message ReservedRange message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: google.protobuf.DescriptorProto.IReservedRange, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a ReservedRange message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns ReservedRange
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.DescriptorProto.ReservedRange;

                /**
                 * Decodes a ReservedRange message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns ReservedRange
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.DescriptorProto.ReservedRange;

                /**
                 * Verifies a ReservedRange message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a ReservedRange message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns ReservedRange
                 */
                public static fromObject(object: { [k: string]: any }): google.protobuf.DescriptorProto.ReservedRange;

                /**
                 * Creates a plain object from a ReservedRange message. Also converts values to other types if specified.
                 * @param message ReservedRange
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: google.protobuf.DescriptorProto.ReservedRange, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this ReservedRange to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }
        }

        /** Properties of a FieldDescriptorProto. */
        interface IFieldDescriptorProto {

            /** FieldDescriptorProto name */
            name?: (string|null);

            /** FieldDescriptorProto number */
            number?: (number|null);

            /** FieldDescriptorProto label */
            label?: (google.protobuf.FieldDescriptorProto.Label|null);

            /** FieldDescriptorProto type */
            type?: (google.protobuf.FieldDescriptorProto.Type|null);

            /** FieldDescriptorProto typeName */
            typeName?: (string|null);

            /** FieldDescriptorProto extendee */
            extendee?: (string|null);

            /** FieldDescriptorProto defaultValue */
            defaultValue?: (string|null);

            /** FieldDescriptorProto oneofIndex */
            oneofIndex?: (number|null);

            /** FieldDescriptorProto jsonName */
            jsonName?: (string|null);

            /** FieldDescriptorProto options */
            options?: (google.protobuf.IFieldOptions|null);
        }

        /** Represents a FieldDescriptorProto. */
        class FieldDescriptorProto implements IFieldDescriptorProto {

            /**
             * Constructs a new FieldDescriptorProto.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.IFieldDescriptorProto);

            /** FieldDescriptorProto name. */
            public name: string;

            /** FieldDescriptorProto number. */
            public number: number;

            /** FieldDescriptorProto label. */
            public label: google.protobuf.FieldDescriptorProto.Label;

            /** FieldDescriptorProto type. */
            public type: google.protobuf.FieldDescriptorProto.Type;

            /** FieldDescriptorProto typeName. */
            public typeName: string;

            /** FieldDescriptorProto extendee. */
            public extendee: string;

            /** FieldDescriptorProto defaultValue. */
            public defaultValue: string;

            /** FieldDescriptorProto oneofIndex. */
            public oneofIndex: number;

            /** FieldDescriptorProto jsonName. */
            public jsonName: string;

            /** FieldDescriptorProto options. */
            public options?: (google.protobuf.IFieldOptions|null);

            /**
             * Creates a new FieldDescriptorProto instance using the specified properties.
             * @param [properties] Properties to set
             * @returns FieldDescriptorProto instance
             */
            public static create(properties?: google.protobuf.IFieldDescriptorProto): google.protobuf.FieldDescriptorProto;

            /**
             * Encodes the specified FieldDescriptorProto message. Does not implicitly {@link google.protobuf.FieldDescriptorProto.verify|verify} messages.
             * @param message FieldDescriptorProto message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.IFieldDescriptorProto, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified FieldDescriptorProto message, length delimited. Does not implicitly {@link google.protobuf.FieldDescriptorProto.verify|verify} messages.
             * @param message FieldDescriptorProto message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.IFieldDescriptorProto, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a FieldDescriptorProto message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns FieldDescriptorProto
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.FieldDescriptorProto;

            /**
             * Decodes a FieldDescriptorProto message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns FieldDescriptorProto
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.FieldDescriptorProto;

            /**
             * Verifies a FieldDescriptorProto message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a FieldDescriptorProto message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns FieldDescriptorProto
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.FieldDescriptorProto;

            /**
             * Creates a plain object from a FieldDescriptorProto message. Also converts values to other types if specified.
             * @param message FieldDescriptorProto
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.FieldDescriptorProto, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this FieldDescriptorProto to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        namespace FieldDescriptorProto {

            /** Type enum. */
            enum Type {
                TYPE_DOUBLE = 1,
                TYPE_FLOAT = 2,
                TYPE_INT64 = 3,
                TYPE_UINT64 = 4,
                TYPE_INT32 = 5,
                TYPE_FIXED64 = 6,
                TYPE_FIXED32 = 7,
                TYPE_BOOL = 8,
                TYPE_STRING = 9,
                TYPE_GROUP = 10,
                TYPE_MESSAGE = 11,
                TYPE_BYTES = 12,
                TYPE_UINT32 = 13,
                TYPE_ENUM = 14,
                TYPE_SFIXED32 = 15,
                TYPE_SFIXED64 = 16,
                TYPE_SINT32 = 17,
                TYPE_SINT64 = 18
            }

            /** Label enum. */
            enum Label {
                LABEL_OPTIONAL = 1,
                LABEL_REQUIRED = 2,
                LABEL_REPEATED = 3
            }
        }

        /** Properties of an OneofDescriptorProto. */
        interface IOneofDescriptorProto {

            /** OneofDescriptorProto name */
            name?: (string|null);

            /** OneofDescriptorProto options */
            options?: (google.protobuf.IOneofOptions|null);
        }

        /** Represents an OneofDescriptorProto. */
        class OneofDescriptorProto implements IOneofDescriptorProto {

            /**
             * Constructs a new OneofDescriptorProto.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.IOneofDescriptorProto);

            /** OneofDescriptorProto name. */
            public name: string;

            /** OneofDescriptorProto options. */
            public options?: (google.protobuf.IOneofOptions|null);

            /**
             * Creates a new OneofDescriptorProto instance using the specified properties.
             * @param [properties] Properties to set
             * @returns OneofDescriptorProto instance
             */
            public static create(properties?: google.protobuf.IOneofDescriptorProto): google.protobuf.OneofDescriptorProto;

            /**
             * Encodes the specified OneofDescriptorProto message. Does not implicitly {@link google.protobuf.OneofDescriptorProto.verify|verify} messages.
             * @param message OneofDescriptorProto message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.IOneofDescriptorProto, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified OneofDescriptorProto message, length delimited. Does not implicitly {@link google.protobuf.OneofDescriptorProto.verify|verify} messages.
             * @param message OneofDescriptorProto message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.IOneofDescriptorProto, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an OneofDescriptorProto message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns OneofDescriptorProto
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.OneofDescriptorProto;

            /**
             * Decodes an OneofDescriptorProto message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns OneofDescriptorProto
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.OneofDescriptorProto;

            /**
             * Verifies an OneofDescriptorProto message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an OneofDescriptorProto message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns OneofDescriptorProto
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.OneofDescriptorProto;

            /**
             * Creates a plain object from an OneofDescriptorProto message. Also converts values to other types if specified.
             * @param message OneofDescriptorProto
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.OneofDescriptorProto, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this OneofDescriptorProto to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of an EnumDescriptorProto. */
        interface IEnumDescriptorProto {

            /** EnumDescriptorProto name */
            name?: (string|null);

            /** EnumDescriptorProto value */
            value?: (google.protobuf.IEnumValueDescriptorProto[]|null);

            /** EnumDescriptorProto options */
            options?: (google.protobuf.IEnumOptions|null);
        }

        /** Represents an EnumDescriptorProto. */
        class EnumDescriptorProto implements IEnumDescriptorProto {

            /**
             * Constructs a new EnumDescriptorProto.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.IEnumDescriptorProto);

            /** EnumDescriptorProto name. */
            public name: string;

            /** EnumDescriptorProto value. */
            public value: google.protobuf.IEnumValueDescriptorProto[];

            /** EnumDescriptorProto options. */
            public options?: (google.protobuf.IEnumOptions|null);

            /**
             * Creates a new EnumDescriptorProto instance using the specified properties.
             * @param [properties] Properties to set
             * @returns EnumDescriptorProto instance
             */
            public static create(properties?: google.protobuf.IEnumDescriptorProto): google.protobuf.EnumDescriptorProto;

            /**
             * Encodes the specified EnumDescriptorProto message. Does not implicitly {@link google.protobuf.EnumDescriptorProto.verify|verify} messages.
             * @param message EnumDescriptorProto message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.IEnumDescriptorProto, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified EnumDescriptorProto message, length delimited. Does not implicitly {@link google.protobuf.EnumDescriptorProto.verify|verify} messages.
             * @param message EnumDescriptorProto message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.IEnumDescriptorProto, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an EnumDescriptorProto message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns EnumDescriptorProto
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.EnumDescriptorProto;

            /**
             * Decodes an EnumDescriptorProto message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns EnumDescriptorProto
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.EnumDescriptorProto;

            /**
             * Verifies an EnumDescriptorProto message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an EnumDescriptorProto message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns EnumDescriptorProto
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.EnumDescriptorProto;

            /**
             * Creates a plain object from an EnumDescriptorProto message. Also converts values to other types if specified.
             * @param message EnumDescriptorProto
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.EnumDescriptorProto, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this EnumDescriptorProto to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of an EnumValueDescriptorProto. */
        interface IEnumValueDescriptorProto {

            /** EnumValueDescriptorProto name */
            name?: (string|null);

            /** EnumValueDescriptorProto number */
            number?: (number|null);

            /** EnumValueDescriptorProto options */
            options?: (google.protobuf.IEnumValueOptions|null);
        }

        /** Represents an EnumValueDescriptorProto. */
        class EnumValueDescriptorProto implements IEnumValueDescriptorProto {

            /**
             * Constructs a new EnumValueDescriptorProto.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.IEnumValueDescriptorProto);

            /** EnumValueDescriptorProto name. */
            public name: string;

            /** EnumValueDescriptorProto number. */
            public number: number;

            /** EnumValueDescriptorProto options. */
            public options?: (google.protobuf.IEnumValueOptions|null);

            /**
             * Creates a new EnumValueDescriptorProto instance using the specified properties.
             * @param [properties] Properties to set
             * @returns EnumValueDescriptorProto instance
             */
            public static create(properties?: google.protobuf.IEnumValueDescriptorProto): google.protobuf.EnumValueDescriptorProto;

            /**
             * Encodes the specified EnumValueDescriptorProto message. Does not implicitly {@link google.protobuf.EnumValueDescriptorProto.verify|verify} messages.
             * @param message EnumValueDescriptorProto message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.IEnumValueDescriptorProto, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified EnumValueDescriptorProto message, length delimited. Does not implicitly {@link google.protobuf.EnumValueDescriptorProto.verify|verify} messages.
             * @param message EnumValueDescriptorProto message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.IEnumValueDescriptorProto, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an EnumValueDescriptorProto message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns EnumValueDescriptorProto
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.EnumValueDescriptorProto;

            /**
             * Decodes an EnumValueDescriptorProto message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns EnumValueDescriptorProto
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.EnumValueDescriptorProto;

            /**
             * Verifies an EnumValueDescriptorProto message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an EnumValueDescriptorProto message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns EnumValueDescriptorProto
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.EnumValueDescriptorProto;

            /**
             * Creates a plain object from an EnumValueDescriptorProto message. Also converts values to other types if specified.
             * @param message EnumValueDescriptorProto
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.EnumValueDescriptorProto, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this EnumValueDescriptorProto to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a ServiceDescriptorProto. */
        interface IServiceDescriptorProto {

            /** ServiceDescriptorProto name */
            name?: (string|null);

            /** ServiceDescriptorProto method */
            method?: (google.protobuf.IMethodDescriptorProto[]|null);

            /** ServiceDescriptorProto options */
            options?: (google.protobuf.IServiceOptions|null);
        }

        /** Represents a ServiceDescriptorProto. */
        class ServiceDescriptorProto implements IServiceDescriptorProto {

            /**
             * Constructs a new ServiceDescriptorProto.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.IServiceDescriptorProto);

            /** ServiceDescriptorProto name. */
            public name: string;

            /** ServiceDescriptorProto method. */
            public method: google.protobuf.IMethodDescriptorProto[];

            /** ServiceDescriptorProto options. */
            public options?: (google.protobuf.IServiceOptions|null);

            /**
             * Creates a new ServiceDescriptorProto instance using the specified properties.
             * @param [properties] Properties to set
             * @returns ServiceDescriptorProto instance
             */
            public static create(properties?: google.protobuf.IServiceDescriptorProto): google.protobuf.ServiceDescriptorProto;

            /**
             * Encodes the specified ServiceDescriptorProto message. Does not implicitly {@link google.protobuf.ServiceDescriptorProto.verify|verify} messages.
             * @param message ServiceDescriptorProto message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.IServiceDescriptorProto, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified ServiceDescriptorProto message, length delimited. Does not implicitly {@link google.protobuf.ServiceDescriptorProto.verify|verify} messages.
             * @param message ServiceDescriptorProto message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.IServiceDescriptorProto, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a ServiceDescriptorProto message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns ServiceDescriptorProto
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.ServiceDescriptorProto;

            /**
             * Decodes a ServiceDescriptorProto message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns ServiceDescriptorProto
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.ServiceDescriptorProto;

            /**
             * Verifies a ServiceDescriptorProto message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a ServiceDescriptorProto message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns ServiceDescriptorProto
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.ServiceDescriptorProto;

            /**
             * Creates a plain object from a ServiceDescriptorProto message. Also converts values to other types if specified.
             * @param message ServiceDescriptorProto
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.ServiceDescriptorProto, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this ServiceDescriptorProto to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a MethodDescriptorProto. */
        interface IMethodDescriptorProto {

            /** MethodDescriptorProto name */
            name?: (string|null);

            /** MethodDescriptorProto inputType */
            inputType?: (string|null);

            /** MethodDescriptorProto outputType */
            outputType?: (string|null);

            /** MethodDescriptorProto options */
            options?: (google.protobuf.IMethodOptions|null);

            /** MethodDescriptorProto clientStreaming */
            clientStreaming?: (boolean|null);

            /** MethodDescriptorProto serverStreaming */
            serverStreaming?: (boolean|null);
        }

        /** Represents a MethodDescriptorProto. */
        class MethodDescriptorProto implements IMethodDescriptorProto {

            /**
             * Constructs a new MethodDescriptorProto.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.IMethodDescriptorProto);

            /** MethodDescriptorProto name. */
            public name: string;

            /** MethodDescriptorProto inputType. */
            public inputType: string;

            /** MethodDescriptorProto outputType. */
            public outputType: string;

            /** MethodDescriptorProto options. */
            public options?: (google.protobuf.IMethodOptions|null);

            /** MethodDescriptorProto clientStreaming. */
            public clientStreaming: boolean;

            /** MethodDescriptorProto serverStreaming. */
            public serverStreaming: boolean;

            /**
             * Creates a new MethodDescriptorProto instance using the specified properties.
             * @param [properties] Properties to set
             * @returns MethodDescriptorProto instance
             */
            public static create(properties?: google.protobuf.IMethodDescriptorProto): google.protobuf.MethodDescriptorProto;

            /**
             * Encodes the specified MethodDescriptorProto message. Does not implicitly {@link google.protobuf.MethodDescriptorProto.verify|verify} messages.
             * @param message MethodDescriptorProto message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.IMethodDescriptorProto, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified MethodDescriptorProto message, length delimited. Does not implicitly {@link google.protobuf.MethodDescriptorProto.verify|verify} messages.
             * @param message MethodDescriptorProto message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.IMethodDescriptorProto, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a MethodDescriptorProto message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns MethodDescriptorProto
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.MethodDescriptorProto;

            /**
             * Decodes a MethodDescriptorProto message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns MethodDescriptorProto
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.MethodDescriptorProto;

            /**
             * Verifies a MethodDescriptorProto message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a MethodDescriptorProto message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns MethodDescriptorProto
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.MethodDescriptorProto;

            /**
             * Creates a plain object from a MethodDescriptorProto message. Also converts values to other types if specified.
             * @param message MethodDescriptorProto
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.MethodDescriptorProto, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this MethodDescriptorProto to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a FileOptions. */
        interface IFileOptions {

            /** FileOptions javaPackage */
            javaPackage?: (string|null);

            /** FileOptions javaOuterClassname */
            javaOuterClassname?: (string|null);

            /** FileOptions javaMultipleFiles */
            javaMultipleFiles?: (boolean|null);

            /** FileOptions javaGenerateEqualsAndHash */
            javaGenerateEqualsAndHash?: (boolean|null);

            /** FileOptions javaStringCheckUtf8 */
            javaStringCheckUtf8?: (boolean|null);

            /** FileOptions optimizeFor */
            optimizeFor?: (google.protobuf.FileOptions.OptimizeMode|null);

            /** FileOptions goPackage */
            goPackage?: (string|null);

            /** FileOptions ccGenericServices */
            ccGenericServices?: (boolean|null);

            /** FileOptions javaGenericServices */
            javaGenericServices?: (boolean|null);

            /** FileOptions pyGenericServices */
            pyGenericServices?: (boolean|null);

            /** FileOptions deprecated */
            deprecated?: (boolean|null);

            /** FileOptions ccEnableArenas */
            ccEnableArenas?: (boolean|null);

            /** FileOptions objcClassPrefix */
            objcClassPrefix?: (string|null);

            /** FileOptions csharpNamespace */
            csharpNamespace?: (string|null);

            /** FileOptions uninterpretedOption */
            uninterpretedOption?: (google.protobuf.IUninterpretedOption[]|null);
        }

        /** Represents a FileOptions. */
        class FileOptions implements IFileOptions {

            /**
             * Constructs a new FileOptions.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.IFileOptions);

            /** FileOptions javaPackage. */
            public javaPackage: string;

            /** FileOptions javaOuterClassname. */
            public javaOuterClassname: string;

            /** FileOptions javaMultipleFiles. */
            public javaMultipleFiles: boolean;

            /** FileOptions javaGenerateEqualsAndHash. */
            public javaGenerateEqualsAndHash: boolean;

            /** FileOptions javaStringCheckUtf8. */
            public javaStringCheckUtf8: boolean;

            /** FileOptions optimizeFor. */
            public optimizeFor: google.protobuf.FileOptions.OptimizeMode;

            /** FileOptions goPackage. */
            public goPackage: string;

            /** FileOptions ccGenericServices. */
            public ccGenericServices: boolean;

            /** FileOptions javaGenericServices. */
            public javaGenericServices: boolean;

            /** FileOptions pyGenericServices. */
            public pyGenericServices: boolean;

            /** FileOptions deprecated. */
            public deprecated: boolean;

            /** FileOptions ccEnableArenas. */
            public ccEnableArenas: boolean;

            /** FileOptions objcClassPrefix. */
            public objcClassPrefix: string;

            /** FileOptions csharpNamespace. */
            public csharpNamespace: string;

            /** FileOptions uninterpretedOption. */
            public uninterpretedOption: google.protobuf.IUninterpretedOption[];

            /**
             * Creates a new FileOptions instance using the specified properties.
             * @param [properties] Properties to set
             * @returns FileOptions instance
             */
            public static create(properties?: google.protobuf.IFileOptions): google.protobuf.FileOptions;

            /**
             * Encodes the specified FileOptions message. Does not implicitly {@link google.protobuf.FileOptions.verify|verify} messages.
             * @param message FileOptions message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.IFileOptions, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified FileOptions message, length delimited. Does not implicitly {@link google.protobuf.FileOptions.verify|verify} messages.
             * @param message FileOptions message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.IFileOptions, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a FileOptions message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns FileOptions
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.FileOptions;

            /**
             * Decodes a FileOptions message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns FileOptions
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.FileOptions;

            /**
             * Verifies a FileOptions message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a FileOptions message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns FileOptions
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.FileOptions;

            /**
             * Creates a plain object from a FileOptions message. Also converts values to other types if specified.
             * @param message FileOptions
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.FileOptions, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this FileOptions to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        namespace FileOptions {

            /** OptimizeMode enum. */
            enum OptimizeMode {
                SPEED = 1,
                CODE_SIZE = 2,
                LITE_RUNTIME = 3
            }
        }

        /** Properties of a MessageOptions. */
        interface IMessageOptions {

            /** MessageOptions messageSetWireFormat */
            messageSetWireFormat?: (boolean|null);

            /** MessageOptions noStandardDescriptorAccessor */
            noStandardDescriptorAccessor?: (boolean|null);

            /** MessageOptions deprecated */
            deprecated?: (boolean|null);

            /** MessageOptions mapEntry */
            mapEntry?: (boolean|null);

            /** MessageOptions uninterpretedOption */
            uninterpretedOption?: (google.protobuf.IUninterpretedOption[]|null);
        }

        /** Represents a MessageOptions. */
        class MessageOptions implements IMessageOptions {

            /**
             * Constructs a new MessageOptions.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.IMessageOptions);

            /** MessageOptions messageSetWireFormat. */
            public messageSetWireFormat: boolean;

            /** MessageOptions noStandardDescriptorAccessor. */
            public noStandardDescriptorAccessor: boolean;

            /** MessageOptions deprecated. */
            public deprecated: boolean;

            /** MessageOptions mapEntry. */
            public mapEntry: boolean;

            /** MessageOptions uninterpretedOption. */
            public uninterpretedOption: google.protobuf.IUninterpretedOption[];

            /**
             * Creates a new MessageOptions instance using the specified properties.
             * @param [properties] Properties to set
             * @returns MessageOptions instance
             */
            public static create(properties?: google.protobuf.IMessageOptions): google.protobuf.MessageOptions;

            /**
             * Encodes the specified MessageOptions message. Does not implicitly {@link google.protobuf.MessageOptions.verify|verify} messages.
             * @param message MessageOptions message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.IMessageOptions, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified MessageOptions message, length delimited. Does not implicitly {@link google.protobuf.MessageOptions.verify|verify} messages.
             * @param message MessageOptions message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.IMessageOptions, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a MessageOptions message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns MessageOptions
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.MessageOptions;

            /**
             * Decodes a MessageOptions message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns MessageOptions
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.MessageOptions;

            /**
             * Verifies a MessageOptions message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a MessageOptions message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns MessageOptions
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.MessageOptions;

            /**
             * Creates a plain object from a MessageOptions message. Also converts values to other types if specified.
             * @param message MessageOptions
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.MessageOptions, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this MessageOptions to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a FieldOptions. */
        interface IFieldOptions {

            /** FieldOptions ctype */
            ctype?: (google.protobuf.FieldOptions.CType|null);

            /** FieldOptions packed */
            packed?: (boolean|null);

            /** FieldOptions jstype */
            jstype?: (google.protobuf.FieldOptions.JSType|null);

            /** FieldOptions lazy */
            lazy?: (boolean|null);

            /** FieldOptions deprecated */
            deprecated?: (boolean|null);

            /** FieldOptions weak */
            weak?: (boolean|null);

            /** FieldOptions uninterpretedOption */
            uninterpretedOption?: (google.protobuf.IUninterpretedOption[]|null);
        }

        /** Represents a FieldOptions. */
        class FieldOptions implements IFieldOptions {

            /**
             * Constructs a new FieldOptions.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.IFieldOptions);

            /** FieldOptions ctype. */
            public ctype: google.protobuf.FieldOptions.CType;

            /** FieldOptions packed. */
            public packed: boolean;

            /** FieldOptions jstype. */
            public jstype: google.protobuf.FieldOptions.JSType;

            /** FieldOptions lazy. */
            public lazy: boolean;

            /** FieldOptions deprecated. */
            public deprecated: boolean;

            /** FieldOptions weak. */
            public weak: boolean;

            /** FieldOptions uninterpretedOption. */
            public uninterpretedOption: google.protobuf.IUninterpretedOption[];

            /**
             * Creates a new FieldOptions instance using the specified properties.
             * @param [properties] Properties to set
             * @returns FieldOptions instance
             */
            public static create(properties?: google.protobuf.IFieldOptions): google.protobuf.FieldOptions;

            /**
             * Encodes the specified FieldOptions message. Does not implicitly {@link google.protobuf.FieldOptions.verify|verify} messages.
             * @param message FieldOptions message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.IFieldOptions, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified FieldOptions message, length delimited. Does not implicitly {@link google.protobuf.FieldOptions.verify|verify} messages.
             * @param message FieldOptions message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.IFieldOptions, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a FieldOptions message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns FieldOptions
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.FieldOptions;

            /**
             * Decodes a FieldOptions message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns FieldOptions
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.FieldOptions;

            /**
             * Verifies a FieldOptions message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a FieldOptions message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns FieldOptions
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.FieldOptions;

            /**
             * Creates a plain object from a FieldOptions message. Also converts values to other types if specified.
             * @param message FieldOptions
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.FieldOptions, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this FieldOptions to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        namespace FieldOptions {

            /** CType enum. */
            enum CType {
                STRING = 0,
                CORD = 1,
                STRING_PIECE = 2
            }

            /** JSType enum. */
            enum JSType {
                JS_NORMAL = 0,
                JS_STRING = 1,
                JS_NUMBER = 2
            }
        }

        /** Properties of an OneofOptions. */
        interface IOneofOptions {

            /** OneofOptions uninterpretedOption */
            uninterpretedOption?: (google.protobuf.IUninterpretedOption[]|null);
        }

        /** Represents an OneofOptions. */
        class OneofOptions implements IOneofOptions {

            /**
             * Constructs a new OneofOptions.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.IOneofOptions);

            /** OneofOptions uninterpretedOption. */
            public uninterpretedOption: google.protobuf.IUninterpretedOption[];

            /**
             * Creates a new OneofOptions instance using the specified properties.
             * @param [properties] Properties to set
             * @returns OneofOptions instance
             */
            public static create(properties?: google.protobuf.IOneofOptions): google.protobuf.OneofOptions;

            /**
             * Encodes the specified OneofOptions message. Does not implicitly {@link google.protobuf.OneofOptions.verify|verify} messages.
             * @param message OneofOptions message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.IOneofOptions, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified OneofOptions message, length delimited. Does not implicitly {@link google.protobuf.OneofOptions.verify|verify} messages.
             * @param message OneofOptions message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.IOneofOptions, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an OneofOptions message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns OneofOptions
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.OneofOptions;

            /**
             * Decodes an OneofOptions message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns OneofOptions
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.OneofOptions;

            /**
             * Verifies an OneofOptions message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an OneofOptions message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns OneofOptions
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.OneofOptions;

            /**
             * Creates a plain object from an OneofOptions message. Also converts values to other types if specified.
             * @param message OneofOptions
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.OneofOptions, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this OneofOptions to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of an EnumOptions. */
        interface IEnumOptions {

            /** EnumOptions allowAlias */
            allowAlias?: (boolean|null);

            /** EnumOptions deprecated */
            deprecated?: (boolean|null);

            /** EnumOptions uninterpretedOption */
            uninterpretedOption?: (google.protobuf.IUninterpretedOption[]|null);
        }

        /** Represents an EnumOptions. */
        class EnumOptions implements IEnumOptions {

            /**
             * Constructs a new EnumOptions.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.IEnumOptions);

            /** EnumOptions allowAlias. */
            public allowAlias: boolean;

            /** EnumOptions deprecated. */
            public deprecated: boolean;

            /** EnumOptions uninterpretedOption. */
            public uninterpretedOption: google.protobuf.IUninterpretedOption[];

            /**
             * Creates a new EnumOptions instance using the specified properties.
             * @param [properties] Properties to set
             * @returns EnumOptions instance
             */
            public static create(properties?: google.protobuf.IEnumOptions): google.protobuf.EnumOptions;

            /**
             * Encodes the specified EnumOptions message. Does not implicitly {@link google.protobuf.EnumOptions.verify|verify} messages.
             * @param message EnumOptions message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.IEnumOptions, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified EnumOptions message, length delimited. Does not implicitly {@link google.protobuf.EnumOptions.verify|verify} messages.
             * @param message EnumOptions message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.IEnumOptions, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an EnumOptions message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns EnumOptions
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.EnumOptions;

            /**
             * Decodes an EnumOptions message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns EnumOptions
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.EnumOptions;

            /**
             * Verifies an EnumOptions message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an EnumOptions message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns EnumOptions
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.EnumOptions;

            /**
             * Creates a plain object from an EnumOptions message. Also converts values to other types if specified.
             * @param message EnumOptions
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.EnumOptions, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this EnumOptions to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of an EnumValueOptions. */
        interface IEnumValueOptions {

            /** EnumValueOptions deprecated */
            deprecated?: (boolean|null);

            /** EnumValueOptions uninterpretedOption */
            uninterpretedOption?: (google.protobuf.IUninterpretedOption[]|null);
        }

        /** Represents an EnumValueOptions. */
        class EnumValueOptions implements IEnumValueOptions {

            /**
             * Constructs a new EnumValueOptions.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.IEnumValueOptions);

            /** EnumValueOptions deprecated. */
            public deprecated: boolean;

            /** EnumValueOptions uninterpretedOption. */
            public uninterpretedOption: google.protobuf.IUninterpretedOption[];

            /**
             * Creates a new EnumValueOptions instance using the specified properties.
             * @param [properties] Properties to set
             * @returns EnumValueOptions instance
             */
            public static create(properties?: google.protobuf.IEnumValueOptions): google.protobuf.EnumValueOptions;

            /**
             * Encodes the specified EnumValueOptions message. Does not implicitly {@link google.protobuf.EnumValueOptions.verify|verify} messages.
             * @param message EnumValueOptions message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.IEnumValueOptions, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified EnumValueOptions message, length delimited. Does not implicitly {@link google.protobuf.EnumValueOptions.verify|verify} messages.
             * @param message EnumValueOptions message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.IEnumValueOptions, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an EnumValueOptions message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns EnumValueOptions
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.EnumValueOptions;

            /**
             * Decodes an EnumValueOptions message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns EnumValueOptions
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.EnumValueOptions;

            /**
             * Verifies an EnumValueOptions message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an EnumValueOptions message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns EnumValueOptions
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.EnumValueOptions;

            /**
             * Creates a plain object from an EnumValueOptions message. Also converts values to other types if specified.
             * @param message EnumValueOptions
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.EnumValueOptions, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this EnumValueOptions to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a ServiceOptions. */
        interface IServiceOptions {

            /** ServiceOptions deprecated */
            deprecated?: (boolean|null);

            /** ServiceOptions uninterpretedOption */
            uninterpretedOption?: (google.protobuf.IUninterpretedOption[]|null);
        }

        /** Represents a ServiceOptions. */
        class ServiceOptions implements IServiceOptions {

            /**
             * Constructs a new ServiceOptions.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.IServiceOptions);

            /** ServiceOptions deprecated. */
            public deprecated: boolean;

            /** ServiceOptions uninterpretedOption. */
            public uninterpretedOption: google.protobuf.IUninterpretedOption[];

            /**
             * Creates a new ServiceOptions instance using the specified properties.
             * @param [properties] Properties to set
             * @returns ServiceOptions instance
             */
            public static create(properties?: google.protobuf.IServiceOptions): google.protobuf.ServiceOptions;

            /**
             * Encodes the specified ServiceOptions message. Does not implicitly {@link google.protobuf.ServiceOptions.verify|verify} messages.
             * @param message ServiceOptions message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.IServiceOptions, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified ServiceOptions message, length delimited. Does not implicitly {@link google.protobuf.ServiceOptions.verify|verify} messages.
             * @param message ServiceOptions message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.IServiceOptions, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a ServiceOptions message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns ServiceOptions
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.ServiceOptions;

            /**
             * Decodes a ServiceOptions message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns ServiceOptions
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.ServiceOptions;

            /**
             * Verifies a ServiceOptions message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a ServiceOptions message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns ServiceOptions
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.ServiceOptions;

            /**
             * Creates a plain object from a ServiceOptions message. Also converts values to other types if specified.
             * @param message ServiceOptions
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.ServiceOptions, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this ServiceOptions to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of a MethodOptions. */
        interface IMethodOptions {

            /** MethodOptions deprecated */
            deprecated?: (boolean|null);

            /** MethodOptions uninterpretedOption */
            uninterpretedOption?: (google.protobuf.IUninterpretedOption[]|null);
        }

        /** Represents a MethodOptions. */
        class MethodOptions implements IMethodOptions {

            /**
             * Constructs a new MethodOptions.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.IMethodOptions);

            /** MethodOptions deprecated. */
            public deprecated: boolean;

            /** MethodOptions uninterpretedOption. */
            public uninterpretedOption: google.protobuf.IUninterpretedOption[];

            /**
             * Creates a new MethodOptions instance using the specified properties.
             * @param [properties] Properties to set
             * @returns MethodOptions instance
             */
            public static create(properties?: google.protobuf.IMethodOptions): google.protobuf.MethodOptions;

            /**
             * Encodes the specified MethodOptions message. Does not implicitly {@link google.protobuf.MethodOptions.verify|verify} messages.
             * @param message MethodOptions message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.IMethodOptions, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified MethodOptions message, length delimited. Does not implicitly {@link google.protobuf.MethodOptions.verify|verify} messages.
             * @param message MethodOptions message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.IMethodOptions, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a MethodOptions message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns MethodOptions
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.MethodOptions;

            /**
             * Decodes a MethodOptions message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns MethodOptions
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.MethodOptions;

            /**
             * Verifies a MethodOptions message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a MethodOptions message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns MethodOptions
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.MethodOptions;

            /**
             * Creates a plain object from a MethodOptions message. Also converts values to other types if specified.
             * @param message MethodOptions
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.MethodOptions, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this MethodOptions to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of an UninterpretedOption. */
        interface IUninterpretedOption {

            /** UninterpretedOption name */
            name?: (google.protobuf.UninterpretedOption.INamePart[]|null);

            /** UninterpretedOption identifierValue */
            identifierValue?: (string|null);

            /** UninterpretedOption positiveIntValue */
            positiveIntValue?: (number|Long|null);

            /** UninterpretedOption negativeIntValue */
            negativeIntValue?: (number|Long|null);

            /** UninterpretedOption doubleValue */
            doubleValue?: (number|null);

            /** UninterpretedOption stringValue */
            stringValue?: (Uint8Array|null);

            /** UninterpretedOption aggregateValue */
            aggregateValue?: (string|null);
        }

        /** Represents an UninterpretedOption. */
        class UninterpretedOption implements IUninterpretedOption {

            /**
             * Constructs a new UninterpretedOption.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.IUninterpretedOption);

            /** UninterpretedOption name. */
            public name: google.protobuf.UninterpretedOption.INamePart[];

            /** UninterpretedOption identifierValue. */
            public identifierValue: string;

            /** UninterpretedOption positiveIntValue. */
            public positiveIntValue: (number|Long);

            /** UninterpretedOption negativeIntValue. */
            public negativeIntValue: (number|Long);

            /** UninterpretedOption doubleValue. */
            public doubleValue: number;

            /** UninterpretedOption stringValue. */
            public stringValue: Uint8Array;

            /** UninterpretedOption aggregateValue. */
            public aggregateValue: string;

            /**
             * Creates a new UninterpretedOption instance using the specified properties.
             * @param [properties] Properties to set
             * @returns UninterpretedOption instance
             */
            public static create(properties?: google.protobuf.IUninterpretedOption): google.protobuf.UninterpretedOption;

            /**
             * Encodes the specified UninterpretedOption message. Does not implicitly {@link google.protobuf.UninterpretedOption.verify|verify} messages.
             * @param message UninterpretedOption message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.IUninterpretedOption, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified UninterpretedOption message, length delimited. Does not implicitly {@link google.protobuf.UninterpretedOption.verify|verify} messages.
             * @param message UninterpretedOption message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.IUninterpretedOption, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an UninterpretedOption message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns UninterpretedOption
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.UninterpretedOption;

            /**
             * Decodes an UninterpretedOption message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns UninterpretedOption
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.UninterpretedOption;

            /**
             * Verifies an UninterpretedOption message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an UninterpretedOption message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns UninterpretedOption
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.UninterpretedOption;

            /**
             * Creates a plain object from an UninterpretedOption message. Also converts values to other types if specified.
             * @param message UninterpretedOption
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.UninterpretedOption, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this UninterpretedOption to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        namespace UninterpretedOption {

            /** Properties of a NamePart. */
            interface INamePart {

                /** NamePart namePart */
                namePart: string;

                /** NamePart isExtension */
                isExtension: boolean;
            }

            /** Represents a NamePart. */
            class NamePart implements INamePart {

                /**
                 * Constructs a new NamePart.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: google.protobuf.UninterpretedOption.INamePart);

                /** NamePart namePart. */
                public namePart: string;

                /** NamePart isExtension. */
                public isExtension: boolean;

                /**
                 * Creates a new NamePart instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns NamePart instance
                 */
                public static create(properties?: google.protobuf.UninterpretedOption.INamePart): google.protobuf.UninterpretedOption.NamePart;

                /**
                 * Encodes the specified NamePart message. Does not implicitly {@link google.protobuf.UninterpretedOption.NamePart.verify|verify} messages.
                 * @param message NamePart message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: google.protobuf.UninterpretedOption.INamePart, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified NamePart message, length delimited. Does not implicitly {@link google.protobuf.UninterpretedOption.NamePart.verify|verify} messages.
                 * @param message NamePart message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: google.protobuf.UninterpretedOption.INamePart, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a NamePart message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns NamePart
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.UninterpretedOption.NamePart;

                /**
                 * Decodes a NamePart message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns NamePart
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.UninterpretedOption.NamePart;

                /**
                 * Verifies a NamePart message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a NamePart message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns NamePart
                 */
                public static fromObject(object: { [k: string]: any }): google.protobuf.UninterpretedOption.NamePart;

                /**
                 * Creates a plain object from a NamePart message. Also converts values to other types if specified.
                 * @param message NamePart
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: google.protobuf.UninterpretedOption.NamePart, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this NamePart to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }
        }

        /** Properties of a SourceCodeInfo. */
        interface ISourceCodeInfo {

            /** SourceCodeInfo location */
            location?: (google.protobuf.SourceCodeInfo.ILocation[]|null);
        }

        /** Represents a SourceCodeInfo. */
        class SourceCodeInfo implements ISourceCodeInfo {

            /**
             * Constructs a new SourceCodeInfo.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.ISourceCodeInfo);

            /** SourceCodeInfo location. */
            public location: google.protobuf.SourceCodeInfo.ILocation[];

            /**
             * Creates a new SourceCodeInfo instance using the specified properties.
             * @param [properties] Properties to set
             * @returns SourceCodeInfo instance
             */
            public static create(properties?: google.protobuf.ISourceCodeInfo): google.protobuf.SourceCodeInfo;

            /**
             * Encodes the specified SourceCodeInfo message. Does not implicitly {@link google.protobuf.SourceCodeInfo.verify|verify} messages.
             * @param message SourceCodeInfo message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.ISourceCodeInfo, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified SourceCodeInfo message, length delimited. Does not implicitly {@link google.protobuf.SourceCodeInfo.verify|verify} messages.
             * @param message SourceCodeInfo message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.ISourceCodeInfo, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a SourceCodeInfo message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns SourceCodeInfo
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.SourceCodeInfo;

            /**
             * Decodes a SourceCodeInfo message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns SourceCodeInfo
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.SourceCodeInfo;

            /**
             * Verifies a SourceCodeInfo message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a SourceCodeInfo message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns SourceCodeInfo
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.SourceCodeInfo;

            /**
             * Creates a plain object from a SourceCodeInfo message. Also converts values to other types if specified.
             * @param message SourceCodeInfo
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.SourceCodeInfo, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this SourceCodeInfo to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        namespace SourceCodeInfo {

            /** Properties of a Location. */
            interface ILocation {

                /** Location path */
                path?: (number[]|null);

                /** Location span */
                span?: (number[]|null);

                /** Location leadingComments */
                leadingComments?: (string|null);

                /** Location trailingComments */
                trailingComments?: (string|null);

                /** Location leadingDetachedComments */
                leadingDetachedComments?: (string[]|null);
            }

            /** Represents a Location. */
            class Location implements ILocation {

                /**
                 * Constructs a new Location.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: google.protobuf.SourceCodeInfo.ILocation);

                /** Location path. */
                public path: number[];

                /** Location span. */
                public span: number[];

                /** Location leadingComments. */
                public leadingComments: string;

                /** Location trailingComments. */
                public trailingComments: string;

                /** Location leadingDetachedComments. */
                public leadingDetachedComments: string[];

                /**
                 * Creates a new Location instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Location instance
                 */
                public static create(properties?: google.protobuf.SourceCodeInfo.ILocation): google.protobuf.SourceCodeInfo.Location;

                /**
                 * Encodes the specified Location message. Does not implicitly {@link google.protobuf.SourceCodeInfo.Location.verify|verify} messages.
                 * @param message Location message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: google.protobuf.SourceCodeInfo.ILocation, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Location message, length delimited. Does not implicitly {@link google.protobuf.SourceCodeInfo.Location.verify|verify} messages.
                 * @param message Location message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: google.protobuf.SourceCodeInfo.ILocation, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes a Location message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Location
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.SourceCodeInfo.Location;

                /**
                 * Decodes a Location message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Location
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.SourceCodeInfo.Location;

                /**
                 * Verifies a Location message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates a Location message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Location
                 */
                public static fromObject(object: { [k: string]: any }): google.protobuf.SourceCodeInfo.Location;

                /**
                 * Creates a plain object from a Location message. Also converts values to other types if specified.
                 * @param message Location
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: google.protobuf.SourceCodeInfo.Location, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Location to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }
        }

        /** Properties of a GeneratedCodeInfo. */
        interface IGeneratedCodeInfo {

            /** GeneratedCodeInfo annotation */
            annotation?: (google.protobuf.GeneratedCodeInfo.IAnnotation[]|null);
        }

        /** Represents a GeneratedCodeInfo. */
        class GeneratedCodeInfo implements IGeneratedCodeInfo {

            /**
             * Constructs a new GeneratedCodeInfo.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.IGeneratedCodeInfo);

            /** GeneratedCodeInfo annotation. */
            public annotation: google.protobuf.GeneratedCodeInfo.IAnnotation[];

            /**
             * Creates a new GeneratedCodeInfo instance using the specified properties.
             * @param [properties] Properties to set
             * @returns GeneratedCodeInfo instance
             */
            public static create(properties?: google.protobuf.IGeneratedCodeInfo): google.protobuf.GeneratedCodeInfo;

            /**
             * Encodes the specified GeneratedCodeInfo message. Does not implicitly {@link google.protobuf.GeneratedCodeInfo.verify|verify} messages.
             * @param message GeneratedCodeInfo message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.IGeneratedCodeInfo, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified GeneratedCodeInfo message, length delimited. Does not implicitly {@link google.protobuf.GeneratedCodeInfo.verify|verify} messages.
             * @param message GeneratedCodeInfo message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.IGeneratedCodeInfo, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a GeneratedCodeInfo message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns GeneratedCodeInfo
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.GeneratedCodeInfo;

            /**
             * Decodes a GeneratedCodeInfo message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns GeneratedCodeInfo
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.GeneratedCodeInfo;

            /**
             * Verifies a GeneratedCodeInfo message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a GeneratedCodeInfo message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns GeneratedCodeInfo
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.GeneratedCodeInfo;

            /**
             * Creates a plain object from a GeneratedCodeInfo message. Also converts values to other types if specified.
             * @param message GeneratedCodeInfo
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.GeneratedCodeInfo, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this GeneratedCodeInfo to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        namespace GeneratedCodeInfo {

            /** Properties of an Annotation. */
            interface IAnnotation {

                /** Annotation path */
                path?: (number[]|null);

                /** Annotation sourceFile */
                sourceFile?: (string|null);

                /** Annotation begin */
                begin?: (number|null);

                /** Annotation end */
                end?: (number|null);
            }

            /** Represents an Annotation. */
            class Annotation implements IAnnotation {

                /**
                 * Constructs a new Annotation.
                 * @param [properties] Properties to set
                 */
                constructor(properties?: google.protobuf.GeneratedCodeInfo.IAnnotation);

                /** Annotation path. */
                public path: number[];

                /** Annotation sourceFile. */
                public sourceFile: string;

                /** Annotation begin. */
                public begin: number;

                /** Annotation end. */
                public end: number;

                /**
                 * Creates a new Annotation instance using the specified properties.
                 * @param [properties] Properties to set
                 * @returns Annotation instance
                 */
                public static create(properties?: google.protobuf.GeneratedCodeInfo.IAnnotation): google.protobuf.GeneratedCodeInfo.Annotation;

                /**
                 * Encodes the specified Annotation message. Does not implicitly {@link google.protobuf.GeneratedCodeInfo.Annotation.verify|verify} messages.
                 * @param message Annotation message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encode(message: google.protobuf.GeneratedCodeInfo.IAnnotation, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Encodes the specified Annotation message, length delimited. Does not implicitly {@link google.protobuf.GeneratedCodeInfo.Annotation.verify|verify} messages.
                 * @param message Annotation message or plain object to encode
                 * @param [writer] Writer to encode to
                 * @returns Writer
                 */
                public static encodeDelimited(message: google.protobuf.GeneratedCodeInfo.IAnnotation, writer?: $protobuf.Writer): $protobuf.Writer;

                /**
                 * Decodes an Annotation message from the specified reader or buffer.
                 * @param reader Reader or buffer to decode from
                 * @param [length] Message length if known beforehand
                 * @returns Annotation
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.GeneratedCodeInfo.Annotation;

                /**
                 * Decodes an Annotation message from the specified reader or buffer, length delimited.
                 * @param reader Reader or buffer to decode from
                 * @returns Annotation
                 * @throws {Error} If the payload is not a reader or valid buffer
                 * @throws {$protobuf.util.ProtocolError} If required fields are missing
                 */
                public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.GeneratedCodeInfo.Annotation;

                /**
                 * Verifies an Annotation message.
                 * @param message Plain object to verify
                 * @returns `null` if valid, otherwise the reason why it is not
                 */
                public static verify(message: { [k: string]: any }): (string|null);

                /**
                 * Creates an Annotation message from a plain object. Also converts values to their respective internal types.
                 * @param object Plain object
                 * @returns Annotation
                 */
                public static fromObject(object: { [k: string]: any }): google.protobuf.GeneratedCodeInfo.Annotation;

                /**
                 * Creates a plain object from an Annotation message. Also converts values to other types if specified.
                 * @param message Annotation
                 * @param [options] Conversion options
                 * @returns Plain object
                 */
                public static toObject(message: google.protobuf.GeneratedCodeInfo.Annotation, options?: $protobuf.IConversionOptions): { [k: string]: any };

                /**
                 * Converts this Annotation to JSON.
                 * @returns JSON object
                 */
                public toJSON(): { [k: string]: any };
            }
        }

        /** Properties of a FieldMask. */
        interface IFieldMask {

            /** FieldMask paths */
            paths?: (string[]|null);
        }

        /** Represents a FieldMask. */
        class FieldMask implements IFieldMask {

            /**
             * Constructs a new FieldMask.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.IFieldMask);

            /** FieldMask paths. */
            public paths: string[];

            /**
             * Creates a new FieldMask instance using the specified properties.
             * @param [properties] Properties to set
             * @returns FieldMask instance
             */
            public static create(properties?: google.protobuf.IFieldMask): google.protobuf.FieldMask;

            /**
             * Encodes the specified FieldMask message. Does not implicitly {@link google.protobuf.FieldMask.verify|verify} messages.
             * @param message FieldMask message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.IFieldMask, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified FieldMask message, length delimited. Does not implicitly {@link google.protobuf.FieldMask.verify|verify} messages.
             * @param message FieldMask message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.IFieldMask, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes a FieldMask message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns FieldMask
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.FieldMask;

            /**
             * Decodes a FieldMask message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns FieldMask
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.FieldMask;

            /**
             * Verifies a FieldMask message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates a FieldMask message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns FieldMask
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.FieldMask;

            /**
             * Creates a plain object from a FieldMask message. Also converts values to other types if specified.
             * @param message FieldMask
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.FieldMask, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this FieldMask to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }

        /** Properties of an Any. */
        interface IAny {

            /** Any type_url */
            type_url?: (string|null);

            /** Any value */
            value?: (Uint8Array|null);
        }

        /** Represents an Any. */
        class Any implements IAny {

            /**
             * Constructs a new Any.
             * @param [properties] Properties to set
             */
            constructor(properties?: google.protobuf.IAny);

            /** Any type_url. */
            public type_url: string;

            /** Any value. */
            public value: Uint8Array;

            /**
             * Creates a new Any instance using the specified properties.
             * @param [properties] Properties to set
             * @returns Any instance
             */
            public static create(properties?: google.protobuf.IAny): google.protobuf.Any;

            /**
             * Encodes the specified Any message. Does not implicitly {@link google.protobuf.Any.verify|verify} messages.
             * @param message Any message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encode(message: google.protobuf.IAny, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Encodes the specified Any message, length delimited. Does not implicitly {@link google.protobuf.Any.verify|verify} messages.
             * @param message Any message or plain object to encode
             * @param [writer] Writer to encode to
             * @returns Writer
             */
            public static encodeDelimited(message: google.protobuf.IAny, writer?: $protobuf.Writer): $protobuf.Writer;

            /**
             * Decodes an Any message from the specified reader or buffer.
             * @param reader Reader or buffer to decode from
             * @param [length] Message length if known beforehand
             * @returns Any
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decode(reader: ($protobuf.Reader|Uint8Array), length?: number): google.protobuf.Any;

            /**
             * Decodes an Any message from the specified reader or buffer, length delimited.
             * @param reader Reader or buffer to decode from
             * @returns Any
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            public static decodeDelimited(reader: ($protobuf.Reader|Uint8Array)): google.protobuf.Any;

            /**
             * Verifies an Any message.
             * @param message Plain object to verify
             * @returns `null` if valid, otherwise the reason why it is not
             */
            public static verify(message: { [k: string]: any }): (string|null);

            /**
             * Creates an Any message from a plain object. Also converts values to their respective internal types.
             * @param object Plain object
             * @returns Any
             */
            public static fromObject(object: { [k: string]: any }): google.protobuf.Any;

            /**
             * Creates a plain object from an Any message. Also converts values to other types if specified.
             * @param message Any
             * @param [options] Conversion options
             * @returns Plain object
             */
            public static toObject(message: google.protobuf.Any, options?: $protobuf.IConversionOptions): { [k: string]: any };

            /**
             * Converts this Any to JSON.
             * @returns JSON object
             */
            public toJSON(): { [k: string]: any };
        }
    }
}
