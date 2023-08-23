//
//  Generated code. Do not modify.
//  source: sdk/transaction/transaction.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use transactionRequestPayloadDescriptor instead')
const TransactionRequestPayload$json = {
  '1': 'TransactionRequestPayload',
  '2': [
    {'1': 'nonce', '3': 1, '4': 1, '5': 4, '10': 'nonce'},
    {'1': 'timestamp', '3': 2, '4': 1, '5': 4, '10': 'timestamp'},
    {'1': 'context_id', '3': 5, '4': 1, '5': 12, '10': 'contextId'},
    {'1': 'data', '3': 6, '4': 1, '5': 11, '6': '.m10.sdk.transaction.TransactionData', '10': 'data'},
  ],
};

/// Descriptor for `TransactionRequestPayload`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List transactionRequestPayloadDescriptor = $convert.base64Decode(
    'ChlUcmFuc2FjdGlvblJlcXVlc3RQYXlsb2FkEhQKBW5vbmNlGAEgASgEUgVub25jZRIcCgl0aW'
    '1lc3RhbXAYAiABKARSCXRpbWVzdGFtcBIdCgpjb250ZXh0X2lkGAUgASgMUgljb250ZXh0SWQS'
    'OAoEZGF0YRgGIAEoCzIkLm0xMC5zZGsudHJhbnNhY3Rpb24uVHJhbnNhY3Rpb25EYXRhUgRkYX'
    'Rh');

@$core.Deprecated('Use transactionDataDescriptor instead')
const TransactionData$json = {
  '1': 'TransactionData',
  '2': [
    {'1': 'transfer', '3': 10, '4': 1, '5': 11, '6': '.m10.sdk.transaction.CreateTransfer', '9': 0, '10': 'transfer'},
    {'1': 'create_ledger_account', '3': 11, '4': 1, '5': 11, '6': '.m10.sdk.transaction.CreateLedgerAccount', '9': 0, '10': 'createLedgerAccount'},
    {'1': 'set_freeze_state', '3': 12, '4': 1, '5': 11, '6': '.m10.sdk.transaction.SetFreezeState', '9': 0, '10': 'setFreezeState'},
    {'1': 'document_operations', '3': 16, '4': 1, '5': 11, '6': '.m10.sdk.DocumentOperations', '9': 0, '10': 'documentOperations'},
    {'1': 'invoke_action', '3': 20, '4': 1, '5': 11, '6': '.m10.sdk.transaction.InvokeAction', '9': 0, '10': 'invokeAction'},
    {'1': 'initiate_transfer', '3': 21, '4': 1, '5': 11, '6': '.m10.sdk.transaction.CreateTransfer', '9': 0, '10': 'initiateTransfer'},
    {'1': 'commit_transfer', '3': 22, '4': 1, '5': 11, '6': '.m10.sdk.transaction.CommitTransfer', '9': 0, '10': 'commitTransfer'},
    {'1': 'set_instrument', '3': 23, '4': 1, '5': 11, '6': '.m10.sdk.transaction.SetInstrument', '9': 0, '10': 'setInstrument'},
    {'1': 'set_balance_limit', '3': 24, '4': 1, '5': 11, '6': '.m10.sdk.transaction.SetBalanceLimit', '9': 0, '10': 'setBalanceLimit'},
    {'1': 'create_token', '3': 100, '4': 1, '5': 11, '6': '.m10.sdk.transaction.CreateToken', '9': 0, '10': 'createToken'},
    {'1': 'redeem_token', '3': 101, '4': 1, '5': 11, '6': '.m10.sdk.transaction.RedeemToken', '9': 0, '10': 'redeemToken'},
  ],
  '8': [
    {'1': 'data'},
  ],
};

/// Descriptor for `TransactionData`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List transactionDataDescriptor = $convert.base64Decode(
    'Cg9UcmFuc2FjdGlvbkRhdGESQQoIdHJhbnNmZXIYCiABKAsyIy5tMTAuc2RrLnRyYW5zYWN0aW'
    '9uLkNyZWF0ZVRyYW5zZmVySABSCHRyYW5zZmVyEl4KFWNyZWF0ZV9sZWRnZXJfYWNjb3VudBgL'
    'IAEoCzIoLm0xMC5zZGsudHJhbnNhY3Rpb24uQ3JlYXRlTGVkZ2VyQWNjb3VudEgAUhNjcmVhdG'
    'VMZWRnZXJBY2NvdW50Ek8KEHNldF9mcmVlemVfc3RhdGUYDCABKAsyIy5tMTAuc2RrLnRyYW5z'
    'YWN0aW9uLlNldEZyZWV6ZVN0YXRlSABSDnNldEZyZWV6ZVN0YXRlEk4KE2RvY3VtZW50X29wZX'
    'JhdGlvbnMYECABKAsyGy5tMTAuc2RrLkRvY3VtZW50T3BlcmF0aW9uc0gAUhJkb2N1bWVudE9w'
    'ZXJhdGlvbnMSSAoNaW52b2tlX2FjdGlvbhgUIAEoCzIhLm0xMC5zZGsudHJhbnNhY3Rpb24uSW'
    '52b2tlQWN0aW9uSABSDGludm9rZUFjdGlvbhJSChFpbml0aWF0ZV90cmFuc2ZlchgVIAEoCzIj'
    'Lm0xMC5zZGsudHJhbnNhY3Rpb24uQ3JlYXRlVHJhbnNmZXJIAFIQaW5pdGlhdGVUcmFuc2Zlch'
    'JOCg9jb21taXRfdHJhbnNmZXIYFiABKAsyIy5tMTAuc2RrLnRyYW5zYWN0aW9uLkNvbW1pdFRy'
    'YW5zZmVySABSDmNvbW1pdFRyYW5zZmVyEksKDnNldF9pbnN0cnVtZW50GBcgASgLMiIubTEwLn'
    'Nkay50cmFuc2FjdGlvbi5TZXRJbnN0cnVtZW50SABSDXNldEluc3RydW1lbnQSUgoRc2V0X2Jh'
    'bGFuY2VfbGltaXQYGCABKAsyJC5tMTAuc2RrLnRyYW5zYWN0aW9uLlNldEJhbGFuY2VMaW1pdE'
    'gAUg9zZXRCYWxhbmNlTGltaXQSRQoMY3JlYXRlX3Rva2VuGGQgASgLMiAubTEwLnNkay50cmFu'
    'c2FjdGlvbi5DcmVhdGVUb2tlbkgAUgtjcmVhdGVUb2tlbhJFCgxyZWRlZW1fdG9rZW4YZSABKA'
    'syIC5tMTAuc2RrLnRyYW5zYWN0aW9uLlJlZGVlbVRva2VuSABSC3JlZGVlbVRva2VuQgYKBGRh'
    'dGE=');

