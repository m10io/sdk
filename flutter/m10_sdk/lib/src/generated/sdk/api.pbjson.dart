///
//  Generated code. Do not modify.
//  source: sdk/api.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use requestEnvelopeDescriptor instead')
const RequestEnvelope$json = const {
  '1': 'RequestEnvelope',
  '2': const [
    const {'1': 'payload', '3': 2, '4': 1, '5': 12, '10': 'payload'},
    const {'1': 'signature', '3': 3, '4': 1, '5': 11, '6': '.m10.sdk.Signature', '10': 'signature'},
  ],
};

/// Descriptor for `RequestEnvelope`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List requestEnvelopeDescriptor = $convert.base64Decode('Cg9SZXF1ZXN0RW52ZWxvcGUSGAoHcGF5bG9hZBgCIAEoDFIHcGF5bG9hZBIwCglzaWduYXR1cmUYAyABKAsyEi5tMTAuc2RrLlNpZ25hdHVyZVIJc2lnbmF0dXJl');
@$core.Deprecated('Use signatureDescriptor instead')
const Signature$json = const {
  '1': 'Signature',
  '2': const [
    const {'1': 'public_key', '3': 2, '4': 1, '5': 12, '10': 'publicKey'},
    const {'1': 'signature', '3': 3, '4': 1, '5': 12, '10': 'signature'},
    const {'1': 'algorithm', '3': 4, '4': 1, '5': 14, '6': '.m10.sdk.Signature.Algorithm', '10': 'algorithm'},
  ],
  '4': const [Signature_Algorithm$json],
};

@$core.Deprecated('Use signatureDescriptor instead')
const Signature_Algorithm$json = const {
  '1': 'Algorithm',
  '2': const [
    const {'1': 'P256_SHA256_ASN1', '2': 0},
    const {'1': 'ED25519', '2': 1},
  ],
};

/// Descriptor for `Signature`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List signatureDescriptor = $convert.base64Decode('CglTaWduYXR1cmUSHQoKcHVibGljX2tleRgCIAEoDFIJcHVibGljS2V5EhwKCXNpZ25hdHVyZRgDIAEoDFIJc2lnbmF0dXJlEjoKCWFsZ29yaXRobRgEIAEoDjIcLm0xMC5zZGsuU2lnbmF0dXJlLkFsZ29yaXRobVIJYWxnb3JpdGhtIi4KCUFsZ29yaXRobRIUChBQMjU2X1NIQTI1Nl9BU04xEAASCwoHRUQyNTUxORAB');
@$core.Deprecated('Use pageDescriptor instead')
const Page$json = const {
  '1': 'Page',
  '2': const [
    const {'1': 'limit', '3': 1, '4': 1, '5': 13, '10': 'limit'},
    const {'1': 'last_id', '3': 2, '4': 1, '5': 12, '10': 'lastId'},
  ],
};

/// Descriptor for `Page`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List pageDescriptor = $convert.base64Decode('CgRQYWdlEhQKBWxpbWl0GAEgASgNUgVsaW1pdBIXCgdsYXN0X2lkGAIgASgMUgZsYXN0SWQ=');
@$core.Deprecated('Use getAccountSetRequestDescriptor instead')
const GetAccountSetRequest$json = const {
  '1': 'GetAccountSetRequest',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
  ],
};

/// Descriptor for `GetAccountSetRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getAccountSetRequestDescriptor = $convert.base64Decode('ChRHZXRBY2NvdW50U2V0UmVxdWVzdBIOCgJpZBgBIAEoDFICaWQ=');
@$core.Deprecated('Use listAccountSetsRequestDescriptor instead')
const ListAccountSetsRequest$json = const {
  '1': 'ListAccountSetsRequest',
  '2': const [
    const {'1': 'owner', '3': 1, '4': 1, '5': 12, '9': 0, '10': 'owner'},
    const {'1': 'name', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'name'},
    const {'1': 'page', '3': 4, '4': 1, '5': 11, '6': '.m10.sdk.Page', '10': 'page'},
  ],
  '8': const [
    const {'1': 'filter'},
  ],
};

