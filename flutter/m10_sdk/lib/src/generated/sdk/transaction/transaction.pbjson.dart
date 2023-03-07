///
//  Generated code. Do not modify.
//  source: sdk/transaction/transaction.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields,deprecated_member_use_from_same_package

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use transactionRequestPayloadDescriptor instead')
const TransactionRequestPayload$json = const {
  '1': 'TransactionRequestPayload',
  '2': const [
    const {'1': 'nonce', '3': 1, '4': 1, '5': 4, '10': 'nonce'},
    const {'1': 'timestamp', '3': 2, '4': 1, '5': 4, '10': 'timestamp'},
    const {'1': 'context_id', '3': 5, '4': 1, '5': 12, '10': 'contextId'},
    const {'1': 'data', '3': 6, '4': 1, '5': 11, '6': '.m10.sdk.transaction.TransactionData', '10': 'data'},
  ],
};

/// Descriptor for `TransactionRequestPayload`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List transactionRequestPayloadDescriptor = $convert.base64Decode('ChlUcmFuc2FjdGlvblJlcXVlc3RQYXlsb2FkEhQKBW5vbmNlGAEgASgEUgVub25jZRIcCgl0aW1lc3RhbXAYAiABKARSCXRpbWVzdGFtcBIdCgpjb250ZXh0X2lkGAUgASgMUgljb250ZXh0SWQSOAoEZGF0YRgGIAEoCzIkLm0xMC5zZGsudHJhbnNhY3Rpb24uVHJhbnNhY3Rpb25EYXRhUgRkYXRh');
@$core.Deprecated('Use transactionDataDescriptor instead')
const TransactionData$json = const {
  '1': 'TransactionData',
  '2': const [
    const {'1': 'transfer', '3': 10, '4': 1, '5': 11, '6': '.m10.sdk.transaction.CreateTransfer', '9': 0, '10': 'transfer'},
    const {'1': 'create_ledger_account', '3': 11, '4': 1, '5': 11, '6': '.m10.sdk.transaction.CreateLedgerAccount', '9': 0, '10': 'createLedgerAccount'},
    const {'1': 'set_freeze_state', '3': 12, '4': 1, '5': 11, '6': '.m10.sdk.transaction.SetFreezeState', '9': 0, '10': 'setFreezeState'},
    const {'1': 'document_operations', '3': 16, '4': 1, '5': 11, '6': '.m10.sdk.DocumentOperations', '9': 0, '10': 'documentOperations'},
    const {'1': 'invoke_action', '3': 20, '4': 1, '5': 11, '6': '.m10.sdk.transaction.InvokeAction', '9': 0, '10': 'invokeAction'},
    const {'1': 'initiate_transfer', '3': 21, '4': 1, '5': 11, '6': '.m10.sdk.transaction.CreateTransfer', '9': 0, '10': 'initiateTransfer'},
    const {'1': 'commit_transfer', '3': 22, '4': 1, '5': 11, '6': '.m10.sdk.transaction.CommitTransfer', '9': 0, '10': 'commitTransfer'},
    const {'1': 'set_instrument', '3': 23, '4': 1, '5': 11, '6': '.m10.sdk.transaction.SetInstrument', '9': 0, '10': 'setInstrument'},
    const {'1': 'set_balance_limit', '3': 24, '4': 1, '5': 11, '6': '.m10.sdk.transaction.SetBalanceLimit', '9': 0, '10': 'setBalanceLimit'},
  ],
  '8': const [
    const {'1': 'data'},
  ],
};

