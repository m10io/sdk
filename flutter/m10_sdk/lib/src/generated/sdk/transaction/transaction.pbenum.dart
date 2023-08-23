//
//  Generated code. Do not modify.
//  source: sdk/transaction/transaction.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class TransactionError_Code extends $pb.ProtobufEnum {
  static const TransactionError_Code UNKNOWN = TransactionError_Code._(0, _omitEnumNames ? '' : 'UNKNOWN');
  static const TransactionError_Code UNIMPLEMENTED = TransactionError_Code._(1, _omitEnumNames ? '' : 'UNIMPLEMENTED');
  static const TransactionError_Code NOT_FOUND = TransactionError_Code._(2, _omitEnumNames ? '' : 'NOT_FOUND');
  static const TransactionError_Code ALREADY_EXISTS = TransactionError_Code._(3, _omitEnumNames ? '' : 'ALREADY_EXISTS');
  static const TransactionError_Code UNAUTHORIZED = TransactionError_Code._(4, _omitEnumNames ? '' : 'UNAUTHORIZED');
  static const TransactionError_Code BAD_REQUEST = TransactionError_Code._(5, _omitEnumNames ? '' : 'BAD_REQUEST');
  static const TransactionError_Code INVALID_REQUEST_TYPE = TransactionError_Code._(6, _omitEnumNames ? '' : 'INVALID_REQUEST_TYPE');
  static const TransactionError_Code INVALID_ACCOUNT_ID = TransactionError_Code._(7, _omitEnumNames ? '' : 'INVALID_ACCOUNT_ID');
  static const TransactionError_Code INVALID_TRANSFER = TransactionError_Code._(8, _omitEnumNames ? '' : 'INVALID_TRANSFER');
  static const TransactionError_Code MESSAGE_TOO_LARGE = TransactionError_Code._(10, _omitEnumNames ? '' : 'MESSAGE_TOO_LARGE');
  static const TransactionError_Code INVALID_SIGNATURE = TransactionError_Code._(11, _omitEnumNames ? '' : 'INVALID_SIGNATURE');
  static const TransactionError_Code VERIFICATION_FAILED = TransactionError_Code._(12, _omitEnumNames ? '' : 'VERIFICATION_FAILED');
  static const TransactionError_Code REPLAY_PROTECTION = TransactionError_Code._(20, _omitEnumNames ? '' : 'REPLAY_PROTECTION');
  static const TransactionError_Code INVALID_EXPRESSION = TransactionError_Code._(21, _omitEnumNames ? '' : 'INVALID_EXPRESSION');
  static const TransactionError_Code INCORRECT_TYPE = TransactionError_Code._(22, _omitEnumNames ? '' : 'INCORRECT_TYPE');
  static const TransactionError_Code ACCOUNT_FROZEN = TransactionError_Code._(23, _omitEnumNames ? '' : 'ACCOUNT_FROZEN');
  static const TransactionError_Code UNMODIFIED_STATE = TransactionError_Code._(24, _omitEnumNames ? '' : 'UNMODIFIED_STATE');
  static const TransactionError_Code INSUFFICIENT_BALANCE = TransactionError_Code._(25, _omitEnumNames ? '' : 'INSUFFICIENT_BALANCE');
  static const TransactionError_Code BALANCE_OVERFLOW = TransactionError_Code._(26, _omitEnumNames ? '' : 'BALANCE_OVERFLOW');
  static const TransactionError_Code ACCOUNT_DEPTH_EXCEEDED = TransactionError_Code._(27, _omitEnumNames ? '' : 'ACCOUNT_DEPTH_EXCEEDED');
  static const TransactionError_Code HOLDING_LIMIT_EXCEEDED = TransactionError_Code._(28, _omitEnumNames ? '' : 'HOLDING_LIMIT_EXCEEDED');
  static const TransactionError_Code INVALID_TARGET = TransactionError_Code._(30, _omitEnumNames ? '' : 'INVALID_TARGET');

  static const $core.List<TransactionError_Code> values = <TransactionError_Code> [
    UNKNOWN,
    UNIMPLEMENTED,
    NOT_FOUND,
    ALREADY_EXISTS,
    UNAUTHORIZED,
    BAD_REQUEST,
    INVALID_REQUEST_TYPE,
    INVALID_ACCOUNT_ID,
    INVALID_TRANSFER,
    MESSAGE_TOO_LARGE,
    INVALID_SIGNATURE,
    VERIFICATION_FAILED,
    REPLAY_PROTECTION,
    INVALID_EXPRESSION,
    INCORRECT_TYPE,
    ACCOUNT_FROZEN,
    UNMODIFIED_STATE,
    INSUFFICIENT_BALANCE,
    BALANCE_OVERFLOW,
    ACCOUNT_DEPTH_EXCEEDED,
    HOLDING_LIMIT_EXCEEDED,
    INVALID_TARGET,
  ];

  static final $core.Map<$core.int, TransactionError_Code> _byValue = $pb.ProtobufEnum.initByValue(values);
  static TransactionError_Code? valueOf($core.int value) => _byValue[value];

  const TransactionError_Code._($core.int v, $core.String n) : super(v, n);
}

