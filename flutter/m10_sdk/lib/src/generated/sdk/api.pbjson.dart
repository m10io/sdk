//
//  Generated code. Do not modify.
//  source: sdk/api.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use bulkTransactionsDescriptor instead')
const BulkTransactions$json = {
  '1': 'BulkTransactions',
  '2': [
    {'1': 'transactions', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.RequestEnvelope', '10': 'transactions'},
  ],
};

/// Descriptor for `BulkTransactions`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bulkTransactionsDescriptor = $convert.base64Decode(
    'ChBCdWxrVHJhbnNhY3Rpb25zEjwKDHRyYW5zYWN0aW9ucxgBIAMoCzIYLm0xMC5zZGsuUmVxdW'
    'VzdEVudmVsb3BlUgx0cmFuc2FjdGlvbnM=');

@$core.Deprecated('Use bulkTransactionsResponseDescriptor instead')
const BulkTransactionsResponse$json = {
  '1': 'BulkTransactionsResponse',
  '2': [
    {'1': 'responses', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.transaction.TransactionResponse', '10': 'responses'},
  ],
};

/// Descriptor for `BulkTransactionsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List bulkTransactionsResponseDescriptor = $convert.base64Decode(
    'ChhCdWxrVHJhbnNhY3Rpb25zUmVzcG9uc2USRgoJcmVzcG9uc2VzGAEgAygLMigubTEwLnNkay'
    '50cmFuc2FjdGlvbi5UcmFuc2FjdGlvblJlc3BvbnNlUglyZXNwb25zZXM=');

@$core.Deprecated('Use requestEnvelopeDescriptor instead')
const RequestEnvelope$json = {
  '1': 'RequestEnvelope',
  '2': [
    {'1': 'payload', '3': 2, '4': 1, '5': 12, '10': 'payload'},
    {'1': 'signature', '3': 3, '4': 1, '5': 11, '6': '.m10.sdk.transaction.Signature', '10': 'signature'},
  ],
};

/// Descriptor for `RequestEnvelope`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List requestEnvelopeDescriptor = $convert.base64Decode(
    'Cg9SZXF1ZXN0RW52ZWxvcGUSGAoHcGF5bG9hZBgCIAEoDFIHcGF5bG9hZBI8CglzaWduYXR1cm'
    'UYAyABKAsyHi5tMTAuc2RrLnRyYW5zYWN0aW9uLlNpZ25hdHVyZVIJc2lnbmF0dXJl');

@$core.Deprecated('Use pageDescriptor instead')
const Page$json = {
  '1': 'Page',
  '2': [
    {'1': 'limit', '3': 1, '4': 1, '5': 13, '10': 'limit'},
    {'1': 'last_id', '3': 2, '4': 1, '5': 12, '10': 'lastId'},
  ],
};

/// Descriptor for `Page`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List pageDescriptor = $convert.base64Decode(
    'CgRQYWdlEhQKBWxpbWl0GAEgASgNUgVsaW1pdBIXCgdsYXN0X2lkGAIgASgMUgZsYXN0SWQ=');

@$core.Deprecated('Use getAccountSetRequestDescriptor instead')
const GetAccountSetRequest$json = {
  '1': 'GetAccountSetRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
  ],
};

/// Descriptor for `GetAccountSetRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getAccountSetRequestDescriptor = $convert.base64Decode(
    'ChRHZXRBY2NvdW50U2V0UmVxdWVzdBIOCgJpZBgBIAEoDFICaWQ=');

@$core.Deprecated('Use listAccountSetsRequestDescriptor instead')
const ListAccountSetsRequest$json = {
  '1': 'ListAccountSetsRequest',
  '2': [
    {'1': 'owner', '3': 1, '4': 1, '5': 12, '9': 0, '10': 'owner'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'name'},
    {'1': 'page', '3': 4, '4': 1, '5': 11, '6': '.m10.sdk.Page', '10': 'page'},
  ],
  '8': [
    {'1': 'filter'},
  ],
};

/// Descriptor for `ListAccountSetsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listAccountSetsRequestDescriptor = $convert.base64Decode(
    'ChZMaXN0QWNjb3VudFNldHNSZXF1ZXN0EhYKBW93bmVyGAEgASgMSABSBW93bmVyEhQKBG5hbW'
    'UYAiABKAlIAFIEbmFtZRIhCgRwYWdlGAQgASgLMg0ubTEwLnNkay5QYWdlUgRwYWdlQggKBmZp'
    'bHRlcg==');

