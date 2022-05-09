///
//  Generated code. Do not modify.
//  source: sdk/transaction/transaction.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class TransactionError_Code extends $pb.ProtobufEnum {
  static const TransactionError_Code UNKNOWN = TransactionError_Code._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'UNKNOWN');
  static const TransactionError_Code UNIMPLEMENTED = TransactionError_Code._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'UNIMPLEMENTED');
  static const TransactionError_Code NOT_FOUND = TransactionError_Code._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'NOT_FOUND');
  static const TransactionError_Code ALREADY_EXISTS = TransactionError_Code._(3, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'ALREADY_EXISTS');
  static const TransactionError_Code UNAUTHORIZED = TransactionError_Code._(4, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'UNAUTHORIZED');
  static const TransactionError_Code BAD_REQUEST = TransactionError_Code._(5, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'BAD_REQUEST');
  static const TransactionError_Code INVALID_REQUEST_TYPE = TransactionError_Code._(6, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'INVALID_REQUEST_TYPE');
  static const TransactionError_Code INVALID_ACCOUNT_ID = TransactionError_Code._(7, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'INVALID_ACCOUNT_ID');
  static const TransactionError_Code INVALID_TRANSFER = TransactionError_Code._(8, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'INVALID_TRANSFER');
  static const TransactionError_Code MESSAGE_TOO_LARGE = TransactionError_Code._(10, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'MESSAGE_TOO_LARGE');
  static const TransactionError_Code INVALID_SIGNATURE = TransactionError_Code._(11, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'INVALID_SIGNATURE');
  static const TransactionError_Code VERIFICATION_FAILED = TransactionError_Code._(12, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'VERIFICATION_FAILED');
  static const TransactionError_Code REPLAY_PROTECTION = TransactionError_Code._(20, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'REPLAY_PROTECTION');
  static const TransactionError_Code INVALID_EXPRESSION = TransactionError_Code._(21, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'INVALID_EXPRESSION');
  static const TransactionError_Code INCORRECT_TYPE = TransactionError_Code._(22, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'INCORRECT_TYPE');
  static const TransactionError_Code ACCOUNT_FROZEN = TransactionError_Code._(23, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'ACCOUNT_FROZEN');
  static const TransactionError_Code UNMODIFIED_STATE = TransactionError_Code._(24, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'UNMODIFIED_STATE');
  static const TransactionError_Code INSUFFICIENT_BALANCE = TransactionError_Code._(25, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'INSUFFICIENT_BALANCE');
  static const TransactionError_Code BALANCE_OVERFLOW = TransactionError_Code._(26, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'BALANCE_OVERFLOW');
  static const TransactionError_Code ACCOUNT_DEPTH_EXCEEDED = TransactionError_Code._(27, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'ACCOUNT_DEPTH_EXCEEDED');
  static const TransactionError_Code INVALID_TARGET = TransactionError_Code._(30, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'INVALID_TARGET');

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
    INVALID_TARGET,
  ];

  static final $core.Map<$core.int, TransactionError_Code> _byValue = $pb.ProtobufEnum.initByValue(values);
  static TransactionError_Code? valueOf($core.int value) => _byValue[value];

  const TransactionError_Code._($core.int v, $core.String n) : super(v, n);
}

class FinalizedTransfer_TransferState extends $pb.ProtobufEnum {
  static const FinalizedTransfer_TransferState ACCEPTED = FinalizedTransfer_TransferState._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'ACCEPTED');
  static const FinalizedTransfer_TransferState REJECTED = FinalizedTransfer_TransferState._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'REJECTED');
  static const FinalizedTransfer_TransferState PENDING = FinalizedTransfer_TransferState._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'PENDING');
  static const FinalizedTransfer_TransferState EXPIRED = FinalizedTransfer_TransferState._(3, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'EXPIRED');

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
  static const CommitTransfer_TransferState ACCEPTED = CommitTransfer_TransferState._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'ACCEPTED');
  static const CommitTransfer_TransferState REJECTED = CommitTransfer_TransferState._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'REJECTED');

  static const $core.List<CommitTransfer_TransferState> values = <CommitTransfer_TransferState> [
    ACCEPTED,
    REJECTED,
  ];

  static final $core.Map<$core.int, CommitTransfer_TransferState> _byValue = $pb.ProtobufEnum.initByValue(values);
  static CommitTransfer_TransferState? valueOf($core.int value) => _byValue[value];

  const CommitTransfer_TransferState._($core.int v, $core.String n) : super(v, n);
}