/// Descriptor for `TransactionData`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List transactionDataDescriptor = $convert.base64Decode('Cg9UcmFuc2FjdGlvbkRhdGESQQoIdHJhbnNmZXIYCiABKAsyIy5tMTAuc2RrLnRyYW5zYWN0aW9uLkNyZWF0ZVRyYW5zZmVySABSCHRyYW5zZmVyEl4KFWNyZWF0ZV9sZWRnZXJfYWNjb3VudBgLIAEoCzIoLm0xMC5zZGsudHJhbnNhY3Rpb24uQ3JlYXRlTGVkZ2VyQWNjb3VudEgAUhNjcmVhdGVMZWRnZXJBY2NvdW50Ek8KEHNldF9mcmVlemVfc3RhdGUYDCABKAsyIy5tMTAuc2RrLnRyYW5zYWN0aW9uLlNldEZyZWV6ZVN0YXRlSABSDnNldEZyZWV6ZVN0YXRlEk4KE2RvY3VtZW50X29wZXJhdGlvbnMYECABKAsyGy5tMTAuc2RrLkRvY3VtZW50T3BlcmF0aW9uc0gAUhJkb2N1bWVudE9wZXJhdGlvbnMSSAoNaW52b2tlX2FjdGlvbhgUIAEoCzIhLm0xMC5zZGsudHJhbnNhY3Rpb24uSW52b2tlQWN0aW9uSABSDGludm9rZUFjdGlvbhJSChFpbml0aWF0ZV90cmFuc2ZlchgVIAEoCzIjLm0xMC5zZGsudHJhbnNhY3Rpb24uQ3JlYXRlVHJhbnNmZXJIAFIQaW5pdGlhdGVUcmFuc2ZlchJOCg9jb21taXRfdHJhbnNmZXIYFiABKAsyIy5tMTAuc2RrLnRyYW5zYWN0aW9uLkNvbW1pdFRyYW5zZmVySABSDmNvbW1pdFRyYW5zZmVyEksKDnNldF9pbnN0cnVtZW50GBcgASgLMiIubTEwLnNkay50cmFuc2FjdGlvbi5TZXRJbnN0cnVtZW50SABSDXNldEluc3RydW1lbnQSUgoRc2V0X2JhbGFuY2VfbGltaXQYGCABKAsyJC5tMTAuc2RrLnRyYW5zYWN0aW9uLlNldEJhbGFuY2VMaW1pdEgAUg9zZXRCYWxhbmNlTGltaXRCBgoEZGF0YQ==');
@$core.Deprecated('Use transactionResponseDescriptor instead')
const TransactionResponse$json = const {
  '1': 'TransactionResponse',
  '2': const [
    const {'1': 'tx_id', '3': 1, '4': 1, '5': 4, '10': 'txId'},
    const {'1': 'error', '3': 2, '4': 1, '5': 11, '6': '.m10.sdk.transaction.TransactionError', '10': 'error'},
    const {'1': 'timestamp', '3': 4, '4': 1, '5': 6, '10': 'timestamp'},
    const {'1': 'account_created', '3': 3, '4': 1, '5': 12, '10': 'accountCreated'},
    const {'1': 'transfer_committed', '3': 5, '4': 1, '5': 11, '6': '.m10.sdk.transaction.CreateTransfer', '10': 'transferCommitted'},
  ],
};

/// Descriptor for `TransactionResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List transactionResponseDescriptor = $convert.base64Decode('ChNUcmFuc2FjdGlvblJlc3BvbnNlEhMKBXR4X2lkGAEgASgEUgR0eElkEjsKBWVycm9yGAIgASgLMiUubTEwLnNkay50cmFuc2FjdGlvbi5UcmFuc2FjdGlvbkVycm9yUgVlcnJvchIcCgl0aW1lc3RhbXAYBCABKAZSCXRpbWVzdGFtcBInCg9hY2NvdW50X2NyZWF0ZWQYAyABKAxSDmFjY291bnRDcmVhdGVkElIKEnRyYW5zZmVyX2NvbW1pdHRlZBgFIAEoCzIjLm0xMC5zZGsudHJhbnNhY3Rpb24uQ3JlYXRlVHJhbnNmZXJSEXRyYW5zZmVyQ29tbWl0dGVk');
@$core.Deprecated('Use transactionErrorDescriptor instead')
const TransactionError$json = const {
  '1': 'TransactionError',
  '2': const [
    const {'1': 'code', '3': 1, '4': 1, '5': 14, '6': '.m10.sdk.transaction.TransactionError.Code', '10': 'code'},
    const {'1': 'message', '3': 2, '4': 1, '5': 9, '10': 'message'},
  ],
  '4': const [TransactionError_Code$json],
};

@$core.Deprecated('Use transactionErrorDescriptor instead')
const TransactionError_Code$json = const {
  '1': 'Code',
  '2': const [
    const {'1': 'UNKNOWN', '2': 0},
    const {'1': 'UNIMPLEMENTED', '2': 1},
    const {'1': 'NOT_FOUND', '2': 2},
    const {'1': 'ALREADY_EXISTS', '2': 3},
    const {'1': 'UNAUTHORIZED', '2': 4},
    const {'1': 'BAD_REQUEST', '2': 5},
    const {'1': 'INVALID_REQUEST_TYPE', '2': 6},
    const {'1': 'INVALID_ACCOUNT_ID', '2': 7},
    const {'1': 'INVALID_TRANSFER', '2': 8},
    const {'1': 'MESSAGE_TOO_LARGE', '2': 10},
    const {'1': 'INVALID_SIGNATURE', '2': 11},
    const {'1': 'VERIFICATION_FAILED', '2': 12},
    const {'1': 'REPLAY_PROTECTION', '2': 20},
    const {'1': 'INVALID_EXPRESSION', '2': 21},
    const {'1': 'INCORRECT_TYPE', '2': 22},
    const {'1': 'ACCOUNT_FROZEN', '2': 23},
    const {'1': 'UNMODIFIED_STATE', '2': 24},
    const {'1': 'INSUFFICIENT_BALANCE', '2': 25},
    const {'1': 'BALANCE_OVERFLOW', '2': 26},
    const {'1': 'ACCOUNT_DEPTH_EXCEEDED', '2': 27},
    const {'1': 'HOLDING_LIMIT_EXCEEDED', '2': 28},
    const {'1': 'INVALID_TARGET', '2': 30},
  ],
};