@$core.Deprecated('Use transactionResponseDescriptor instead')
const TransactionResponse$json = {
  '1': 'TransactionResponse',
  '2': [
    {'1': 'tx_id', '3': 1, '4': 1, '5': 4, '10': 'txId'},
    {'1': 'error', '3': 2, '4': 1, '5': 11, '6': '.m10.sdk.transaction.TransactionError', '10': 'error'},
    {'1': 'timestamp', '3': 4, '4': 1, '5': 6, '10': 'timestamp'},
    {'1': 'account_created', '3': 3, '4': 1, '5': 12, '10': 'accountCreated'},
    {'1': 'transfer_committed', '3': 5, '4': 1, '5': 11, '6': '.m10.sdk.transaction.CreateTransfer', '10': 'transferCommitted'},
    {'1': 'token', '3': 6, '4': 1, '5': 11, '6': '.m10.sdk.transaction.OfflineToken', '10': 'token'},
  ],
};

/// Descriptor for `TransactionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List transactionResponseDescriptor = $convert.base64Decode(
    'ChNUcmFuc2FjdGlvblJlc3BvbnNlEhMKBXR4X2lkGAEgASgEUgR0eElkEjsKBWVycm9yGAIgAS'
    'gLMiUubTEwLnNkay50cmFuc2FjdGlvbi5UcmFuc2FjdGlvbkVycm9yUgVlcnJvchIcCgl0aW1l'
    'c3RhbXAYBCABKAZSCXRpbWVzdGFtcBInCg9hY2NvdW50X2NyZWF0ZWQYAyABKAxSDmFjY291bn'
    'RDcmVhdGVkElIKEnRyYW5zZmVyX2NvbW1pdHRlZBgFIAEoCzIjLm0xMC5zZGsudHJhbnNhY3Rp'
    'b24uQ3JlYXRlVHJhbnNmZXJSEXRyYW5zZmVyQ29tbWl0dGVkEjcKBXRva2VuGAYgASgLMiEubT'
    'EwLnNkay50cmFuc2FjdGlvbi5PZmZsaW5lVG9rZW5SBXRva2Vu');

@$core.Deprecated('Use transactionErrorDescriptor instead')
const TransactionError$json = {
  '1': 'TransactionError',
  '2': [
    {'1': 'code', '3': 1, '4': 1, '5': 14, '6': '.m10.sdk.transaction.TransactionError.Code', '10': 'code'},
    {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
  '4': [TransactionError_Code$json],
};

@$core.Deprecated('Use transactionErrorDescriptor instead')
const TransactionError_Code$json = {
  '1': 'Code',
  '2': [
    {'1': 'UNKNOWN', '2': 0},
    {'1': 'UNIMPLEMENTED', '2': 1},
    {'1': 'NOT_FOUND', '2': 2},
    {'1': 'ALREADY_EXISTS', '2': 3},
    {'1': 'UNAUTHORIZED', '2': 4},
    {'1': 'BAD_REQUEST', '2': 5},
    {'1': 'INVALID_REQUEST_TYPE', '2': 6},
    {'1': 'INVALID_ACCOUNT_ID', '2': 7},
    {'1': 'INVALID_TRANSFER', '2': 8},
    {'1': 'MESSAGE_TOO_LARGE', '2': 10},
    {'1': 'INVALID_SIGNATURE', '2': 11},
    {'1': 'VERIFICATION_FAILED', '2': 12},
    {'1': 'REPLAY_PROTECTION', '2': 20},
    {'1': 'INVALID_EXPRESSION', '2': 21},
    {'1': 'INCORRECT_TYPE', '2': 22},
    {'1': 'ACCOUNT_FROZEN', '2': 23},
    {'1': 'UNMODIFIED_STATE', '2': 24},
    {'1': 'INSUFFICIENT_BALANCE', '2': 25},
    {'1': 'BALANCE_OVERFLOW', '2': 26},
    {'1': 'ACCOUNT_DEPTH_EXCEEDED', '2': 27},
    {'1': 'HOLDING_LIMIT_EXCEEDED', '2': 28},
    {'1': 'INVALID_TARGET', '2': 30},
  ],
};

/// Descriptor for `TransactionError`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List transactionErrorDescriptor = $convert.base64Decode(
    'ChBUcmFuc2FjdGlvbkVycm9yEj4KBGNvZGUYASABKA4yKi5tMTAuc2RrLnRyYW5zYWN0aW9uLl'
    'RyYW5zYWN0aW9uRXJyb3IuQ29kZVIEY29kZRIYCgdtZXNzYWdlGAIgASgJUgdtZXNzYWdlIuQD'
    'CgRDb2RlEgsKB1VOS05PV04QABIRCg1VTklNUExFTUVOVEVEEAESDQoJTk9UX0ZPVU5EEAISEg'
    'oOQUxSRUFEWV9FWElTVFMQAxIQCgxVTkFVVEhPUklaRUQQBBIPCgtCQURfUkVRVUVTVBAFEhgK'
    'FElOVkFMSURfUkVRVUVTVF9UWVBFEAYSFgoSSU5WQUxJRF9BQ0NPVU5UX0lEEAcSFAoQSU5WQU'
    'xJRF9UUkFOU0ZFUhAIEhUKEU1FU1NBR0VfVE9PX0xBUkdFEAoSFQoRSU5WQUxJRF9TSUdOQVRV'
    'UkUQCxIXChNWRVJJRklDQVRJT05fRkFJTEVEEAwSFQoRUkVQTEFZX1BST1RFQ1RJT04QFBIWCh'
    'JJTlZBTElEX0VYUFJFU1NJT04QFRISCg5JTkNPUlJFQ1RfVFlQRRAWEhIKDkFDQ09VTlRfRlJP'
    'WkVOEBcSFAoQVU5NT0RJRklFRF9TVEFURRAYEhgKFElOU1VGRklDSUVOVF9CQUxBTkNFEBkSFA'
    'oQQkFMQU5DRV9PVkVSRkxPVxAaEhoKFkFDQ09VTlRfREVQVEhfRVhDRUVERUQQGxIaChZIT0xE'
    'SU5HX0xJTUlUX0VYQ0VFREVEEBwSEgoOSU5WQUxJRF9UQVJHRVQQHg==');

@$core.Deprecated('Use createLedgerTransferDescriptor instead')
const CreateLedgerTransfer$json = {
  '1': 'CreateLedgerTransfer',
  '2': [
    {'1': 'ledger_id', '3': 1, '4': 1, '5': 9, '10': 'ledgerId'},
    {'1': 'nonce', '3': 2, '4': 1, '5': 4, '10': 'nonce'},
    {'1': 'transfer', '3': 3, '4': 1, '5': 11, '6': '.m10.sdk.transaction.CreateTransfer', '10': 'transfer'},
  ],
};

/// Descriptor for `CreateLedgerTransfer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createLedgerTransferDescriptor = $convert.base64Decode(
    'ChRDcmVhdGVMZWRnZXJUcmFuc2ZlchIbCglsZWRnZXJfaWQYASABKAlSCGxlZGdlcklkEhQKBW'
    '5vbmNlGAIgASgEUgVub25jZRI/Cgh0cmFuc2ZlchgDIAEoCzIjLm0xMC5zZGsudHJhbnNhY3Rp'
    'b24uQ3JlYXRlVHJhbnNmZXJSCHRyYW5zZmVy');

