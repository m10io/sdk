//
//  Generated code. Do not modify.
//  source: sdk/model/model.proto
//
// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'model.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'model.pbenum.dart';

class AccountMetadata extends $pb.GeneratedMessage {
  factory AccountMetadata({
    $core.List<$core.int>? owner,
    $core.String? profileImageUrl,
    $core.String? name,
    $core.String? publicName,
    $core.List<$core.int>? id,
  }) {
    final $result = create();
    if (owner != null) {
      $result.owner = owner;
    }
    if (profileImageUrl != null) {
      $result.profileImageUrl = profileImageUrl;
    }
    if (name != null) {
      $result.name = name;
    }
    if (publicName != null) {
      $result.publicName = publicName;
    }
    if (id != null) {
      $result.id = id;
    }
    return $result;
  }
  AccountMetadata._() : super();
  factory AccountMetadata.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AccountMetadata.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'AccountMetadata', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.model'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'owner', $pb.PbFieldType.OY)
    ..aOS(9, _omitFieldNames ? '' : 'profileImageUrl')
    ..aOS(10, _omitFieldNames ? '' : 'name')
    ..aOS(11, _omitFieldNames ? '' : 'publicName')
    ..a<$core.List<$core.int>>(12, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AccountMetadata clone() => AccountMetadata()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AccountMetadata copyWith(void Function(AccountMetadata) updates) => super.copyWith((message) => updates(message as AccountMetadata)) as AccountMetadata;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AccountMetadata create() => AccountMetadata._();
  AccountMetadata createEmptyInstance() => create();
  static $pb.PbList<AccountMetadata> createRepeated() => $pb.PbList<AccountMetadata>();
  @$core.pragma('dart2js:noInline')
  static AccountMetadata getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AccountMetadata>(create);
  static AccountMetadata? _defaultInstance;

  /// / The owner of the AccountMetadata
  @$pb.TagNumber(1)
  $core.List<$core.int> get owner => $_getN(0);
  @$pb.TagNumber(1)
  set owner($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasOwner() => $_has(0);
  @$pb.TagNumber(1)
  void clearOwner() => $_clearField(1);

  /// / URL of the profile image associated with the account.
  @$pb.TagNumber(9)
  $core.String get profileImageUrl => $_getSZ(1);
  @$pb.TagNumber(9)
  set profileImageUrl($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(9)
  $core.bool hasProfileImageUrl() => $_has(1);
  @$pb.TagNumber(9)
  void clearProfileImageUrl() => $_clearField(9);

  /// / Name of the AccountMetadata.
  @$pb.TagNumber(10)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(10)
  set name($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(10)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(10)
  void clearName() => $_clearField(10);

  /// / Public name of the AccountMetadata.
  @$pb.TagNumber(11)
  $core.String get publicName => $_getSZ(3);
  @$pb.TagNumber(11)
  set publicName($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(11)
  $core.bool hasPublicName() => $_has(3);
  @$pb.TagNumber(11)
  void clearPublicName() => $_clearField(11);

  /// / Unique identifier (e.g. UUID) for the AccountMetadata.
  @$pb.TagNumber(12)
  $core.List<$core.int> get id => $_getN(4);
  @$pb.TagNumber(12)
  set id($core.List<$core.int> v) { $_setBytes(4, v); }
  @$pb.TagNumber(12)
  $core.bool hasId() => $_has(4);
  @$pb.TagNumber(12)
  void clearId() => $_clearField(12);
}

class AccountSet extends $pb.GeneratedMessage {
  factory AccountSet({
    $core.List<$core.int>? owner,
    $core.Iterable<$core.List<$core.int>>? accounts,
    $core.List<$core.int>? id,
  }) {
    final $result = create();
    if (owner != null) {
      $result.owner = owner;
    }
    if (accounts != null) {
      $result.accounts.addAll(accounts);
    }
    if (id != null) {
      $result.id = id;
    }
    return $result;
  }
  AccountSet._() : super();
  factory AccountSet.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AccountSet.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'AccountSet', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.model'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'owner', $pb.PbFieldType.OY)
    ..p<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'accounts', $pb.PbFieldType.PY)
    ..a<$core.List<$core.int>>(7, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AccountSet clone() => AccountSet()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AccountSet copyWith(void Function(AccountSet) updates) => super.copyWith((message) => updates(message as AccountSet)) as AccountSet;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AccountSet create() => AccountSet._();
  AccountSet createEmptyInstance() => create();
  static $pb.PbList<AccountSet> createRepeated() => $pb.PbList<AccountSet>();
  @$core.pragma('dart2js:noInline')
  static AccountSet getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AccountSet>(create);
  static AccountSet? _defaultInstance;

  /// / The owner of the AccountSet.
  @$pb.TagNumber(1)
  $core.List<$core.int> get owner => $_getN(0);
  @$pb.TagNumber(1)
  set owner($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasOwner() => $_has(0);
  @$pb.TagNumber(1)
  void clearOwner() => $_clearField(1);

  /// / List of account references within the AccountSet.
  @$pb.TagNumber(2)
  $pb.PbList<$core.List<$core.int>> get accounts => $_getList(1);

  /// / Unique identifier (e.g. UUID) for the AccountSet.
  @$pb.TagNumber(7)
  $core.List<$core.int> get id => $_getN(2);
  @$pb.TagNumber(7)
  set id($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(7)
  $core.bool hasId() => $_has(2);
  @$pb.TagNumber(7)
  void clearId() => $_clearField(7);
}

class AccountInfo extends $pb.GeneratedMessage {
  factory AccountInfo({
    $core.List<$core.int>? accountId,
    $core.List<$core.int>? parentAccountId,
    $core.String? publicName,
    $core.String? profileImageUrl,
    $core.String? code,
    $core.int? decimalPlaces,
  }) {
    final $result = create();
    if (accountId != null) {
      $result.accountId = accountId;
    }
    if (parentAccountId != null) {
      $result.parentAccountId = parentAccountId;
    }
    if (publicName != null) {
      $result.publicName = publicName;
    }
    if (profileImageUrl != null) {
      $result.profileImageUrl = profileImageUrl;
    }
    if (code != null) {
      $result.code = code;
    }
    if (decimalPlaces != null) {
      $result.decimalPlaces = decimalPlaces;
    }
    return $result;
  }
  AccountInfo._() : super();
  factory AccountInfo.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AccountInfo.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'AccountInfo', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.model'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'accountId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'parentAccountId', $pb.PbFieldType.OY)
    ..aOS(3, _omitFieldNames ? '' : 'publicName')
    ..aOS(4, _omitFieldNames ? '' : 'profileImageUrl')
    ..aOS(5, _omitFieldNames ? '' : 'code')
    ..a<$core.int>(6, _omitFieldNames ? '' : 'decimalPlaces', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AccountInfo clone() => AccountInfo()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AccountInfo copyWith(void Function(AccountInfo) updates) => super.copyWith((message) => updates(message as AccountInfo)) as AccountInfo;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static AccountInfo create() => AccountInfo._();
  AccountInfo createEmptyInstance() => create();
  static $pb.PbList<AccountInfo> createRepeated() => $pb.PbList<AccountInfo>();
  @$core.pragma('dart2js:noInline')
  static AccountInfo getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AccountInfo>(create);
  static AccountInfo? _defaultInstance;

  /// / Unique identifier for the account.
  @$pb.TagNumber(1)
  $core.List<$core.int> get accountId => $_getN(0);
  @$pb.TagNumber(1)
  set accountId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearAccountId() => $_clearField(1);

  /// / Identifier of the parent account, if any.
  @$pb.TagNumber(2)
  $core.List<$core.int> get parentAccountId => $_getN(1);
  @$pb.TagNumber(2)
  set parentAccountId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasParentAccountId() => $_has(1);
  @$pb.TagNumber(2)
  void clearParentAccountId() => $_clearField(2);

  /// / Public name of the account.
  @$pb.TagNumber(3)
  $core.String get publicName => $_getSZ(2);
  @$pb.TagNumber(3)
  set publicName($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasPublicName() => $_has(2);
  @$pb.TagNumber(3)
  void clearPublicName() => $_clearField(3);

  /// / URL of the account's profile image.
  @$pb.TagNumber(4)
  $core.String get profileImageUrl => $_getSZ(3);
  @$pb.TagNumber(4)
  set profileImageUrl($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasProfileImageUrl() => $_has(3);
  @$pb.TagNumber(4)
  void clearProfileImageUrl() => $_clearField(4);

  /// / Account code (e.g., currency code).
  @$pb.TagNumber(5)
  $core.String get code => $_getSZ(4);
  @$pb.TagNumber(5)
  set code($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasCode() => $_has(4);
  @$pb.TagNumber(5)
  void clearCode() => $_clearField(5);

  /// / Number of decimal places for the account's value.
  @$pb.TagNumber(6)
  $core.int get decimalPlaces => $_getIZ(5);
  @$pb.TagNumber(6)
  set decimalPlaces($core.int v) { $_setUnsignedInt32(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasDecimalPlaces() => $_has(5);
  @$pb.TagNumber(6)
  void clearDecimalPlaces() => $_clearField(6);
}

class BankAccountRef extends $pb.GeneratedMessage {
  factory BankAccountRef({
    $core.List<$core.int>? accountId,
    BankAccountRef_BankAccountType? accountType,
  }) {
    final $result = create();
    if (accountId != null) {
      $result.accountId = accountId;
    }
    if (accountType != null) {
      $result.accountType = accountType;
    }
    return $result;
  }
  BankAccountRef._() : super();
  factory BankAccountRef.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BankAccountRef.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'BankAccountRef', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.model'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'accountId', $pb.PbFieldType.OY)
    ..e<BankAccountRef_BankAccountType>(2, _omitFieldNames ? '' : 'accountType', $pb.PbFieldType.OE, defaultOrMaker: BankAccountRef_BankAccountType.CBDC, valueOf: BankAccountRef_BankAccountType.valueOf, enumValues: BankAccountRef_BankAccountType.values)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BankAccountRef clone() => BankAccountRef()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BankAccountRef copyWith(void Function(BankAccountRef) updates) => super.copyWith((message) => updates(message as BankAccountRef)) as BankAccountRef;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BankAccountRef create() => BankAccountRef._();
  BankAccountRef createEmptyInstance() => create();
  static $pb.PbList<BankAccountRef> createRepeated() => $pb.PbList<BankAccountRef>();
  @$core.pragma('dart2js:noInline')
  static BankAccountRef getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BankAccountRef>(create);
  static BankAccountRef? _defaultInstance;

  /// / Unique identifier for the bank account.
  @$pb.TagNumber(1)
  $core.List<$core.int> get accountId => $_getN(0);
  @$pb.TagNumber(1)
  set accountId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearAccountId() => $_clearField(1);

  /// / Bank account type.
  @$pb.TagNumber(2)
  BankAccountRef_BankAccountType get accountType => $_getN(1);
  @$pb.TagNumber(2)
  set accountType(BankAccountRef_BankAccountType v) { $_setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasAccountType() => $_has(1);
  @$pb.TagNumber(2)
  void clearAccountType() => $_clearField(2);
}

class Bank extends $pb.GeneratedMessage {
  factory Bank({
    $core.List<$core.int>? id,
    $core.List<$core.int>? owner,
    $core.String? shortName,
    $core.String? displayName,
    $core.Iterable<BankAccountRef>? accounts,
  }) {
    final $result = create();
    if (id != null) {
      $result.id = id;
    }
    if (owner != null) {
      $result.owner = owner;
    }
    if (shortName != null) {
      $result.shortName = shortName;
    }
    if (displayName != null) {
      $result.displayName = displayName;
    }
    if (accounts != null) {
      $result.accounts.addAll(accounts);
    }
    return $result;
  }
  Bank._() : super();
  factory Bank.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Bank.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Bank', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk.model'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'owner', $pb.PbFieldType.OY)
    ..aOS(3, _omitFieldNames ? '' : 'shortName')
    ..aOS(4, _omitFieldNames ? '' : 'displayName')
    ..pc<BankAccountRef>(5, _omitFieldNames ? '' : 'accounts', $pb.PbFieldType.PM, subBuilder: BankAccountRef.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Bank clone() => Bank()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Bank copyWith(void Function(Bank) updates) => super.copyWith((message) => updates(message as Bank)) as Bank;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Bank create() => Bank._();
  Bank createEmptyInstance() => create();
  static $pb.PbList<Bank> createRepeated() => $pb.PbList<Bank>();
  @$core.pragma('dart2js:noInline')
  static Bank getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Bank>(create);
  static Bank? _defaultInstance;

  /// / Unique identifier for the bank.
  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  /// / Owner of the bank collection record.
  @$pb.TagNumber(2)
  $core.List<$core.int> get owner => $_getN(1);
  @$pb.TagNumber(2)
  set owner($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasOwner() => $_has(1);
  @$pb.TagNumber(2)
  void clearOwner() => $_clearField(2);

  /// / Short name of the bank.
  @$pb.TagNumber(3)
  $core.String get shortName => $_getSZ(2);
  @$pb.TagNumber(3)
  set shortName($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasShortName() => $_has(2);
  @$pb.TagNumber(3)
  void clearShortName() => $_clearField(3);

  /// / Display name of the bank.
  @$pb.TagNumber(4)
  $core.String get displayName => $_getSZ(3);
  @$pb.TagNumber(4)
  set displayName($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasDisplayName() => $_has(3);
  @$pb.TagNumber(4)
  void clearDisplayName() => $_clearField(4);

  /// / List of BankAccountRef's.
  @$pb.TagNumber(5)
  $pb.PbList<BankAccountRef> get accounts => $_getList(4);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