/// Descriptor for `TransactionError`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List transactionErrorDescriptor = $convert.base64Decode('ChBUcmFuc2FjdGlvbkVycm9yEj4KBGNvZGUYASABKA4yKi5tMTAuc2RrLnRyYW5zYWN0aW9uLlRyYW5zYWN0aW9uRXJyb3IuQ29kZVIEY29kZRIYCgdtZXNzYWdlGAIgASgJUgdtZXNzYWdlIuQDCgRDb2RlEgsKB1VOS05PV04QABIRCg1VTklNUExFTUVOVEVEEAESDQoJTk9UX0ZPVU5EEAISEgoOQUxSRUFEWV9FWElTVFMQAxIQCgxVTkFVVEhPUklaRUQQBBIPCgtCQURfUkVRVUVTVBAFEhgKFElOVkFMSURfUkVRVUVTVF9UWVBFEAYSFgoSSU5WQUxJRF9BQ0NPVU5UX0lEEAcSFAoQSU5WQUxJRF9UUkFOU0ZFUhAIEhUKEU1FU1NBR0VfVE9PX0xBUkdFEAoSFQoRSU5WQUxJRF9TSUdOQVRVUkUQCxIXChNWRVJJRklDQVRJT05fRkFJTEVEEAwSFQoRUkVQTEFZX1BST1RFQ1RJT04QFBIWChJJTlZBTElEX0VYUFJFU1NJT04QFRISCg5JTkNPUlJFQ1RfVFlQRRAWEhIKDkFDQ09VTlRfRlJPWkVOEBcSFAoQVU5NT0RJRklFRF9TVEFURRAYEhgKFElOU1VGRklDSUVOVF9CQUxBTkNFEBkSFAoQQkFMQU5DRV9PVkVSRkxPVxAaEhoKFkFDQ09VTlRfREVQVEhfRVhDRUVERUQQGxIaChZIT0xESU5HX0xJTUlUX0VYQ0VFREVEEBwSEgoOSU5WQUxJRF9UQVJHRVQQHg==');
@$core.Deprecated('Use createLedgerTransferDescriptor instead')
const CreateLedgerTransfer$json = const {
  '1': 'CreateLedgerTransfer',
  '2': const [
    const {'1': 'ledger_id', '3': 1, '4': 1, '5': 9, '10': 'ledgerId'},
    const {'1': 'nonce', '3': 2, '4': 1, '5': 4, '10': 'nonce'},
    const {'1': 'transfer', '3': 3, '4': 1, '5': 11, '6': '.m10.sdk.transaction.CreateTransfer', '10': 'transfer'},
  ],
};