@$core.Deprecated('Use createLedgerTransfersDescriptor instead')
const CreateLedgerTransfers$json = {
  '1': 'CreateLedgerTransfers',
  '2': [
    {'1': 'transfers', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.transaction.CreateLedgerTransfer', '10': 'transfers'},
    {'1': 'valid_until', '3': 2, '4': 1, '5': 4, '10': 'validUntil'},
  ],
};

/// Descriptor for `CreateLedgerTransfers`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createLedgerTransfersDescriptor = $convert.base64Decode(
    'ChVDcmVhdGVMZWRnZXJUcmFuc2ZlcnMSRwoJdHJhbnNmZXJzGAEgAygLMikubTEwLnNkay50cm'
    'Fuc2FjdGlvbi5DcmVhdGVMZWRnZXJUcmFuc2ZlclIJdHJhbnNmZXJzEh8KC3ZhbGlkX3VudGls'
    'GAIgASgEUgp2YWxpZFVudGls');

@$core.Deprecated('Use getTransferRequestDescriptor instead')
const GetTransferRequest$json = {
  '1': 'GetTransferRequest',
  '2': [
    {'1': 'tx_id', '3': 1, '4': 1, '5': 4, '10': 'txId'},
  ],
};

/// Descriptor for `GetTransferRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getTransferRequestDescriptor = $convert.base64Decode(
    'ChJHZXRUcmFuc2ZlclJlcXVlc3QSEwoFdHhfaWQYASABKARSBHR4SWQ=');

@$core.Deprecated('Use listTransferRequestDescriptor instead')
const ListTransferRequest$json = {
  '1': 'ListTransferRequest',
  '2': [
    {'1': 'account_id', '3': 1, '4': 1, '5': 12, '9': 0, '10': 'accountId'},
    {'1': 'context_id', '3': 2, '4': 1, '5': 12, '9': 0, '10': 'contextId'},
    {'1': 'limit', '3': 4, '4': 1, '5': 4, '10': 'limit'},
    {'1': 'include_child_accounts', '3': 5, '4': 1, '5': 8, '10': 'includeChildAccounts'},
    {'1': 'min_tx_id', '3': 6, '4': 1, '5': 4, '10': 'minTxId'},
    {'1': 'max_tx_id', '3': 7, '4': 1, '5': 4, '10': 'maxTxId'},
  ],
  '8': [
    {'1': 'filter'},
  ],
};

/// Descriptor for `ListTransferRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listTransferRequestDescriptor = $convert.base64Decode(
    'ChNMaXN0VHJhbnNmZXJSZXF1ZXN0Eh8KCmFjY291bnRfaWQYASABKAxIAFIJYWNjb3VudElkEh'
    '8KCmNvbnRleHRfaWQYAiABKAxIAFIJY29udGV4dElkEhQKBWxpbWl0GAQgASgEUgVsaW1pdBI0'
    'ChZpbmNsdWRlX2NoaWxkX2FjY291bnRzGAUgASgIUhRpbmNsdWRlQ2hpbGRBY2NvdW50cxIaCg'
    'ltaW5fdHhfaWQYBiABKARSB21pblR4SWQSGgoJbWF4X3R4X2lkGAcgASgEUgdtYXhUeElkQggK'
    'BmZpbHRlcg==');

