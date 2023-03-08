///
//  Generated code. Do not modify.
//  source: sdk/metadata.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use attachmentDescriptor instead')
const Attachment$json = const {
  '1': 'Attachment',
  '2': const [
    const {'1': 'object_id', '3': 1, '4': 1, '5': 9, '10': 'objectId'},
    const {'1': 'type', '3': 2, '4': 1, '5': 14, '6': '.m10.sdk.metadata.Attachment.AttachmentType', '10': 'type'},
  ],
  '4': const [Attachment_AttachmentType$json],
};

@$core.Deprecated('Use attachmentDescriptor instead')
const Attachment_AttachmentType$json = const {
  '1': 'AttachmentType',
  '2': const [
    const {'1': 'OBJECT', '2': 0},
    const {'1': 'IMAGE', '2': 1},
  ],
};

/// Descriptor for `Attachment`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List attachmentDescriptor = $convert.base64Decode('CgpBdHRhY2htZW50EhsKCW9iamVjdF9pZBgBIAEoCVIIb2JqZWN0SWQSPwoEdHlwZRgCIAEoDjIrLm0xMC5zZGsubWV0YWRhdGEuQXR0YWNobWVudC5BdHRhY2htZW50VHlwZVIEdHlwZSInCg5BdHRhY2htZW50VHlwZRIKCgZPQkpFQ1QQABIJCgVJTUFHRRAB');
@$core.Deprecated('Use memoDescriptor instead')
const Memo$json = const {
  '1': 'Memo',
  '2': const [
    const {'1': 'plaintext', '3': 1, '4': 1, '5': 9, '10': 'plaintext'},
  ],
};

/// Descriptor for `Memo`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List memoDescriptor = $convert.base64Decode('CgRNZW1vEhwKCXBsYWludGV4dBgBIAEoCVIJcGxhaW50ZXh0');
@$core.Deprecated('Use feeDescriptor instead')
const Fee$json = const {
  '1': 'Fee',
};

/// Descriptor for `Fee`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List feeDescriptor = $convert.base64Decode('CgNGZWU=');
@$core.Deprecated('Use withdrawDescriptor instead')
const Withdraw$json = const {
  '1': 'Withdraw',
  '2': const [
    const {'1': 'bank_account_id', '3': 1, '4': 1, '5': 9, '10': 'bankAccountId'},
  ],
};

/// Descriptor for `Withdraw`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List withdrawDescriptor = $convert.base64Decode('CghXaXRoZHJhdxImCg9iYW5rX2FjY291bnRfaWQYASABKAlSDWJhbmtBY2NvdW50SWQ=');
@$core.Deprecated('Use depositDescriptor instead')
const Deposit$json = const {
  '1': 'Deposit',
  '2': const [
    const {'1': 'bank_account_id', '3': 1, '4': 1, '5': 9, '10': 'bankAccountId'},
  ],
};

/// Descriptor for `Deposit`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List depositDescriptor = $convert.base64Decode('CgdEZXBvc2l0EiYKD2JhbmtfYWNjb3VudF9pZBgBIAEoCVINYmFua0FjY291bnRJZA==');
@$core.Deprecated('Use contractDescriptor instead')
const Contract$json = const {
  '1': 'Contract',
  '2': const [
    const {'1': 'transactions', '3': 1, '4': 1, '5': 12, '10': 'transactions'},
    const {'1': 'endorsements', '3': 2, '4': 3, '5': 11, '6': '.m10.sdk.metadata.Endorsement', '10': 'endorsements'},
  ],
};

/// Descriptor for `Contract`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List contractDescriptor = $convert.base64Decode('CghDb250cmFjdBIiCgx0cmFuc2FjdGlvbnMYASABKAxSDHRyYW5zYWN0aW9ucxJBCgxlbmRvcnNlbWVudHMYAiADKAsyHS5tMTAuc2RrLm1ldGFkYXRhLkVuZG9yc2VtZW50UgxlbmRvcnNlbWVudHM=');
@$core.Deprecated('Use endorsementDescriptor instead')
const Endorsement$json = const {
  '1': 'Endorsement',
  '2': const [
    const {'1': 'ledger_id', '3': 1, '4': 1, '5': 9, '10': 'ledgerId'},
    const {'1': 'signature', '3': 2, '4': 1, '5': 11, '6': '.m10.sdk.Signature', '10': 'signature'},
  ],
};

/// Descriptor for `Endorsement`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List endorsementDescriptor = $convert.base64Decode('CgtFbmRvcnNlbWVudBIbCglsZWRnZXJfaWQYASABKAlSCGxlZGdlcklkEjAKCXNpZ25hdHVyZRgCIAEoCzISLm0xMC5zZGsuU2lnbmF0dXJlUglzaWduYXR1cmU=');
@$core.Deprecated('Use paymentRequestDescriptor instead')
const PaymentRequest$json = const {
  '1': 'PaymentRequest',
  '2': const [
    const {'1': 'transfer', '3': 1, '4': 1, '5': 11, '6': '.m10.sdk.transaction.CreateTransfer', '10': 'transfer'},
    const {'1': 'status', '3': 2, '4': 1, '5': 14, '6': '.m10.sdk.metadata.PaymentRequest.PaymentRequestStatus', '10': 'status'},
  ],
  '4': const [PaymentRequest_PaymentRequestStatus$json],
};

@$core.Deprecated('Use paymentRequestDescriptor instead')
const PaymentRequest_PaymentRequestStatus$json = const {
  '1': 'PaymentRequestStatus',
  '2': const [
    const {'1': 'PENDING', '2': 0},
    const {'1': 'DECLINED', '2': 1},
    const {'1': 'CANCELED', '2': 2},
    const {'1': 'IN_PROGRESS', '2': 3},
  ],
};

