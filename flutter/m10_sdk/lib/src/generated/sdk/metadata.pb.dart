//
//  Generated code. Do not modify.
//  source: sdk/metadata.proto
//
// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'metadata.pbenum.dart';
import 'transaction/transaction.pb.dart' as $1;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'metadata.pbenum.dart';

/// Attachment represents a file or object linked to a transaction.
class Attachment extends $pb.GeneratedMessage {
  factory Attachment({
    $core.String? objectId,
    Attachment_AttachmentType? type,
  }) {
    final $result = create();
    if (objectId != null) {
      $result.objectId = objectId;
    }
    if (type != null) {
      $result.type = type;
    }
    return $result;
  }
  Attachment._() : super();
  factory Attachment.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Attachment.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Attachment', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'objectId')
    ..e<Attachment_AttachmentType>(2, _omitFieldNames ? '' : 'type', $pb.PbFieldType.OE, defaultOrMaker: Attachment_AttachmentType.OBJECT, valueOf: Attachment_AttachmentType.valueOf, enumValues: Attachment_AttachmentType.values)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Attachment clone() => Attachment()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Attachment copyWith(void Function(Attachment) updates) => super.copyWith((message) => updates(message as Attachment)) as Attachment;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Attachment create() => Attachment._();
  Attachment createEmptyInstance() => create();
  static $pb.PbList<Attachment> createRepeated() => $pb.PbList<Attachment>();
  @$core.pragma('dart2js:noInline')
  static Attachment getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Attachment>(create);
  static Attachment? _defaultInstance;

  /// ID of the attached object.
  @$pb.TagNumber(1)
  $core.String get objectId => $_getSZ(0);
  @$pb.TagNumber(1)
  set objectId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasObjectId() => $_has(0);
  @$pb.TagNumber(1)
  void clearObjectId() => $_clearField(1);

  /// Type of the attachment.
  @$pb.TagNumber(2)
  Attachment_AttachmentType get type => $_getN(1);
  @$pb.TagNumber(2)
  set type(Attachment_AttachmentType v) { $_setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasType() => $_has(1);
  @$pb.TagNumber(2)
  void clearType() => $_clearField(2);
}

/// Memo contains a plaintext message associated with a transaction.
class Memo extends $pb.GeneratedMessage {
  factory Memo({
    $core.String? plaintext,
  }) {
    final $result = create();
    if (plaintext != null) {
      $result.plaintext = plaintext;
    }
    return $result;
  }
  Memo._() : super();
  factory Memo.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Memo.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Memo', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'plaintext')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Memo clone() => Memo()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Memo copyWith(void Function(Memo) updates) => super.copyWith((message) => updates(message as Memo)) as Memo;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Memo create() => Memo._();
  Memo createEmptyInstance() => create();
  static $pb.PbList<Memo> createRepeated() => $pb.PbList<Memo>();
  @$core.pragma('dart2js:noInline')
  static Memo getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Memo>(create);
  static Memo? _defaultInstance;

  /// The plaintext message.
  @$pb.TagNumber(1)
  $core.String get plaintext => $_getSZ(0);
  @$pb.TagNumber(1)
  set plaintext($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasPlaintext() => $_has(0);
  @$pb.TagNumber(1)
  void clearPlaintext() => $_clearField(1);
}

/// Fee is a placeholder for future fee-related metadata.  Currently unused.
class Fee extends $pb.GeneratedMessage {
  factory Fee() => create();
  Fee._() : super();
  factory Fee.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Fee.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Fee', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Fee clone() => Fee()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Fee copyWith(void Function(Fee) updates) => super.copyWith((message) => updates(message as Fee)) as Fee;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Fee create() => Fee._();
  Fee createEmptyInstance() => create();
  static $pb.PbList<Fee> createRepeated() => $pb.PbList<Fee>();
  @$core.pragma('dart2js:noInline')
  static Fee getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Fee>(create);
  static Fee? _defaultInstance;
}

/// Metadata for a withdraw (redeem) transaction.
class Withdraw extends $pb.GeneratedMessage {
  factory Withdraw({
    $core.String? bankAccountId,
  }) {
    final $result = create();
    if (bankAccountId != null) {
      $result.bankAccountId = bankAccountId;
    }
    return $result;
  }
  Withdraw._() : super();
  factory Withdraw.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Withdraw.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Withdraw', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'bankAccountId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Withdraw clone() => Withdraw()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Withdraw copyWith(void Function(Withdraw) updates) => super.copyWith((message) => updates(message as Withdraw)) as Withdraw;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Withdraw create() => Withdraw._();
  Withdraw createEmptyInstance() => create();
  static $pb.PbList<Withdraw> createRepeated() => $pb.PbList<Withdraw>();
  @$core.pragma('dart2js:noInline')
  static Withdraw getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Withdraw>(create);
  static Withdraw? _defaultInstance;