@$core.Deprecated('Use createTransferDescriptor instead')
const CreateTransfer$json = {
  '1': 'CreateTransfer',
  '2': [
    {'1': 'transfer_steps', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.transaction.TransferStep', '10': 'transferSteps'},
  ],
};

/// Descriptor for `CreateTransfer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createTransferDescriptor = $convert.base64Decode(
    'Cg5DcmVhdGVUcmFuc2ZlchJICg50cmFuc2Zlcl9zdGVwcxgBIAMoCzIhLm0xMC5zZGsudHJhbn'
    'NhY3Rpb24uVHJhbnNmZXJTdGVwUg10cmFuc2ZlclN0ZXBz');

@$core.Deprecated('Use transferStepDescriptor instead')
const TransferStep$json = {
  '1': 'TransferStep',
  '2': [
    {'1': 'from_account_id', '3': 1, '4': 1, '5': 12, '10': 'fromAccountId'},
    {'1': 'to_account_id', '3': 2, '4': 1, '5': 12, '10': 'toAccountId'},
    {'1': 'amount', '3': 4, '4': 1, '5': 4, '10': 'amount'},
    {'1': 'metadata', '3': 7, '4': 3, '5': 11, '6': '.google.protobuf.Any', '10': 'metadata'},
  ],
};

/// Descriptor for `TransferStep`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List transferStepDescriptor = $convert.base64Decode(
    'CgxUcmFuc2ZlclN0ZXASJgoPZnJvbV9hY2NvdW50X2lkGAEgASgMUg1mcm9tQWNjb3VudElkEi'
    'IKDXRvX2FjY291bnRfaWQYAiABKAxSC3RvQWNjb3VudElkEhYKBmFtb3VudBgEIAEoBFIGYW1v'
    'dW50EjAKCG1ldGFkYXRhGAcgAygLMhQuZ29vZ2xlLnByb3RvYnVmLkFueVIIbWV0YWRhdGE=');

@$core.Deprecated('Use finalizedTransferDescriptor instead')
const FinalizedTransfer$json = {
  '1': 'FinalizedTransfer',
  '2': [
    {'1': 'tx_id', '3': 1, '4': 1, '5': 4, '10': 'txId'},
    {'1': 'context_id', '3': 2, '4': 1, '5': 12, '10': 'contextId'},
    {'1': 'transfer_steps', '3': 3, '4': 3, '5': 11, '6': '.m10.sdk.transaction.TransferStep', '10': 'transferSteps'},
    {'1': 'error', '3': 4, '4': 1, '5': 11, '6': '.m10.sdk.transaction.TransactionError', '10': 'error'},
    {'1': 'timestamp', '3': 5, '4': 1, '5': 6, '10': 'timestamp'},
    {'1': 'state', '3': 7, '4': 1, '5': 14, '6': '.m10.sdk.transaction.FinalizedTransfer.TransferState', '10': 'state'},
  ],
  '4': [FinalizedTransfer_TransferState$json],
};

@$core.Deprecated('Use finalizedTransferDescriptor instead')
const FinalizedTransfer_TransferState$json = {
  '1': 'TransferState',
  '2': [
    {'1': 'ACCEPTED', '2': 0},
    {'1': 'REJECTED', '2': 1},
    {'1': 'PENDING', '2': 2},
    {'1': 'EXPIRED', '2': 3},
  ],
};

/// Descriptor for `FinalizedTransfer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List finalizedTransferDescriptor = $convert.base64Decode(
    'ChFGaW5hbGl6ZWRUcmFuc2ZlchITCgV0eF9pZBgBIAEoBFIEdHhJZBIdCgpjb250ZXh0X2lkGA'
    'IgASgMUgljb250ZXh0SWQSSAoOdHJhbnNmZXJfc3RlcHMYAyADKAsyIS5tMTAuc2RrLnRyYW5z'
    'YWN0aW9uLlRyYW5zZmVyU3RlcFINdHJhbnNmZXJTdGVwcxI7CgVlcnJvchgEIAEoCzIlLm0xMC'
    '5zZGsudHJhbnNhY3Rpb24uVHJhbnNhY3Rpb25FcnJvclIFZXJyb3ISHAoJdGltZXN0YW1wGAUg'
    'ASgGUgl0aW1lc3RhbXASSgoFc3RhdGUYByABKA4yNC5tMTAuc2RrLnRyYW5zYWN0aW9uLkZpbm'
    'FsaXplZFRyYW5zZmVyLlRyYW5zZmVyU3RhdGVSBXN0YXRlIkUKDVRyYW5zZmVyU3RhdGUSDAoI'
    'QUNDRVBURUQQABIMCghSRUpFQ1RFRBABEgsKB1BFTkRJTkcQAhILCgdFWFBJUkVEEAM=');

@$core.Deprecated('Use finalizedTransfersDescriptor instead')
const FinalizedTransfers$json = {
  '1': 'FinalizedTransfers',
  '2': [
    {'1': 'transfers', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.transaction.FinalizedTransfer', '10': 'transfers'},
  ],
};

/// Descriptor for `FinalizedTransfers`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List finalizedTransfersDescriptor = $convert.base64Decode(
    'ChJGaW5hbGl6ZWRUcmFuc2ZlcnMSRAoJdHJhbnNmZXJzGAEgAygLMiYubTEwLnNkay50cmFuc2'
    'FjdGlvbi5GaW5hbGl6ZWRUcmFuc2ZlclIJdHJhbnNmZXJz');

@$core.Deprecated('Use instrumentDescriptor instead')
const Instrument$json = {
  '1': 'Instrument',
  '2': [
    {'1': 'code', '3': 1, '4': 1, '5': 9, '10': 'code'},
    {'1': 'decimal_places', '3': 2, '4': 1, '5': 13, '10': 'decimalPlaces'},
    {'1': 'description', '3': 3, '4': 1, '5': 9, '10': 'description'},
  ],
};

