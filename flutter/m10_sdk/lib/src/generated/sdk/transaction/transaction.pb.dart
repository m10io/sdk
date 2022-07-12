///
//  Generated code. Do not modify.
//  source: sdk/transaction/transaction.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import '../document.pb.dart' as $0;
import '../../google/protobuf/any.pb.dart' as $1;
import '../../google/protobuf/empty.pb.dart' as $2;

import 'transaction.pbenum.dart';

export 'transaction.pbenum.dart';

class TransactionRequestPayload extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'TransactionRequestPayload', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'nonce', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'timestamp', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'contextId', $pb.PbFieldType.OY)
    ..aOM<TransactionData>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'data', subBuilder: TransactionData.create)
  ;

  TransactionRequestPayload._() : super();
  factory TransactionRequestPayload({
    $fixnum.Int64? nonce,
    $fixnum.Int64? timestamp,
    $core.List<$core.int>? contextId,
    TransactionData? data,
  }) {
    final _result = create();
    if (nonce != null) {
      _result.nonce = nonce;
    }
    if (timestamp != null) {
      _result.timestamp = timestamp;
    }
    if (contextId != null) {
      _result.contextId = contextId;
    }
    if (data != null) {
      _result.data = data;
    }
    return _result;
  }
  factory TransactionRequestPayload.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TransactionRequestPayload.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TransactionRequestPayload clone() => TransactionRequestPayload()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TransactionRequestPayload copyWith(void Function(TransactionRequestPayload) updates) => super.copyWith((message) => updates(message as TransactionRequestPayload)) as TransactionRequestPayload; // ignore: deprecated_member_use
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
  notSet
}

class TransactionData extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, TransactionData_Data> _TransactionData_DataByTag = {
    10 : TransactionData_Data.transfer,
    11 : TransactionData_Data.createLedgerAccount,
    12 : TransactionData_Data.setFreezeState,
    16 : TransactionData_Data.documentOperations,
    20 : TransactionData_Data.invokeAction,
    21 : TransactionData_Data.initiateTransfer,
    22 : TransactionData_Data.commitTransfer,
    23 : TransactionData_Data.setInstrument,
    0 : TransactionData_Data.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'TransactionData', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..oo(0, [10, 11, 12, 16, 20, 21, 22, 23])
    ..aOM<CreateTransfer>(10, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transfer', subBuilder: CreateTransfer.create)
    ..aOM<CreateLedgerAccount>(11, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'createLedgerAccount', subBuilder: CreateLedgerAccount.create)
    ..aOM<SetFreezeState>(12, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'setFreezeState', subBuilder: SetFreezeState.create)
    ..aOM<$0.DocumentOperations>(16, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'documentOperations', subBuilder: $0.DocumentOperations.create)
    ..aOM<InvokeAction>(20, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'invokeAction', subBuilder: InvokeAction.create)
    ..aOM<CreateTransfer>(21, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'initiateTransfer', subBuilder: CreateTransfer.create)
    ..aOM<CommitTransfer>(22, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'commitTransfer', subBuilder: CommitTransfer.create)
    ..aOM<SetInstrument>(23, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'setInstrument', subBuilder: SetInstrument.create)
  ;

  TransactionData._() : super();
  factory TransactionData({
    CreateTransfer? transfer,
    CreateLedgerAccount? createLedgerAccount,
    SetFreezeState? setFreezeState,
    $0.DocumentOperations? documentOperations,
    InvokeAction? invokeAction,
    CreateTransfer? initiateTransfer,
    CommitTransfer? commitTransfer,
    SetInstrument? setInstrument,
  }) {
    final _result = create();
    if (transfer != null) {
      _result.transfer = transfer;
    }
    if (createLedgerAccount != null) {
      _result.createLedgerAccount = createLedgerAccount;
    }
    if (setFreezeState != null) {
      _result.setFreezeState = setFreezeState;
    }
    if (documentOperations != null) {
      _result.documentOperations = documentOperations;
    }
    if (invokeAction != null) {
      _result.invokeAction = invokeAction;
    }
    if (initiateTransfer != null) {
      _result.initiateTransfer = initiateTransfer;
    }
    if (commitTransfer != null) {
      _result.commitTransfer = commitTransfer;
    }
    if (setInstrument != null) {
      _result.setInstrument = setInstrument;
    }
    return _result;
  }
  factory TransactionData.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TransactionData.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TransactionData clone() => TransactionData()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TransactionData copyWith(void Function(TransactionData) updates) => super.copyWith((message) => updates(message as TransactionData)) as TransactionData; // ignore: deprecated_member_use
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
}

class TransactionResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'TransactionResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'txId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<TransactionError>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'error', subBuilder: TransactionError.create)
    ..a<$core.List<$core.int>>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountCreated', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'timestamp', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<CreateTransfer>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transferCommitted', subBuilder: CreateTransfer.create)
    ..hasRequiredFields = false
  ;

  TransactionResponse._() : super();
  factory TransactionResponse({
    $fixnum.Int64? txId,
    TransactionError? error,
    $core.List<$core.int>? accountCreated,
    $fixnum.Int64? timestamp,
    CreateTransfer? transferCommitted,
  }) {
    final _result = create();
    if (txId != null) {
      _result.txId = txId;
    }
    if (error != null) {
      _result.error = error;
    }
    if (accountCreated != null) {
      _result.accountCreated = accountCreated;
    }
    if (timestamp != null) {
      _result.timestamp = timestamp;
    }
    if (transferCommitted != null) {
      _result.transferCommitted = transferCommitted;
    }
    return _result;
  }
  factory TransactionResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TransactionResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TransactionResponse clone() => TransactionResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TransactionResponse copyWith(void Function(TransactionResponse) updates) => super.copyWith((message) => updates(message as TransactionResponse)) as TransactionResponse; // ignore: deprecated_member_use
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
}