  /// The bank account the withdrawal is to.
  @$pb.TagNumber(1)
  $core.String get bankAccountId => $_getSZ(0);
  @$pb.TagNumber(1)
  set bankAccountId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasBankAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearBankAccountId() => $_clearField(1);
}

/// Metadata for a deposit (issue) transaction.
class Deposit extends $pb.GeneratedMessage {
  factory Deposit({
    $core.String? bankAccountId,
  }) {
    final $result = create();
    if (bankAccountId != null) {
      $result.bankAccountId = bankAccountId;
    }
    return $result;
  }
  Deposit._() : super();
  factory Deposit.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Deposit.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Deposit', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'bankAccountId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Deposit clone() => Deposit()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Deposit copyWith(void Function(Deposit) updates) => super.copyWith((message) => updates(message as Deposit)) as Deposit;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Deposit create() => Deposit._();
  Deposit createEmptyInstance() => create();
  static $pb.PbList<Deposit> createRepeated() => $pb.PbList<Deposit>();
  @$core.pragma('dart2js:noInline')
  static Deposit getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Deposit>(create);
  static Deposit? _defaultInstance;

  /// The bank account the deposit is from.
  @$pb.TagNumber(1)
  $core.String get bankAccountId => $_getSZ(0);
  @$pb.TagNumber(1)
  set bankAccountId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasBankAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearBankAccountId() => $_clearField(1);
}

/// Contract represents metadata for a smart contract.
class Contract extends $pb.GeneratedMessage {
  factory Contract({
    $core.List<$core.int>? transactions,
    $core.Iterable<Endorsement>? endorsements,
  }) {
    final $result = create();
    if (transactions != null) {
      $result.transactions = transactions;
    }
    if (endorsements != null) {
      $result.endorsements.addAll(endorsements);
    }
    return $result;
  }
  Contract._() : super();
  factory Contract.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Contract.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Contract', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'transactions', $pb.PbFieldType.OY)
    ..pc<Endorsement>(2, _omitFieldNames ? '' : 'endorsements', $pb.PbFieldType.PM, subBuilder: Endorsement.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Contract clone() => Contract()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Contract copyWith(void Function(Contract) updates) => super.copyWith((message) => updates(message as Contract)) as Contract;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Contract create() => Contract._();
  Contract createEmptyInstance() => create();
  static $pb.PbList<Contract> createRepeated() => $pb.PbList<Contract>();
  @$core.pragma('dart2js:noInline')
  static Contract getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Contract>(create);
  static Contract? _defaultInstance;

  /// Ledger transactions to be processed, serialized as `CreateLedgerTransfers`.
  @$pb.TagNumber(1)
  $core.List<$core.int> get transactions => $_getN(0);
  @$pb.TagNumber(1)
  set transactions($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTransactions() => $_has(0);
  @$pb.TagNumber(1)
  void clearTransactions() => $_clearField(1);

  /// Endorsements of the contract by authorized parties.  Can be signed by multiple parties.
  @$pb.TagNumber(2)
  $pb.PbList<Endorsement> get endorsements => $_getList(1);
}

/// Approval of a contract by a ledger.
class Endorsement extends $pb.GeneratedMessage {
  factory Endorsement({
    $core.String? ledgerId,
    $1.Signature? signature,
  }) {
    final $result = create();
    if (ledgerId != null) {
      $result.ledgerId = ledgerId;
    }
    if (signature != null) {
      $result.signature = signature;
    }
    return $result;
  }
  Endorsement._() : super();
  factory Endorsement.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Endorsement.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Endorsement', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'ledgerId')
    ..aOM<$1.Signature>(2, _omitFieldNames ? '' : 'signature', subBuilder: $1.Signature.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Endorsement clone() => Endorsement()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Endorsement copyWith(void Function(Endorsement) updates) => super.copyWith((message) => updates(message as Endorsement)) as Endorsement;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Endorsement create() => Endorsement._();
  Endorsement createEmptyInstance() => create();
  static $pb.PbList<Endorsement> createRepeated() => $pb.PbList<Endorsement>();
  @$core.pragma('dart2js:noInline')
  static Endorsement getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Endorsement>(create);
  static Endorsement? _defaultInstance;

  /// The ID of the endorsing ledger.
  @$pb.TagNumber(1)
  $core.String get ledgerId => $_getSZ(0);
  @$pb.TagNumber(1)
  set ledgerId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasLedgerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearLedgerId() => $_clearField(1);

  /// The signature of the endorsement.
  @$pb.TagNumber(2)
  $1.Signature get signature => $_getN(1);
  @$pb.TagNumber(2)
  set signature($1.Signature v) { $_setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasSignature() => $_has(1);
  @$pb.TagNumber(2)
  void clearSignature() => $_clearField(2);
  @$pb.TagNumber(2)
  $1.Signature ensureSignature() => $_ensure(1);
}

/// Metadata for a payment request.
class PaymentRequest extends $pb.GeneratedMessage {
  factory PaymentRequest({
    $1.CreateTransfer? transfer,
    PaymentRequest_PaymentRequestStatus? status,
  }) {
    final $result = create();
    if (transfer != null) {
      $result.transfer = transfer;
    }
    if (status != null) {
      $result.status = status;
    }
    return $result;
  }
  PaymentRequest._() : super();
  factory PaymentRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory PaymentRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'PaymentRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..aOM<$1.CreateTransfer>(1, _omitFieldNames ? '' : 'transfer', subBuilder: $1.CreateTransfer.create)
    ..e<PaymentRequest_PaymentRequestStatus>(2, _omitFieldNames ? '' : 'status', $pb.PbFieldType.OE, defaultOrMaker: PaymentRequest_PaymentRequestStatus.PENDING, valueOf: PaymentRequest_PaymentRequestStatus.valueOf, enumValues: PaymentRequest_PaymentRequestStatus.values)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  PaymentRequest clone() => PaymentRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  PaymentRequest copyWith(void Function(PaymentRequest) updates) => super.copyWith((message) => updates(message as PaymentRequest)) as PaymentRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PaymentRequest create() => PaymentRequest._();
  PaymentRequest createEmptyInstance() => create();
  static $pb.PbList<PaymentRequest> createRepeated() => $pb.PbList<PaymentRequest>();
  @$core.pragma('dart2js:noInline')
  static PaymentRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PaymentRequest>(create);
  static PaymentRequest? _defaultInstance;

  /// The transfer details of a payment request.
  @$pb.TagNumber(1)
  $1.CreateTransfer get transfer => $_getN(0);
  @$pb.TagNumber(1)
  set transfer($1.CreateTransfer v) { $_setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasTransfer() => $_has(0);
  @$pb.TagNumber(1)
  void clearTransfer() => $_clearField(1);
  @$pb.TagNumber(1)
  $1.CreateTransfer ensureTransfer() => $_ensure(0);

  /// The current status of the payment request.
  @$pb.TagNumber(2)
  PaymentRequest_PaymentRequestStatus get status => $_getN(1);
  @$pb.TagNumber(2)
  set status(PaymentRequest_PaymentRequestStatus v) { $_setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasStatus() => $_has(1);
  @$pb.TagNumber(2)
  void clearStatus() => $_clearField(2);
}

/// QuoteRequest represents a request for a foreign exchange quote.
/// Amount should be provided exclusively for either `base` or `target`.
class QuoteRequest extends $pb.GeneratedMessage {
  factory QuoteRequest({
    AccountCurrency? base,
    AccountCurrency? target,
    $core.String? memo,
  }) {
    final $result = create();
    if (base != null) {
      $result.base = base;
    }
    if (target != null) {
      $result.target = target;
    }
    if (memo != null) {
      $result.memo = memo;
    }
    return $result;
  }
  QuoteRequest._() : super();
  factory QuoteRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory QuoteRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'QuoteRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..aOM<AccountCurrency>(1, _omitFieldNames ? '' : 'base', subBuilder: AccountCurrency.create)
    ..aOM<AccountCurrency>(2, _omitFieldNames ? '' : 'target', subBuilder: AccountCurrency.create)
    ..aOS(3, _omitFieldNames ? '' : 'memo')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  QuoteRequest clone() => QuoteRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  QuoteRequest copyWith(void Function(QuoteRequest) updates) => super.copyWith((message) => updates(message as QuoteRequest)) as QuoteRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static QuoteRequest create() => QuoteRequest._();
  QuoteRequest createEmptyInstance() => create();
  static $pb.PbList<QuoteRequest> createRepeated() => $pb.PbList<QuoteRequest>();
  @$core.pragma('dart2js:noInline')
  static QuoteRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<QuoteRequest>(create);
  static QuoteRequest? _defaultInstance;

  /// The base currency and amount for the quote.
  @$pb.TagNumber(1)
  AccountCurrency get base => $_getN(0);
  @$pb.TagNumber(1)
  set base(AccountCurrency v) { $_setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasBase() => $_has(0);
  @$pb.TagNumber(1)
  void clearBase() => $_clearField(1);
  @$pb.TagNumber(1)
  AccountCurrency ensureBase() => $_ensure(0);

  /// The target currency for the quote.
  @$pb.TagNumber(2)
  AccountCurrency get target => $_getN(1);
  @$pb.TagNumber(2)
  set target(AccountCurrency v) { $_setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasTarget() => $_has(1);
  @$pb.TagNumber(2)
  void clearTarget() => $_clearField(2);
  @$pb.TagNumber(2)
  AccountCurrency ensureTarget() => $_ensure(1);

  /// An optional memo associated with the quote request.
  @$pb.TagNumber(3)
  $core.String get memo => $_getSZ(2);
  @$pb.TagNumber(3)
  set memo($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasMemo() => $_has(2);
  @$pb.TagNumber(3)
  void clearMemo() => $_clearField(3);
}

/// AccountCurrency represents a currency and amount associated with a specific account.
class AccountCurrency extends $pb.GeneratedMessage {
  factory AccountCurrency({
    $core.String? operator,
    $core.String? currency,
    $core.List<$core.int>? accountId,
    $fixnum.Int64? amount,
  }) {
    final $result = create();
    if (operator != null) {
      $result.operator = operator;
    }
    if (currency != null) {
      $result.currency = currency;
    }
    if (accountId != null) {
      $result.accountId = accountId;
    }
    if (amount != null) {
      $result.amount = amount;
    }
    return $result;
  }
  AccountCurrency._() : super();
  factory AccountCurrency.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AccountCurrency.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'AccountCurrency', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'operator')
    ..aOS(2, _omitFieldNames ? '' : 'currency')
    ..a<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'accountId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(4, _omitFieldNames ? '' : 'amount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AccountCurrency clone() => AccountCurrency()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AccountCurrency copyWith(void Function(AccountCurrency) updates) => super.copyWith((message) => updates(message as AccountCurrency)) as AccountCurrency;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AccountCurrency create() => AccountCurrency._();
  AccountCurrency createEmptyInstance() => create();
  static $pb.PbList<AccountCurrency> createRepeated() => $pb.PbList<AccountCurrency>();
  @$core.pragma('dart2js:noInline')
  static AccountCurrency getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AccountCurrency>(create);
  static AccountCurrency? _defaultInstance;

  /// The operator of the account.
  @$pb.TagNumber(1)
  $core.String get operator => $_getSZ(0);
  @$pb.TagNumber(1)
  set operator($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasOperator() => $_has(0);
  @$pb.TagNumber(1)
  void clearOperator() => $_clearField(1);

  /// The currency code (e.g., USD, EUR).
  @$pb.TagNumber(2)
  $core.String get currency => $_getSZ(1);
  @$pb.TagNumber(2)
  set currency($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasCurrency() => $_has(1);
  @$pb.TagNumber(2)
  void clearCurrency() => $_clearField(2);

  /// The ledger account ID.
  @$pb.TagNumber(3)
  $core.List<$core.int> get accountId => $_getN(2);
  @$pb.TagNumber(3)
  set accountId($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasAccountId() => $_has(2);
  @$pb.TagNumber(3)
  void clearAccountId() => $_clearField(3);

  /// The amount of the currency.
  @$pb.TagNumber(4)
  $fixnum.Int64 get amount => $_getI64(3);
  @$pb.TagNumber(4)
  set amount($fixnum.Int64 v) { $_setInt64(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasAmount() => $_has(3);
  @$pb.TagNumber(4)
  void clearAmount() => $_clearField(4);
}

enum QuoteEvent_Event {
  request, 
  proposal, 
  notSet
}

/// QuoteEvent represents an event related to a foreign exchange quote.
class QuoteEvent extends $pb.GeneratedMessage {
  factory QuoteEvent({
    QuoteRequest? request,
    Contract? proposal,
  }) {
    final $result = create();
    if (request != null) {
      $result.request = request;
    }
    if (proposal != null) {
      $result.proposal = proposal;
    }
    return $result;
  }
  QuoteEvent._() : super();
  factory QuoteEvent.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory QuoteEvent.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, QuoteEvent_Event> _QuoteEvent_EventByTag = {
    1 : QuoteEvent_Event.request,
    2 : QuoteEvent_Event.proposal,
    0 : QuoteEvent_Event.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'QuoteEvent', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..aOM<QuoteRequest>(1, _omitFieldNames ? '' : 'request', subBuilder: QuoteRequest.create)
    ..aOM<Contract>(2, _omitFieldNames ? '' : 'proposal', subBuilder: Contract.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  QuoteEvent clone() => QuoteEvent()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  QuoteEvent copyWith(void Function(QuoteEvent) updates) => super.copyWith((message) => updates(message as QuoteEvent)) as QuoteEvent;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static QuoteEvent create() => QuoteEvent._();
  QuoteEvent createEmptyInstance() => create();
  static $pb.PbList<QuoteEvent> createRepeated() => $pb.PbList<QuoteEvent>();
  @$core.pragma('dart2js:noInline')
  static QuoteEvent getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<QuoteEvent>(create);
  static QuoteEvent? _defaultInstance;

  QuoteEvent_Event whichEvent() => _QuoteEvent_EventByTag[$_whichOneof(0)]!;
  void clearEvent() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  QuoteRequest get request => $_getN(0);
  @$pb.TagNumber(1)
  set request(QuoteRequest v) { $_setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearRequest() => $_clearField(1);
  @$pb.TagNumber(1)
  QuoteRequest ensureRequest() => $_ensure(0);

  @$pb.TagNumber(2)
  Contract get proposal => $_getN(1);
  @$pb.TagNumber(2)
  set proposal(Contract v) { $_setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasProposal() => $_has(1);
  @$pb.TagNumber(2)
  void clearProposal() => $_clearField(2);
  @$pb.TagNumber(2)
  Contract ensureProposal() => $_ensure(1);
}

/// SelfTransfer represents a transfer between accounts owned by the same user.
class SelfTransfer extends $pb.GeneratedMessage {
  factory SelfTransfer({
    $core.String? fromAccountName,
    $core.String? toAccountName,
  }) {
    final $result = create();
    if (fromAccountName != null) {
      $result.fromAccountName = fromAccountName;
    }
    if (toAccountName != null) {
      $result.toAccountName = toAccountName;
    }
    return $result;
  }
  SelfTransfer._() : super();
  factory SelfTransfer.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SelfTransfer.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'SelfTransfer', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'fromAccountName')
    ..aOS(2, _omitFieldNames ? '' : 'toAccountName')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SelfTransfer clone() => SelfTransfer()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SelfTransfer copyWith(void Function(SelfTransfer) updates) => super.copyWith((message) => updates(message as SelfTransfer)) as SelfTransfer;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SelfTransfer create() => SelfTransfer._();
  SelfTransfer createEmptyInstance() => create();
  static $pb.PbList<SelfTransfer> createRepeated() => $pb.PbList<SelfTransfer>();
  @$core.pragma('dart2js:noInline')
  static SelfTransfer getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SelfTransfer>(create);
  static SelfTransfer? _defaultInstance;

  /// The source account.
  @$pb.TagNumber(1)
  $core.String get fromAccountName => $_getSZ(0);
  @$pb.TagNumber(1)
  set fromAccountName($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasFromAccountName() => $_has(0);
  @$pb.TagNumber(1)
  void clearFromAccountName() => $_clearField(1);

  /// The destination account.
  @$pb.TagNumber(2)
  $core.String get toAccountName => $_getSZ(1);
  @$pb.TagNumber(2)
  set toAccountName($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasToAccountName() => $_has(1);
  @$pb.TagNumber(2)
  void clearToAccountName() => $_clearField(2);
}

/// Metadata for a rebalance transfer.
class RebalanceTransfer extends $pb.GeneratedMessage {
  factory RebalanceTransfer() => create();
  RebalanceTransfer._() : super();
  factory RebalanceTransfer.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RebalanceTransfer.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RebalanceTransfer', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RebalanceTransfer clone() => RebalanceTransfer()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RebalanceTransfer copyWith(void Function(RebalanceTransfer) updates) => super.copyWith((message) => updates(message as RebalanceTransfer)) as RebalanceTransfer;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RebalanceTransfer create() => RebalanceTransfer._();
  RebalanceTransfer createEmptyInstance() => create();
  static $pb.PbList<RebalanceTransfer> createRepeated() => $pb.PbList<RebalanceTransfer>();
  @$core.pragma('dart2js:noInline')
  static RebalanceTransfer getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RebalanceTransfer>(create);
  static RebalanceTransfer? _defaultInstance;
}

/// Metadata for a token withdrawal.
class TokenWithdraw extends $pb.GeneratedMessage {
  factory TokenWithdraw() => create();
  TokenWithdraw._() : super();
  factory TokenWithdraw.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TokenWithdraw.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'TokenWithdraw', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TokenWithdraw clone() => TokenWithdraw()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TokenWithdraw copyWith(void Function(TokenWithdraw) updates) => super.copyWith((message) => updates(message as TokenWithdraw)) as TokenWithdraw;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TokenWithdraw create() => TokenWithdraw._();
  TokenWithdraw createEmptyInstance() => create();
  static $pb.PbList<TokenWithdraw> createRepeated() => $pb.PbList<TokenWithdraw>();
  @$core.pragma('dart2js:noInline')
  static TokenWithdraw getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TokenWithdraw>(create);
  static TokenWithdraw? _defaultInstance;
}

/// Metadata for an offline transfer.
class OfflineTransfer extends $pb.GeneratedMessage {
  factory OfflineTransfer({
    $fixnum.Int64? input,
  }) {
    final $result = create();
    if (input != null) {
      $result.input = input;
    }
    return $result;
  }
  OfflineTransfer._() : super();
  factory OfflineTransfer.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory OfflineTransfer.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'OfflineTransfer', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.metadata'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'input', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  OfflineTransfer clone() => OfflineTransfer()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  OfflineTransfer copyWith(void Function(OfflineTransfer) updates) => super.copyWith((message) => updates(message as OfflineTransfer)) as OfflineTransfer;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static OfflineTransfer create() => OfflineTransfer._();
  OfflineTransfer createEmptyInstance() => create();
  static $pb.PbList<OfflineTransfer> createRepeated() => $pb.PbList<OfflineTransfer>();
  @$core.pragma('dart2js:noInline')
  static OfflineTransfer getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<OfflineTransfer>(create);
  static OfflineTransfer? _defaultInstance;

  /// Input value for the offline transfer.
  @$pb.TagNumber(1)
  $fixnum.Int64 get input => $_getI64(0);
  @$pb.TagNumber(1)
  set input($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasInput() => $_has(0);
  @$pb.TagNumber(1)
  void clearInput() => $_clearField(1);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