/// Descriptor for `Instrument`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List instrumentDescriptor = $convert.base64Decode(
    'CgpJbnN0cnVtZW50EhIKBGNvZGUYASABKAlSBGNvZGUSJQoOZGVjaW1hbF9wbGFjZXMYAiABKA'
    '1SDWRlY2ltYWxQbGFjZXMSIAoLZGVzY3JpcHRpb24YAyABKAlSC2Rlc2NyaXB0aW9u');

@$core.Deprecated('Use createLedgerAccountDescriptor instead')
const CreateLedgerAccount$json = {
  '1': 'CreateLedgerAccount',
  '2': [
    {'1': 'parent_id', '3': 1, '4': 1, '5': 12, '10': 'parentId'},
    {'1': 'issuance', '3': 2, '4': 1, '5': 8, '10': 'issuance'},
    {'1': 'frozen', '3': 3, '4': 1, '5': 8, '10': 'frozen'},
    {'1': 'instrument', '3': 4, '4': 1, '5': 11, '6': '.m10.sdk.transaction.Instrument', '10': 'instrument'},
    {'1': 'balance_limit', '3': 5, '4': 1, '5': 4, '10': 'balanceLimit'},
  ],
};

/// Descriptor for `CreateLedgerAccount`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createLedgerAccountDescriptor = $convert.base64Decode(
    'ChNDcmVhdGVMZWRnZXJBY2NvdW50EhsKCXBhcmVudF9pZBgBIAEoDFIIcGFyZW50SWQSGgoIaX'
    'NzdWFuY2UYAiABKAhSCGlzc3VhbmNlEhYKBmZyb3plbhgDIAEoCFIGZnJvemVuEj8KCmluc3Ry'
    'dW1lbnQYBCABKAsyHy5tMTAuc2RrLnRyYW5zYWN0aW9uLkluc3RydW1lbnRSCmluc3RydW1lbn'
    'QSIwoNYmFsYW5jZV9saW1pdBgFIAEoBFIMYmFsYW5jZUxpbWl0');

@$core.Deprecated('Use setFreezeStateDescriptor instead')
const SetFreezeState$json = {
  '1': 'SetFreezeState',
  '2': [
    {'1': 'account_id', '3': 1, '4': 1, '5': 12, '10': 'accountId'},
    {'1': 'frozen', '3': 2, '4': 1, '5': 8, '10': 'frozen'},
  ],
};

/// Descriptor for `SetFreezeState`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setFreezeStateDescriptor = $convert.base64Decode(
    'Cg5TZXRGcmVlemVTdGF0ZRIdCgphY2NvdW50X2lkGAEgASgMUglhY2NvdW50SWQSFgoGZnJvem'
    'VuGAIgASgIUgZmcm96ZW4=');

@$core.Deprecated('Use setInstrumentDescriptor instead')
const SetInstrument$json = {
  '1': 'SetInstrument',
  '2': [
    {'1': 'account_id', '3': 1, '4': 1, '5': 12, '10': 'accountId'},
    {'1': 'code', '3': 2, '4': 1, '5': 9, '10': 'code'},
    {'1': 'decimal_places', '3': 3, '4': 1, '5': 13, '10': 'decimalPlaces'},
    {'1': 'description', '3': 4, '4': 1, '5': 9, '10': 'description'},
  ],
};

/// Descriptor for `SetInstrument`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setInstrumentDescriptor = $convert.base64Decode(
    'Cg1TZXRJbnN0cnVtZW50Eh0KCmFjY291bnRfaWQYASABKAxSCWFjY291bnRJZBISCgRjb2RlGA'
    'IgASgJUgRjb2RlEiUKDmRlY2ltYWxfcGxhY2VzGAMgASgNUg1kZWNpbWFsUGxhY2VzEiAKC2Rl'
    'c2NyaXB0aW9uGAQgASgJUgtkZXNjcmlwdGlvbg==');

@$core.Deprecated('Use setBalanceLimitDescriptor instead')
const SetBalanceLimit$json = {
  '1': 'SetBalanceLimit',
  '2': [
    {'1': 'account_id', '3': 1, '4': 1, '5': 12, '10': 'accountId'},
    {'1': 'balance_limit', '3': 2, '4': 1, '5': 4, '10': 'balanceLimit'},
  ],
};

/// Descriptor for `SetBalanceLimit`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setBalanceLimitDescriptor = $convert.base64Decode(
    'Cg9TZXRCYWxhbmNlTGltaXQSHQoKYWNjb3VudF9pZBgBIAEoDFIJYWNjb3VudElkEiMKDWJhbG'
    'FuY2VfbGltaXQYAiABKARSDGJhbGFuY2VMaW1pdA==');

@$core.Deprecated('Use getAccountRequestDescriptor instead')
const GetAccountRequest$json = {
  '1': 'GetAccountRequest',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
  ],
};

/// Descriptor for `GetAccountRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getAccountRequestDescriptor = $convert.base64Decode(
    'ChFHZXRBY2NvdW50UmVxdWVzdBIOCgJpZBgBIAEoDFICaWQ=');

@$core.Deprecated('Use indexedAccountDescriptor instead')
const IndexedAccount$json = {
  '1': 'IndexedAccount',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
    {'1': 'issuance', '3': 3, '4': 1, '5': 11, '6': '.m10.sdk.transaction.IndexedAccount.Issuance', '10': 'issuance'},
    {'1': 'balance', '3': 4, '4': 1, '5': 4, '10': 'balance'},
    {'1': 'frozen', '3': 5, '4': 1, '5': 8, '10': 'frozen'},
    {'1': 'instrument', '3': 6, '4': 1, '5': 11, '6': '.m10.sdk.transaction.Instrument', '10': 'instrument'},
    {'1': 'balance_limit', '3': 7, '4': 1, '5': 4, '10': 'balanceLimit'},
  ],
  '3': [IndexedAccount_Issuance$json],
};