@$core.Deprecated('Use listAccountSetsResponseDescriptor instead')
const ListAccountSetsResponse$json = {
  '1': 'ListAccountSetsResponse',
  '2': [
    {'1': 'account_sets', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.model.AccountSet', '10': 'accountSets'},
    {'1': 'next_request', '3': 2, '4': 1, '5': 11, '6': '.m10.sdk.ListAccountSetsRequest', '10': 'nextRequest'},
  ],
};

/// Descriptor for `ListAccountSetsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listAccountSetsResponseDescriptor = $convert.base64Decode(
    'ChdMaXN0QWNjb3VudFNldHNSZXNwb25zZRI8CgxhY2NvdW50X3NldHMYASADKAsyGS5tMTAuc2'
    'RrLm1vZGVsLkFjY291bnRTZXRSC2FjY291bnRTZXRzEkIKDG5leHRfcmVxdWVzdBgCIAEoCzIf'
    'Lm0xMC5zZGsuTGlzdEFjY291bnRTZXRzUmVxdWVzdFILbmV4dFJlcXVlc3Q=');

@$core.Deprecated('Use listAccountMetadataRequestDescriptor instead')
const ListAccountMetadataRequest$json = {
  '1': 'ListAccountMetadataRequest',
  '2': [
    {'1': 'owner', '3': 1, '4': 1, '5': 12, '9': 0, '10': 'owner'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'name'},
    {'1': 'page', '3': 4, '4': 1, '5': 11, '6': '.m10.sdk.Page', '10': 'page'},
  ],
  '8': [
    {'1': 'filter'},
  ],
};

/// Descriptor for `ListAccountMetadataRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listAccountMetadataRequestDescriptor = $convert.base64Decode(
    'ChpMaXN0QWNjb3VudE1ldGFkYXRhUmVxdWVzdBIWCgVvd25lchgBIAEoDEgAUgVvd25lchIUCg'
    'RuYW1lGAIgASgJSABSBG5hbWUSIQoEcGFnZRgEIAEoCzINLm0xMC5zZGsuUGFnZVIEcGFnZUII'
    'CgZmaWx0ZXI=');

@$core.Deprecated('Use listAccountMetadataResponseDescriptor instead')
const ListAccountMetadataResponse$json = {
  '1': 'ListAccountMetadataResponse',
  '2': [
    {'1': 'accounts', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.model.AccountMetadata', '10': 'accounts'},
    {'1': 'next_request', '3': 2, '4': 1, '5': 11, '6': '.m10.sdk.ListAccountMetadataRequest', '10': 'nextRequest'},
  ],
};

/// Descriptor for `ListAccountMetadataResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listAccountMetadataResponseDescriptor = $convert.base64Decode(
    'ChtMaXN0QWNjb3VudE1ldGFkYXRhUmVzcG9uc2USOgoIYWNjb3VudHMYASADKAsyHi5tMTAuc2'
    'RrLm1vZGVsLkFjY291bnRNZXRhZGF0YVIIYWNjb3VudHMSRgoMbmV4dF9yZXF1ZXN0GAIgASgL'
    'MiMubTEwLnNkay5MaXN0QWNjb3VudE1ldGFkYXRhUmVxdWVzdFILbmV4dFJlcXVlc3Q=');

@$core.Deprecated('Use getRoleBindingRequestDescriptor instead')
const GetRoleBindingRequest$json = {
  '1': 'GetRoleBindingRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
  ],
};

/// Descriptor for `GetRoleBindingRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getRoleBindingRequestDescriptor = $convert.base64Decode(
    'ChVHZXRSb2xlQmluZGluZ1JlcXVlc3QSDgoCaWQYASABKAxSAmlk');

@$core.Deprecated('Use listRoleBindingsRequestDescriptor instead')
const ListRoleBindingsRequest$json = {
  '1': 'ListRoleBindingsRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'name'},
    {'1': 'page', '3': 4, '4': 1, '5': 11, '6': '.m10.sdk.Page', '10': 'page'},
  ],
  '8': [
    {'1': 'filter'},
  ],
};

