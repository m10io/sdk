///
//  Generated code. Do not modify.
//  source: sdk/metadata.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'api.pb.dart' as $0;
import 'transaction/transaction.pb.dart' as $1;

import 'metadata.pbenum.dart';

export 'metadata.pbenum.dart';

class Attachment extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Attachment', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'objectId')
    ..e<Attachment_AttachmentType>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'type', $pb.PbFieldType.OE, defaultOrMaker: Attachment_AttachmentType.OBJECT, valueOf: Attachment_AttachmentType.valueOf, enumValues: Attachment_AttachmentType.values)
    ..hasRequiredFields = false
  ;

  Attachment._() : super();
  factory Attachment({
    $core.String? objectId,
    Attachment_AttachmentType? type,
  }) {
    final _result = create();
    if (objectId != null) {
      _result.objectId = objectId;
    }
    if (type != null) {
      _result.type = type;
    }
    return _result;
  }
  factory Attachment.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Attachment.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Attachment clone() => Attachment()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Attachment copyWith(void Function(Attachment) updates) => super.copyWith((message) => updates(message as Attachment)) as Attachment; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Attachment create() => Attachment._();
  Attachment createEmptyInstance() => create();
  static $pb.PbList<Attachment> createRepeated() => $pb.PbList<Attachment>();
  @$core.pragma('dart2js:noInline')
  static Attachment getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Attachment>(create);
  static Attachment? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get objectId => $_getSZ(0);
  @$pb.TagNumber(1)
  set objectId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasObjectId() => $_has(0);
  @$pb.TagNumber(1)
  void clearObjectId() => clearField(1);

  @$pb.TagNumber(2)
  Attachment_AttachmentType get type => $_getN(1);
  @$pb.TagNumber(2)
  set type(Attachment_AttachmentType v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasType() => $_has(1);
  @$pb.TagNumber(2)
  void clearType() => clearField(2);
}

class Memo extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Memo', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'plaintext')
    ..hasRequiredFields = false
  ;

  Memo._() : super();
  factory Memo({
    $core.String? plaintext,
  }) {
    final _result = create();
    if (plaintext != null) {
      _result.plaintext = plaintext;
    }
    return _result;
  }
  factory Memo.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Memo.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Memo clone() => Memo()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Memo copyWith(void Function(Memo) updates) => super.copyWith((message) => updates(message as Memo)) as Memo; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Memo create() => Memo._();
  Memo createEmptyInstance() => create();
  static $pb.PbList<Memo> createRepeated() => $pb.PbList<Memo>();
  @$core.pragma('dart2js:noInline')
  static Memo getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Memo>(create);
  static Memo? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get plaintext => $_getSZ(0);
  @$pb.TagNumber(1)
  set plaintext($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasPlaintext() => $_has(0);
  @$pb.TagNumber(1)
  void clearPlaintext() => clearField(1);
}

class Fee extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Fee', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  Fee._() : super();
  factory Fee() => create();
  factory Fee.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Fee.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Fee clone() => Fee()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Fee copyWith(void Function(Fee) updates) => super.copyWith((message) => updates(message as Fee)) as Fee; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Fee create() => Fee._();
  Fee createEmptyInstance() => create();
  static $pb.PbList<Fee> createRepeated() => $pb.PbList<Fee>();
  @$core.pragma('dart2js:noInline')
  static Fee getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Fee>(create);
  static Fee? _defaultInstance;
}

class Withdraw extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Withdraw', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'bankAccountId')
    ..hasRequiredFields = false
  ;

  Withdraw._() : super();
  factory Withdraw({
    $core.String? bankAccountId,
  }) {
    final _result = create();
    if (bankAccountId != null) {
      _result.bankAccountId = bankAccountId;
    }
    return _result;
  }
  factory Withdraw.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Withdraw.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Withdraw clone() => Withdraw()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Withdraw copyWith(void Function(Withdraw) updates) => super.copyWith((message) => updates(message as Withdraw)) as Withdraw; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Withdraw create() => Withdraw._();
  Withdraw createEmptyInstance() => create();
  static $pb.PbList<Withdraw> createRepeated() => $pb.PbList<Withdraw>();
  @$core.pragma('dart2js:noInline')
  static Withdraw getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Withdraw>(create);
  static Withdraw? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get bankAccountId => $_getSZ(0);
  @$pb.TagNumber(1)
  set bankAccountId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasBankAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearBankAccountId() => clearField(1);
}

class Deposit extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Deposit', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'bankAccountId')
    ..hasRequiredFields = false
  ;

  Deposit._() : super();
  factory Deposit({
    $core.String? bankAccountId,
  }) {
    final _result = create();
    if (bankAccountId != null) {
      _result.bankAccountId = bankAccountId;
    }
    return _result;
  }
  factory Deposit.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Deposit.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Deposit clone() => Deposit()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Deposit copyWith(void Function(Deposit) updates) => super.copyWith((message) => updates(message as Deposit)) as Deposit; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Deposit create() => Deposit._();
  Deposit createEmptyInstance() => create();
  static $pb.PbList<Deposit> createRepeated() => $pb.PbList<Deposit>();
  @$core.pragma('dart2js:noInline')
  static Deposit getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Deposit>(create);
  static Deposit? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get bankAccountId => $_getSZ(0);
  @$pb.TagNumber(1)
  set bankAccountId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasBankAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearBankAccountId() => clearField(1);
}