class FinalizedTransfer_TransferState extends $pb.ProtobufEnum {
  static const FinalizedTransfer_TransferState ACCEPTED = FinalizedTransfer_TransferState._(0, _omitEnumNames ? '' : 'ACCEPTED');
  static const FinalizedTransfer_TransferState REJECTED = FinalizedTransfer_TransferState._(1, _omitEnumNames ? '' : 'REJECTED');
  static const FinalizedTransfer_TransferState PENDING = FinalizedTransfer_TransferState._(2, _omitEnumNames ? '' : 'PENDING');
  static const FinalizedTransfer_TransferState EXPIRED = FinalizedTransfer_TransferState._(3, _omitEnumNames ? '' : 'EXPIRED');

  static const $core.List<FinalizedTransfer_TransferState> values = <FinalizedTransfer_TransferState> [
    ACCEPTED,
    REJECTED,
    PENDING,
    EXPIRED,
  ];

  static final $core.Map<$core.int, FinalizedTransfer_TransferState> _byValue = $pb.ProtobufEnum.initByValue(values);
  static FinalizedTransfer_TransferState? valueOf($core.int value) => _byValue[value];

  const FinalizedTransfer_TransferState._($core.int v, $core.String n) : super(v, n);
}

class CommitTransfer_TransferState extends $pb.ProtobufEnum {
  static const CommitTransfer_TransferState ACCEPTED = CommitTransfer_TransferState._(0, _omitEnumNames ? '' : 'ACCEPTED');
  static const CommitTransfer_TransferState REJECTED = CommitTransfer_TransferState._(1, _omitEnumNames ? '' : 'REJECTED');

  static const $core.List<CommitTransfer_TransferState> values = <CommitTransfer_TransferState> [
    ACCEPTED,
    REJECTED,
  ];

  static final $core.Map<$core.int, CommitTransfer_TransferState> _byValue = $pb.ProtobufEnum.initByValue(values);
  static CommitTransfer_TransferState? valueOf($core.int value) => _byValue[value];

  const CommitTransfer_TransferState._($core.int v, $core.String n) : super(v, n);
}

class Signature_Algorithm extends $pb.ProtobufEnum {
  static const Signature_Algorithm P256_SHA256_ASN1 = Signature_Algorithm._(0, _omitEnumNames ? '' : 'P256_SHA256_ASN1');
  static const Signature_Algorithm ED25519 = Signature_Algorithm._(1, _omitEnumNames ? '' : 'ED25519');

  static const $core.List<Signature_Algorithm> values = <Signature_Algorithm> [
    P256_SHA256_ASN1,
    ED25519,
  ];

  static final $core.Map<$core.int, Signature_Algorithm> _byValue = $pb.ProtobufEnum.initByValue(values);
  static Signature_Algorithm? valueOf($core.int value) => _byValue[value];

  const Signature_Algorithm._($core.int v, $core.String n) : super(v, n);
}


const _omitEnumNames = $core.bool.fromEnvironment('protobuf.omit_enum_names');