/// Descriptor for `ListRoleBindingsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listRoleBindingsRequestDescriptor = $convert.base64Decode(
    'ChdMaXN0Um9sZUJpbmRpbmdzUmVxdWVzdBIUCgRuYW1lGAEgASgJSABSBG5hbWUSIQoEcGFnZR'
    'gEIAEoCzINLm0xMC5zZGsuUGFnZVIEcGFnZUIICgZmaWx0ZXI=');

@$core.Deprecated('Use listRoleBindingsResponseDescriptor instead')
const ListRoleBindingsResponse$json = {
  '1': 'ListRoleBindingsResponse',
  '2': [
    {'1': 'role_bindings', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.RoleBinding', '10': 'roleBindings'},
    {'1': 'next_request', '3': 2, '4': 1, '5': 11, '6': '.m10.sdk.ListRoleBindingsRequest', '10': 'nextRequest'},
  ],
};

/// Descriptor for `ListRoleBindingsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listRoleBindingsResponseDescriptor = $convert.base64Decode(
    'ChhMaXN0Um9sZUJpbmRpbmdzUmVzcG9uc2USOQoNcm9sZV9iaW5kaW5ncxgBIAMoCzIULm0xMC'
    '5zZGsuUm9sZUJpbmRpbmdSDHJvbGVCaW5kaW5ncxJDCgxuZXh0X3JlcXVlc3QYAiABKAsyIC5t'
    'MTAuc2RrLkxpc3RSb2xlQmluZGluZ3NSZXF1ZXN0UgtuZXh0UmVxdWVzdA==');

@$core.Deprecated('Use getRoleRequestDescriptor instead')
const GetRoleRequest$json = {
  '1': 'GetRoleRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
  ],
};

/// Descriptor for `GetRoleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getRoleRequestDescriptor = $convert.base64Decode(
    'Cg5HZXRSb2xlUmVxdWVzdBIOCgJpZBgBIAEoDFICaWQ=');

@$core.Deprecated('Use listRolesRequestDescriptor instead')
const ListRolesRequest$json = {
  '1': 'ListRolesRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'name'},
    {'1': 'page', '3': 4, '4': 1, '5': 11, '6': '.m10.sdk.Page', '10': 'page'},
  ],
  '8': [
    {'1': 'filter'},
  ],
};

/// Descriptor for `ListRolesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listRolesRequestDescriptor = $convert.base64Decode(
    'ChBMaXN0Um9sZXNSZXF1ZXN0EhQKBG5hbWUYASABKAlIAFIEbmFtZRIhCgRwYWdlGAQgASgLMg'
    '0ubTEwLnNkay5QYWdlUgRwYWdlQggKBmZpbHRlcg==');

@$core.Deprecated('Use listRolesResponseDescriptor instead')
const ListRolesResponse$json = {
  '1': 'ListRolesResponse',
  '2': [
    {'1': 'roles', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.Role', '10': 'roles'},
    {'1': 'next_request', '3': 2, '4': 1, '5': 11, '6': '.m10.sdk.ListRolesRequest', '10': 'nextRequest'},
  ],
};

/// Descriptor for `ListRolesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listRolesResponseDescriptor = $convert.base64Decode(
    'ChFMaXN0Um9sZXNSZXNwb25zZRIjCgVyb2xlcxgBIAMoCzINLm0xMC5zZGsuUm9sZVIFcm9sZX'
    'MSPAoMbmV4dF9yZXF1ZXN0GAIgASgLMhkubTEwLnNkay5MaXN0Um9sZXNSZXF1ZXN0UgtuZXh0'
    'UmVxdWVzdA==');

@$core.Deprecated('Use getTransactionRequestDescriptor instead')
const GetTransactionRequest$json = {
  '1': 'GetTransactionRequest',
  '2': [
    {'1': 'tx_id', '3': 1, '4': 1, '5': 4, '10': 'txId'},
  ],
};

/// Descriptor for `GetTransactionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getTransactionRequestDescriptor = $convert.base64Decode(
    'ChVHZXRUcmFuc2FjdGlvblJlcXVlc3QSEwoFdHhfaWQYASABKARSBHR4SWQ=');

