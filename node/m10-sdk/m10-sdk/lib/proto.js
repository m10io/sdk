const services = require('./generated/sdk/api_grpc_pb')

const serviceRequests = require('./generated/sdk/api_pb')
const model = require('./generated/sdk/model/model_pb')
const transaction = require('./generated/sdk/transaction/transaction_pb')
const metadata = require('./generated/sdk/metadata_pb')

exports.QueryServiceClient = services.M10QueryServiceClient
exports.TxServiceClient = services.M10TxServiceClient

exports.Queries = {
  GetAccountRequest: transaction.GetAccountRequest,
  ListAccountsRequest: serviceRequests.ListAccountsRequest,
  GetRequestRequest: serviceRequests.GetRequestRequest,
  GetAccountSetRequest: serviceRequests.GetAccountSetRequest,
  ListAccountSetsRequest: serviceRequests.ListAccountSetsRequest,
  ListRoleBindingsRequest: serviceRequests.ListRoleBindingsRequest,
  ListTransactionsRequest: serviceRequests.ListTransactionsRequest,
  GroupTransactionsRequest: serviceRequests.GroupTransactionsRequest,
}

exports.Observations = {
  Actions: serviceRequests.ObserveActionsRequest,
  Resources: serviceRequests.ObserveResourcesRequest,
  Transfers: serviceRequests.ObserveAccountsRequest,
}

exports.metadata = metadata
exports.sdk = serviceRequests
exports.RequestEnvelope = serviceRequests.RequestEnvelope
exports.Transaction = transaction
exports.Model = model
exports.Empty = require('google-protobuf/google/protobuf/empty_pb.js').Empty
exports.Any = require('google-protobuf/google/protobuf/any_pb.js').Any

exports.ops = require('./generated/sdk/document_pb')
exports.rbac = require('./generated/sdk/rbac_pb')