class TransactionError extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'TransactionError', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..e<TransactionError_Code>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'code', $pb.PbFieldType.OE, defaultOrMaker: TransactionError_Code.UNKNOWN, valueOf: TransactionError_Code.valueOf, enumValues: TransactionError_Code.values)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'message')
    ..hasRequiredFields = false
  ;

  TransactionError._() : super();
  factory TransactionError({
    TransactionError_Code? code,
    $core.String? message,
  }) {
    final _result = create();
    if (code != null) {
      _result.code = code;
    }
    if (message != null) {
      _result.message = message;
    }
    return _result;
  }
  factory TransactionError.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TransactionError.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TransactionError clone() => TransactionError()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TransactionError copyWith(void Function(TransactionError) updates) => super.copyWith((message) => updates(message as TransactionError)) as TransactionError; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'CreateLedgerTransfer', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'ledgerId')
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'nonce', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<CreateTransfer>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transfer', subBuilder: CreateTransfer.create)
    ..hasRequiredFields = false
  ;

  CreateLedgerTransfer._() : super();
  factory CreateLedgerTransfer({
    $core.String? ledgerId,
    $fixnum.Int64? nonce,
    CreateTransfer? transfer,
  }) {
    final _result = create();
    if (ledgerId != null) {
      _result.ledgerId = ledgerId;
    }
    if (nonce != null) {
      _result.nonce = nonce;
    }
    if (transfer != null) {
      _result.transfer = transfer;
    }
    return _result;
  }
  factory CreateLedgerTransfer.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CreateLedgerTransfer.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CreateLedgerTransfer clone() => CreateLedgerTransfer()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CreateLedgerTransfer copyWith(void Function(CreateLedgerTransfer) updates) => super.copyWith((message) => updates(message as CreateLedgerTransfer)) as CreateLedgerTransfer; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'CreateLedgerTransfers', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..pc<CreateLedgerTransfer>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transfers', $pb.PbFieldType.PM, subBuilder: CreateLedgerTransfer.create)
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'validUntil', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  CreateLedgerTransfers._() : super();
  factory CreateLedgerTransfers({
    $core.Iterable<CreateLedgerTransfer>? transfers,
    $fixnum.Int64? validUntil,
  }) {
    final _result = create();
    if (transfers != null) {
      _result.transfers.addAll(transfers);
    }
    if (validUntil != null) {
      _result.validUntil = validUntil;
    }
    return _result;
  }
  factory CreateLedgerTransfers.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CreateLedgerTransfers.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CreateLedgerTransfers clone() => CreateLedgerTransfers()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CreateLedgerTransfers copyWith(void Function(CreateLedgerTransfers) updates) => super.copyWith((message) => updates(message as CreateLedgerTransfers)) as CreateLedgerTransfers; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetTransferRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'txId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  GetTransferRequest._() : super();
  factory GetTransferRequest({
    $fixnum.Int64? txId,
  }) {
    final _result = create();
    if (txId != null) {
      _result.txId = txId;
    }
    return _result;
  }
  factory GetTransferRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetTransferRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetTransferRequest clone() => GetTransferRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetTransferRequest copyWith(void Function(GetTransferRequest) updates) => super.copyWith((message) => updates(message as GetTransferRequest)) as GetTransferRequest; // ignore: deprecated_member_use
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
  static const $core.Map<$core.int, ListTransferRequest_Filter> _ListTransferRequest_FilterByTag = {
    1 : ListTransferRequest_Filter.accountId,
    2 : ListTransferRequest_Filter.contextId,
    0 : ListTransferRequest_Filter.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ListTransferRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'contextId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'limit', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOB(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'includeChildAccounts')
    ..a<$fixnum.Int64>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'minTxId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'maxTxId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  ListTransferRequest._() : super();
  factory ListTransferRequest({
    $core.List<$core.int>? accountId,
    $core.List<$core.int>? contextId,
    $fixnum.Int64? limit,
    $core.bool? includeChildAccounts,
    $fixnum.Int64? minTxId,
    $fixnum.Int64? maxTxId,
  }) {
    final _result = create();
    if (accountId != null) {
      _result.accountId = accountId;
    }
    if (contextId != null) {
      _result.contextId = contextId;
    }
    if (limit != null) {
      _result.limit = limit;
    }
    if (includeChildAccounts != null) {
      _result.includeChildAccounts = includeChildAccounts;
    }
    if (minTxId != null) {
      _result.minTxId = minTxId;
    }
    if (maxTxId != null) {
      _result.maxTxId = maxTxId;
    }
    return _result;
  }
  factory ListTransferRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListTransferRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListTransferRequest clone() => ListTransferRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListTransferRequest copyWith(void Function(ListTransferRequest) updates) => super.copyWith((message) => updates(message as ListTransferRequest)) as ListTransferRequest; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'CreateTransfer', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..pc<TransferStep>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transferSteps', $pb.PbFieldType.PM, subBuilder: TransferStep.create)
    ..hasRequiredFields = false
  ;

  CreateTransfer._() : super();
  factory CreateTransfer({
    $core.Iterable<TransferStep>? transferSteps,
  }) {
    final _result = create();
    if (transferSteps != null) {
      _result.transferSteps.addAll(transferSteps);
    }
    return _result;
  }
  factory CreateTransfer.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CreateTransfer.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CreateTransfer clone() => CreateTransfer()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CreateTransfer copyWith(void Function(CreateTransfer) updates) => super.copyWith((message) => updates(message as CreateTransfer)) as CreateTransfer; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'TransferStep', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fromAccountId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'toAccountId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'amount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..pc<$1.Any>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'metadata', $pb.PbFieldType.PM, subBuilder: $1.Any.create)
    ..hasRequiredFields = false
  ;

  TransferStep._() : super();
  factory TransferStep({
    $core.List<$core.int>? fromAccountId,
    $core.List<$core.int>? toAccountId,
    $fixnum.Int64? amount,
    $core.Iterable<$1.Any>? metadata,
  }) {
    final _result = create();
    if (fromAccountId != null) {
      _result.fromAccountId = fromAccountId;
    }
    if (toAccountId != null) {
      _result.toAccountId = toAccountId;
    }
    if (amount != null) {
      _result.amount = amount;
    }
    if (metadata != null) {
      _result.metadata.addAll(metadata);
    }
    return _result;
  }
  factory TransferStep.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TransferStep.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TransferStep clone() => TransferStep()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TransferStep copyWith(void Function(TransferStep) updates) => super.copyWith((message) => updates(message as TransferStep)) as TransferStep; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FinalizedTransfer', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'txId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'contextId', $pb.PbFieldType.OY)
    ..pc<TransferStep>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transferSteps', $pb.PbFieldType.PM, subBuilder: TransferStep.create)
    ..aOM<TransactionError>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'error', subBuilder: TransactionError.create)
    ..a<$fixnum.Int64>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'timestamp', $pb.PbFieldType.OF6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..e<FinalizedTransfer_TransferState>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'state', $pb.PbFieldType.OE, defaultOrMaker: FinalizedTransfer_TransferState.ACCEPTED, valueOf: FinalizedTransfer_TransferState.valueOf, enumValues: FinalizedTransfer_TransferState.values)
    ..hasRequiredFields = false
  ;

  FinalizedTransfer._() : super();
  factory FinalizedTransfer({
    $fixnum.Int64? txId,
    $core.List<$core.int>? contextId,
    $core.Iterable<TransferStep>? transferSteps,
    TransactionError? error,
    $fixnum.Int64? timestamp,
    FinalizedTransfer_TransferState? state,
  }) {
    final _result = create();
    if (txId != null) {
      _result.txId = txId;
    }
    if (contextId != null) {
      _result.contextId = contextId;
    }
    if (transferSteps != null) {
      _result.transferSteps.addAll(transferSteps);
    }
    if (error != null) {
      _result.error = error;
    }
    if (timestamp != null) {
      _result.timestamp = timestamp;
    }
    if (state != null) {
      _result.state = state;
    }
    return _result;
  }
  factory FinalizedTransfer.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FinalizedTransfer.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FinalizedTransfer clone() => FinalizedTransfer()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FinalizedTransfer copyWith(void Function(FinalizedTransfer) updates) => super.copyWith((message) => updates(message as FinalizedTransfer)) as FinalizedTransfer; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FinalizedTransfers', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..pc<FinalizedTransfer>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transfers', $pb.PbFieldType.PM, subBuilder: FinalizedTransfer.create)
    ..hasRequiredFields = false
  ;

  FinalizedTransfers._() : super();
  factory FinalizedTransfers({
    $core.Iterable<FinalizedTransfer>? transfers,
  }) {
    final _result = create();
    if (transfers != null) {
      _result.transfers.addAll(transfers);
    }
    return _result;
  }
  factory FinalizedTransfers.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FinalizedTransfers.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FinalizedTransfers clone() => FinalizedTransfers()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FinalizedTransfers copyWith(void Function(FinalizedTransfers) updates) => super.copyWith((message) => updates(message as FinalizedTransfers)) as FinalizedTransfers; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Instrument', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'code')
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'decimalPlaces', $pb.PbFieldType.OU3)
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'description')
    ..hasRequiredFields = false
  ;

  Instrument._() : super();
  factory Instrument({
    $core.String? code,
    $core.int? decimalPlaces,
    $core.String? description,
  }) {
    final _result = create();
    if (code != null) {
      _result.code = code;
    }
    if (decimalPlaces != null) {
      _result.decimalPlaces = decimalPlaces;
    }
    if (description != null) {
      _result.description = description;
    }
    return _result;
  }
  factory Instrument.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Instrument.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Instrument clone() => Instrument()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Instrument copyWith(void Function(Instrument) updates) => super.copyWith((message) => updates(message as Instrument)) as Instrument; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'CreateLedgerAccount', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'parentId', $pb.PbFieldType.OY)
    ..aOB(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'issuance')
    ..aOB(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'frozen')
    ..aOM<Instrument>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'instrument', subBuilder: Instrument.create)
    ..hasRequiredFields = false
  ;

  CreateLedgerAccount._() : super();
  factory CreateLedgerAccount({
    $core.List<$core.int>? parentId,
    $core.bool? issuance,
    $core.bool? frozen,
    Instrument? instrument,
  }) {
    final _result = create();
    if (parentId != null) {
      _result.parentId = parentId;
    }
    if (issuance != null) {
      _result.issuance = issuance;
    }
    if (frozen != null) {
      _result.frozen = frozen;
    }
    if (instrument != null) {
      _result.instrument = instrument;
    }
    return _result;
  }
  factory CreateLedgerAccount.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CreateLedgerAccount.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CreateLedgerAccount clone() => CreateLedgerAccount()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CreateLedgerAccount copyWith(void Function(CreateLedgerAccount) updates) => super.copyWith((message) => updates(message as CreateLedgerAccount)) as CreateLedgerAccount; // ignore: deprecated_member_use
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
}