@$core.Deprecated('Use indexedAccountDescriptor instead')
const IndexedAccount_Issuance$json = {
  '1': 'Issuance',
  '2': [
    {'1': 'issued_balance', '3': 1, '4': 1, '5': 4, '10': 'issuedBalance'},
    {'1': 'non_leaf_children', '3': 2, '4': 1, '5': 4, '10': 'nonLeafChildren'},
    {'1': 'leaf_children', '3': 3, '4': 1, '5': 4, '10': 'leafChildren'},
  ],
};

/// Descriptor for `IndexedAccount`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List indexedAccountDescriptor = $convert.base64Decode(
    'Cg5JbmRleGVkQWNjb3VudBIOCgJpZBgBIAEoDFICaWQSSAoIaXNzdWFuY2UYAyABKAsyLC5tMT'
    'Auc2RrLnRyYW5zYWN0aW9uLkluZGV4ZWRBY2NvdW50Lklzc3VhbmNlUghpc3N1YW5jZRIYCgdi'
    'YWxhbmNlGAQgASgEUgdiYWxhbmNlEhYKBmZyb3plbhgFIAEoCFIGZnJvemVuEj8KCmluc3RydW'
    '1lbnQYBiABKAsyHy5tMTAuc2RrLnRyYW5zYWN0aW9uLkluc3RydW1lbnRSCmluc3RydW1lbnQS'
    'IwoNYmFsYW5jZV9saW1pdBgHIAEoBFIMYmFsYW5jZUxpbWl0GoIBCghJc3N1YW5jZRIlCg5pc3'
    'N1ZWRfYmFsYW5jZRgBIAEoBFINaXNzdWVkQmFsYW5jZRIqChFub25fbGVhZl9jaGlsZHJlbhgC'
    'IAEoBFIPbm9uTGVhZkNoaWxkcmVuEiMKDWxlYWZfY2hpbGRyZW4YAyABKARSDGxlYWZDaGlsZH'
    'Jlbg==');

@$core.Deprecated('Use invokeActionDescriptor instead')
const InvokeAction$json = {
  '1': 'InvokeAction',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    {'1': 'from_account', '3': 3, '4': 1, '5': 12, '10': 'fromAccount'},
    {'1': 'target', '3': 4, '4': 1, '5': 11, '6': '.m10.sdk.transaction.Target', '10': 'target'},
    {'1': 'payload', '3': 5, '4': 1, '5': 12, '10': 'payload'},
  ],
};

/// Descriptor for `InvokeAction`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List invokeActionDescriptor = $convert.base64Decode(
    'CgxJbnZva2VBY3Rpb24SEgoEbmFtZRgBIAEoCVIEbmFtZRIhCgxmcm9tX2FjY291bnQYAyABKA'
    'xSC2Zyb21BY2NvdW50EjMKBnRhcmdldBgEIAEoCzIbLm0xMC5zZGsudHJhbnNhY3Rpb24uVGFy'
    'Z2V0UgZ0YXJnZXQSGAoHcGF5bG9hZBgFIAEoDFIHcGF5bG9hZA==');

@$core.Deprecated('Use targetDescriptor instead')
const Target$json = {
  '1': 'Target',
  '2': [
    {'1': 'account_id', '3': 1, '4': 1, '5': 12, '9': 0, '10': 'accountId'},
    {'1': 'any_account', '3': 2, '4': 1, '5': 11, '6': '.google.protobuf.Empty', '9': 0, '10': 'anyAccount'},
  ],
  '8': [
    {'1': 'target'},
  ],
};

/// Descriptor for `Target`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List targetDescriptor = $convert.base64Decode(
    'CgZUYXJnZXQSHwoKYWNjb3VudF9pZBgBIAEoDEgAUglhY2NvdW50SWQSOQoLYW55X2FjY291bn'
    'QYAiABKAsyFi5nb29nbGUucHJvdG9idWYuRW1wdHlIAFIKYW55QWNjb3VudEIICgZ0YXJnZXQ=');

@$core.Deprecated('Use actionDescriptor instead')
const Action$json = {
  '1': 'Action',
  '2': [
    {'1': 'tx_id', '3': 1, '4': 1, '5': 4, '10': 'txId'},
    {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    {'1': 'context_id', '3': 3, '4': 1, '5': 12, '10': 'contextId'},
    {'1': 'from_account', '3': 4, '4': 1, '5': 12, '10': 'fromAccount'},
    {'1': 'target', '3': 5, '4': 1, '5': 11, '6': '.m10.sdk.transaction.Target', '10': 'target'},
    {'1': 'payload', '3': 6, '4': 1, '5': 12, '10': 'payload'},
  ],
};

/// Descriptor for `Action`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List actionDescriptor = $convert.base64Decode(
    'CgZBY3Rpb24SEwoFdHhfaWQYASABKARSBHR4SWQSEgoEbmFtZRgCIAEoCVIEbmFtZRIdCgpjb2'
    '50ZXh0X2lkGAMgASgMUgljb250ZXh0SWQSIQoMZnJvbV9hY2NvdW50GAQgASgMUgtmcm9tQWNj'
    'b3VudBIzCgZ0YXJnZXQYBSABKAsyGy5tMTAuc2RrLnRyYW5zYWN0aW9uLlRhcmdldFIGdGFyZ2'
    'V0EhgKB3BheWxvYWQYBiABKAxSB3BheWxvYWQ=');

@$core.Deprecated('Use actionsDescriptor instead')
const Actions$json = {
  '1': 'Actions',
  '2': [
    {'1': 'actions', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.transaction.Action', '10': 'actions'},
  ],
};

/// Descriptor for `Actions`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List actionsDescriptor = $convert.base64Decode(
    'CgdBY3Rpb25zEjUKB2FjdGlvbnMYASADKAsyGy5tMTAuc2RrLnRyYW5zYWN0aW9uLkFjdGlvbl'
    'IHYWN0aW9ucw==');