/// Descriptor for `PaymentRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List paymentRequestDescriptor = $convert.base64Decode('Cg5QYXltZW50UmVxdWVzdBI/Cgh0cmFuc2ZlchgBIAEoCzIjLm0xMC5zZGsudHJhbnNhY3Rpb24uQ3JlYXRlVHJhbnNmZXJSCHRyYW5zZmVyEk0KBnN0YXR1cxgCIAEoDjI1Lm0xMC5zZGsubWV0YWRhdGEuUGF5bWVudFJlcXVlc3QuUGF5bWVudFJlcXVlc3RTdGF0dXNSBnN0YXR1cyJQChRQYXltZW50UmVxdWVzdFN0YXR1cxILCgdQRU5ESU5HEAASDAoIREVDTElORUQQARIMCghDQU5DRUxFRBACEg8KC0lOX1BST0dSRVNTEAM=');
@$core.Deprecated('Use quoteRequestDescriptor instead')
const QuoteRequest$json = const {
  '1': 'QuoteRequest',
  '2': const [
    const {'1': 'base', '3': 1, '4': 1, '5': 11, '6': '.m10.sdk.metadata.AccountCurrency', '10': 'base'},
    const {'1': 'target', '3': 2, '4': 1, '5': 11, '6': '.m10.sdk.metadata.AccountCurrency', '10': 'target'},
    const {'1': 'memo', '3': 3, '4': 1, '5': 9, '10': 'memo'},
  ],
};

/// Descriptor for `QuoteRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List quoteRequestDescriptor = $convert.base64Decode('CgxRdW90ZVJlcXVlc3QSNQoEYmFzZRgBIAEoCzIhLm0xMC5zZGsubWV0YWRhdGEuQWNjb3VudEN1cnJlbmN5UgRiYXNlEjkKBnRhcmdldBgCIAEoCzIhLm0xMC5zZGsubWV0YWRhdGEuQWNjb3VudEN1cnJlbmN5UgZ0YXJnZXQSEgoEbWVtbxgDIAEoCVIEbWVtbw==');
@$core.Deprecated('Use accountCurrencyDescriptor instead')
const AccountCurrency$json = const {
  '1': 'AccountCurrency',
  '2': const [
    const {'1': 'operator', '3': 1, '4': 1, '5': 9, '10': 'operator'},
    const {'1': 'currency', '3': 2, '4': 1, '5': 9, '10': 'currency'},
    const {'1': 'account_id', '3': 3, '4': 1, '5': 12, '10': 'accountId'},
    const {'1': 'amount', '3': 4, '4': 1, '5': 4, '10': 'amount'},
  ],
};

/// Descriptor for `AccountCurrency`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List accountCurrencyDescriptor = $convert.base64Decode('Cg9BY2NvdW50Q3VycmVuY3kSGgoIb3BlcmF0b3IYASABKAlSCG9wZXJhdG9yEhoKCGN1cnJlbmN5GAIgASgJUghjdXJyZW5jeRIdCgphY2NvdW50X2lkGAMgASgMUglhY2NvdW50SWQSFgoGYW1vdW50GAQgASgEUgZhbW91bnQ=');
@$core.Deprecated('Use quoteEventDescriptor instead')
const QuoteEvent$json = const {
  '1': 'QuoteEvent',
  '2': const [
    const {'1': 'request', '3': 1, '4': 1, '5': 11, '6': '.m10.sdk.metadata.QuoteRequest', '9': 0, '10': 'request'},
    const {'1': 'proposal', '3': 2, '4': 1, '5': 11, '6': '.m10.sdk.metadata.Contract', '9': 0, '10': 'proposal'},
  ],
  '8': const [
    const {'1': 'event'},
  ],
};

/// Descriptor for `QuoteEvent`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List quoteEventDescriptor = $convert.base64Decode('CgpRdW90ZUV2ZW50EjoKB3JlcXVlc3QYASABKAsyHi5tMTAuc2RrLm1ldGFkYXRhLlF1b3RlUmVxdWVzdEgAUgdyZXF1ZXN0EjgKCHByb3Bvc2FsGAIgASgLMhoubTEwLnNkay5tZXRhZGF0YS5Db250cmFjdEgAUghwcm9wb3NhbEIHCgVldmVudA==');
@$core.Deprecated('Use selfTransferDescriptor instead')
const SelfTransfer$json = const {
  '1': 'SelfTransfer',
  '2': const [
    const {'1': 'from_account_name', '3': 1, '4': 1, '5': 9, '10': 'fromAccountName'},
    const {'1': 'to_account_name', '3': 2, '4': 1, '5': 9, '10': 'toAccountName'},
  ],
};

/// Descriptor for `SelfTransfer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List selfTransferDescriptor = $convert.base64Decode('CgxTZWxmVHJhbnNmZXISKgoRZnJvbV9hY2NvdW50X25hbWUYASABKAlSD2Zyb21BY2NvdW50TmFtZRImCg90b19hY2NvdW50X25hbWUYAiABKAlSDXRvQWNjb3VudE5hbWU=');
@$core.Deprecated('Use rebalanceTransferDescriptor instead')
const RebalanceTransfer$json = const {
  '1': 'RebalanceTransfer',
};

/// Descriptor for `RebalanceTransfer`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List rebalanceTransferDescriptor = $convert.base64Decode('ChFSZWJhbGFuY2VUcmFuc2Zlcg==');