/// Descriptor for `ListAccountSetsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listAccountSetsRequestDescriptor = $convert.base64Decode('ChZMaXN0QWNjb3VudFNldHNSZXF1ZXN0EhYKBW93bmVyGAEgASgMSABSBW93bmVyEhQKBG5hbWUYAiABKAlIAFIEbmFtZRIhCgRwYWdlGAQgASgLMg0ubTEwLnNkay5QYWdlUgRwYWdlQggKBmZpbHRlcg==');
@$core.Deprecated('Use listAccountSetsResponseDescriptor instead')
const ListAccountSetsResponse$json = const {
  '1': 'ListAccountSetsResponse',
  '2': const [
    const {'1': 'account_sets', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.model.AccountSet', '10': 'accountSets'},
    const {'1': 'next_request', '3': 2, '4': 1, '5': 11, '6': '.m10.sdk.ListAccountSetsRequest', '10': 'nextRequest'},
  ],
};

/// Descriptor for `ListAccountSetsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listAccountSetsResponseDescriptor = $convert.base64Decode('ChdMaXN0QWNjb3VudFNldHNSZXNwb25zZRI8CgxhY2NvdW50X3NldHMYASADKAsyGS5tMTAuc2RrLm1vZGVsLkFjY291bnRTZXRSC2FjY291bnRTZXRzEkIKDG5leHRfcmVxdWVzdBgCIAEoCzIfLm0xMC5zZGsuTGlzdEFjY291bnRTZXRzUmVxdWVzdFILbmV4dFJlcXVlc3Q=');
@$core.Deprecated('Use listAccountsRequestDescriptor instead')
const ListAccountsRequest$json = const {
  '1': 'ListAccountsRequest',
  '2': const [
    const {'1': 'owner', '3': 1, '4': 1, '5': 12, '9': 0, '10': 'owner'},
    const {'1': 'page', '3': 4, '4': 1, '5': 11, '6': '.m10.sdk.Page', '10': 'page'},
  ],
  '8': const [
    const {'1': 'filter'},
  ],
};

/// Descriptor for `ListAccountsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listAccountsRequestDescriptor = $convert.base64Decode('ChNMaXN0QWNjb3VudHNSZXF1ZXN0EhYKBW93bmVyGAEgASgMSABSBW93bmVyEiEKBHBhZ2UYBCABKAsyDS5tMTAuc2RrLlBhZ2VSBHBhZ2VCCAoGZmlsdGVy');
@$core.Deprecated('Use listAccountsResponseDescriptor instead')
const ListAccountsResponse$json = const {
  '1': 'ListAccountsResponse',
  '2': const [
    const {'1': 'accounts', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.model.Account', '10': 'accounts'},
    const {'1': 'next_request', '3': 2, '4': 1, '5': 11, '6': '.m10.sdk.ListAccountsRequest', '10': 'nextRequest'},
  ],
};

/// Descriptor for `ListAccountsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listAccountsResponseDescriptor = $convert.base64Decode('ChRMaXN0QWNjb3VudHNSZXNwb25zZRIyCghhY2NvdW50cxgBIAMoCzIWLm0xMC5zZGsubW9kZWwuQWNjb3VudFIIYWNjb3VudHMSPwoMbmV4dF9yZXF1ZXN0GAIgASgLMhwubTEwLnNkay5MaXN0QWNjb3VudHNSZXF1ZXN0UgtuZXh0UmVxdWVzdA==');
@$core.Deprecated('Use getRoleBindingRequestDescriptor instead')
const GetRoleBindingRequest$json = const {
  '1': 'GetRoleBindingRequest',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
  ],
};

/// Descriptor for `GetRoleBindingRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getRoleBindingRequestDescriptor = $convert.base64Decode('ChVHZXRSb2xlQmluZGluZ1JlcXVlc3QSDgoCaWQYASABKAxSAmlk');
@$core.Deprecated('Use listRoleBindingsRequestDescriptor instead')
const ListRoleBindingsRequest$json = const {
  '1': 'ListRoleBindingsRequest',
  '2': const [
    const {'1': 'name', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'name'},
    const {'1': 'page', '3': 4, '4': 1, '5': 11, '6': '.m10.sdk.Page', '10': 'page'},
  ],
  '8': const [
    const {'1': 'filter'},
  ],
};

/// Descriptor for `ListRoleBindingsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listRoleBindingsRequestDescriptor = $convert.base64Decode('ChdMaXN0Um9sZUJpbmRpbmdzUmVxdWVzdBIUCgRuYW1lGAEgASgJSABSBG5hbWUSIQoEcGFnZRgEIAEoCzINLm0xMC5zZGsuUGFnZVIEcGFnZUIICgZmaWx0ZXI=');
@$core.Deprecated('Use listRoleBindingsResponseDescriptor instead')
const ListRoleBindingsResponse$json = const {
  '1': 'ListRoleBindingsResponse',
  '2': const [
    const {'1': 'role_bindings', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.RoleBinding', '10': 'roleBindings'},
    const {'1': 'next_request', '3': 2, '4': 1, '5': 11, '6': '.m10.sdk.ListRoleBindingsRequest', '10': 'nextRequest'},
  ],
};

/// Descriptor for `ListRoleBindingsResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listRoleBindingsResponseDescriptor = $convert.base64Decode('ChhMaXN0Um9sZUJpbmRpbmdzUmVzcG9uc2USOQoNcm9sZV9iaW5kaW5ncxgBIAMoCzIULm0xMC5zZGsuUm9sZUJpbmRpbmdSDHJvbGVCaW5kaW5ncxJDCgxuZXh0X3JlcXVlc3QYAiABKAsyIC5tMTAuc2RrLkxpc3RSb2xlQmluZGluZ3NSZXF1ZXN0UgtuZXh0UmVxdWVzdA==');
@$core.Deprecated('Use getRoleRequestDescriptor instead')
const GetRoleRequest$json = const {
  '1': 'GetRoleRequest',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
  ],
};

/// Descriptor for `GetRoleRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getRoleRequestDescriptor = $convert.base64Decode('Cg5HZXRSb2xlUmVxdWVzdBIOCgJpZBgBIAEoDFICaWQ=');
@$core.Deprecated('Use listRolesRequestDescriptor instead')
const ListRolesRequest$json = const {
  '1': 'ListRolesRequest',
  '2': const [
    const {'1': 'name', '3': 1, '4': 1, '5': 9, '9': 0, '10': 'name'},
    const {'1': 'page', '3': 4, '4': 1, '5': 11, '6': '.m10.sdk.Page', '10': 'page'},
  ],
  '8': const [
    const {'1': 'filter'},
  ],
};

/// Descriptor for `ListRolesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listRolesRequestDescriptor = $convert.base64Decode('ChBMaXN0Um9sZXNSZXF1ZXN0EhQKBG5hbWUYASABKAlIAFIEbmFtZRIhCgRwYWdlGAQgASgLMg0ubTEwLnNkay5QYWdlUgRwYWdlQggKBmZpbHRlcg==');
@$core.Deprecated('Use listRolesResponseDescriptor instead')
const ListRolesResponse$json = const {
  '1': 'ListRolesResponse',
  '2': const [
    const {'1': 'roles', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.Role', '10': 'roles'},
    const {'1': 'next_request', '3': 2, '4': 1, '5': 11, '6': '.m10.sdk.ListRolesRequest', '10': 'nextRequest'},
  ],
};

/// Descriptor for `ListRolesResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listRolesResponseDescriptor = $convert.base64Decode('ChFMaXN0Um9sZXNSZXNwb25zZRIjCgVyb2xlcxgBIAMoCzINLm0xMC5zZGsuUm9sZVIFcm9sZXMSPAoMbmV4dF9yZXF1ZXN0GAIgASgLMhkubTEwLnNkay5MaXN0Um9sZXNSZXF1ZXN0UgtuZXh0UmVxdWVzdA==');
@$core.Deprecated('Use getTransactionRequestDescriptor instead')
const GetTransactionRequest$json = const {
  '1': 'GetTransactionRequest',
  '2': const [
    const {'1': 'tx_id', '3': 1, '4': 1, '5': 4, '10': 'txId'},
  ],
};

/// Descriptor for `GetTransactionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getTransactionRequestDescriptor = $convert.base64Decode('ChVHZXRUcmFuc2FjdGlvblJlcXVlc3QSEwoFdHhfaWQYASABKARSBHR4SWQ=');
@$core.Deprecated('Use listTransactionsRequestDescriptor instead')
const ListTransactionsRequest$json = const {
  '1': 'ListTransactionsRequest',
  '2': const [
    const {'1': 'context_id', '3': 1, '4': 1, '5': 12, '10': 'contextId'},
    const {'1': 'limit', '3': 3, '4': 1, '5': 4, '10': 'limit'},
    const {'1': 'min_tx_id', '3': 4, '4': 1, '5': 4, '10': 'minTxId'},
    const {'1': 'max_tx_id', '3': 5, '4': 1, '5': 4, '10': 'maxTxId'},
  ],
};

/// Descriptor for `ListTransactionsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listTransactionsRequestDescriptor = $convert.base64Decode('ChdMaXN0VHJhbnNhY3Rpb25zUmVxdWVzdBIdCgpjb250ZXh0X2lkGAEgASgMUgljb250ZXh0SWQSFAoFbGltaXQYAyABKARSBWxpbWl0EhoKCW1pbl90eF9pZBgEIAEoBFIHbWluVHhJZBIaCgltYXhfdHhfaWQYBSABKARSB21heFR4SWQ=');
@$core.Deprecated('Use groupTransactionsRequestDescriptor instead')
const GroupTransactionsRequest$json = const {
  '1': 'GroupTransactionsRequest',
  '2': const [
    const {'1': 'account_id', '3': 1, '4': 1, '5': 12, '10': 'accountId'},
    const {'1': 'limit_groups', '3': 2, '4': 1, '5': 4, '10': 'limitGroups'},
    const {'1': 'min_tx_id', '3': 3, '4': 1, '5': 4, '10': 'minTxId'},
    const {'1': 'max_tx_id', '3': 4, '4': 1, '5': 4, '10': 'maxTxId'},
  ],
};

/// Descriptor for `GroupTransactionsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupTransactionsRequestDescriptor = $convert.base64Decode('ChhHcm91cFRyYW5zYWN0aW9uc1JlcXVlc3QSHQoKYWNjb3VudF9pZBgBIAEoDFIJYWNjb3VudElkEiEKDGxpbWl0X2dyb3VwcxgCIAEoBFILbGltaXRHcm91cHMSGgoJbWluX3R4X2lkGAMgASgEUgdtaW5UeElkEhoKCW1heF90eF9pZBgEIAEoBFIHbWF4VHhJZA==');
@$core.Deprecated('Use observeAccountsRequestDescriptor instead')
const ObserveAccountsRequest$json = const {
  '1': 'ObserveAccountsRequest',
  '2': const [
    const {'1': 'starting_from', '3': 1, '4': 1, '5': 11, '6': '.m10.sdk.TxId', '10': 'startingFrom'},
    const {'1': 'involved_accounts', '3': 2, '4': 3, '5': 12, '10': 'involvedAccounts'},
  ],
};

/// Descriptor for `ObserveAccountsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List observeAccountsRequestDescriptor = $convert.base64Decode('ChZPYnNlcnZlQWNjb3VudHNSZXF1ZXN0EjIKDXN0YXJ0aW5nX2Zyb20YASABKAsyDS5tMTAuc2RrLlR4SWRSDHN0YXJ0aW5nRnJvbRIrChFpbnZvbHZlZF9hY2NvdW50cxgCIAMoDFIQaW52b2x2ZWRBY2NvdW50cw==');
@$core.Deprecated('Use observeResourcesRequestDescriptor instead')
const ObserveResourcesRequest$json = const {
  '1': 'ObserveResourcesRequest',
  '2': const [
    const {'1': 'expression', '3': 1, '4': 1, '5': 11, '6': '.m10.sdk.Exp', '10': 'expression'},
    const {'1': 'collection', '3': 2, '4': 1, '5': 9, '10': 'collection'},
    const {'1': 'starting_from', '3': 3, '4': 1, '5': 11, '6': '.m10.sdk.TxId', '10': 'startingFrom'},
  ],
};

/// Descriptor for `ObserveResourcesRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List observeResourcesRequestDescriptor = $convert.base64Decode('ChdPYnNlcnZlUmVzb3VyY2VzUmVxdWVzdBIsCgpleHByZXNzaW9uGAEgASgLMgwubTEwLnNkay5FeHBSCmV4cHJlc3Npb24SHgoKY29sbGVjdGlvbhgCIAEoCVIKY29sbGVjdGlvbhIyCg1zdGFydGluZ19mcm9tGAMgASgLMg0ubTEwLnNkay5UeElkUgxzdGFydGluZ0Zyb20=');
@$core.Deprecated('Use txIdDescriptor instead')
const TxId$json = const {
  '1': 'TxId',
  '2': const [
    const {'1': 'tx_id', '3': 1, '4': 1, '5': 4, '10': 'txId'},
  ],
};

/// Descriptor for `TxId`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List txIdDescriptor = $convert.base64Decode('CgRUeElkEhMKBXR4X2lkGAEgASgEUgR0eElk');
@$core.Deprecated('Use observeActionsRequestDescriptor instead')
const ObserveActionsRequest$json = const {
  '1': 'ObserveActionsRequest',
  '2': const [
    const {'1': 'starting_from', '3': 1, '4': 1, '5': 11, '6': '.m10.sdk.TxId', '10': 'startingFrom'},
    const {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'involves_accounts', '3': 3, '4': 3, '5': 12, '10': 'involvesAccounts'},
  ],
};

/// Descriptor for `ObserveActionsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List observeActionsRequestDescriptor = $convert.base64Decode('ChVPYnNlcnZlQWN0aW9uc1JlcXVlc3QSMgoNc3RhcnRpbmdfZnJvbRgBIAEoCzINLm0xMC5zZGsuVHhJZFIMc3RhcnRpbmdGcm9tEhIKBG5hbWUYAiABKAlSBG5hbWUSKwoRaW52b2x2ZXNfYWNjb3VudHMYAyADKAxSEGludm9sdmVzQWNjb3VudHM=');
@$core.Deprecated('Use finalizedTransactionDescriptor instead')
const FinalizedTransaction$json = const {
  '1': 'FinalizedTransaction',
  '2': const [
    const {'1': 'request', '3': 1, '4': 1, '5': 11, '6': '.m10.sdk.transaction.TransactionRequestPayload', '10': 'request'},
    const {'1': 'response', '3': 2, '4': 1, '5': 11, '6': '.m10.sdk.transaction.TransactionResponse', '10': 'response'},
  ],
};

/// Descriptor for `FinalizedTransaction`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List finalizedTransactionDescriptor = $convert.base64Decode('ChRGaW5hbGl6ZWRUcmFuc2FjdGlvbhJICgdyZXF1ZXN0GAEgASgLMi4ubTEwLnNkay50cmFuc2FjdGlvbi5UcmFuc2FjdGlvblJlcXVlc3RQYXlsb2FkUgdyZXF1ZXN0EkQKCHJlc3BvbnNlGAIgASgLMigubTEwLnNkay50cmFuc2FjdGlvbi5UcmFuc2FjdGlvblJlc3BvbnNlUghyZXNwb25zZQ==');
@$core.Deprecated('Use finalizedTransactionsDescriptor instead')
const FinalizedTransactions$json = const {
  '1': 'FinalizedTransactions',
  '2': const [
    const {'1': 'transactions', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.FinalizedTransaction', '10': 'transactions'},
  ],
};

/// Descriptor for `FinalizedTransactions`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List finalizedTransactionsDescriptor = $convert.base64Decode('ChVGaW5hbGl6ZWRUcmFuc2FjdGlvbnMSQQoMdHJhbnNhY3Rpb25zGAEgAygLMh0ubTEwLnNkay5GaW5hbGl6ZWRUcmFuc2FjdGlvblIMdHJhbnNhY3Rpb25z');
@$core.Deprecated('Use groupedFinalizedTransactionsDescriptor instead')
const GroupedFinalizedTransactions$json = const {
  '1': 'GroupedFinalizedTransactions',
  '2': const [
    const {'1': 'groups', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.FinalizedTransactions', '10': 'groups'},
  ],
};

/// Descriptor for `GroupedFinalizedTransactions`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List groupedFinalizedTransactionsDescriptor = $convert.base64Decode('ChxHcm91cGVkRmluYWxpemVkVHJhbnNhY3Rpb25zEjYKBmdyb3VwcxgBIAMoCzIeLm0xMC5zZGsuRmluYWxpemVkVHJhbnNhY3Rpb25zUgZncm91cHM=');
@$core.Deprecated('Use chainInfoDescriptor instead')
const ChainInfo$json = const {
  '1': 'ChainInfo',
  '2': const [
    const {'1': 'block_height', '3': 1, '4': 1, '5': 4, '10': 'blockHeight'},
  ],
};

/// Descriptor for `ChainInfo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chainInfoDescriptor = $convert.base64Decode('CglDaGFpbkluZm8SIQoMYmxvY2tfaGVpZ2h0GAEgASgEUgtibG9ja0hlaWdodA==');
@$core.Deprecated('Use transactionMetricsDescriptor instead')
const TransactionMetrics$json = const {
  '1': 'TransactionMetrics',
  '2': const [
    const {'1': 'transfer_volume', '3': 1, '4': 1, '5': 4, '10': 'transferVolume'},
    const {'1': 'transfer_count', '3': 2, '4': 1, '5': 4, '10': 'transferCount'},
    const {'1': 'transfer_errors', '3': 3, '4': 1, '5': 4, '10': 'transferErrors'},
    const {'1': 'accounts_created', '3': 4, '4': 1, '5': 4, '10': 'accountsCreated'},
  ],
};

/// Descriptor for `TransactionMetrics`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List transactionMetricsDescriptor = $convert.base64Decode('ChJUcmFuc2FjdGlvbk1ldHJpY3MSJwoPdHJhbnNmZXJfdm9sdW1lGAEgASgEUg50cmFuc2ZlclZvbHVtZRIlCg50cmFuc2Zlcl9jb3VudBgCIAEoBFINdHJhbnNmZXJDb3VudBInCg90cmFuc2Zlcl9lcnJvcnMYAyABKARSDnRyYW5zZmVyRXJyb3JzEikKEGFjY291bnRzX2NyZWF0ZWQYBCABKARSD2FjY291bnRzQ3JlYXRlZA==');