/// Descriptor for `CreateLedgerTransfer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createLedgerTransferDescriptor = $convert.base64Decode('ChRDcmVhdGVMZWRnZXJUcmFuc2ZlchIbCglsZWRnZXJfaWQYASABKAlSCGxlZGdlcklkEhQKBW5vbmNlGAIgASgEUgVub25jZRI/Cgh0cmFuc2ZlchgDIAEoCzIjLm0xMC5zZGsudHJhbnNhY3Rpb24uQ3JlYXRlVHJhbnNmZXJSCHRyYW5zZmVy');
@$core.Deprecated('Use createLedgerTransfersDescriptor instead')
const CreateLedgerTransfers$json = const {
  '1': 'CreateLedgerTransfers',
  '2': const [
    const {'1': 'transfers', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.transaction.CreateLedgerTransfer', '10': 'transfers'},
    const {'1': 'valid_until', '3': 2, '4': 1, '5': 4, '10': 'validUntil'},
  ],
};

/// Descriptor for `CreateLedgerTransfers`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createLedgerTransfersDescriptor = $convert.base64Decode('ChVDcmVhdGVMZWRnZXJUcmFuc2ZlcnMSRwoJdHJhbnNmZXJzGAEgAygLMikubTEwLnNkay50cmFuc2FjdGlvbi5DcmVhdGVMZWRnZXJUcmFuc2ZlclIJdHJhbnNmZXJzEh8KC3ZhbGlkX3VudGlsGAIgASgEUgp2YWxpZFVudGls');
@$core.Deprecated('Use getTransferRequestDescriptor instead')
const GetTransferRequest$json = const {
  '1': 'GetTransferRequest',
  '2': const [
    const {'1': 'tx_id', '3': 1, '4': 1, '5': 4, '10': 'txId'},
  ],
};

/// Descriptor for `GetTransferRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getTransferRequestDescriptor = $convert.base64Decode('ChJHZXRUcmFuc2ZlclJlcXVlc3QSEwoFdHhfaWQYASABKARSBHR4SWQ=');
@$core.Deprecated('Use listTransferRequestDescriptor instead')
const ListTransferRequest$json = const {
  '1': 'ListTransferRequest',
  '2': const [
    const {'1': 'account_id', '3': 1, '4': 1, '5': 12, '9': 0, '10': 'accountId'},
    const {'1': 'context_id', '3': 2, '4': 1, '5': 12, '9': 0, '10': 'contextId'},
    const {'1': 'limit', '3': 4, '4': 1, '5': 4, '10': 'limit'},
    const {'1': 'include_child_accounts', '3': 5, '4': 1, '5': 8, '10': 'includeChildAccounts'},
    const {'1': 'min_tx_id', '3': 6, '4': 1, '5': 4, '10': 'minTxId'},
    const {'1': 'max_tx_id', '3': 7, '4': 1, '5': 4, '10': 'maxTxId'},
  ],
  '8': const [
    const {'1': 'filter'},
  ],
};

/// Descriptor for `ListTransferRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listTransferRequestDescriptor = $convert.base64Decode('ChNMaXN0VHJhbnNmZXJSZXF1ZXN0Eh8KCmFjY291bnRfaWQYASABKAxIAFIJYWNjb3VudElkEh8KCmNvbnRleHRfaWQYAiABKAxIAFIJY29udGV4dElkEhQKBWxpbWl0GAQgASgEUgVsaW1pdBI0ChZpbmNsdWRlX2NoaWxkX2FjY291bnRzGAUgASgIUhRpbmNsdWRlQ2hpbGRBY2NvdW50cxIaCgltaW5fdHhfaWQYBiABKARSB21pblR4SWQSGgoJbWF4X3R4X2lkGAcgASgEUgdtYXhUeElkQggKBmZpbHRlcg==');
@$core.Deprecated('Use createTransferDescriptor instead')
const CreateTransfer$json = const {
  '1': 'CreateTransfer',
  '2': const [
    const {'1': 'transfer_steps', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.transaction.TransferStep', '10': 'transferSteps'},
  ],
};

/// Descriptor for `CreateTransfer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createTransferDescriptor = $convert.base64Decode('Cg5DcmVhdGVUcmFuc2ZlchJICg50cmFuc2Zlcl9zdGVwcxgBIAMoCzIhLm0xMC5zZGsudHJhbnNhY3Rpb24uVHJhbnNmZXJTdGVwUg10cmFuc2ZlclN0ZXBz');
@$core.Deprecated('Use transferStepDescriptor instead')
const TransferStep$json = const {
  '1': 'TransferStep',
  '2': const [
    const {'1': 'from_account_id', '3': 1, '4': 1, '5': 12, '10': 'fromAccountId'},
    const {'1': 'to_account_id', '3': 2, '4': 1, '5': 12, '10': 'toAccountId'},
    const {'1': 'amount', '3': 4, '4': 1, '5': 4, '10': 'amount'},
    const {'1': 'metadata', '3': 7, '4': 3, '5': 11, '6': '.google.protobuf.Any', '10': 'metadata'},
  ],
};

/// Descriptor for `TransferStep`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List transferStepDescriptor = $convert.base64Decode('CgxUcmFuc2ZlclN0ZXASJgoPZnJvbV9hY2NvdW50X2lkGAEgASgMUg1mcm9tQWNjb3VudElkEiIKDXRvX2FjY291bnRfaWQYAiABKAxSC3RvQWNjb3VudElkEhYKBmFtb3VudBgEIAEoBFIGYW1vdW50EjAKCG1ldGFkYXRhGAcgAygLMhQuZ29vZ2xlLnByb3RvYnVmLkFueVIIbWV0YWRhdGE=');
@$core.Deprecated('Use finalizedTransferDescriptor instead')
const FinalizedTransfer$json = const {
  '1': 'FinalizedTransfer',
  '2': const [
    const {'1': 'tx_id', '3': 1, '4': 1, '5': 4, '10': 'txId'},
    const {'1': 'context_id', '3': 2, '4': 1, '5': 12, '10': 'contextId'},
    const {'1': 'transfer_steps', '3': 3, '4': 3, '5': 11, '6': '.m10.sdk.transaction.TransferStep', '10': 'transferSteps'},
    const {'1': 'error', '3': 4, '4': 1, '5': 11, '6': '.m10.sdk.transaction.TransactionError', '10': 'error'},
    const {'1': 'timestamp', '3': 5, '4': 1, '5': 6, '10': 'timestamp'},
    const {'1': 'state', '3': 7, '4': 1, '5': 14, '6': '.m10.sdk.transaction.FinalizedTransfer.TransferState', '10': 'state'},
  ],
  '4': const [FinalizedTransfer_TransferState$json],
};

@$core.Deprecated('Use finalizedTransferDescriptor instead')
const FinalizedTransfer_TransferState$json = const {
  '1': 'TransferState',
  '2': const [
    const {'1': 'ACCEPTED', '2': 0},
    const {'1': 'REJECTED', '2': 1},
    const {'1': 'PENDING', '2': 2},
    const {'1': 'EXPIRED', '2': 3},
  ],
};

/// Descriptor for `FinalizedTransfer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List finalizedTransferDescriptor = $convert.base64Decode('ChFGaW5hbGl6ZWRUcmFuc2ZlchITCgV0eF9pZBgBIAEoBFIEdHhJZBIdCgpjb250ZXh0X2lkGAIgASgMUgljb250ZXh0SWQSSAoOdHJhbnNmZXJfc3RlcHMYAyADKAsyIS5tMTAuc2RrLnRyYW5zYWN0aW9uLlRyYW5zZmVyU3RlcFINdHJhbnNmZXJTdGVwcxI7CgVlcnJvchgEIAEoCzIlLm0xMC5zZGsudHJhbnNhY3Rpb24uVHJhbnNhY3Rpb25FcnJvclIFZXJyb3ISHAoJdGltZXN0YW1wGAUgASgGUgl0aW1lc3RhbXASSgoFc3RhdGUYByABKA4yNC5tMTAuc2RrLnRyYW5zYWN0aW9uLkZpbmFsaXplZFRyYW5zZmVyLlRyYW5zZmVyU3RhdGVSBXN0YXRlIkUKDVRyYW5zZmVyU3RhdGUSDAoIQUNDRVBURUQQABIMCghSRUpFQ1RFRBABEgsKB1BFTkRJTkcQAhILCgdFWFBJUkVEEAM=');
@$core.Deprecated('Use finalizedTransfersDescriptor instead')
const FinalizedTransfers$json = const {
  '1': 'FinalizedTransfers',
  '2': const [
    const {'1': 'transfers', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.transaction.FinalizedTransfer', '10': 'transfers'},
  ],
};

/// Descriptor for `FinalizedTransfers`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List finalizedTransfersDescriptor = $convert.base64Decode('ChJGaW5hbGl6ZWRUcmFuc2ZlcnMSRAoJdHJhbnNmZXJzGAEgAygLMiYubTEwLnNkay50cmFuc2FjdGlvbi5GaW5hbGl6ZWRUcmFuc2ZlclIJdHJhbnNmZXJz');
@$core.Deprecated('Use instrumentDescriptor instead')
const Instrument$json = const {
  '1': 'Instrument',
  '2': const [
    const {'1': 'code', '3': 1, '4': 1, '5': 9, '10': 'code'},
    const {'1': 'decimal_places', '3': 2, '4': 1, '5': 13, '10': 'decimalPlaces'},
    const {'1': 'description', '3': 3, '4': 1, '5': 9, '10': 'description'},
  ],
};

/// Descriptor for `Instrument`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List instrumentDescriptor = $convert.base64Decode('CgpJbnN0cnVtZW50EhIKBGNvZGUYASABKAlSBGNvZGUSJQoOZGVjaW1hbF9wbGFjZXMYAiABKA1SDWRlY2ltYWxQbGFjZXMSIAoLZGVzY3JpcHRpb24YAyABKAlSC2Rlc2NyaXB0aW9u');
@$core.Deprecated('Use createLedgerAccountDescriptor instead')
const CreateLedgerAccount$json = const {
  '1': 'CreateLedgerAccount',
  '2': const [
    const {'1': 'parent_id', '3': 1, '4': 1, '5': 12, '10': 'parentId'},
    const {'1': 'issuance', '3': 2, '4': 1, '5': 8, '10': 'issuance'},
    const {'1': 'frozen', '3': 3, '4': 1, '5': 8, '10': 'frozen'},
    const {'1': 'instrument', '3': 4, '4': 1, '5': 11, '6': '.m10.sdk.transaction.Instrument', '10': 'instrument'},
    const {'1': 'balance_limit', '3': 5, '4': 1, '5': 4, '10': 'balanceLimit'},
  ],
};

/// Descriptor for `CreateLedgerAccount`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List createLedgerAccountDescriptor = $convert.base64Decode('ChNDcmVhdGVMZWRnZXJBY2NvdW50EhsKCXBhcmVudF9pZBgBIAEoDFIIcGFyZW50SWQSGgoIaXNzdWFuY2UYAiABKAhSCGlzc3VhbmNlEhYKBmZyb3plbhgDIAEoCFIGZnJvemVuEj8KCmluc3RydW1lbnQYBCABKAsyHy5tMTAuc2RrLnRyYW5zYWN0aW9uLkluc3RydW1lbnRSCmluc3RydW1lbnQSIwoNYmFsYW5jZV9saW1pdBgFIAEoBFIMYmFsYW5jZUxpbWl0');
@$core.Deprecated('Use setFreezeStateDescriptor instead')
const SetFreezeState$json = const {
  '1': 'SetFreezeState',
  '2': const [
    const {'1': 'account_id', '3': 1, '4': 1, '5': 12, '10': 'accountId'},
    const {'1': 'frozen', '3': 2, '4': 1, '5': 8, '10': 'frozen'},
  ],
};

/// Descriptor for `SetFreezeState`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setFreezeStateDescriptor = $convert.base64Decode('Cg5TZXRGcmVlemVTdGF0ZRIdCgphY2NvdW50X2lkGAEgASgMUglhY2NvdW50SWQSFgoGZnJvemVuGAIgASgIUgZmcm96ZW4=');
@$core.Deprecated('Use setInstrumentDescriptor instead')
const SetInstrument$json = const {
  '1': 'SetInstrument',
  '2': const [
    const {'1': 'account_id', '3': 1, '4': 1, '5': 12, '10': 'accountId'},
    const {'1': 'code', '3': 2, '4': 1, '5': 9, '10': 'code'},
    const {'1': 'decimal_places', '3': 3, '4': 1, '5': 13, '10': 'decimalPlaces'},
    const {'1': 'description', '3': 4, '4': 1, '5': 9, '10': 'description'},
  ],
};

/// Descriptor for `SetInstrument`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setInstrumentDescriptor = $convert.base64Decode('Cg1TZXRJbnN0cnVtZW50Eh0KCmFjY291bnRfaWQYASABKAxSCWFjY291bnRJZBISCgRjb2RlGAIgASgJUgRjb2RlEiUKDmRlY2ltYWxfcGxhY2VzGAMgASgNUg1kZWNpbWFsUGxhY2VzEiAKC2Rlc2NyaXB0aW9uGAQgASgJUgtkZXNjcmlwdGlvbg==');
@$core.Deprecated('Use setBalanceLimitDescriptor instead')
const SetBalanceLimit$json = const {
  '1': 'SetBalanceLimit',
  '2': const [
    const {'1': 'account_id', '3': 1, '4': 1, '5': 12, '10': 'accountId'},
    const {'1': 'balance_limit', '3': 2, '4': 1, '5': 4, '10': 'balanceLimit'},
  ],
};

/// Descriptor for `SetBalanceLimit`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List setBalanceLimitDescriptor = $convert.base64Decode('Cg9TZXRCYWxhbmNlTGltaXQSHQoKYWNjb3VudF9pZBgBIAEoDFIJYWNjb3VudElkEiMKDWJhbGFuY2VfbGltaXQYAiABKARSDGJhbGFuY2VMaW1pdA==');
@$core.Deprecated('Use getAccountRequestDescriptor instead')
const GetAccountRequest$json = const {
  '1': 'GetAccountRequest',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
  ],
};

/// Descriptor for `GetAccountRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getAccountRequestDescriptor = $convert.base64Decode('ChFHZXRBY2NvdW50UmVxdWVzdBIOCgJpZBgBIAEoDFICaWQ=');
@$core.Deprecated('Use indexedAccountDescriptor instead')
const IndexedAccount$json = const {
  '1': 'IndexedAccount',
  '2': const [
    const {'1': 'id', '3': 1, '4': 1, '5': 12, '10': 'id'},
    const {'1': 'issuance', '3': 3, '4': 1, '5': 11, '6': '.m10.sdk.transaction.IndexedAccount.Issuance', '10': 'issuance'},
    const {'1': 'balance', '3': 4, '4': 1, '5': 4, '10': 'balance'},
    const {'1': 'frozen', '3': 5, '4': 1, '5': 8, '10': 'frozen'},
    const {'1': 'instrument', '3': 6, '4': 1, '5': 11, '6': '.m10.sdk.transaction.Instrument', '10': 'instrument'},
    const {'1': 'balance_limit', '3': 7, '4': 1, '5': 4, '10': 'balanceLimit'},
  ],
  '3': const [IndexedAccount_Issuance$json],
};

@$core.Deprecated('Use indexedAccountDescriptor instead')
const IndexedAccount_Issuance$json = const {
  '1': 'Issuance',
  '2': const [
    const {'1': 'issued_balance', '3': 1, '4': 1, '5': 4, '10': 'issuedBalance'},
    const {'1': 'non_leaf_children', '3': 2, '4': 1, '5': 4, '10': 'nonLeafChildren'},
    const {'1': 'leaf_children', '3': 3, '4': 1, '5': 4, '10': 'leafChildren'},
  ],
};

/// Descriptor for `IndexedAccount`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List indexedAccountDescriptor = $convert.base64Decode('Cg5JbmRleGVkQWNjb3VudBIOCgJpZBgBIAEoDFICaWQSSAoIaXNzdWFuY2UYAyABKAsyLC5tMTAuc2RrLnRyYW5zYWN0aW9uLkluZGV4ZWRBY2NvdW50Lklzc3VhbmNlUghpc3N1YW5jZRIYCgdiYWxhbmNlGAQgASgEUgdiYWxhbmNlEhYKBmZyb3plbhgFIAEoCFIGZnJvemVuEj8KCmluc3RydW1lbnQYBiABKAsyHy5tMTAuc2RrLnRyYW5zYWN0aW9uLkluc3RydW1lbnRSCmluc3RydW1lbnQSIwoNYmFsYW5jZV9saW1pdBgHIAEoBFIMYmFsYW5jZUxpbWl0GoIBCghJc3N1YW5jZRIlCg5pc3N1ZWRfYmFsYW5jZRgBIAEoBFINaXNzdWVkQmFsYW5jZRIqChFub25fbGVhZl9jaGlsZHJlbhgCIAEoBFIPbm9uTGVhZkNoaWxkcmVuEiMKDWxlYWZfY2hpbGRyZW4YAyABKARSDGxlYWZDaGlsZHJlbg==');
@$core.Deprecated('Use invokeActionDescriptor instead')
const InvokeAction$json = const {
  '1': 'InvokeAction',
  '2': const [
    const {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'from_account', '3': 3, '4': 1, '5': 12, '10': 'fromAccount'},
    const {'1': 'target', '3': 4, '4': 1, '5': 11, '6': '.m10.sdk.transaction.Target', '10': 'target'},
    const {'1': 'payload', '3': 5, '4': 1, '5': 12, '10': 'payload'},
  ],
};

/// Descriptor for `InvokeAction`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List invokeActionDescriptor = $convert.base64Decode('CgxJbnZva2VBY3Rpb24SEgoEbmFtZRgBIAEoCVIEbmFtZRIhCgxmcm9tX2FjY291bnQYAyABKAxSC2Zyb21BY2NvdW50EjMKBnRhcmdldBgEIAEoCzIbLm0xMC5zZGsudHJhbnNhY3Rpb24uVGFyZ2V0UgZ0YXJnZXQSGAoHcGF5bG9hZBgFIAEoDFIHcGF5bG9hZA==');
@$core.Deprecated('Use targetDescriptor instead')
const Target$json = const {
  '1': 'Target',
  '2': const [
    const {'1': 'account_id', '3': 1, '4': 1, '5': 12, '9': 0, '10': 'accountId'},
    const {'1': 'any_account', '3': 2, '4': 1, '5': 11, '6': '.google.protobuf.Empty', '9': 0, '10': 'anyAccount'},
  ],
  '8': const [
    const {'1': 'target'},
  ],
};

/// Descriptor for `Target`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List targetDescriptor = $convert.base64Decode('CgZUYXJnZXQSHwoKYWNjb3VudF9pZBgBIAEoDEgAUglhY2NvdW50SWQSOQoLYW55X2FjY291bnQYAiABKAsyFi5nb29nbGUucHJvdG9idWYuRW1wdHlIAFIKYW55QWNjb3VudEIICgZ0YXJnZXQ=');
@$core.Deprecated('Use actionDescriptor instead')
const Action$json = const {
  '1': 'Action',
  '2': const [
    const {'1': 'tx_id', '3': 1, '4': 1, '5': 4, '10': 'txId'},
    const {'1': 'name', '3': 2, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'context_id', '3': 3, '4': 1, '5': 12, '10': 'contextId'},
    const {'1': 'from_account', '3': 4, '4': 1, '5': 12, '10': 'fromAccount'},
    const {'1': 'target', '3': 5, '4': 1, '5': 11, '6': '.m10.sdk.transaction.Target', '10': 'target'},
    const {'1': 'payload', '3': 6, '4': 1, '5': 12, '10': 'payload'},
  ],
};

/// Descriptor for `Action`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List actionDescriptor = $convert.base64Decode('CgZBY3Rpb24SEwoFdHhfaWQYASABKARSBHR4SWQSEgoEbmFtZRgCIAEoCVIEbmFtZRIdCgpjb250ZXh0X2lkGAMgASgMUgljb250ZXh0SWQSIQoMZnJvbV9hY2NvdW50GAQgASgMUgtmcm9tQWNjb3VudBIzCgZ0YXJnZXQYBSABKAsyGy5tMTAuc2RrLnRyYW5zYWN0aW9uLlRhcmdldFIGdGFyZ2V0EhgKB3BheWxvYWQYBiABKAxSB3BheWxvYWQ=');
@$core.Deprecated('Use actionsDescriptor instead')
const Actions$json = const {
  '1': 'Actions',
  '2': const [
    const {'1': 'actions', '3': 1, '4': 3, '5': 11, '6': '.m10.sdk.transaction.Action', '10': 'actions'},
  ],
};

/// Descriptor for `Actions`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List actionsDescriptor = $convert.base64Decode('CgdBY3Rpb25zEjUKB2FjdGlvbnMYASADKAsyGy5tMTAuc2RrLnRyYW5zYWN0aW9uLkFjdGlvblIHYWN0aW9ucw==');
@$core.Deprecated('Use getActionRequestDescriptor instead')
const GetActionRequest$json = const {
  '1': 'GetActionRequest',
  '2': const [
    const {'1': 'tx_id', '3': 1, '4': 1, '5': 4, '10': 'txId'},
  ],
};

/// Descriptor for `GetActionRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List getActionRequestDescriptor = $convert.base64Decode('ChBHZXRBY3Rpb25SZXF1ZXN0EhMKBXR4X2lkGAEgASgEUgR0eElk');
@$core.Deprecated('Use listActionsRequestDescriptor instead')
const ListActionsRequest$json = const {
  '1': 'ListActionsRequest',
  '2': const [
    const {'1': 'name', '3': 1, '4': 1, '5': 9, '10': 'name'},
    const {'1': 'account_id', '3': 2, '4': 1, '5': 12, '9': 0, '10': 'accountId'},
    const {'1': 'context_id', '3': 3, '4': 1, '5': 12, '9': 0, '10': 'contextId'},
    const {'1': 'limit', '3': 4, '4': 1, '5': 4, '10': 'limit'},
    const {'1': 'min_tx_id', '3': 5, '4': 1, '5': 4, '10': 'minTxId'},
    const {'1': 'max_tx_id', '3': 6, '4': 1, '5': 4, '10': 'maxTxId'},
  ],
  '8': const [
    const {'1': 'filter'},
  ],
};

/// Descriptor for `ListActionsRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listActionsRequestDescriptor = $convert.base64Decode('ChJMaXN0QWN0aW9uc1JlcXVlc3QSEgoEbmFtZRgBIAEoCVIEbmFtZRIfCgphY2NvdW50X2lkGAIgASgMSABSCWFjY291bnRJZBIfCgpjb250ZXh0X2lkGAMgASgMSABSCWNvbnRleHRJZBIUCgVsaW1pdBgEIAEoBFIFbGltaXQSGgoJbWluX3R4X2lkGAUgASgEUgdtaW5UeElkEhoKCW1heF90eF9pZBgGIAEoBFIHbWF4VHhJZEIICgZmaWx0ZXI=');
@$core.Deprecated('Use commitTransferDescriptor instead')
const CommitTransfer$json = const {
  '1': 'CommitTransfer',
  '2': const [
    const {'1': 'pending_tx_id', '3': 1, '4': 1, '5': 4, '10': 'pendingTxId'},
    const {'1': 'new_state', '3': 2, '4': 1, '5': 14, '6': '.m10.sdk.transaction.CommitTransfer.TransferState', '10': 'newState'},
  ],
  '4': const [CommitTransfer_TransferState$json],
};

@$core.Deprecated('Use commitTransferDescriptor instead')
const CommitTransfer_TransferState$json = const {
  '1': 'TransferState',
  '2': const [
    const {'1': 'ACCEPTED', '2': 0},
    const {'1': 'REJECTED', '2': 1},
  ],
};

/// Descriptor for `CommitTransfer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List commitTransferDescriptor = $convert.base64Decode('Cg5Db21taXRUcmFuc2ZlchIiCg1wZW5kaW5nX3R4X2lkGAEgASgEUgtwZW5kaW5nVHhJZBJOCgluZXdfc3RhdGUYAiABKA4yMS5tMTAuc2RrLnRyYW5zYWN0aW9uLkNvbW1pdFRyYW5zZmVyLlRyYW5zZmVyU3RhdGVSCG5ld1N0YXRlIisKDVRyYW5zZmVyU3RhdGUSDAoIQUNDRVBURUQQABIMCghSRUpFQ1RFRBAB');