@$core.Deprecated('Use getActionRequestDescriptor instead')
const GetActionRequest$json = {
  '1': 'GetActionRequest',
  '2': [
    {'1': 'tx_id', '3': 1, '4': 1, '5': 4, '10': 'txId'},
  ],
};

/// Descriptor for `GetActionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getActionRequestDescriptor = $convert.base64Decode(
    'ChBHZXRBY3Rpb25SZXF1ZXN0EhMKBXR4X2lkGAEgASgEUgR0eElk');

@$core.Deprecated('Use listActionsRequestDescriptor instead')
const ListActionsRequest$json = {
  '1': 'ListActionsRequest',
  '2': [
    {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    {'1': 'account_id', '3': 2, '4': 1, '5': 12, '9': 0, '10': 'accountId'},
    {'1': 'context_id', '3': 3, '4': 1, '5': 12, '9': 0, '10': 'contextId'},
    {'1': 'limit', '3': 4, '4': 1, '5': 4, '10': 'limit'},
    {'1': 'min_tx_id', '3': 5, '4': 1, '5': 4, '10': 'minTxId'},
    {'1': 'max_tx_id', '3': 6, '4': 1, '5': 4, '10': 'maxTxId'},
  ],
  '8': [
    {'1': 'filter'},
  ],
};

/// Descriptor for `ListActionsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listActionsRequestDescriptor = $convert.base64Decode(
    'ChJMaXN0QWN0aW9uc1JlcXVlc3QSEgoEbmFtZRgBIAEoCVIEbmFtZRIfCgphY2NvdW50X2lkGA'
    'IgASgMSABSCWFjY291bnRJZBIfCgpjb250ZXh0X2lkGAMgASgMSABSCWNvbnRleHRJZBIUCgVs'
    'aW1pdBgEIAEoBFIFbGltaXQSGgoJbWluX3R4X2lkGAUgASgEUgdtaW5UeElkEhoKCW1heF90eF'
    '9pZBgGIAEoBFIHbWF4VHhJZEIICgZmaWx0ZXI=');

@$core.Deprecated('Use commitTransferDescriptor instead')
const CommitTransfer$json = {
  '1': 'CommitTransfer',
  '2': [
    {'1': 'pending_tx_id', '3': 1, '4': 1, '5': 4, '10': 'pendingTxId'},
    {'1': 'new_state', '3': 2, '4': 1, '5': 14, '6': '.m10.sdk.transaction.CommitTransfer.TransferState', '10': 'newState'},
  ],
  '4': [CommitTransfer_TransferState$json],
};

@$core.Deprecated('Use commitTransferDescriptor instead')
const CommitTransfer_TransferState$json = {
  '1': 'TransferState',
  '2': [
    {'1': 'ACCEPTED', '2': 0},
    {'1': 'REJECTED', '2': 1},
  ],
};

/// Descriptor for `CommitTransfer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List commitTransferDescriptor = $convert.base64Decode(
    'Cg5Db21taXRUcmFuc2ZlchIiCg1wZW5kaW5nX3R4X2lkGAEgASgEUgtwZW5kaW5nVHhJZBJOCg'
    'luZXdfc3RhdGUYAiABKA4yMS5tMTAuc2RrLnRyYW5zYWN0aW9uLkNvbW1pdFRyYW5zZmVyLlRy'
    'YW5zZmVyU3RhdGVSCG5ld1N0YXRlIisKDVRyYW5zZmVyU3RhdGUSDAoIQUNDRVBURUQQABIMCg'
    'hSRUpFQ1RFRBAB');

@$core.Deprecated('Use signatureDescriptor instead')
const Signature$json = {
  '1': 'Signature',
  '2': [
    {'1': 'public_key', '3': 2, '4': 1, '5': 12, '10': 'publicKey'},
    {'1': 'signature', '3': 3, '4': 1, '5': 12, '10': 'signature'},
    {'1': 'algorithm', '3': 4, '4': 1, '5': 14, '6': '.m10.sdk.transaction.Signature.Algorithm', '10': 'algorithm'},
  ],
  '4': [Signature_Algorithm$json],
};

@$core.Deprecated('Use signatureDescriptor instead')
const Signature_Algorithm$json = {
  '1': 'Algorithm',
  '2': [
    {'1': 'P256_SHA256_ASN1', '2': 0},
    {'1': 'ED25519', '2': 1},
  ],
};

/// Descriptor for `Signature`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List signatureDescriptor = $convert.base64Decode(
    'CglTaWduYXR1cmUSHQoKcHVibGljX2tleRgCIAEoDFIJcHVibGljS2V5EhwKCXNpZ25hdHVyZR'
    'gDIAEoDFIJc2lnbmF0dXJlEkYKCWFsZ29yaXRobRgEIAEoDjIoLm0xMC5zZGsudHJhbnNhY3Rp'
    'b24uU2lnbmF0dXJlLkFsZ29yaXRobVIJYWxnb3JpdGhtIi4KCUFsZ29yaXRobRIUChBQMjU2X1'
    'NIQTI1Nl9BU04xEAASCwoHRUQyNTUxORAB');

@$core.Deprecated('Use createTokenDescriptor instead')
const CreateToken$json = {
  '1': 'CreateToken',
  '2': [
    {'1': 'address', '3': 1, '4': 1, '5': 12, '10': 'address'},
    {'1': 'account_id', '3': 2, '4': 1, '5': 12, '10': 'accountId'},
    {'1': 'value', '3': 3, '4': 1, '5': 4, '10': 'value'},
  ],
};

/// Descriptor for `CreateToken`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createTokenDescriptor = $convert.base64Decode(
    'CgtDcmVhdGVUb2tlbhIYCgdhZGRyZXNzGAEgASgMUgdhZGRyZXNzEh0KCmFjY291bnRfaWQYAi'
    'ABKAxSCWFjY291bnRJZBIUCgV2YWx1ZRgDIAEoBFIFdmFsdWU=');

