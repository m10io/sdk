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

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import '../../google/protobuf/any.pb.dart' as $1;
import '../../google/protobuf/empty.pb.dart' as $2;
import '../document.pb.dart' as $0;
import 'transaction.pbenum.dart';

export 'transaction.pbenum.dart';

class TransactionRequestPayload extends $pb.GeneratedMessage {
  factory TransactionRequestPayload() => create();
  TransactionRequestPayload._() : super();
  factory TransactionRequestPayload.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TransactionRequestPayload.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'TransactionRequestPayload', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'nonce', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(2, _omitFieldNames ? '' : 'timestamp', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(5, _omitFieldNames ? '' : 'contextId', $pb.PbFieldType.OY)
    ..aOM<TransactionData>(6, _omitFieldNames ? '' : 'data', subBuilder: TransactionData.create)
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TransactionRequestPayload clone() => TransactionRequestPayload()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TransactionRequestPayload copyWith(void Function(TransactionRequestPayload) updates) => super.copyWith((message) => updates(message as TransactionRequestPayload)) as TransactionRequestPayload;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TransactionRequestPayload create() => TransactionRequestPayload._();
  TransactionRequestPayload createEmptyInstance() => create();
  static $pb.PbList<TransactionRequestPayload> createRepeated() => $pb.PbList<TransactionRequestPayload>();
  @$core.pragma('dart2js:noInline')
  static TransactionRequestPayload getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TransactionRequestPayload>(create);
  static TransactionRequestPayload? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get nonce => $_getI64(0);
  @$pb.TagNumber(1)
  set nonce($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasNonce() => $_has(0);
  @$pb.TagNumber(1)
  void clearNonce() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get timestamp => $_getI64(1);
  @$pb.TagNumber(2)
  set timestamp($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasTimestamp() => $_has(1);
  @$pb.TagNumber(2)
  void clearTimestamp() => clearField(2);

  @$pb.TagNumber(5)
  $core.List<$core.int> get contextId => $_getN(2);
  @$pb.TagNumber(5)
  set contextId($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(5)
  $core.bool hasContextId() => $_has(2);
  @$pb.TagNumber(5)
  void clearContextId() => clearField(5);

  @$pb.TagNumber(6)
  TransactionData get data => $_getN(3);
  @$pb.TagNumber(6)
  set data(TransactionData v) { setField(6, v); }
  @$pb.TagNumber(6)
  $core.bool hasData() => $_has(3);
  @$pb.TagNumber(6)
  void clearData() => clearField(6);
  @$pb.TagNumber(6)
  TransactionData ensureData() => $_ensure(3);
}

enum TransactionData_Data {
  transfer, 
  createLedgerAccount, 
  setFreezeState, 
  documentOperations, 
  invokeAction, 
  initiateTransfer, 
  commitTransfer, 
  setInstrument, 
  setBalanceLimit, 
  createToken, 
  redeemToken, 
  notSet
}

class TransactionData extends $pb.GeneratedMessage {
  factory TransactionData() => create();
  TransactionData._() : super();
  factory TransactionData.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TransactionData.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, TransactionData_Data> _TransactionData_DataByTag = {
    10 : TransactionData_Data.transfer,
    11 : TransactionData_Data.createLedgerAccount,
    12 : TransactionData_Data.setFreezeState,
    16 : TransactionData_Data.documentOperations,
    20 : TransactionData_Data.invokeAction,
    21 : TransactionData_Data.initiateTransfer,
    22 : TransactionData_Data.commitTransfer,
    23 : TransactionData_Data.setInstrument,
    24 : TransactionData_Data.setBalanceLimit,
    100 : TransactionData_Data.createToken,
    101 : TransactionData_Data.redeemToken,
    0 : TransactionData_Data.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'TransactionData', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..oo(0, [10, 11, 12, 16, 20, 21, 22, 23, 24, 100, 101])
    ..aOM<CreateTransfer>(10, _omitFieldNames ? '' : 'transfer', subBuilder: CreateTransfer.create)
    ..aOM<CreateLedgerAccount>(11, _omitFieldNames ? '' : 'createLedgerAccount', subBuilder: CreateLedgerAccount.create)
    ..aOM<SetFreezeState>(12, _omitFieldNames ? '' : 'setFreezeState', subBuilder: SetFreezeState.create)
    ..aOM<$0.DocumentOperations>(16, _omitFieldNames ? '' : 'documentOperations', subBuilder: $0.DocumentOperations.create)
    ..aOM<InvokeAction>(20, _omitFieldNames ? '' : 'invokeAction', subBuilder: InvokeAction.create)
    ..aOM<CreateTransfer>(21, _omitFieldNames ? '' : 'initiateTransfer', subBuilder: CreateTransfer.create)
    ..aOM<CommitTransfer>(22, _omitFieldNames ? '' : 'commitTransfer', subBuilder: CommitTransfer.create)
    ..aOM<SetInstrument>(23, _omitFieldNames ? '' : 'setInstrument', subBuilder: SetInstrument.create)
    ..aOM<SetBalanceLimit>(24, _omitFieldNames ? '' : 'setBalanceLimit', subBuilder: SetBalanceLimit.create)
    ..aOM<CreateToken>(100, _omitFieldNames ? '' : 'createToken', subBuilder: CreateToken.create)
    ..aOM<RedeemToken>(101, _omitFieldNames ? '' : 'redeemToken', subBuilder: RedeemToken.create)
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TransactionData clone() => TransactionData()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TransactionData copyWith(void Function(TransactionData) updates) => super.copyWith((message) => updates(message as TransactionData)) as TransactionData;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TransactionData create() => TransactionData._();
  TransactionData createEmptyInstance() => create();
  static $pb.PbList<TransactionData> createRepeated() => $pb.PbList<TransactionData>();
  @$core.pragma('dart2js:noInline')
  static TransactionData getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TransactionData>(create);
  static TransactionData? _defaultInstance;

  TransactionData_Data whichData() => _TransactionData_DataByTag[$_whichOneof(0)]!;
  void clearData() => clearField($_whichOneof(0));

  @$pb.TagNumber(10)
  CreateTransfer get transfer => $_getN(0);
  @$pb.TagNumber(10)
  set transfer(CreateTransfer v) { setField(10, v); }
  @$pb.TagNumber(10)
  $core.bool hasTransfer() => $_has(0);
  @$pb.TagNumber(10)
  void clearTransfer() => clearField(10);
  @$pb.TagNumber(10)
  CreateTransfer ensureTransfer() => $_ensure(0);

  @$pb.TagNumber(11)
  CreateLedgerAccount get createLedgerAccount => $_getN(1);
  @$pb.TagNumber(11)
  set createLedgerAccount(CreateLedgerAccount v) { setField(11, v); }
  @$pb.TagNumber(11)
  $core.bool hasCreateLedgerAccount() => $_has(1);
  @$pb.TagNumber(11)
  void clearCreateLedgerAccount() => clearField(11);
  @$pb.TagNumber(11)
  CreateLedgerAccount ensureCreateLedgerAccount() => $_ensure(1);

  @$pb.TagNumber(12)
  SetFreezeState get setFreezeState => $_getN(2);
  @$pb.TagNumber(12)
  set setFreezeState(SetFreezeState v) { setField(12, v); }
  @$pb.TagNumber(12)
  $core.bool hasSetFreezeState() => $_has(2);
  @$pb.TagNumber(12)
  void clearSetFreezeState() => clearField(12);
  @$pb.TagNumber(12)
  SetFreezeState ensureSetFreezeState() => $_ensure(2);

  @$pb.TagNumber(16)
  $0.DocumentOperations get documentOperations => $_getN(3);
  @$pb.TagNumber(16)
  set documentOperations($0.DocumentOperations v) { setField(16, v); }
  @$pb.TagNumber(16)
  $core.bool hasDocumentOperations() => $_has(3);
  @$pb.TagNumber(16)
  void clearDocumentOperations() => clearField(16);
  @$pb.TagNumber(16)
  $0.DocumentOperations ensureDocumentOperations() => $_ensure(3);

  @$pb.TagNumber(20)
  InvokeAction get invokeAction => $_getN(4);
  @$pb.TagNumber(20)
  set invokeAction(InvokeAction v) { setField(20, v); }
  @$pb.TagNumber(20)
  $core.bool hasInvokeAction() => $_has(4);
  @$pb.TagNumber(20)
  void clearInvokeAction() => clearField(20);
  @$pb.TagNumber(20)
  InvokeAction ensureInvokeAction() => $_ensure(4);

  @$pb.TagNumber(21)
  CreateTransfer get initiateTransfer => $_getN(5);
  @$pb.TagNumber(21)
  set initiateTransfer(CreateTransfer v) { setField(21, v); }
  @$pb.TagNumber(21)
  $core.bool hasInitiateTransfer() => $_has(5);
  @$pb.TagNumber(21)
  void clearInitiateTransfer() => clearField(21);
  @$pb.TagNumber(21)
  CreateTransfer ensureInitiateTransfer() => $_ensure(5);

  @$pb.TagNumber(22)
  CommitTransfer get commitTransfer => $_getN(6);
  @$pb.TagNumber(22)
  set commitTransfer(CommitTransfer v) { setField(22, v); }
  @$pb.TagNumber(22)
  $core.bool hasCommitTransfer() => $_has(6);
  @$pb.TagNumber(22)
  void clearCommitTransfer() => clearField(22);
  @$pb.TagNumber(22)
  CommitTransfer ensureCommitTransfer() => $_ensure(6);

  @$pb.TagNumber(23)
  SetInstrument get setInstrument => $_getN(7);
  @$pb.TagNumber(23)
  set setInstrument(SetInstrument v) { setField(23, v); }
  @$pb.TagNumber(23)
  $core.bool hasSetInstrument() => $_has(7);
  @$pb.TagNumber(23)
  void clearSetInstrument() => clearField(23);
  @$pb.TagNumber(23)
  SetInstrument ensureSetInstrument() => $_ensure(7);

  @$pb.TagNumber(24)
  SetBalanceLimit get setBalanceLimit => $_getN(8);
  @$pb.TagNumber(24)
  set setBalanceLimit(SetBalanceLimit v) { setField(24, v); }
  @$pb.TagNumber(24)
  $core.bool hasSetBalanceLimit() => $_has(8);
  @$pb.TagNumber(24)
  void clearSetBalanceLimit() => clearField(24);
  @$pb.TagNumber(24)
  SetBalanceLimit ensureSetBalanceLimit() => $_ensure(8);

  @$pb.TagNumber(100)
  CreateToken get createToken => $_getN(9);
  @$pb.TagNumber(100)
  set createToken(CreateToken v) { setField(100, v); }
  @$pb.TagNumber(100)
  $core.bool hasCreateToken() => $_has(9);
  @$pb.TagNumber(100)
  void clearCreateToken() => clearField(100);
  @$pb.TagNumber(100)
  CreateToken ensureCreateToken() => $_ensure(9);

  @$pb.TagNumber(101)
  RedeemToken get redeemToken => $_getN(10);
  @$pb.TagNumber(101)
  set redeemToken(RedeemToken v) { setField(101, v); }
  @$pb.TagNumber(101)
  $core.bool hasRedeemToken() => $_has(10);
  @$pb.TagNumber(101)
  void clearRedeemToken() => clearField(101);
  @$pb.TagNumber(101)
  RedeemToken ensureRedeemToken() => $_ensure(10);
}

class TransactionResponse extends $pb.GeneratedMessage {
  factory TransactionResponse() => create();
  TransactionResponse._() : super();
  factory TransactionResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TransactionResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'TransactionResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'txId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<TransactionError>(2, _omitFieldNames ? '' : 'error', subBuilder: TransactionError.create)
    ..a<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'accountCreated', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(4, _omitFieldNames ? '' : 'timestamp', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<CreateTransfer>(5, _omitFieldNames ? '' : 'transferCommitted', subBuilder: CreateTransfer.create)
    ..aOM<OfflineToken>(6, _omitFieldNames ? '' : 'token', subBuilder: OfflineToken.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TransactionResponse clone() => TransactionResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TransactionResponse copyWith(void Function(TransactionResponse) updates) => super.copyWith((message) => updates(message as TransactionResponse)) as TransactionResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TransactionResponse create() => TransactionResponse._();
  TransactionResponse createEmptyInstance() => create();
  static $pb.PbList<TransactionResponse> createRepeated() => $pb.PbList<TransactionResponse>();
  @$core.pragma('dart2js:noInline')
  static TransactionResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TransactionResponse>(create);
  static TransactionResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get txId => $_getI64(0);
  @$pb.TagNumber(1)
  set txId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTxId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTxId() => clearField(1);

  @$pb.TagNumber(2)
  TransactionError get error => $_getN(1);
  @$pb.TagNumber(2)
  set error(TransactionError v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasError() => $_has(1);
  @$pb.TagNumber(2)
  void clearError() => clearField(2);
  @$pb.TagNumber(2)
  TransactionError ensureError() => $_ensure(1);

  @$pb.TagNumber(3)
  $core.List<$core.int> get accountCreated => $_getN(2);
  @$pb.TagNumber(3)
  set accountCreated($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasAccountCreated() => $_has(2);
  @$pb.TagNumber(3)
  void clearAccountCreated() => clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get timestamp => $_getI64(3);
  @$pb.TagNumber(4)
  set timestamp($fixnum.Int64 v) { $_setInt64(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasTimestamp() => $_has(3);
  @$pb.TagNumber(4)
  void clearTimestamp() => clearField(4);

  @$pb.TagNumber(5)
  CreateTransfer get transferCommitted => $_getN(4);
  @$pb.TagNumber(5)
  set transferCommitted(CreateTransfer v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasTransferCommitted() => $_has(4);
  @$pb.TagNumber(5)
  void clearTransferCommitted() => clearField(5);
  @$pb.TagNumber(5)
  CreateTransfer ensureTransferCommitted() => $_ensure(4);

  @$pb.TagNumber(6)
  OfflineToken get token => $_getN(5);
  @$pb.TagNumber(6)
  set token(OfflineToken v) { setField(6, v); }
  @$pb.TagNumber(6)
  $core.bool hasToken() => $_has(5);
  @$pb.TagNumber(6)
  void clearToken() => clearField(6);
  @$pb.TagNumber(6)
  OfflineToken ensureToken() => $_ensure(5);
}

class TransactionError extends $pb.GeneratedMessage {
  factory TransactionError() => create();
  TransactionError._() : super();
  factory TransactionError.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TransactionError.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'TransactionError', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..e<TransactionError_Code>(1, _omitFieldNames ? '' : 'code', $pb.PbFieldType.OE, defaultOrMaker: TransactionError_Code.UNKNOWN, valueOf: TransactionError_Code.valueOf, enumValues: TransactionError_Code.values)
    ..aOS(2, _omitFieldNames ? '' : 'message')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TransactionError clone() => TransactionError()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TransactionError copyWith(void Function(TransactionError) updates) => super.copyWith((message) => updates(message as TransactionError)) as TransactionError;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TransactionError create() => TransactionError._();
  TransactionError createEmptyInstance() => create();
  static $pb.PbList<TransactionError> createRepeated() => $pb.PbList<TransactionError>();
  @$core.pragma('dart2js:noInline')
  static TransactionError getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TransactionError>(create);
  static TransactionError? _defaultInstance;

  @$pb.TagNumber(1)
  TransactionError_Code get code => $_getN(0);
  @$pb.TagNumber(1)
  set code(TransactionError_Code v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasCode() => $_has(0);
  @$pb.TagNumber(1)
  void clearCode() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get message => $_getSZ(1);
  @$pb.TagNumber(2)
  set message($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessage() => clearField(2);
}

class CreateLedgerTransfer extends $pb.GeneratedMessage {
  factory CreateLedgerTransfer() => create();
  CreateLedgerTransfer._() : super();
  factory CreateLedgerTransfer.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CreateLedgerTransfer.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'CreateLedgerTransfer', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'ledgerId')
    ..a<$fixnum.Int64>(2, _omitFieldNames ? '' : 'nonce', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<CreateTransfer>(3, _omitFieldNames ? '' : 'transfer', subBuilder: CreateTransfer.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CreateLedgerTransfer clone() => CreateLedgerTransfer()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CreateLedgerTransfer copyWith(void Function(CreateLedgerTransfer) updates) => super.copyWith((message) => updates(message as CreateLedgerTransfer)) as CreateLedgerTransfer;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CreateLedgerTransfer create() => CreateLedgerTransfer._();
  CreateLedgerTransfer createEmptyInstance() => create();
  static $pb.PbList<CreateLedgerTransfer> createRepeated() => $pb.PbList<CreateLedgerTransfer>();
  @$core.pragma('dart2js:noInline')
  static CreateLedgerTransfer getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CreateLedgerTransfer>(create);
  static CreateLedgerTransfer? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get ledgerId => $_getSZ(0);
  @$pb.TagNumber(1)
  set ledgerId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasLedgerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearLedgerId() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get nonce => $_getI64(1);
  @$pb.TagNumber(2)
  set nonce($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasNonce() => $_has(1);
  @$pb.TagNumber(2)
  void clearNonce() => clearField(2);

  @$pb.TagNumber(3)
  CreateTransfer get transfer => $_getN(2);
  @$pb.TagNumber(3)
  set transfer(CreateTransfer v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasTransfer() => $_has(2);
  @$pb.TagNumber(3)
  void clearTransfer() => clearField(3);
  @$pb.TagNumber(3)
  CreateTransfer ensureTransfer() => $_ensure(2);
}

class CreateLedgerTransfers extends $pb.GeneratedMessage {
  factory CreateLedgerTransfers() => create();
  CreateLedgerTransfers._() : super();
  factory CreateLedgerTransfers.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CreateLedgerTransfers.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'CreateLedgerTransfers', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..pc<CreateLedgerTransfer>(1, _omitFieldNames ? '' : 'transfers', $pb.PbFieldType.PM, subBuilder: CreateLedgerTransfer.create)
    ..a<$fixnum.Int64>(2, _omitFieldNames ? '' : 'validUntil', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CreateLedgerTransfers clone() => CreateLedgerTransfers()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CreateLedgerTransfers copyWith(void Function(CreateLedgerTransfers) updates) => super.copyWith((message) => updates(message as CreateLedgerTransfers)) as CreateLedgerTransfers;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CreateLedgerTransfers create() => CreateLedgerTransfers._();
  CreateLedgerTransfers createEmptyInstance() => create();
  static $pb.PbList<CreateLedgerTransfers> createRepeated() => $pb.PbList<CreateLedgerTransfers>();
  @$core.pragma('dart2js:noInline')
  static CreateLedgerTransfers getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CreateLedgerTransfers>(create);
  static CreateLedgerTransfers? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<CreateLedgerTransfer> get transfers => $_getList(0);

  @$pb.TagNumber(2)
  $fixnum.Int64 get validUntil => $_getI64(1);
  @$pb.TagNumber(2)
  set validUntil($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasValidUntil() => $_has(1);
  @$pb.TagNumber(2)
  void clearValidUntil() => clearField(2);
}

class GetTransferRequest extends $pb.GeneratedMessage {
  factory GetTransferRequest() => create();
  GetTransferRequest._() : super();
  factory GetTransferRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetTransferRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetTransferRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'txId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetTransferRequest clone() => GetTransferRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetTransferRequest copyWith(void Function(GetTransferRequest) updates) => super.copyWith((message) => updates(message as GetTransferRequest)) as GetTransferRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetTransferRequest create() => GetTransferRequest._();
  GetTransferRequest createEmptyInstance() => create();
  static $pb.PbList<GetTransferRequest> createRepeated() => $pb.PbList<GetTransferRequest>();
  @$core.pragma('dart2js:noInline')
  static GetTransferRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetTransferRequest>(create);
  static GetTransferRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get txId => $_getI64(0);
  @$pb.TagNumber(1)
  set txId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTxId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTxId() => clearField(1);
}

enum ListTransferRequest_Filter {
  accountId, 
  contextId, 
  notSet
}

class ListTransferRequest extends $pb.GeneratedMessage {
  factory ListTransferRequest() => create();
  ListTransferRequest._() : super();
  factory ListTransferRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListTransferRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, ListTransferRequest_Filter> _ListTransferRequest_FilterByTag = {
    1 : ListTransferRequest_Filter.accountId,
    2 : ListTransferRequest_Filter.contextId,
    0 : ListTransferRequest_Filter.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ListTransferRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'accountId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'contextId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(4, _omitFieldNames ? '' : 'limit', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOB(5, _omitFieldNames ? '' : 'includeChildAccounts')
    ..a<$fixnum.Int64>(6, _omitFieldNames ? '' : 'minTxId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(7, _omitFieldNames ? '' : 'maxTxId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListTransferRequest clone() => ListTransferRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListTransferRequest copyWith(void Function(ListTransferRequest) updates) => super.copyWith((message) => updates(message as ListTransferRequest)) as ListTransferRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListTransferRequest create() => ListTransferRequest._();
  ListTransferRequest createEmptyInstance() => create();
  static $pb.PbList<ListTransferRequest> createRepeated() => $pb.PbList<ListTransferRequest>();
  @$core.pragma('dart2js:noInline')
  static ListTransferRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListTransferRequest>(create);
  static ListTransferRequest? _defaultInstance;

  ListTransferRequest_Filter whichFilter() => _ListTransferRequest_FilterByTag[$_whichOneof(0)]!;
  void clearFilter() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.List<$core.int> get accountId => $_getN(0);
  @$pb.TagNumber(1)
  set accountId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearAccountId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get contextId => $_getN(1);
  @$pb.TagNumber(2)
  set contextId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasContextId() => $_has(1);
  @$pb.TagNumber(2)
  void clearContextId() => clearField(2);

  @$pb.TagNumber(4)
  $fixnum.Int64 get limit => $_getI64(2);
  @$pb.TagNumber(4)
  set limit($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(4)
  $core.bool hasLimit() => $_has(2);
  @$pb.TagNumber(4)
  void clearLimit() => clearField(4);

  @$pb.TagNumber(5)
  $core.bool get includeChildAccounts => $_getBF(3);
  @$pb.TagNumber(5)
  set includeChildAccounts($core.bool v) { $_setBool(3, v); }
  @$pb.TagNumber(5)
  $core.bool hasIncludeChildAccounts() => $_has(3);
  @$pb.TagNumber(5)
  void clearIncludeChildAccounts() => clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get minTxId => $_getI64(4);
  @$pb.TagNumber(6)
  set minTxId($fixnum.Int64 v) { $_setInt64(4, v); }
  @$pb.TagNumber(6)
  $core.bool hasMinTxId() => $_has(4);
  @$pb.TagNumber(6)
  void clearMinTxId() => clearField(6);

  @$pb.TagNumber(7)
  $fixnum.Int64 get maxTxId => $_getI64(5);
  @$pb.TagNumber(7)
  set maxTxId($fixnum.Int64 v) { $_setInt64(5, v); }
  @$pb.TagNumber(7)
  $core.bool hasMaxTxId() => $_has(5);
  @$pb.TagNumber(7)
  void clearMaxTxId() => clearField(7);
}

class CreateTransfer extends $pb.GeneratedMessage {
  factory CreateTransfer() => create();
  CreateTransfer._() : super();
  factory CreateTransfer.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CreateTransfer.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'CreateTransfer', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..pc<TransferStep>(1, _omitFieldNames ? '' : 'transferSteps', $pb.PbFieldType.PM, subBuilder: TransferStep.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CreateTransfer clone() => CreateTransfer()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CreateTransfer copyWith(void Function(CreateTransfer) updates) => super.copyWith((message) => updates(message as CreateTransfer)) as CreateTransfer;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CreateTransfer create() => CreateTransfer._();
  CreateTransfer createEmptyInstance() => create();
  static $pb.PbList<CreateTransfer> createRepeated() => $pb.PbList<CreateTransfer>();
  @$core.pragma('dart2js:noInline')
  static CreateTransfer getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CreateTransfer>(create);
  static CreateTransfer? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<TransferStep> get transferSteps => $_getList(0);
}

class TransferStep extends $pb.GeneratedMessage {
  factory TransferStep() => create();
  TransferStep._() : super();
  factory TransferStep.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TransferStep.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'TransferStep', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'fromAccountId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'toAccountId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(4, _omitFieldNames ? '' : 'amount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..pc<$1.Any>(7, _omitFieldNames ? '' : 'metadata', $pb.PbFieldType.PM, subBuilder: $1.Any.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TransferStep clone() => TransferStep()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TransferStep copyWith(void Function(TransferStep) updates) => super.copyWith((message) => updates(message as TransferStep)) as TransferStep;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TransferStep create() => TransferStep._();
  TransferStep createEmptyInstance() => create();
  static $pb.PbList<TransferStep> createRepeated() => $pb.PbList<TransferStep>();
  @$core.pragma('dart2js:noInline')
  static TransferStep getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TransferStep>(create);
  static TransferStep? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get fromAccountId => $_getN(0);
  @$pb.TagNumber(1)
  set fromAccountId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasFromAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearFromAccountId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get toAccountId => $_getN(1);
  @$pb.TagNumber(2)
  set toAccountId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasToAccountId() => $_has(1);
  @$pb.TagNumber(2)
  void clearToAccountId() => clearField(2);

  @$pb.TagNumber(4)
  $fixnum.Int64 get amount => $_getI64(2);
  @$pb.TagNumber(4)
  set amount($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(4)
  $core.bool hasAmount() => $_has(2);
  @$pb.TagNumber(4)
  void clearAmount() => clearField(4);

  @$pb.TagNumber(7)
  $core.List<$1.Any> get metadata => $_getList(3);
}

class FinalizedTransfer extends $pb.GeneratedMessage {
  factory FinalizedTransfer() => create();
  FinalizedTransfer._() : super();
  factory FinalizedTransfer.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FinalizedTransfer.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'FinalizedTransfer', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'txId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'contextId', $pb.PbFieldType.OY)
    ..pc<TransferStep>(3, _omitFieldNames ? '' : 'transferSteps', $pb.PbFieldType.PM, subBuilder: TransferStep.create)
    ..aOM<TransactionError>(4, _omitFieldNames ? '' : 'error', subBuilder: TransactionError.create)
    ..a<$fixnum.Int64>(5, _omitFieldNames ? '' : 'timestamp', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..e<FinalizedTransfer_TransferState>(7, _omitFieldNames ? '' : 'state', $pb.PbFieldType.OE, defaultOrMaker: FinalizedTransfer_TransferState.ACCEPTED, valueOf: FinalizedTransfer_TransferState.valueOf, enumValues: FinalizedTransfer_TransferState.values)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FinalizedTransfer clone() => FinalizedTransfer()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FinalizedTransfer copyWith(void Function(FinalizedTransfer) updates) => super.copyWith((message) => updates(message as FinalizedTransfer)) as FinalizedTransfer;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FinalizedTransfer create() => FinalizedTransfer._();
  FinalizedTransfer createEmptyInstance() => create();
  static $pb.PbList<FinalizedTransfer> createRepeated() => $pb.PbList<FinalizedTransfer>();
  @$core.pragma('dart2js:noInline')
  static FinalizedTransfer getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FinalizedTransfer>(create);
  static FinalizedTransfer? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get txId => $_getI64(0);
  @$pb.TagNumber(1)
  set txId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTxId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTxId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get contextId => $_getN(1);
  @$pb.TagNumber(2)
  set contextId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasContextId() => $_has(1);
  @$pb.TagNumber(2)
  void clearContextId() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<TransferStep> get transferSteps => $_getList(2);

  @$pb.TagNumber(4)
  TransactionError get error => $_getN(3);
  @$pb.TagNumber(4)
  set error(TransactionError v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasError() => $_has(3);
  @$pb.TagNumber(4)
  void clearError() => clearField(4);
  @$pb.TagNumber(4)
  TransactionError ensureError() => $_ensure(3);

  @$pb.TagNumber(5)
  $fixnum.Int64 get timestamp => $_getI64(4);
  @$pb.TagNumber(5)
  set timestamp($fixnum.Int64 v) { $_setInt64(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasTimestamp() => $_has(4);
  @$pb.TagNumber(5)
  void clearTimestamp() => clearField(5);

  @$pb.TagNumber(7)
  FinalizedTransfer_TransferState get state => $_getN(5);
  @$pb.TagNumber(7)
  set state(FinalizedTransfer_TransferState v) { setField(7, v); }
  @$pb.TagNumber(7)
  $core.bool hasState() => $_has(5);
  @$pb.TagNumber(7)
  void clearState() => clearField(7);
}

class FinalizedTransfers extends $pb.GeneratedMessage {
  factory FinalizedTransfers() => create();
  FinalizedTransfers._() : super();
  factory FinalizedTransfers.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FinalizedTransfers.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'FinalizedTransfers', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..pc<FinalizedTransfer>(1, _omitFieldNames ? '' : 'transfers', $pb.PbFieldType.PM, subBuilder: FinalizedTransfer.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FinalizedTransfers clone() => FinalizedTransfers()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FinalizedTransfers copyWith(void Function(FinalizedTransfers) updates) => super.copyWith((message) => updates(message as FinalizedTransfers)) as FinalizedTransfers;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FinalizedTransfers create() => FinalizedTransfers._();
  FinalizedTransfers createEmptyInstance() => create();
  static $pb.PbList<FinalizedTransfers> createRepeated() => $pb.PbList<FinalizedTransfers>();
  @$core.pragma('dart2js:noInline')
  static FinalizedTransfers getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FinalizedTransfers>(create);
  static FinalizedTransfers? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<FinalizedTransfer> get transfers => $_getList(0);
}

class Instrument extends $pb.GeneratedMessage {
  factory Instrument() => create();
  Instrument._() : super();
  factory Instrument.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Instrument.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Instrument', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'code')
    ..a<$core.int>(2, _omitFieldNames ? '' : 'decimalPlaces', $pb.PbFieldType.OU3)
    ..aOS(3, _omitFieldNames ? '' : 'description')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Instrument clone() => Instrument()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Instrument copyWith(void Function(Instrument) updates) => super.copyWith((message) => updates(message as Instrument)) as Instrument;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Instrument create() => Instrument._();
  Instrument createEmptyInstance() => create();
  static $pb.PbList<Instrument> createRepeated() => $pb.PbList<Instrument>();
  @$core.pragma('dart2js:noInline')
  static Instrument getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Instrument>(create);
  static Instrument? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get code => $_getSZ(0);
  @$pb.TagNumber(1)
  set code($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasCode() => $_has(0);
  @$pb.TagNumber(1)
  void clearCode() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get decimalPlaces => $_getIZ(1);
  @$pb.TagNumber(2)
  set decimalPlaces($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasDecimalPlaces() => $_has(1);
  @$pb.TagNumber(2)
  void clearDecimalPlaces() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get description => $_getSZ(2);
  @$pb.TagNumber(3)
  set description($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasDescription() => $_has(2);
  @$pb.TagNumber(3)
  void clearDescription() => clearField(3);
}

class CreateLedgerAccount extends $pb.GeneratedMessage {
  factory CreateLedgerAccount() => create();
  CreateLedgerAccount._() : super();
  factory CreateLedgerAccount.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CreateLedgerAccount.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'CreateLedgerAccount', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'parentId', $pb.PbFieldType.OY)
    ..aOB(2, _omitFieldNames ? '' : 'issuance')
    ..aOB(3, _omitFieldNames ? '' : 'frozen')
    ..aOM<Instrument>(4, _omitFieldNames ? '' : 'instrument', subBuilder: Instrument.create)
    ..a<$fixnum.Int64>(5, _omitFieldNames ? '' : 'balanceLimit', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CreateLedgerAccount clone() => CreateLedgerAccount()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CreateLedgerAccount copyWith(void Function(CreateLedgerAccount) updates) => super.copyWith((message) => updates(message as CreateLedgerAccount)) as CreateLedgerAccount;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CreateLedgerAccount create() => CreateLedgerAccount._();
  CreateLedgerAccount createEmptyInstance() => create();
  static $pb.PbList<CreateLedgerAccount> createRepeated() => $pb.PbList<CreateLedgerAccount>();
  @$core.pragma('dart2js:noInline')
  static CreateLedgerAccount getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CreateLedgerAccount>(create);
  static CreateLedgerAccount? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get parentId => $_getN(0);
  @$pb.TagNumber(1)
  set parentId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasParentId() => $_has(0);
  @$pb.TagNumber(1)
  void clearParentId() => clearField(1);

  @$pb.TagNumber(2)
  $core.bool get issuance => $_getBF(1);
  @$pb.TagNumber(2)
  set issuance($core.bool v) { $_setBool(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasIssuance() => $_has(1);
  @$pb.TagNumber(2)
  void clearIssuance() => clearField(2);

  @$pb.TagNumber(3)
  $core.bool get frozen => $_getBF(2);
  @$pb.TagNumber(3)
  set frozen($core.bool v) { $_setBool(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasFrozen() => $_has(2);
  @$pb.TagNumber(3)
  void clearFrozen() => clearField(3);

  @$pb.TagNumber(4)
  Instrument get instrument => $_getN(3);
  @$pb.TagNumber(4)
  set instrument(Instrument v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasInstrument() => $_has(3);
  @$pb.TagNumber(4)
  void clearInstrument() => clearField(4);
  @$pb.TagNumber(4)
  Instrument ensureInstrument() => $_ensure(3);

  @$pb.TagNumber(5)
  $fixnum.Int64 get balanceLimit => $_getI64(4);
  @$pb.TagNumber(5)
  set balanceLimit($fixnum.Int64 v) { $_setInt64(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasBalanceLimit() => $_has(4);
  @$pb.TagNumber(5)
  void clearBalanceLimit() => clearField(5);
}

class SetFreezeState extends $pb.GeneratedMessage {
  factory SetFreezeState() => create();
  SetFreezeState._() : super();
  factory SetFreezeState.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SetFreezeState.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'SetFreezeState', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'accountId', $pb.PbFieldType.OY)
    ..aOB(2, _omitFieldNames ? '' : 'frozen')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SetFreezeState clone() => SetFreezeState()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SetFreezeState copyWith(void Function(SetFreezeState) updates) => super.copyWith((message) => updates(message as SetFreezeState)) as SetFreezeState;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SetFreezeState create() => SetFreezeState._();
  SetFreezeState createEmptyInstance() => create();
  static $pb.PbList<SetFreezeState> createRepeated() => $pb.PbList<SetFreezeState>();
  @$core.pragma('dart2js:noInline')
  static SetFreezeState getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SetFreezeState>(create);
  static SetFreezeState? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get accountId => $_getN(0);
  @$pb.TagNumber(1)
  set accountId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearAccountId() => clearField(1);

  @$pb.TagNumber(2)
  $core.bool get frozen => $_getBF(1);
  @$pb.TagNumber(2)
  set frozen($core.bool v) { $_setBool(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasFrozen() => $_has(1);
  @$pb.TagNumber(2)
  void clearFrozen() => clearField(2);
}

class SetInstrument extends $pb.GeneratedMessage {
  factory SetInstrument() => create();
  SetInstrument._() : super();
  factory SetInstrument.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SetInstrument.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'SetInstrument', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'accountId', $pb.PbFieldType.OY)
    ..aOS(2, _omitFieldNames ? '' : 'code')
    ..a<$core.int>(3, _omitFieldNames ? '' : 'decimalPlaces', $pb.PbFieldType.OU3)
    ..aOS(4, _omitFieldNames ? '' : 'description')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SetInstrument clone() => SetInstrument()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SetInstrument copyWith(void Function(SetInstrument) updates) => super.copyWith((message) => updates(message as SetInstrument)) as SetInstrument;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SetInstrument create() => SetInstrument._();
  SetInstrument createEmptyInstance() => create();
  static $pb.PbList<SetInstrument> createRepeated() => $pb.PbList<SetInstrument>();
  @$core.pragma('dart2js:noInline')
  static SetInstrument getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SetInstrument>(create);
  static SetInstrument? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get accountId => $_getN(0);
  @$pb.TagNumber(1)
  set accountId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearAccountId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get code => $_getSZ(1);
  @$pb.TagNumber(2)
  set code($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasCode() => $_has(1);
  @$pb.TagNumber(2)
  void clearCode() => clearField(2);

  @$pb.TagNumber(3)
  $core.int get decimalPlaces => $_getIZ(2);
  @$pb.TagNumber(3)
  set decimalPlaces($core.int v) { $_setUnsignedInt32(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasDecimalPlaces() => $_has(2);
  @$pb.TagNumber(3)
  void clearDecimalPlaces() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get description => $_getSZ(3);
  @$pb.TagNumber(4)
  set description($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasDescription() => $_has(3);
  @$pb.TagNumber(4)
  void clearDescription() => clearField(4);
}

class SetBalanceLimit extends $pb.GeneratedMessage {
  factory SetBalanceLimit() => create();
  SetBalanceLimit._() : super();
  factory SetBalanceLimit.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SetBalanceLimit.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'SetBalanceLimit', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'accountId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(2, _omitFieldNames ? '' : 'balanceLimit', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SetBalanceLimit clone() => SetBalanceLimit()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SetBalanceLimit copyWith(void Function(SetBalanceLimit) updates) => super.copyWith((message) => updates(message as SetBalanceLimit)) as SetBalanceLimit;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SetBalanceLimit create() => SetBalanceLimit._();
  SetBalanceLimit createEmptyInstance() => create();
  static $pb.PbList<SetBalanceLimit> createRepeated() => $pb.PbList<SetBalanceLimit>();
  @$core.pragma('dart2js:noInline')
  static SetBalanceLimit getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SetBalanceLimit>(create);
  static SetBalanceLimit? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get accountId => $_getN(0);
  @$pb.TagNumber(1)
  set accountId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearAccountId() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get balanceLimit => $_getI64(1);
  @$pb.TagNumber(2)
  set balanceLimit($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasBalanceLimit() => $_has(1);
  @$pb.TagNumber(2)
  void clearBalanceLimit() => clearField(2);
}

class GetAccountRequest extends $pb.GeneratedMessage {
  factory GetAccountRequest() => create();
  GetAccountRequest._() : super();
  factory GetAccountRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetAccountRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetAccountRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetAccountRequest clone() => GetAccountRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetAccountRequest copyWith(void Function(GetAccountRequest) updates) => super.copyWith((message) => updates(message as GetAccountRequest)) as GetAccountRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetAccountRequest create() => GetAccountRequest._();
  GetAccountRequest createEmptyInstance() => create();
  static $pb.PbList<GetAccountRequest> createRepeated() => $pb.PbList<GetAccountRequest>();
  @$core.pragma('dart2js:noInline')
  static GetAccountRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetAccountRequest>(create);
  static GetAccountRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);
}

class IndexedAccount_Issuance extends $pb.GeneratedMessage {
  factory IndexedAccount_Issuance() => create();
  IndexedAccount_Issuance._() : super();
  factory IndexedAccount_Issuance.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory IndexedAccount_Issuance.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'IndexedAccount.Issuance', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'issuedBalance', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(2, _omitFieldNames ? '' : 'nonLeafChildren', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(3, _omitFieldNames ? '' : 'leafChildren', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  IndexedAccount_Issuance clone() => IndexedAccount_Issuance()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  IndexedAccount_Issuance copyWith(void Function(IndexedAccount_Issuance) updates) => super.copyWith((message) => updates(message as IndexedAccount_Issuance)) as IndexedAccount_Issuance;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static IndexedAccount_Issuance create() => IndexedAccount_Issuance._();
  IndexedAccount_Issuance createEmptyInstance() => create();
  static $pb.PbList<IndexedAccount_Issuance> createRepeated() => $pb.PbList<IndexedAccount_Issuance>();
  @$core.pragma('dart2js:noInline')
  static IndexedAccount_Issuance getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<IndexedAccount_Issuance>(create);
  static IndexedAccount_Issuance? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get issuedBalance => $_getI64(0);
  @$pb.TagNumber(1)
  set issuedBalance($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasIssuedBalance() => $_has(0);
  @$pb.TagNumber(1)
  void clearIssuedBalance() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get nonLeafChildren => $_getI64(1);
  @$pb.TagNumber(2)
  set nonLeafChildren($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasNonLeafChildren() => $_has(1);
  @$pb.TagNumber(2)
  void clearNonLeafChildren() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get leafChildren => $_getI64(2);
  @$pb.TagNumber(3)
  set leafChildren($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasLeafChildren() => $_has(2);
  @$pb.TagNumber(3)
  void clearLeafChildren() => clearField(3);
}

class IndexedAccount extends $pb.GeneratedMessage {
  factory IndexedAccount() => create();
  IndexedAccount._() : super();
  factory IndexedAccount.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory IndexedAccount.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'IndexedAccount', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..aOM<IndexedAccount_Issuance>(3, _omitFieldNames ? '' : 'issuance', subBuilder: IndexedAccount_Issuance.create)
    ..a<$fixnum.Int64>(4, _omitFieldNames ? '' : 'balance', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOB(5, _omitFieldNames ? '' : 'frozen')
    ..aOM<Instrument>(6, _omitFieldNames ? '' : 'instrument', subBuilder: Instrument.create)
    ..a<$fixnum.Int64>(7, _omitFieldNames ? '' : 'balanceLimit', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  IndexedAccount clone() => IndexedAccount()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  IndexedAccount copyWith(void Function(IndexedAccount) updates) => super.copyWith((message) => updates(message as IndexedAccount)) as IndexedAccount;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static IndexedAccount create() => IndexedAccount._();
  IndexedAccount createEmptyInstance() => create();
  static $pb.PbList<IndexedAccount> createRepeated() => $pb.PbList<IndexedAccount>();
  @$core.pragma('dart2js:noInline')
  static IndexedAccount getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<IndexedAccount>(create);
  static IndexedAccount? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(3)
  IndexedAccount_Issuance get issuance => $_getN(1);
  @$pb.TagNumber(3)
  set issuance(IndexedAccount_Issuance v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasIssuance() => $_has(1);
  @$pb.TagNumber(3)
  void clearIssuance() => clearField(3);
  @$pb.TagNumber(3)
  IndexedAccount_Issuance ensureIssuance() => $_ensure(1);

  @$pb.TagNumber(4)
  $fixnum.Int64 get balance => $_getI64(2);
  @$pb.TagNumber(4)
  set balance($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(4)
  $core.bool hasBalance() => $_has(2);
  @$pb.TagNumber(4)
  void clearBalance() => clearField(4);

  @$pb.TagNumber(5)
  $core.bool get frozen => $_getBF(3);
  @$pb.TagNumber(5)
  set frozen($core.bool v) { $_setBool(3, v); }
  @$pb.TagNumber(5)
  $core.bool hasFrozen() => $_has(3);
  @$pb.TagNumber(5)
  void clearFrozen() => clearField(5);

  @$pb.TagNumber(6)
  Instrument get instrument => $_getN(4);
  @$pb.TagNumber(6)
  set instrument(Instrument v) { setField(6, v); }
  @$pb.TagNumber(6)
  $core.bool hasInstrument() => $_has(4);
  @$pb.TagNumber(6)
  void clearInstrument() => clearField(6);
  @$pb.TagNumber(6)
  Instrument ensureInstrument() => $_ensure(4);

  @$pb.TagNumber(7)
  $fixnum.Int64 get balanceLimit => $_getI64(5);
  @$pb.TagNumber(7)
  set balanceLimit($fixnum.Int64 v) { $_setInt64(5, v); }
  @$pb.TagNumber(7)
  $core.bool hasBalanceLimit() => $_has(5);
  @$pb.TagNumber(7)
  void clearBalanceLimit() => clearField(7);
}

class InvokeAction extends $pb.GeneratedMessage {
  factory InvokeAction() => create();
  InvokeAction._() : super();
  factory InvokeAction.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory InvokeAction.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'InvokeAction', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..a<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'fromAccount', $pb.PbFieldType.OY)
    ..aOM<Target>(4, _omitFieldNames ? '' : 'target', subBuilder: Target.create)
    ..a<$core.List<$core.int>>(5, _omitFieldNames ? '' : 'payload', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  InvokeAction clone() => InvokeAction()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  InvokeAction copyWith(void Function(InvokeAction) updates) => super.copyWith((message) => updates(message as InvokeAction)) as InvokeAction;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static InvokeAction create() => InvokeAction._();
  InvokeAction createEmptyInstance() => create();
  static $pb.PbList<InvokeAction> createRepeated() => $pb.PbList<InvokeAction>();
  @$core.pragma('dart2js:noInline')
  static InvokeAction getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<InvokeAction>(create);
  static InvokeAction? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => clearField(1);

  @$pb.TagNumber(3)
  $core.List<$core.int> get fromAccount => $_getN(1);
  @$pb.TagNumber(3)
  set fromAccount($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(3)
  $core.bool hasFromAccount() => $_has(1);
  @$pb.TagNumber(3)
  void clearFromAccount() => clearField(3);

  @$pb.TagNumber(4)
  Target get target => $_getN(2);
  @$pb.TagNumber(4)
  set target(Target v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasTarget() => $_has(2);
  @$pb.TagNumber(4)
  void clearTarget() => clearField(4);
  @$pb.TagNumber(4)
  Target ensureTarget() => $_ensure(2);

  @$pb.TagNumber(5)
  $core.List<$core.int> get payload => $_getN(3);
  @$pb.TagNumber(5)
  set payload($core.List<$core.int> v) { $_setBytes(3, v); }
  @$pb.TagNumber(5)
  $core.bool hasPayload() => $_has(3);
  @$pb.TagNumber(5)
  void clearPayload() => clearField(5);
}

enum Target_Target {
  accountId, 
  anyAccount, 
  notSet
}

class Target extends $pb.GeneratedMessage {
  factory Target() => create();
  Target._() : super();
  factory Target.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Target.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, Target_Target> _Target_TargetByTag = {
    1 : Target_Target.accountId,
    2 : Target_Target.anyAccount,
    0 : Target_Target.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Target', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'accountId', $pb.PbFieldType.OY)
    ..aOM<$2.Empty>(2, _omitFieldNames ? '' : 'anyAccount', subBuilder: $2.Empty.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Target clone() => Target()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Target copyWith(void Function(Target) updates) => super.copyWith((message) => updates(message as Target)) as Target;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Target create() => Target._();
  Target createEmptyInstance() => create();
  static $pb.PbList<Target> createRepeated() => $pb.PbList<Target>();
  @$core.pragma('dart2js:noInline')
  static Target getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Target>(create);
  static Target? _defaultInstance;

  Target_Target whichTarget() => _Target_TargetByTag[$_whichOneof(0)]!;
  void clearTarget() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.List<$core.int> get accountId => $_getN(0);
  @$pb.TagNumber(1)
  set accountId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearAccountId() => clearField(1);

  @$pb.TagNumber(2)
  $2.Empty get anyAccount => $_getN(1);
  @$pb.TagNumber(2)
  set anyAccount($2.Empty v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasAnyAccount() => $_has(1);
  @$pb.TagNumber(2)
  void clearAnyAccount() => clearField(2);
  @$pb.TagNumber(2)
  $2.Empty ensureAnyAccount() => $_ensure(1);
}

class Action extends $pb.GeneratedMessage {
  factory Action() => create();
  Action._() : super();
  factory Action.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Action.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Action', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'txId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..a<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'contextId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(4, _omitFieldNames ? '' : 'fromAccount', $pb.PbFieldType.OY)
    ..aOM<Target>(5, _omitFieldNames ? '' : 'target', subBuilder: Target.create)
    ..a<$core.List<$core.int>>(6, _omitFieldNames ? '' : 'payload', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Action clone() => Action()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Action copyWith(void Function(Action) updates) => super.copyWith((message) => updates(message as Action)) as Action;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Action create() => Action._();
  Action createEmptyInstance() => create();
  static $pb.PbList<Action> createRepeated() => $pb.PbList<Action>();
  @$core.pragma('dart2js:noInline')
  static Action getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Action>(create);
  static Action? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get txId => $_getI64(0);
  @$pb.TagNumber(1)
  set txId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTxId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTxId() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.int> get contextId => $_getN(2);
  @$pb.TagNumber(3)
  set contextId($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasContextId() => $_has(2);
  @$pb.TagNumber(3)
  void clearContextId() => clearField(3);

  @$pb.TagNumber(4)
  $core.List<$core.int> get fromAccount => $_getN(3);
  @$pb.TagNumber(4)
  set fromAccount($core.List<$core.int> v) { $_setBytes(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasFromAccount() => $_has(3);
  @$pb.TagNumber(4)
  void clearFromAccount() => clearField(4);

  @$pb.TagNumber(5)
  Target get target => $_getN(4);
  @$pb.TagNumber(5)
  set target(Target v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasTarget() => $_has(4);
  @$pb.TagNumber(5)
  void clearTarget() => clearField(5);
  @$pb.TagNumber(5)
  Target ensureTarget() => $_ensure(4);

  @$pb.TagNumber(6)
  $core.List<$core.int> get payload => $_getN(5);
  @$pb.TagNumber(6)
  set payload($core.List<$core.int> v) { $_setBytes(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasPayload() => $_has(5);
  @$pb.TagNumber(6)
  void clearPayload() => clearField(6);
}

class Actions extends $pb.GeneratedMessage {
  factory Actions() => create();
  Actions._() : super();
  factory Actions.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Actions.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Actions', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..pc<Action>(1, _omitFieldNames ? '' : 'actions', $pb.PbFieldType.PM, subBuilder: Action.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Actions clone() => Actions()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Actions copyWith(void Function(Actions) updates) => super.copyWith((message) => updates(message as Actions)) as Actions;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Actions create() => Actions._();
  Actions createEmptyInstance() => create();
  static $pb.PbList<Actions> createRepeated() => $pb.PbList<Actions>();
  @$core.pragma('dart2js:noInline')
  static Actions getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Actions>(create);
  static Actions? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<Action> get actions => $_getList(0);
}

class GetActionRequest extends $pb.GeneratedMessage {
  factory GetActionRequest() => create();
  GetActionRequest._() : super();
  factory GetActionRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetActionRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetActionRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'txId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetActionRequest clone() => GetActionRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetActionRequest copyWith(void Function(GetActionRequest) updates) => super.copyWith((message) => updates(message as GetActionRequest)) as GetActionRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetActionRequest create() => GetActionRequest._();
  GetActionRequest createEmptyInstance() => create();
  static $pb.PbList<GetActionRequest> createRepeated() => $pb.PbList<GetActionRequest>();
  @$core.pragma('dart2js:noInline')
  static GetActionRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetActionRequest>(create);
  static GetActionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get txId => $_getI64(0);
  @$pb.TagNumber(1)
  set txId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTxId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTxId() => clearField(1);
}

enum ListActionsRequest_Filter {
  accountId, 
  contextId, 
  notSet
}

class ListActionsRequest extends $pb.GeneratedMessage {
  factory ListActionsRequest() => create();
  ListActionsRequest._() : super();
  factory ListActionsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListActionsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, ListActionsRequest_Filter> _ListActionsRequest_FilterByTag = {
    2 : ListActionsRequest_Filter.accountId,
    3 : ListActionsRequest_Filter.contextId,
    0 : ListActionsRequest_Filter.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ListActionsRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..oo(0, [2, 3])
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'accountId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'contextId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(4, _omitFieldNames ? '' : 'limit', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(5, _omitFieldNames ? '' : 'minTxId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(6, _omitFieldNames ? '' : 'maxTxId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListActionsRequest clone() => ListActionsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListActionsRequest copyWith(void Function(ListActionsRequest) updates) => super.copyWith((message) => updates(message as ListActionsRequest)) as ListActionsRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListActionsRequest create() => ListActionsRequest._();
  ListActionsRequest createEmptyInstance() => create();
  static $pb.PbList<ListActionsRequest> createRepeated() => $pb.PbList<ListActionsRequest>();
  @$core.pragma('dart2js:noInline')
  static ListActionsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListActionsRequest>(create);
  static ListActionsRequest? _defaultInstance;

  ListActionsRequest_Filter whichFilter() => _ListActionsRequest_FilterByTag[$_whichOneof(0)]!;
  void clearFilter() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get accountId => $_getN(1);
  @$pb.TagNumber(2)
  set accountId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasAccountId() => $_has(1);
  @$pb.TagNumber(2)
  void clearAccountId() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.int> get contextId => $_getN(2);
  @$pb.TagNumber(3)
  set contextId($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasContextId() => $_has(2);
  @$pb.TagNumber(3)
  void clearContextId() => clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get limit => $_getI64(3);
  @$pb.TagNumber(4)
  set limit($fixnum.Int64 v) { $_setInt64(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasLimit() => $_has(3);
  @$pb.TagNumber(4)
  void clearLimit() => clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get minTxId => $_getI64(4);
  @$pb.TagNumber(5)
  set minTxId($fixnum.Int64 v) { $_setInt64(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasMinTxId() => $_has(4);
  @$pb.TagNumber(5)
  void clearMinTxId() => clearField(5);

  @$pb.TagNumber(6)
  $fixnum.Int64 get maxTxId => $_getI64(5);
  @$pb.TagNumber(6)
  set maxTxId($fixnum.Int64 v) { $_setInt64(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasMaxTxId() => $_has(5);
  @$pb.TagNumber(6)
  void clearMaxTxId() => clearField(6);
}

class CommitTransfer extends $pb.GeneratedMessage {
  factory CommitTransfer() => create();
  CommitTransfer._() : super();
  factory CommitTransfer.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CommitTransfer.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'CommitTransfer', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'pendingTxId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..e<CommitTransfer_TransferState>(2, _omitFieldNames ? '' : 'newState', $pb.PbFieldType.OE, defaultOrMaker: CommitTransfer_TransferState.ACCEPTED, valueOf: CommitTransfer_TransferState.valueOf, enumValues: CommitTransfer_TransferState.values)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CommitTransfer clone() => CommitTransfer()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CommitTransfer copyWith(void Function(CommitTransfer) updates) => super.copyWith((message) => updates(message as CommitTransfer)) as CommitTransfer;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CommitTransfer create() => CommitTransfer._();
  CommitTransfer createEmptyInstance() => create();
  static $pb.PbList<CommitTransfer> createRepeated() => $pb.PbList<CommitTransfer>();
  @$core.pragma('dart2js:noInline')
  static CommitTransfer getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CommitTransfer>(create);
  static CommitTransfer? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get pendingTxId => $_getI64(0);
  @$pb.TagNumber(1)
  set pendingTxId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasPendingTxId() => $_has(0);
  @$pb.TagNumber(1)
  void clearPendingTxId() => clearField(1);

  @$pb.TagNumber(2)
  CommitTransfer_TransferState get newState => $_getN(1);
  @$pb.TagNumber(2)
  set newState(CommitTransfer_TransferState v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasNewState() => $_has(1);
  @$pb.TagNumber(2)
  void clearNewState() => clearField(2);
}

class Signature extends $pb.GeneratedMessage {
  factory Signature() => create();
  Signature._() : super();
  factory Signature.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Signature.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Signature', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'publicKey', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'signature', $pb.PbFieldType.OY)
    ..e<Signature_Algorithm>(4, _omitFieldNames ? '' : 'algorithm', $pb.PbFieldType.OE, defaultOrMaker: Signature_Algorithm.P256_SHA256_ASN1, valueOf: Signature_Algorithm.valueOf, enumValues: Signature_Algorithm.values)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Signature clone() => Signature()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Signature copyWith(void Function(Signature) updates) => super.copyWith((message) => updates(message as Signature)) as Signature;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Signature create() => Signature._();
  Signature createEmptyInstance() => create();
  static $pb.PbList<Signature> createRepeated() => $pb.PbList<Signature>();
  @$core.pragma('dart2js:noInline')
  static Signature getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Signature>(create);
  static Signature? _defaultInstance;

  @$pb.TagNumber(2)
  $core.List<$core.int> get publicKey => $_getN(0);
  @$pb.TagNumber(2)
  set publicKey($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(2)
  $core.bool hasPublicKey() => $_has(0);
  @$pb.TagNumber(2)
  void clearPublicKey() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.int> get signature => $_getN(1);
  @$pb.TagNumber(3)
  set signature($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(3)
  $core.bool hasSignature() => $_has(1);
  @$pb.TagNumber(3)
  void clearSignature() => clearField(3);

  @$pb.TagNumber(4)
  Signature_Algorithm get algorithm => $_getN(2);
  @$pb.TagNumber(4)
  set algorithm(Signature_Algorithm v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasAlgorithm() => $_has(2);
  @$pb.TagNumber(4)
  void clearAlgorithm() => clearField(4);
}

class CreateToken extends $pb.GeneratedMessage {
  factory CreateToken() => create();
  CreateToken._() : super();
  factory CreateToken.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CreateToken.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'CreateToken', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'address', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'accountId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(3, _omitFieldNames ? '' : 'value', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CreateToken clone() => CreateToken()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CreateToken copyWith(void Function(CreateToken) updates) => super.copyWith((message) => updates(message as CreateToken)) as CreateToken;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CreateToken create() => CreateToken._();
  CreateToken createEmptyInstance() => create();
  static $pb.PbList<CreateToken> createRepeated() => $pb.PbList<CreateToken>();
  @$core.pragma('dart2js:noInline')
  static CreateToken getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CreateToken>(create);
  static CreateToken? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get address => $_getN(0);
  @$pb.TagNumber(1)
  set address($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasAddress() => $_has(0);
  @$pb.TagNumber(1)
  void clearAddress() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get accountId => $_getN(1);
  @$pb.TagNumber(2)
  set accountId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasAccountId() => $_has(1);
  @$pb.TagNumber(2)
  void clearAccountId() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get value => $_getI64(2);
  @$pb.TagNumber(3)
  set value($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasValue() => $_has(2);
  @$pb.TagNumber(3)
  void clearValue() => clearField(3);
}

class OfflineToken_Data extends $pb.GeneratedMessage {
  factory OfflineToken_Data() => create();
  OfflineToken_Data._() : super();
  factory OfflineToken_Data.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory OfflineToken_Data.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'OfflineToken.Data', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'address', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'accountId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(4, _omitFieldNames ? '' : 'value', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(5, _omitFieldNames ? '' : 'currency')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  OfflineToken_Data clone() => OfflineToken_Data()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  OfflineToken_Data copyWith(void Function(OfflineToken_Data) updates) => super.copyWith((message) => updates(message as OfflineToken_Data)) as OfflineToken_Data;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static OfflineToken_Data create() => OfflineToken_Data._();
  OfflineToken_Data createEmptyInstance() => create();
  static $pb.PbList<OfflineToken_Data> createRepeated() => $pb.PbList<OfflineToken_Data>();
  @$core.pragma('dart2js:noInline')
  static OfflineToken_Data getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<OfflineToken_Data>(create);
  static OfflineToken_Data? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get address => $_getN(1);
  @$pb.TagNumber(2)
  set address($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasAddress() => $_has(1);
  @$pb.TagNumber(2)
  void clearAddress() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.int> get accountId => $_getN(2);
  @$pb.TagNumber(3)
  set accountId($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasAccountId() => $_has(2);
  @$pb.TagNumber(3)
  void clearAccountId() => clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get value => $_getI64(3);
  @$pb.TagNumber(4)
  set value($fixnum.Int64 v) { $_setInt64(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasValue() => $_has(3);
  @$pb.TagNumber(4)
  void clearValue() => clearField(4);

  @$pb.TagNumber(5)
  $core.String get currency => $_getSZ(4);
  @$pb.TagNumber(5)
  set currency($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasCurrency() => $_has(4);
  @$pb.TagNumber(5)
  void clearCurrency() => clearField(5);
}

class OfflineToken extends $pb.GeneratedMessage {
  factory OfflineToken() => create();
  OfflineToken._() : super();
  factory OfflineToken.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory OfflineToken.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'OfflineToken', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..aOM<OfflineToken_Data>(1, _omitFieldNames ? '' : 'data', subBuilder: OfflineToken_Data.create)
    ..aOM<Signature>(2, _omitFieldNames ? '' : 'signature', subBuilder: Signature.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  OfflineToken clone() => OfflineToken()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  OfflineToken copyWith(void Function(OfflineToken) updates) => super.copyWith((message) => updates(message as OfflineToken)) as OfflineToken;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static OfflineToken create() => OfflineToken._();
  OfflineToken createEmptyInstance() => create();
  static $pb.PbList<OfflineToken> createRepeated() => $pb.PbList<OfflineToken>();
  @$core.pragma('dart2js:noInline')
  static OfflineToken getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<OfflineToken>(create);
  static OfflineToken? _defaultInstance;

  @$pb.TagNumber(1)
  OfflineToken_Data get data => $_getN(0);
  @$pb.TagNumber(1)
  set data(OfflineToken_Data v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasData() => $_has(0);
  @$pb.TagNumber(1)
  void clearData() => clearField(1);
  @$pb.TagNumber(1)
  OfflineToken_Data ensureData() => $_ensure(0);

  @$pb.TagNumber(2)
  Signature get signature => $_getN(1);
  @$pb.TagNumber(2)
  set signature(Signature v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasSignature() => $_has(1);
  @$pb.TagNumber(2)
  void clearSignature() => clearField(2);
  @$pb.TagNumber(2)
  Signature ensureSignature() => $_ensure(1);
}

class RedeemableToken_TokenInput extends $pb.GeneratedMessage {
  factory RedeemableToken_TokenInput() => create();
  RedeemableToken_TokenInput._() : super();
  factory RedeemableToken_TokenInput.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RedeemableToken_TokenInput.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RedeemableToken.TokenInput', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'input', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(2, _omitFieldNames ? '' : 'value', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RedeemableToken_TokenInput clone() => RedeemableToken_TokenInput()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RedeemableToken_TokenInput copyWith(void Function(RedeemableToken_TokenInput) updates) => super.copyWith((message) => updates(message as RedeemableToken_TokenInput)) as RedeemableToken_TokenInput;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RedeemableToken_TokenInput create() => RedeemableToken_TokenInput._();
  RedeemableToken_TokenInput createEmptyInstance() => create();
  static $pb.PbList<RedeemableToken_TokenInput> createRepeated() => $pb.PbList<RedeemableToken_TokenInput>();
  @$core.pragma('dart2js:noInline')
  static RedeemableToken_TokenInput getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RedeemableToken_TokenInput>(create);
  static RedeemableToken_TokenInput? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get input => $_getI64(0);
  @$pb.TagNumber(1)
  set input($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasInput() => $_has(0);
  @$pb.TagNumber(1)
  void clearInput() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get value => $_getI64(1);
  @$pb.TagNumber(2)
  set value($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasValue() => $_has(1);
  @$pb.TagNumber(2)
  void clearValue() => clearField(2);
}

class RedeemableToken_Data extends $pb.GeneratedMessage {
  factory RedeemableToken_Data() => create();
  RedeemableToken_Data._() : super();
  factory RedeemableToken_Data.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RedeemableToken_Data.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RedeemableToken.Data', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'address', $pb.PbFieldType.OY)
    ..pc<RedeemableToken_TokenInput>(3, _omitFieldNames ? '' : 'inputs', $pb.PbFieldType.PM, subBuilder: RedeemableToken_TokenInput.create)
    ..aOS(4, _omitFieldNames ? '' : 'currency')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RedeemableToken_Data clone() => RedeemableToken_Data()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RedeemableToken_Data copyWith(void Function(RedeemableToken_Data) updates) => super.copyWith((message) => updates(message as RedeemableToken_Data)) as RedeemableToken_Data;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RedeemableToken_Data create() => RedeemableToken_Data._();
  RedeemableToken_Data createEmptyInstance() => create();
  static $pb.PbList<RedeemableToken_Data> createRepeated() => $pb.PbList<RedeemableToken_Data>();
  @$core.pragma('dart2js:noInline')
  static RedeemableToken_Data getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RedeemableToken_Data>(create);
  static RedeemableToken_Data? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get address => $_getN(1);
  @$pb.TagNumber(2)
  set address($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasAddress() => $_has(1);
  @$pb.TagNumber(2)
  void clearAddress() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<RedeemableToken_TokenInput> get inputs => $_getList(2);

  @$pb.TagNumber(4)
  $core.String get currency => $_getSZ(3);
  @$pb.TagNumber(4)
  set currency($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasCurrency() => $_has(3);
  @$pb.TagNumber(4)
  void clearCurrency() => clearField(4);
}

class RedeemableToken extends $pb.GeneratedMessage {
  factory RedeemableToken() => create();
  RedeemableToken._() : super();
  factory RedeemableToken.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RedeemableToken.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RedeemableToken', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..aOM<RedeemableToken_Data>(1, _omitFieldNames ? '' : 'data', subBuilder: RedeemableToken_Data.create)
    ..aOM<Signature>(2, _omitFieldNames ? '' : 'signature', subBuilder: Signature.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RedeemableToken clone() => RedeemableToken()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RedeemableToken copyWith(void Function(RedeemableToken) updates) => super.copyWith((message) => updates(message as RedeemableToken)) as RedeemableToken;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RedeemableToken create() => RedeemableToken._();
  RedeemableToken createEmptyInstance() => create();
  static $pb.PbList<RedeemableToken> createRepeated() => $pb.PbList<RedeemableToken>();
  @$core.pragma('dart2js:noInline')
  static RedeemableToken getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RedeemableToken>(create);
  static RedeemableToken? _defaultInstance;

  @$pb.TagNumber(1)
  RedeemableToken_Data get data => $_getN(0);
  @$pb.TagNumber(1)
  set data(RedeemableToken_Data v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasData() => $_has(0);
  @$pb.TagNumber(1)
  void clearData() => clearField(1);
  @$pb.TagNumber(1)
  RedeemableToken_Data ensureData() => $_ensure(0);

  @$pb.TagNumber(2)
  Signature get signature => $_getN(1);
  @$pb.TagNumber(2)
  set signature(Signature v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasSignature() => $_has(1);
  @$pb.TagNumber(2)
  void clearSignature() => clearField(2);
  @$pb.TagNumber(2)
  Signature ensureSignature() => $_ensure(1);
}

class RedeemToken extends $pb.GeneratedMessage {
  factory RedeemToken() => create();
  RedeemToken._() : super();
  factory RedeemToken.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RedeemToken.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RedeemToken', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..aOM<RedeemableToken>(1, _omitFieldNames ? '' : 'token', subBuilder: RedeemableToken.create)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'accountId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RedeemToken clone() => RedeemToken()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RedeemToken copyWith(void Function(RedeemToken) updates) => super.copyWith((message) => updates(message as RedeemToken)) as RedeemToken;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RedeemToken create() => RedeemToken._();
  RedeemToken createEmptyInstance() => create();
  static $pb.PbList<RedeemToken> createRepeated() => $pb.PbList<RedeemToken>();
  @$core.pragma('dart2js:noInline')
  static RedeemToken getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RedeemToken>(create);
  static RedeemToken? _defaultInstance;

  @$pb.TagNumber(1)
  RedeemableToken get token => $_getN(0);
  @$pb.TagNumber(1)
  set token(RedeemableToken v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasToken() => $_has(0);
  @$pb.TagNumber(1)
  void clearToken() => clearField(1);
  @$pb.TagNumber(1)
  RedeemableToken ensureToken() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.List<$core.int> get accountId => $_getN(1);
  @$pb.TagNumber(2)
  set accountId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasAccountId() => $_has(1);
  @$pb.TagNumber(2)
  void clearAccountId() => clearField(2);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