@$core.Deprecated('Use listTransactionsRequestDescriptor instead')
const ListTransactionsRequest$json = {
  '1': 'ListTransactionsRequest',
  '2': [
    {'1': 'context_id', '3': 1, '4': 1, '5': 12, '10': 'contextId'},
    {'1': 'limit', '3': 3, '4': 1, '5': 4, '10': 'limit'},
    {'1': 'min_tx_id', '3': 4, '4': 1, '5': 4, '10': 'minTxId'},
    {'1': 'max_tx_id', '3': 5, '4': 1, '5': 4, '10': 'maxTxId'},
  ],
};

/// Descriptor for `ListTransactionsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listTransactionsRequestDescriptor = $convert.base64Decode(
    'ChdMaXN0VHJhbnNhY3Rpb25zUmVxdWVzdBIdCgpjb250ZXh0X2lkGAEgASgMUgljb250ZXh0SW'
    'QSFAoFbGltaXQYAyABKARSBWxpbWl0EhoKCW1pbl90eF9pZBgEIAEoBFIHbWluVHhJZBIaCglt'
    'YXhfdHhfaWQYBSABKARSB21heFR4SWQ=');

@$core.Deprecated('Use groupTransactionsRequestDescriptor instead')
const GroupTransactionsRequest$json = {
  '1': 'GroupTransactionsRequest',
  '2': [
    {'1': 'account_id', '3': 1, '4': 1, '5': 12, '10': 'accountId'},
    {'1': 'limit_groups', '3': 2, '4': 1, '5': 4, '10': 'limitGroups'},
    {'1': 'min_tx_id', '3': 3, '4': 1, '5': 4, '10': 'minTxId'},
    {'1': 'max_tx_id', '3': 4, '4': 1, '5': 4, '10': 'maxTxId'},
  ],
};

/// Descriptor for `GroupTransactionsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupTransactionsRequestDescriptor = $convert.base64Decode(
    'ChhHcm91cFRyYW5zYWN0aW9uc1JlcXVlc3QSHQoKYWNjb3VudF9pZBgBIAEoDFIJYWNjb3VudE'
    'lkEiEKDGxpbWl0X2dyb3VwcxgCIAEoBFILbGltaXRHcm91cHMSGgoJbWluX3R4X2lkGAMgASgE'
    'UgdtaW5UeElkEhoKCW1heF90eF9pZBgEIAEoBFIHbWF4VHhJZA==');

@$core.Deprecated('Use observeAccountsRequestDescriptor instead')
const ObserveAccountsRequest$json = {
  '1': 'ObserveAccountsRequest',
  '2': [
    {'1': 'starting_from', '3': 1, '4': 1, '5': 11, '6': '.m10.sdk.TxId', '10': 'startingFrom'},
    {'1': 'involved_accounts', '3': 2, '4': 3, '5': 12, '10': 'involvedAccounts'},
  ],
};

/// Descriptor for `ObserveAccountsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List observeAccountsRequestDescriptor = $convert.base64Decode(
    'ChZPYnNlcnZlQWNjb3VudHNSZXF1ZXN0EjIKDXN0YXJ0aW5nX2Zyb20YASABKAsyDS5tMTAuc2'
    'RrLlR4SWRSDHN0YXJ0aW5nRnJvbRIrChFpbnZvbHZlZF9hY2NvdW50cxgCIAMoDFIQaW52b2x2'
    'ZWRBY2NvdW50cw==');

@$core.Deprecated('Use observeResourcesRequestDescriptor instead')
const ObserveResourcesRequest$json = {
  '1': 'ObserveResourcesRequest',
  '2': [
    {'1': 'expression', '3': 1, '4': 1, '5': 11, '6': '.m10.sdk.Exp', '10': 'expression'},
    {'1': 'collection', '3': 2, '4': 1, '5': 9, '10': 'collection'},
    {'1': 'starting_from', '3': 3, '4': 1, '5': 11, '6': '.m10.sdk.TxId', '10': 'startingFrom'},
  ],
};