@$core.Deprecated('Use offlineTokenDescriptor instead')
const OfflineToken$json = {
  '1': 'OfflineToken',
  '2': [
    {'1': 'data', '3': 1, '4': 1, '5': 11, '6': '.m10.sdk.transaction.OfflineToken.Data', '10': 'data'},
    {'1': 'signature', '3': 2, '4': 1, '5': 11, '6': '.m10.sdk.transaction.Signature', '10': 'signature'},
  ],
  '3': [OfflineToken_Data$json],
};

@$core.Deprecated('Use offlineTokenDescriptor instead')
const OfflineToken_Data$json = {
  '1': 'Data',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 4, '10': 'id'},
    {'1': 'address', '3': 2, '4': 1, '5': 12, '10': 'address'},
    {'1': 'account_id', '3': 3, '4': 1, '5': 12, '10': 'accountId'},
    {'1': 'value', '3': 4, '4': 1, '5': 4, '10': 'value'},
    {'1': 'currency', '3': 5, '4': 1, '5': 9, '10': 'currency'},
  ],
};

/// Descriptor for `OfflineToken`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List offlineTokenDescriptor = $convert.base64Decode(
    'CgxPZmZsaW5lVG9rZW4SOgoEZGF0YRgBIAEoCzImLm0xMC5zZGsudHJhbnNhY3Rpb24uT2ZmbG'
    'luZVRva2VuLkRhdGFSBGRhdGESPAoJc2lnbmF0dXJlGAIgASgLMh4ubTEwLnNkay50cmFuc2Fj'
    'dGlvbi5TaWduYXR1cmVSCXNpZ25hdHVyZRqBAQoERGF0YRIOCgJpZBgBIAEoBFICaWQSGAoHYW'
    'RkcmVzcxgCIAEoDFIHYWRkcmVzcxIdCgphY2NvdW50X2lkGAMgASgMUglhY2NvdW50SWQSFAoF'
    'dmFsdWUYBCABKARSBXZhbHVlEhoKCGN1cnJlbmN5GAUgASgJUghjdXJyZW5jeQ==');

@$core.Deprecated('Use redeemableTokenDescriptor instead')
const RedeemableToken$json = {
  '1': 'RedeemableToken',
  '2': [
    {'1': 'data', '3': 1, '4': 1, '5': 11, '6': '.m10.sdk.transaction.RedeemableToken.Data', '10': 'data'},
    {'1': 'signature', '3': 2, '4': 1, '5': 11, '6': '.m10.sdk.transaction.Signature', '10': 'signature'},
  ],
  '3': [RedeemableToken_TokenInput$json, RedeemableToken_Data$json],
};

@$core.Deprecated('Use redeemableTokenDescriptor instead')
const RedeemableToken_TokenInput$json = {
  '1': 'TokenInput',
  '2': [
    {'1': 'input', '3': 1, '4': 1, '5': 4, '10': 'input'},
    {'1': 'value', '3': 2, '4': 1, '5': 4, '10': 'value'},
  ],
};

@$core.Deprecated('Use redeemableTokenDescriptor instead')
const RedeemableToken_Data$json = {
  '1': 'Data',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
    {'1': 'address', '3': 2, '4': 1, '5': 12, '10': 'address'},
    {'1': 'inputs', '3': 3, '4': 3, '5': 11, '6': '.m10.sdk.transaction.RedeemableToken.TokenInput', '10': 'inputs'},
    {'1': 'currency', '3': 4, '4': 1, '5': 9, '10': 'currency'},
  ],
};

/// Descriptor for `RedeemableToken`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List redeemableTokenDescriptor = $convert.base64Decode(
    'Cg9SZWRlZW1hYmxlVG9rZW4SPQoEZGF0YRgBIAEoCzIpLm0xMC5zZGsudHJhbnNhY3Rpb24uUm'
    'VkZWVtYWJsZVRva2VuLkRhdGFSBGRhdGESPAoJc2lnbmF0dXJlGAIgASgLMh4ubTEwLnNkay50'
    'cmFuc2FjdGlvbi5TaWduYXR1cmVSCXNpZ25hdHVyZRo4CgpUb2tlbklucHV0EhQKBWlucHV0GA'
    'EgASgEUgVpbnB1dBIUCgV2YWx1ZRgCIAEoBFIFdmFsdWUalQEKBERhdGESDgoCaWQYASABKAxS'
    'AmlkEhgKB2FkZHJlc3MYAiABKAxSB2FkZHJlc3MSRwoGaW5wdXRzGAMgAygLMi8ubTEwLnNkay'
    '50cmFuc2FjdGlvbi5SZWRlZW1hYmxlVG9rZW4uVG9rZW5JbnB1dFIGaW5wdXRzEhoKCGN1cnJl'
    'bmN5GAQgASgJUghjdXJyZW5jeQ==');

@$core.Deprecated('Use redeemTokenDescriptor instead')
const RedeemToken$json = {
  '1': 'RedeemToken',
  '2': [
    {'1': 'token', '3': 1, '4': 1, '5': 11, '6': '.m10.sdk.transaction.RedeemableToken', '10': 'token'},
    {'1': 'account_id', '3': 2, '4': 1, '5': 12, '10': 'accountId'},
  ],
};

/// Descriptor for `RedeemToken`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List redeemTokenDescriptor = $convert.base64Decode(
    'CgtSZWRlZW1Ub2tlbhI6CgV0b2tlbhgBIAEoCzIkLm0xMC5zZGsudHJhbnNhY3Rpb24uUmVkZW'
    'VtYWJsZVRva2VuUgV0b2tlbhIdCgphY2NvdW50X2lkGAIgASgMUglhY2NvdW50SWQ=');