class Contract extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Contract', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transactions', $pb.PbFieldType.OY)
    ..pc<Endorsement>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'endorsements', $pb.PbFieldType.PM, subBuilder: Endorsement.create)
    ..hasRequiredFields = false
  ;

  Contract._() : super();
  factory Contract({
    $core.List<$core.int>? transactions,
    $core.Iterable<Endorsement>? endorsements,
  }) {
    final _result = create();
    if (transactions != null) {
      _result.transactions = transactions;
    }
    if (endorsements != null) {
      _result.endorsements.addAll(endorsements);
    }
    return _result;
  }
  factory Contract.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Contract.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Contract clone() => Contract()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Contract copyWith(void Function(Contract) updates) => super.copyWith((message) => updates(message as Contract)) as Contract; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Contract create() => Contract._();
  Contract createEmptyInstance() => create();
  static $pb.PbList<Contract> createRepeated() => $pb.PbList<Contract>();
  @$core.pragma('dart2js:noInline')
  static Contract getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Contract>(create);
  static Contract? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get transactions => $_getN(0);
  @$pb.TagNumber(1)
  set transactions($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTransactions() => $_has(0);
  @$pb.TagNumber(1)
  void clearTransactions() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<Endorsement> get endorsements => $_getList(1);
}

class Endorsement extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Endorsement', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'ledgerId')
    ..aOM<$0.Signature>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signature', subBuilder: $0.Signature.create)
    ..hasRequiredFields = false
  ;

  Endorsement._() : super();
  factory Endorsement({
    $core.String? ledgerId,
    $0.Signature? signature,
  }) {
    final _result = create();
    if (ledgerId != null) {
      _result.ledgerId = ledgerId;
    }
    if (signature != null) {
      _result.signature = signature;
    }
    return _result;
  }
  factory Endorsement.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Endorsement.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Endorsement clone() => Endorsement()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Endorsement copyWith(void Function(Endorsement) updates) => super.copyWith((message) => updates(message as Endorsement)) as Endorsement; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Endorsement create() => Endorsement._();
  Endorsement createEmptyInstance() => create();
  static $pb.PbList<Endorsement> createRepeated() => $pb.PbList<Endorsement>();
  @$core.pragma('dart2js:noInline')
  static Endorsement getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Endorsement>(create);
  static Endorsement? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get ledgerId => $_getSZ(0);
  @$pb.TagNumber(1)
  set ledgerId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasLedgerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearLedgerId() => clearField(1);

  @$pb.TagNumber(2)
  $0.Signature get signature => $_getN(1);
  @$pb.TagNumber(2)
  set signature($0.Signature v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasSignature() => $_has(1);
  @$pb.TagNumber(2)
  void clearSignature() => clearField(2);
  @$pb.TagNumber(2)
  $0.Signature ensureSignature() => $_ensure(1);
}

class PaymentRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'PaymentRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..aOM<$1.CreateTransfer>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transfer', subBuilder: $1.CreateTransfer.create)
    ..e<PaymentRequest_PaymentRequestStatus>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'status', $pb.PbFieldType.OE, defaultOrMaker: PaymentRequest_PaymentRequestStatus.PENDING, valueOf: PaymentRequest_PaymentRequestStatus.valueOf, enumValues: PaymentRequest_PaymentRequestStatus.values)
    ..hasRequiredFields = false
  ;

  PaymentRequest._() : super();
  factory PaymentRequest({
    $1.CreateTransfer? transfer,
    PaymentRequest_PaymentRequestStatus? status,
  }) {
    final _result = create();
    if (transfer != null) {
      _result.transfer = transfer;
    }
    if (status != null) {
      _result.status = status;
    }
    return _result;
  }
  factory PaymentRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory PaymentRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  PaymentRequest clone() => PaymentRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  PaymentRequest copyWith(void Function(PaymentRequest) updates) => super.copyWith((message) => updates(message as PaymentRequest)) as PaymentRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static PaymentRequest create() => PaymentRequest._();
  PaymentRequest createEmptyInstance() => create();
  static $pb.PbList<PaymentRequest> createRepeated() => $pb.PbList<PaymentRequest>();
  @$core.pragma('dart2js:noInline')
  static PaymentRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PaymentRequest>(create);
  static PaymentRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $1.CreateTransfer get transfer => $_getN(0);
  @$pb.TagNumber(1)
  set transfer($1.CreateTransfer v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasTransfer() => $_has(0);
  @$pb.TagNumber(1)
  void clearTransfer() => clearField(1);
  @$pb.TagNumber(1)
  $1.CreateTransfer ensureTransfer() => $_ensure(0);

  @$pb.TagNumber(2)
  PaymentRequest_PaymentRequestStatus get status => $_getN(1);
  @$pb.TagNumber(2)
  set status(PaymentRequest_PaymentRequestStatus v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasStatus() => $_has(1);
  @$pb.TagNumber(2)
  void clearStatus() => clearField(2);
}