/// Descriptor for `ObserveResourcesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List observeResourcesRequestDescriptor = $convert.base64Decode(
    'ChdPYnNlcnZlUmVzb3VyY2VzUmVxdWVzdBIsCgpleHByZXNzaW9uGAEgASgLMgwubTEwLnNkay'
    '5FeHBSCmV4cHJlc3Npb24SHgoKY29sbGVjdGlvbhgCIAEoCVIKY29sbGVjdGlvbhIyCg1zdGFy'
    'dGluZ19mcm9tGAMgASgLMg0ubTEwLnNkay5UeElkUgxzdGFydGluZ0Zyb20=');

@$core.Deprecated('Use txIdDescriptor instead')
const TxId$json = {
  '1': 'TxId',
  '2': [
    {'1': 'tx_id', '3': 1, '4': 1, '5': 4, '10': 'txId'},
  ],
};

/// Descriptor for `TxId`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List txIdDescriptor = $convert.base64Decode(
    'CgRUeElkEhMKBXR4X2lkGAEgASgEUgR0eElk');

@$core.Deprecated('Use observeActionsRequestDescriptor instead')
const ObserveActionsRequest$json = {
  '1': 'ObserveActionsRequest',
  '2': [
    {'1': 'starting_from', '3': 1, '4': 1, '5': 11, '6': '.m10.sdk.TxId', '10': 'startingFrom'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {'1': 'involves_accounts', '3': 3, '4': 3, '5': 12, '10': 'involvesAccounts'},
  ],
};

/// Descriptor for `ObserveActionsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List observeActionsRequestDescriptor = $convert.base64Decode(
    'ChVPYnNlcnZlQWN0aW9uc1JlcXVlc3QSMgoNc3RhcnRpbmdfZnJvbRgBIAEoCzINLm0xMC5zZG'
    'suVHhJZFIMc3RhcnRpbmdGcm9tEhIKBG5hbWUYAiABKAlSBG5hbWUSKwoRaW52b2x2ZXNfYWNj'
    'b3VudHMYAyADKAxSEGludm9sdmVzQWNjb3VudHM=');

@$core.Deprecated('Use finalizedTransactionDescriptor instead')
const FinalizedTransaction$json = {
  '1': 'FinalizedTransaction',
  '2': [
    {'1': 'request', '3': 1, '4': 1, '5': 11, '6': '.m10.sdk.transaction.TransactionRequestPayload', '10': 'request'},
    {'1': 'response', '3': 2, '4': 1, '5': 11, '6': '.m10.sdk.transaction.TransactionResponse', '10': 'response'},
  ],
};

/// Descriptor for `FinalizedTransaction`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List finalizedTransactionDescriptor = $convert.base64Decode(
    'ChRGaW5hbGl6ZWRUcmFuc2FjdGlvbhJICgdyZXF1ZXN0GAEgASgLMi4ubTEwLnNkay50cmFuc2'
    'FjdGlvbi5UcmFuc2FjdGlvblJlcXVlc3RQYXlsb2FkUgdyZXF1ZXN0EkQKCHJlc3BvbnNlGAIg'
    'ASgLMigubTEwLnNkay50cmFuc2FjdGlvbi5UcmFuc2FjdGlvblJlc3BvbnNlUghyZXNwb25zZQ'
    '==');

@$core.Deprecated('Use finalizedTransactionsDescriptor instead')
const FinalizedTransactions$json = {
  '1': 'FinalizedTransactions',
  '2': [
    {'1': 'transactions', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.FinalizedTransaction', '10': 'transactions'},
  ],
};

/// Descriptor for `FinalizedTransactions`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List finalizedTransactionsDescriptor = $convert.base64Decode(
    'ChVGaW5hbGl6ZWRUcmFuc2FjdGlvbnMSQQoMdHJhbnNhY3Rpb25zGAEgAygLMh0ubTEwLnNkay'
    '5GaW5hbGl6ZWRUcmFuc2FjdGlvblIMdHJhbnNhY3Rpb25z');

@$core.Deprecated('Use groupedFinalizedTransactionsDescriptor instead')
const GroupedFinalizedTransactions$json = {
  '1': 'GroupedFinalizedTransactions',
  '2': [
    {'1': 'groups', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.FinalizedTransactions', '10': 'groups'},
  ],
};

/// Descriptor for `GroupedFinalizedTransactions`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupedFinalizedTransactionsDescriptor = $convert.base64Decode(
    'ChxHcm91cGVkRmluYWxpemVkVHJhbnNhY3Rpb25zEjYKBmdyb3VwcxgBIAMoCzIeLm0xMC5zZG'
    'suRmluYWxpemVkVHJhbnNhY3Rpb25zUgZncm91cHM=');