class SetFreezeState extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SetFreezeState', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountId', $pb.PbFieldType.OY)
    ..aOB(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'frozen')
    ..hasRequiredFields = false
  ;

  SetFreezeState._() : super();
  factory SetFreezeState({
    $core.List<$core.int>? accountId,
    $core.bool? frozen,
  }) {
    final _result = create();
    if (accountId != null) {
      _result.accountId = accountId;
    }
    if (frozen != null) {
      _result.frozen = frozen;
    }
    return _result;
  }
  factory SetFreezeState.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SetFreezeState.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SetFreezeState clone() => SetFreezeState()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SetFreezeState copyWith(void Function(SetFreezeState) updates) => super.copyWith((message) => updates(message as SetFreezeState)) as SetFreezeState; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SetInstrument', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountId', $pb.PbFieldType.OY)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'code')
    ..a<$core.int>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'decimalPlaces', $pb.PbFieldType.OU3)
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'description')
    ..hasRequiredFields = false
  ;

  SetInstrument._() : super();
  factory SetInstrument({
    $core.List<$core.int>? accountId,
    $core.String? code,
    $core.int? decimalPlaces,
    $core.String? description,
  }) {
    final _result = create();
    if (accountId != null) {
      _result.accountId = accountId;
    }
    if (code != null) {
      _result.code = code;
    }
    if (decimalPlaces != null) {
      _result.decimalPlaces = decimalPlaces;
    }
    if (description != null) {
      _result.description = description;
    }
    return _result;
  }
  factory SetInstrument.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SetInstrument.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SetInstrument clone() => SetInstrument()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SetInstrument copyWith(void Function(SetInstrument) updates) => super.copyWith((message) => updates(message as SetInstrument)) as SetInstrument; // ignore: deprecated_member_use
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

class GetAccountRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetAccountRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  GetAccountRequest._() : super();
  factory GetAccountRequest({
    $core.List<$core.int>? id,
  }) {
    final _result = create();
    if (id != null) {
      _result.id = id;
    }
    return _result;
  }
  factory GetAccountRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetAccountRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetAccountRequest clone() => GetAccountRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetAccountRequest copyWith(void Function(GetAccountRequest) updates) => super.copyWith((message) => updates(message as GetAccountRequest)) as GetAccountRequest; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'IndexedAccount.Issuance', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'issuedBalance', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'nonLeafChildren', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'leafChildren', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  IndexedAccount_Issuance._() : super();
  factory IndexedAccount_Issuance({
    $fixnum.Int64? issuedBalance,
    $fixnum.Int64? nonLeafChildren,
    $fixnum.Int64? leafChildren,
  }) {
    final _result = create();
    if (issuedBalance != null) {
      _result.issuedBalance = issuedBalance;
    }
    if (nonLeafChildren != null) {
      _result.nonLeafChildren = nonLeafChildren;
    }
    if (leafChildren != null) {
      _result.leafChildren = leafChildren;
    }
    return _result;
  }
  factory IndexedAccount_Issuance.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory IndexedAccount_Issuance.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  IndexedAccount_Issuance clone() => IndexedAccount_Issuance()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  IndexedAccount_Issuance copyWith(void Function(IndexedAccount_Issuance) updates) => super.copyWith((message) => updates(message as IndexedAccount_Issuance)) as IndexedAccount_Issuance; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'IndexedAccount', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id', $pb.PbFieldType.OY)
    ..aOM<IndexedAccount_Issuance>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'issuance', subBuilder: IndexedAccount_Issuance.create)
    ..a<$fixnum.Int64>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'balance', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOB(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'frozen')
    ..aOM<Instrument>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'instrument', subBuilder: Instrument.create)
    ..hasRequiredFields = false
  ;

  IndexedAccount._() : super();
  factory IndexedAccount({
    $core.List<$core.int>? id,
    IndexedAccount_Issuance? issuance,
    $fixnum.Int64? balance,
    $core.bool? frozen,
    Instrument? instrument,
  }) {
    final _result = create();
    if (id != null) {
      _result.id = id;
    }
    if (issuance != null) {
      _result.issuance = issuance;
    }
    if (balance != null) {
      _result.balance = balance;
    }
    if (frozen != null) {
      _result.frozen = frozen;
    }
    if (instrument != null) {
      _result.instrument = instrument;
    }
    return _result;
  }
  factory IndexedAccount.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory IndexedAccount.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  IndexedAccount clone() => IndexedAccount()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  IndexedAccount copyWith(void Function(IndexedAccount) updates) => super.copyWith((message) => updates(message as IndexedAccount)) as IndexedAccount; // ignore: deprecated_member_use
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
}

