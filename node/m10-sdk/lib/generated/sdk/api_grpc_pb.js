// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var sdk_api_pb = require('./api_pb.js');
var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js');
var sdk_model_model_pb = require('./model/model_pb.js');
var sdk_transaction_transaction_pb = require('./transaction/transaction_pb.js');
var sdk_rbac_pb = require('./rbac_pb.js');
var sdk_document_pb = require('./document_pb.js');

function serialize_google_protobuf_Empty(arg) {
  if (!(arg instanceof google_protobuf_empty_pb.Empty)) {
    throw new Error('Expected argument of type google.protobuf.Empty');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_google_protobuf_Empty(buffer_arg) {
  return google_protobuf_empty_pb.Empty.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_sdk_ChainInfo(arg) {
  if (!(arg instanceof sdk_api_pb.ChainInfo)) {
    throw new Error('Expected argument of type m10.sdk.ChainInfo');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_sdk_ChainInfo(buffer_arg) {
  return sdk_api_pb.ChainInfo.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_sdk_FinalizedTransaction(arg) {
  if (!(arg instanceof sdk_api_pb.FinalizedTransaction)) {
    throw new Error('Expected argument of type m10.sdk.FinalizedTransaction');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_sdk_FinalizedTransaction(buffer_arg) {
  return sdk_api_pb.FinalizedTransaction.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_sdk_FinalizedTransactions(arg) {
  if (!(arg instanceof sdk_api_pb.FinalizedTransactions)) {
    throw new Error('Expected argument of type m10.sdk.FinalizedTransactions');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_sdk_FinalizedTransactions(buffer_arg) {
  return sdk_api_pb.FinalizedTransactions.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_sdk_GroupedFinalizedTransactions(arg) {
  if (!(arg instanceof sdk_api_pb.GroupedFinalizedTransactions)) {
    throw new Error('Expected argument of type m10.sdk.GroupedFinalizedTransactions');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_sdk_GroupedFinalizedTransactions(buffer_arg) {
  return sdk_api_pb.GroupedFinalizedTransactions.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_sdk_ListAccountSetsResponse(arg) {
  if (!(arg instanceof sdk_api_pb.ListAccountSetsResponse)) {
    throw new Error('Expected argument of type m10.sdk.ListAccountSetsResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_sdk_ListAccountSetsResponse(buffer_arg) {
  return sdk_api_pb.ListAccountSetsResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_sdk_ListAccountsResponse(arg) {
  if (!(arg instanceof sdk_api_pb.ListAccountsResponse)) {
    throw new Error('Expected argument of type m10.sdk.ListAccountsResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_sdk_ListAccountsResponse(buffer_arg) {
  return sdk_api_pb.ListAccountsResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_sdk_ListRoleBindingsResponse(arg) {
  if (!(arg instanceof sdk_api_pb.ListRoleBindingsResponse)) {
    throw new Error('Expected argument of type m10.sdk.ListRoleBindingsResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_sdk_ListRoleBindingsResponse(buffer_arg) {
  return sdk_api_pb.ListRoleBindingsResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_sdk_ListRolesResponse(arg) {
  if (!(arg instanceof sdk_api_pb.ListRolesResponse)) {
    throw new Error('Expected argument of type m10.sdk.ListRolesResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_sdk_ListRolesResponse(buffer_arg) {
  return sdk_api_pb.ListRolesResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_sdk_RequestEnvelope(arg) {
  if (!(arg instanceof sdk_api_pb.RequestEnvelope)) {
    throw new Error('Expected argument of type m10.sdk.RequestEnvelope');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_sdk_RequestEnvelope(buffer_arg) {
  return sdk_api_pb.RequestEnvelope.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_sdk_Role(arg) {
  if (!(arg instanceof sdk_rbac_pb.Role)) {
    throw new Error('Expected argument of type m10.sdk.Role');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_sdk_Role(buffer_arg) {
  return sdk_rbac_pb.Role.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_sdk_RoleBinding(arg) {
  if (!(arg instanceof sdk_rbac_pb.RoleBinding)) {
    throw new Error('Expected argument of type m10.sdk.RoleBinding');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_sdk_RoleBinding(buffer_arg) {
  return sdk_rbac_pb.RoleBinding.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_sdk_model_Account(arg) {
  if (!(arg instanceof sdk_model_model_pb.Account)) {
    throw new Error('Expected argument of type m10.sdk.model.Account');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_sdk_model_Account(buffer_arg) {
  return sdk_model_model_pb.Account.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_sdk_model_AccountInfo(arg) {
  if (!(arg instanceof sdk_model_model_pb.AccountInfo)) {
    throw new Error('Expected argument of type m10.sdk.model.AccountInfo');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_sdk_model_AccountInfo(buffer_arg) {
  return sdk_model_model_pb.AccountInfo.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_sdk_model_AccountSet(arg) {
  if (!(arg instanceof sdk_model_model_pb.AccountSet)) {
    throw new Error('Expected argument of type m10.sdk.model.AccountSet');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_sdk_model_AccountSet(buffer_arg) {
  return sdk_model_model_pb.AccountSet.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_sdk_transaction_Action(arg) {
  if (!(arg instanceof sdk_transaction_transaction_pb.Action)) {
    throw new Error('Expected argument of type m10.sdk.transaction.Action');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_sdk_transaction_Action(buffer_arg) {
  return sdk_transaction_transaction_pb.Action.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_sdk_transaction_Actions(arg) {
  if (!(arg instanceof sdk_transaction_transaction_pb.Actions)) {
    throw new Error('Expected argument of type m10.sdk.transaction.Actions');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_sdk_transaction_Actions(buffer_arg) {
  return sdk_transaction_transaction_pb.Actions.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_sdk_transaction_FinalizedTransfer(arg) {
  if (!(arg instanceof sdk_transaction_transaction_pb.FinalizedTransfer)) {
    throw new Error('Expected argument of type m10.sdk.transaction.FinalizedTransfer');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_sdk_transaction_FinalizedTransfer(buffer_arg) {
  return sdk_transaction_transaction_pb.FinalizedTransfer.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_sdk_transaction_FinalizedTransfers(arg) {
  if (!(arg instanceof sdk_transaction_transaction_pb.FinalizedTransfers)) {
    throw new Error('Expected argument of type m10.sdk.transaction.FinalizedTransfers');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_sdk_transaction_FinalizedTransfers(buffer_arg) {
  return sdk_transaction_transaction_pb.FinalizedTransfers.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_sdk_transaction_IndexedAccount(arg) {
  if (!(arg instanceof sdk_transaction_transaction_pb.IndexedAccount)) {
    throw new Error('Expected argument of type m10.sdk.transaction.IndexedAccount');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_sdk_transaction_IndexedAccount(buffer_arg) {
  return sdk_transaction_transaction_pb.IndexedAccount.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_m10_sdk_transaction_TransactionResponse(arg) {
  if (!(arg instanceof sdk_transaction_transaction_pb.TransactionResponse)) {
    throw new Error('Expected argument of type m10.sdk.transaction.TransactionResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_m10_sdk_transaction_TransactionResponse(buffer_arg) {
  return sdk_transaction_transaction_pb.TransactionResponse.deserializeBinary(new Uint8Array(buffer_arg));
}


var M10TxServiceService = exports.M10TxServiceService = {
  createTransaction: {
    path: '/m10.sdk.M10TxService/CreateTransaction',
    requestStream: false,
    responseStream: false,
    requestType: sdk_api_pb.RequestEnvelope,
    responseType: sdk_transaction_transaction_pb.TransactionResponse,
    requestSerialize: serialize_m10_sdk_RequestEnvelope,
    requestDeserialize: deserialize_m10_sdk_RequestEnvelope,
    responseSerialize: serialize_m10_sdk_transaction_TransactionResponse,
    responseDeserialize: deserialize_m10_sdk_transaction_TransactionResponse,
  },
};

exports.M10TxServiceClient = grpc.makeGenericClientConstructor(M10TxServiceService);
var M10QueryServiceService = exports.M10QueryServiceService = {
  // Transfers
getTransfer: {
    path: '/m10.sdk.M10QueryService/GetTransfer',
    requestStream: false,
    responseStream: false,
    requestType: sdk_api_pb.RequestEnvelope,
    responseType: sdk_transaction_transaction_pb.FinalizedTransfer,
    requestSerialize: serialize_m10_sdk_RequestEnvelope,
    requestDeserialize: deserialize_m10_sdk_RequestEnvelope,
    responseSerialize: serialize_m10_sdk_transaction_FinalizedTransfer,
    responseDeserialize: deserialize_m10_sdk_transaction_FinalizedTransfer,
  },
  listTransfers: {
    path: '/m10.sdk.M10QueryService/ListTransfers',
    requestStream: false,
    responseStream: false,
    requestType: sdk_api_pb.RequestEnvelope,
    responseType: sdk_transaction_transaction_pb.FinalizedTransfers,
    requestSerialize: serialize_m10_sdk_RequestEnvelope,
    requestDeserialize: deserialize_m10_sdk_RequestEnvelope,
    responseSerialize: serialize_m10_sdk_transaction_FinalizedTransfers,
    responseDeserialize: deserialize_m10_sdk_transaction_FinalizedTransfers,
  },
  // Request to observe all transfers processed by the M10 ledger
// Param: [`RequestEnvelope`] should contain [`ObserveAccountsRequest`]
observeTransfers: {
    path: '/m10.sdk.M10QueryService/ObserveTransfers',
    requestStream: false,
    responseStream: true,
    requestType: sdk_api_pb.RequestEnvelope,
    responseType: sdk_api_pb.FinalizedTransactions,
    requestSerialize: serialize_m10_sdk_RequestEnvelope,
    requestDeserialize: deserialize_m10_sdk_RequestEnvelope,
    responseSerialize: serialize_m10_sdk_FinalizedTransactions,
    responseDeserialize: deserialize_m10_sdk_FinalizedTransactions,
  },
  // Accounts
getIndexedAccount: {
    path: '/m10.sdk.M10QueryService/GetIndexedAccount',
    requestStream: false,
    responseStream: false,
    requestType: sdk_api_pb.RequestEnvelope,
    responseType: sdk_transaction_transaction_pb.IndexedAccount,
    requestSerialize: serialize_m10_sdk_RequestEnvelope,
    requestDeserialize: deserialize_m10_sdk_RequestEnvelope,
    responseSerialize: serialize_m10_sdk_transaction_IndexedAccount,
    responseDeserialize: deserialize_m10_sdk_transaction_IndexedAccount,
  },
  getAccount: {
    path: '/m10.sdk.M10QueryService/GetAccount',
    requestStream: false,
    responseStream: false,
    requestType: sdk_api_pb.RequestEnvelope,
    responseType: sdk_model_model_pb.Account,
    requestSerialize: serialize_m10_sdk_RequestEnvelope,
    requestDeserialize: deserialize_m10_sdk_RequestEnvelope,
    responseSerialize: serialize_m10_sdk_model_Account,
    responseDeserialize: deserialize_m10_sdk_model_Account,
  },
  getAccountInfo: {
    path: '/m10.sdk.M10QueryService/GetAccountInfo',
    requestStream: false,
    responseStream: false,
    requestType: sdk_api_pb.RequestEnvelope,
    responseType: sdk_model_model_pb.AccountInfo,
    requestSerialize: serialize_m10_sdk_RequestEnvelope,
    requestDeserialize: deserialize_m10_sdk_RequestEnvelope,
    responseSerialize: serialize_m10_sdk_model_AccountInfo,
    responseDeserialize: deserialize_m10_sdk_model_AccountInfo,
  },
  listAccounts: {
    path: '/m10.sdk.M10QueryService/ListAccounts',
    requestStream: false,
    responseStream: false,
    requestType: sdk_api_pb.RequestEnvelope,
    responseType: sdk_api_pb.ListAccountsResponse,
    requestSerialize: serialize_m10_sdk_RequestEnvelope,
    requestDeserialize: deserialize_m10_sdk_RequestEnvelope,
    responseSerialize: serialize_m10_sdk_ListAccountsResponse,
    responseDeserialize: deserialize_m10_sdk_ListAccountsResponse,
  },
  // Request to observe all account changes processed by the M10 ledger
// Param: [`RequestEnvelope`] should contain [`ObserveAccountsRequest`]
observeAccounts: {
    path: '/m10.sdk.M10QueryService/ObserveAccounts',
    requestStream: false,
    responseStream: true,
    requestType: sdk_api_pb.RequestEnvelope,
    responseType: sdk_api_pb.FinalizedTransactions,
    requestSerialize: serialize_m10_sdk_RequestEnvelope,
    requestDeserialize: deserialize_m10_sdk_RequestEnvelope,
    responseSerialize: serialize_m10_sdk_FinalizedTransactions,
    responseDeserialize: deserialize_m10_sdk_FinalizedTransactions,
  },
  // Actions
getAction: {
    path: '/m10.sdk.M10QueryService/GetAction',
    requestStream: false,
    responseStream: false,
    requestType: sdk_api_pb.RequestEnvelope,
    responseType: sdk_transaction_transaction_pb.Action,
    requestSerialize: serialize_m10_sdk_RequestEnvelope,
    requestDeserialize: deserialize_m10_sdk_RequestEnvelope,
    responseSerialize: serialize_m10_sdk_transaction_Action,
    responseDeserialize: deserialize_m10_sdk_transaction_Action,
  },
  listActions: {
    path: '/m10.sdk.M10QueryService/ListActions',
    requestStream: false,
    responseStream: false,
    requestType: sdk_api_pb.RequestEnvelope,
    responseType: sdk_transaction_transaction_pb.Actions,
    requestSerialize: serialize_m10_sdk_RequestEnvelope,
    requestDeserialize: deserialize_m10_sdk_RequestEnvelope,
    responseSerialize: serialize_m10_sdk_transaction_Actions,
    responseDeserialize: deserialize_m10_sdk_transaction_Actions,
  },
  // Request to observe all actions processed by the M10 ledger
// Param: [`RequestEnvelope`] should contain [`ObserveActionsRequest`]
observeActions: {
    path: '/m10.sdk.M10QueryService/ObserveActions',
    requestStream: false,
    responseStream: true,
    requestType: sdk_api_pb.RequestEnvelope,
    responseType: sdk_api_pb.FinalizedTransactions,
    requestSerialize: serialize_m10_sdk_RequestEnvelope,
    requestDeserialize: deserialize_m10_sdk_RequestEnvelope,
    responseSerialize: serialize_m10_sdk_FinalizedTransactions,
    responseDeserialize: deserialize_m10_sdk_FinalizedTransactions,
  },
  // ChainInfo
getChainInfo: {
    path: '/m10.sdk.M10QueryService/GetChainInfo',
    requestStream: false,
    responseStream: false,
    requestType: google_protobuf_empty_pb.Empty,
    responseType: sdk_api_pb.ChainInfo,
    requestSerialize: serialize_google_protobuf_Empty,
    requestDeserialize: deserialize_google_protobuf_Empty,
    responseSerialize: serialize_m10_sdk_ChainInfo,
    responseDeserialize: deserialize_m10_sdk_ChainInfo,
  },
  // Transactions
// Request a specific transaction by ID
// Param: [`RequestEnvelope`] should contain [`GetTransactionsRequest`]
getTransaction: {
    path: '/m10.sdk.M10QueryService/GetTransaction',
    requestStream: false,
    responseStream: false,
    requestType: sdk_api_pb.RequestEnvelope,
    responseType: sdk_api_pb.FinalizedTransaction,
    requestSerialize: serialize_m10_sdk_RequestEnvelope,
    requestDeserialize: deserialize_m10_sdk_RequestEnvelope,
    responseSerialize: serialize_m10_sdk_FinalizedTransaction,
    responseDeserialize: deserialize_m10_sdk_FinalizedTransaction,
  },
  // Request to list all transactions by context ID
// Param: [`RequestEnvelope`] should contain [`ListTransactionsRequest`]
listTransactions: {
    path: '/m10.sdk.M10QueryService/ListTransactions',
    requestStream: false,
    responseStream: false,
    requestType: sdk_api_pb.RequestEnvelope,
    responseType: sdk_api_pb.FinalizedTransactions,
    requestSerialize: serialize_m10_sdk_RequestEnvelope,
    requestDeserialize: deserialize_m10_sdk_RequestEnvelope,
    responseSerialize: serialize_m10_sdk_FinalizedTransactions,
    responseDeserialize: deserialize_m10_sdk_FinalizedTransactions,
  },
  // Request to group all transactions related to a specific account by context
// ID Param: [`RequestEnvelope`] should contain [`GroupTransactionsRequest`]
groupTransactions: {
    path: '/m10.sdk.M10QueryService/GroupTransactions',
    requestStream: false,
    responseStream: false,
    requestType: sdk_api_pb.RequestEnvelope,
    responseType: sdk_api_pb.GroupedFinalizedTransactions,
    requestSerialize: serialize_m10_sdk_RequestEnvelope,
    requestDeserialize: deserialize_m10_sdk_RequestEnvelope,
    responseSerialize: serialize_m10_sdk_GroupedFinalizedTransactions,
    responseDeserialize: deserialize_m10_sdk_GroupedFinalizedTransactions,
  },
  // AccountSet
getAccountSet: {
    path: '/m10.sdk.M10QueryService/GetAccountSet',
    requestStream: false,
    responseStream: false,
    requestType: sdk_api_pb.RequestEnvelope,
    responseType: sdk_model_model_pb.AccountSet,
    requestSerialize: serialize_m10_sdk_RequestEnvelope,
    requestDeserialize: deserialize_m10_sdk_RequestEnvelope,
    responseSerialize: serialize_m10_sdk_model_AccountSet,
    responseDeserialize: deserialize_m10_sdk_model_AccountSet,
  },
  listAccountSets: {
    path: '/m10.sdk.M10QueryService/ListAccountSets',
    requestStream: false,
    responseStream: false,
    requestType: sdk_api_pb.RequestEnvelope,
    responseType: sdk_api_pb.ListAccountSetsResponse,
    requestSerialize: serialize_m10_sdk_RequestEnvelope,
    requestDeserialize: deserialize_m10_sdk_RequestEnvelope,
    responseSerialize: serialize_m10_sdk_ListAccountSetsResponse,
    responseDeserialize: deserialize_m10_sdk_ListAccountSetsResponse,
  },
  // RoleBinding
getRoleBinding: {
    path: '/m10.sdk.M10QueryService/GetRoleBinding',
    requestStream: false,
    responseStream: false,
    requestType: sdk_api_pb.RequestEnvelope,
    responseType: sdk_rbac_pb.RoleBinding,
    requestSerialize: serialize_m10_sdk_RequestEnvelope,
    requestDeserialize: deserialize_m10_sdk_RequestEnvelope,
    responseSerialize: serialize_m10_sdk_RoleBinding,
    responseDeserialize: deserialize_m10_sdk_RoleBinding,
  },
  listRoleBindings: {
    path: '/m10.sdk.M10QueryService/ListRoleBindings',
    requestStream: false,
    responseStream: false,
    requestType: sdk_api_pb.RequestEnvelope,
    responseType: sdk_api_pb.ListRoleBindingsResponse,
    requestSerialize: serialize_m10_sdk_RequestEnvelope,
    requestDeserialize: deserialize_m10_sdk_RequestEnvelope,
    responseSerialize: serialize_m10_sdk_ListRoleBindingsResponse,
    responseDeserialize: deserialize_m10_sdk_ListRoleBindingsResponse,
  },
  // RoleBinding
getRole: {
    path: '/m10.sdk.M10QueryService/GetRole',
    requestStream: false,
    responseStream: false,
    requestType: sdk_api_pb.RequestEnvelope,
    responseType: sdk_rbac_pb.Role,
    requestSerialize: serialize_m10_sdk_RequestEnvelope,
    requestDeserialize: deserialize_m10_sdk_RequestEnvelope,
    responseSerialize: serialize_m10_sdk_Role,
    responseDeserialize: deserialize_m10_sdk_Role,
  },
  listRoles: {
    path: '/m10.sdk.M10QueryService/ListRoles',
    requestStream: false,
    responseStream: false,
    requestType: sdk_api_pb.RequestEnvelope,
    responseType: sdk_api_pb.ListRolesResponse,
    requestSerialize: serialize_m10_sdk_RequestEnvelope,
    requestDeserialize: deserialize_m10_sdk_RequestEnvelope,
    responseSerialize: serialize_m10_sdk_ListRolesResponse,
    responseDeserialize: deserialize_m10_sdk_ListRolesResponse,
  },
  // Resources
// Request to observe all resources processed by the M10 ledger
// Param: [`RequestEnvelope`] should contain [`ObserveResourcesRequest`]
observeResources: {
    path: '/m10.sdk.M10QueryService/ObserveResources',
    requestStream: false,
    responseStream: true,
    requestType: sdk_api_pb.RequestEnvelope,
    responseType: sdk_api_pb.FinalizedTransactions,
    requestSerialize: serialize_m10_sdk_RequestEnvelope,
    requestDeserialize: deserialize_m10_sdk_RequestEnvelope,
    responseSerialize: serialize_m10_sdk_FinalizedTransactions,
    responseDeserialize: deserialize_m10_sdk_FinalizedTransactions,
  },
};

exports.M10QueryServiceClient = grpc.makeGenericClientConstructor(M10QueryServiceService);