@$core.Deprecated('Use chainInfoDescriptor instead')
const ChainInfo$json = {
  '1': 'ChainInfo',
  '2': [
    {'1': 'block_height', '3': 1, '4': 1, '5': 4, '10': 'blockHeight'},
  ],
};

/// Descriptor for `ChainInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chainInfoDescriptor = $convert.base64Decode(
    'CglDaGFpbkluZm8SIQoMYmxvY2tfaGVpZ2h0GAEgASgEUgtibG9ja0hlaWdodA==');

@$core.Deprecated('Use transactionMetricsDescriptor instead')
const TransactionMetrics$json = {
  '1': 'TransactionMetrics',
  '2': [
    {'1': 'transfer_volume', '3': 1, '4': 1, '5': 4, '10': 'transferVolume'},
    {'1': 'transfer_count', '3': 2, '4': 1, '5': 4, '10': 'transferCount'},
    {'1': 'transfer_errors', '3': 3, '4': 1, '5': 4, '10': 'transferErrors'},
    {'1': 'accounts_created', '3': 4, '4': 1, '5': 4, '10': 'accountsCreated'},
  ],
};

/// Descriptor for `TransactionMetrics`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List transactionMetricsDescriptor = $convert.base64Decode(
    'ChJUcmFuc2FjdGlvbk1ldHJpY3MSJwoPdHJhbnNmZXJfdm9sdW1lGAEgASgEUg50cmFuc2Zlcl'
    'ZvbHVtZRIlCg50cmFuc2Zlcl9jb3VudBgCIAEoBFINdHJhbnNmZXJDb3VudBInCg90cmFuc2Zl'
    'cl9lcnJvcnMYAyABKARSDnRyYW5zZmVyRXJyb3JzEikKEGFjY291bnRzX2NyZWF0ZWQYBCABKA'
    'RSD2FjY291bnRzQ3JlYXRlZA==');

@$core.Deprecated('Use listBanksRequestDescriptor instead')
const ListBanksRequest$json = {
  '1': 'ListBanksRequest',
  '2': [
    {'1': 'page', '3': 1, '4': 1, '5': 11, '6': '.m10.sdk.Page', '10': 'page'},
  ],
};

/// Descriptor for `ListBanksRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listBanksRequestDescriptor = $convert.base64Decode(
    'ChBMaXN0QmFua3NSZXF1ZXN0EiEKBHBhZ2UYASABKAsyDS5tMTAuc2RrLlBhZ2VSBHBhZ2U=');

@$core.Deprecated('Use listBanksResponseDescriptor instead')
const ListBanksResponse$json = {
  '1': 'ListBanksResponse',
  '2': [
    {'1': 'banks', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.model.Bank', '10': 'banks'},
  ],
};

/// Descriptor for `ListBanksResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listBanksResponseDescriptor = $convert.base64Decode(
    'ChFMaXN0QmFua3NSZXNwb25zZRIpCgViYW5rcxgBIAMoCzITLm0xMC5zZGsubW9kZWwuQmFua1'
    'IFYmFua3M=');

@$core.Deprecated('Use getBankRequestDescriptor instead')
const GetBankRequest$json = {
  '1': 'GetBankRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
  ],
};

/// Descriptor for `GetBankRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getBankRequestDescriptor = $convert.base64Decode(
    'Cg5HZXRCYW5rUmVxdWVzdBIOCgJpZBgBIAEoDFICaWQ=');

@$core.Deprecated('Use offlineKeyDescriptor instead')
const OfflineKey$json = {
  '1': 'OfflineKey',
  '2': [
    {'1': 'offline_pk', '3': 1, '4': 1, '5': 12, '10': 'offlinePk'},
  ],
};

/// Descriptor for `OfflineKey`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List offlineKeyDescriptor = $convert.base64Decode(
    'CgpPZmZsaW5lS2V5Eh0KCm9mZmxpbmVfcGsYASABKAxSCW9mZmxpbmVQaw==');