class InvokeAction extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'InvokeAction', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..a<$core.List<$core.int>>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fromAccount', $pb.PbFieldType.OY)
    ..aOM<Target>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'target', subBuilder: Target.create)
    ..a<$core.List<$core.int>>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'payload', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  InvokeAction._() : super();
  factory InvokeAction({
    $core.String? name,
    $core.List<$core.int>? fromAccount,
    Target? target,
    $core.List<$core.int>? payload,
  }) {
    final _result = create();
    if (name != null) {
      _result.name = name;
    }
    if (fromAccount != null) {
      _result.fromAccount = fromAccount;
    }
    if (target != null) {
      _result.target = target;
    }
    if (payload != null) {
      _result.payload = payload;
    }
    return _result;
  }
  factory InvokeAction.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory InvokeAction.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  InvokeAction clone() => InvokeAction()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  InvokeAction copyWith(void Function(InvokeAction) updates) => super.copyWith((message) => updates(message as InvokeAction)) as InvokeAction; // ignore: deprecated_member_use
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
  static const $core.Map<$core.int, Target_Target> _Target_TargetByTag = {
    1 : Target_Target.accountId,
    2 : Target_Target.anyAccount,
    0 : Target_Target.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Target', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountId', $pb.PbFieldType.OY)
    ..aOM<$2.Empty>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'anyAccount', subBuilder: $2.Empty.create)
    ..hasRequiredFields = false
  ;

  Target._() : super();
  factory Target({
    $core.List<$core.int>? accountId,
    $2.Empty? anyAccount,
  }) {
    final _result = create();
    if (accountId != null) {
      _result.accountId = accountId;
    }
    if (anyAccount != null) {
      _result.anyAccount = anyAccount;
    }
    return _result;
  }
  factory Target.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Target.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Target clone() => Target()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Target copyWith(void Function(Target) updates) => super.copyWith((message) => updates(message as Target)) as Target; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Action', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'txId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..a<$core.List<$core.int>>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'contextId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'fromAccount', $pb.PbFieldType.OY)
    ..aOM<Target>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'target', subBuilder: Target.create)
    ..a<$core.List<$core.int>>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'payload', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  Action._() : super();
  factory Action({
    $fixnum.Int64? txId,
    $core.String? name,
    $core.List<$core.int>? contextId,
    $core.List<$core.int>? fromAccount,
    Target? target,
    $core.List<$core.int>? payload,
  }) {
    final _result = create();
    if (txId != null) {
      _result.txId = txId;
    }
    if (name != null) {
      _result.name = name;
    }
    if (contextId != null) {
      _result.contextId = contextId;
    }
    if (fromAccount != null) {
      _result.fromAccount = fromAccount;
    }
    if (target != null) {
      _result.target = target;
    }
    if (payload != null) {
      _result.payload = payload;
    }
    return _result;
  }
  factory Action.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Action.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Action clone() => Action()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Action copyWith(void Function(Action) updates) => super.copyWith((message) => updates(message as Action)) as Action; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Actions', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..pc<Action>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'actions', $pb.PbFieldType.PM, subBuilder: Action.create)
    ..hasRequiredFields = false
  ;

  Actions._() : super();
  factory Actions({
    $core.Iterable<Action>? actions,
  }) {
    final _result = create();
    if (actions != null) {
      _result.actions.addAll(actions);
    }
    return _result;
  }
  factory Actions.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Actions.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Actions clone() => Actions()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Actions copyWith(void Function(Actions) updates) => super.copyWith((message) => updates(message as Actions)) as Actions; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetActionRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'txId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  GetActionRequest._() : super();
  factory GetActionRequest({
    $fixnum.Int64? txId,
  }) {
    final _result = create();
    if (txId != null) {
      _result.txId = txId;
    }
    return _result;
  }
  factory GetActionRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetActionRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetActionRequest clone() => GetActionRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetActionRequest copyWith(void Function(GetActionRequest) updates) => super.copyWith((message) => updates(message as GetActionRequest)) as GetActionRequest; // ignore: deprecated_member_use
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
  static const $core.Map<$core.int, ListActionsRequest_Filter> _ListActionsRequest_FilterByTag = {
    2 : ListActionsRequest_Filter.accountId,
    3 : ListActionsRequest_Filter.contextId,
    0 : ListActionsRequest_Filter.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ListActionsRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..oo(0, [2, 3])
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'contextId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'limit', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'minTxId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'maxTxId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  ListActionsRequest._() : super();
  factory ListActionsRequest({
    $core.String? name,
    $core.List<$core.int>? accountId,
    $core.List<$core.int>? contextId,
    $fixnum.Int64? limit,
    $fixnum.Int64? minTxId,
    $fixnum.Int64? maxTxId,
  }) {
    final _result = create();
    if (name != null) {
      _result.name = name;
    }
    if (accountId != null) {
      _result.accountId = accountId;
    }
    if (contextId != null) {
      _result.contextId = contextId;
    }
    if (limit != null) {
      _result.limit = limit;
    }
    if (minTxId != null) {
      _result.minTxId = minTxId;
    }
    if (maxTxId != null) {
      _result.maxTxId = maxTxId;
    }
    return _result;
  }
  factory ListActionsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListActionsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListActionsRequest clone() => ListActionsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListActionsRequest copyWith(void Function(ListActionsRequest) updates) => super.copyWith((message) => updates(message as ListActionsRequest)) as ListActionsRequest; // ignore: deprecated_member_use
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
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'CommitTransfer', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.transaction'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'pendingTxId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..e<CommitTransfer_TransferState>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'newState', $pb.PbFieldType.OE, defaultOrMaker: CommitTransfer_TransferState.ACCEPTED, valueOf: CommitTransfer_TransferState.valueOf, enumValues: CommitTransfer_TransferState.values)
    ..hasRequiredFields = false
  ;

  CommitTransfer._() : super();
  factory CommitTransfer({
    $fixnum.Int64? pendingTxId,
    CommitTransfer_TransferState? newState,
  }) {
    final _result = create();
    if (pendingTxId != null) {
      _result.pendingTxId = pendingTxId;
    }
    if (newState != null) {
      _result.newState = newState;
    }
    return _result;
  }
  factory CommitTransfer.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CommitTransfer.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CommitTransfer clone() => CommitTransfer()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CommitTransfer copyWith(void Function(CommitTransfer) updates) => super.copyWith((message) => updates(message as CommitTransfer)) as CommitTransfer; // ignore: deprecated_member_use
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

