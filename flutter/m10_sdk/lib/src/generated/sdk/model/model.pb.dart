///
//  Generated code. Do not modify.
//  source: sdk/model/model.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'model.pbenum.dart';

export 'model.pbenum.dart';

class AccountMetadata extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'AccountMetadata', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.model'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'owner', $pb.PbFieldType.OY)
    ..aOS(9, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'profileImageUrl')
    ..aOS(10, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..aOS(11, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'publicName')
    ..a<$core.List<$core.int>>(12, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  AccountMetadata._() : super();
  factory AccountMetadata({
    $core.List<$core.int>? owner,
    $core.String? profileImageUrl,
    $core.String? name,
    $core.String? publicName,
    $core.List<$core.int>? id,
  }) {
    final _result = create();
    if (owner != null) {
      _result.owner = owner;
    }
    if (profileImageUrl != null) {
      _result.profileImageUrl = profileImageUrl;
    }
    if (name != null) {
      _result.name = name;
    }
    if (publicName != null) {
      _result.publicName = publicName;
    }
    if (id != null) {
      _result.id = id;
    }
    return _result;
  }
  factory AccountMetadata.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AccountMetadata.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AccountMetadata clone() => AccountMetadata()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AccountMetadata copyWith(void Function(AccountMetadata) updates) => super.copyWith((message) => updates(message as AccountMetadata)) as AccountMetadata; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static AccountMetadata create() => AccountMetadata._();
  AccountMetadata createEmptyInstance() => create();
  static $pb.PbList<AccountMetadata> createRepeated() => $pb.PbList<AccountMetadata>();
  @$core.pragma('dart2js:noInline')
  static AccountMetadata getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AccountMetadata>(create);
  static AccountMetadata? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get owner => $_getN(0);
  @$pb.TagNumber(1)
  set owner($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasOwner() => $_has(0);
  @$pb.TagNumber(1)
  void clearOwner() => clearField(1);

  @$pb.TagNumber(9)
  $core.String get profileImageUrl => $_getSZ(1);
  @$pb.TagNumber(9)
  set profileImageUrl($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(9)
  $core.bool hasProfileImageUrl() => $_has(1);
  @$pb.TagNumber(9)
  void clearProfileImageUrl() => clearField(9);

  @$pb.TagNumber(10)
  $core.String get name => $_getSZ(2);
  @$pb.TagNumber(10)
  set name($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(10)
  $core.bool hasName() => $_has(2);
  @$pb.TagNumber(10)
  void clearName() => clearField(10);

  @$pb.TagNumber(11)
  $core.String get publicName => $_getSZ(3);
  @$pb.TagNumber(11)
  set publicName($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(11)
  $core.bool hasPublicName() => $_has(3);
  @$pb.TagNumber(11)
  void clearPublicName() => clearField(11);

  @$pb.TagNumber(12)
  $core.List<$core.int> get id => $_getN(4);
  @$pb.TagNumber(12)
  set id($core.List<$core.int> v) { $_setBytes(4, v); }
  @$pb.TagNumber(12)
  $core.bool hasId() => $_has(4);
  @$pb.TagNumber(12)
  void clearId() => clearField(12);
}

class AccountRef extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'AccountRef', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.model'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'ledgerId')
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  AccountRef._() : super();
  factory AccountRef({
    $core.String? ledgerId,
    $core.List<$core.int>? accountId,
  }) {
    final _result = create();
    if (ledgerId != null) {
      _result.ledgerId = ledgerId;
    }
    if (accountId != null) {
      _result.accountId = accountId;
    }
    return _result;
  }
  factory AccountRef.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AccountRef.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AccountRef clone() => AccountRef()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AccountRef copyWith(void Function(AccountRef) updates) => super.copyWith((message) => updates(message as AccountRef)) as AccountRef; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static AccountRef create() => AccountRef._();
  AccountRef createEmptyInstance() => create();
  static $pb.PbList<AccountRef> createRepeated() => $pb.PbList<AccountRef>();
  @$core.pragma('dart2js:noInline')
  static AccountRef getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AccountRef>(create);
  static AccountRef? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get ledgerId => $_getSZ(0);
  @$pb.TagNumber(1)
  set ledgerId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasLedgerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearLedgerId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get accountId => $_getN(1);
  @$pb.TagNumber(2)
  set accountId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasAccountId() => $_has(1);
  @$pb.TagNumber(2)
  void clearAccountId() => clearField(2);
}

class AccountSet extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'AccountSet', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.model'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'owner', $pb.PbFieldType.OY)
    ..pc<AccountRef>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accounts', $pb.PbFieldType.PM, subBuilder: AccountRef.create)
    ..a<$core.List<$core.int>>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  AccountSet._() : super();
  factory AccountSet({
    $core.List<$core.int>? owner,
    $core.Iterable<AccountRef>? accounts,
    $core.List<$core.int>? id,
  }) {
    final _result = create();
    if (owner != null) {
      _result.owner = owner;
    }
    if (accounts != null) {
      _result.accounts.addAll(accounts);
    }
    if (id != null) {
      _result.id = id;
    }
    return _result;
  }
  factory AccountSet.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AccountSet.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AccountSet clone() => AccountSet()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AccountSet copyWith(void Function(AccountSet) updates) => super.copyWith((message) => updates(message as AccountSet)) as AccountSet; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static AccountSet create() => AccountSet._();
  AccountSet createEmptyInstance() => create();
  static $pb.PbList<AccountSet> createRepeated() => $pb.PbList<AccountSet>();
  @$core.pragma('dart2js:noInline')
  static AccountSet getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AccountSet>(create);
  static AccountSet? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get owner => $_getN(0);
  @$pb.TagNumber(1)
  set owner($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasOwner() => $_has(0);
  @$pb.TagNumber(1)
  void clearOwner() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<AccountRef> get accounts => $_getList(1);

  @$pb.TagNumber(7)
  $core.List<$core.int> get id => $_getN(2);
  @$pb.TagNumber(7)
  set id($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(7)
  $core.bool hasId() => $_has(2);
  @$pb.TagNumber(7)
  void clearId() => clearField(7);
}

class AccountInfo extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'AccountInfo', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.model'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountId', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'parentAccountId', $pb.PbFieldType.OY)
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'publicName')
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'profileImageUrl')
    ..aOS(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'code')
    ..a<$core.int>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'decimalPlaces', $pb.PbFieldType.OU3)
    ..hasRequiredFields = false
  ;

  AccountInfo._() : super();
  factory AccountInfo({
    $core.List<$core.int>? accountId,
    $core.List<$core.int>? parentAccountId,
    $core.String? publicName,
    $core.String? profileImageUrl,
    $core.String? code,
    $core.int? decimalPlaces,
  }) {
    final _result = create();
    if (accountId != null) {
      _result.accountId = accountId;
    }
    if (parentAccountId != null) {
      _result.parentAccountId = parentAccountId;
    }
    if (publicName != null) {
      _result.publicName = publicName;
    }
    if (profileImageUrl != null) {
      _result.profileImageUrl = profileImageUrl;
    }
    if (code != null) {
      _result.code = code;
    }
    if (decimalPlaces != null) {
      _result.decimalPlaces = decimalPlaces;
    }
    return _result;
  }
  factory AccountInfo.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory AccountInfo.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  AccountInfo clone() => AccountInfo()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  AccountInfo copyWith(void Function(AccountInfo) updates) => super.copyWith((message) => updates(message as AccountInfo)) as AccountInfo; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static AccountInfo create() => AccountInfo._();
  AccountInfo createEmptyInstance() => create();
  static $pb.PbList<AccountInfo> createRepeated() => $pb.PbList<AccountInfo>();
  @$core.pragma('dart2js:noInline')
  static AccountInfo getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<AccountInfo>(create);
  static AccountInfo? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get accountId => $_getN(0);
  @$pb.TagNumber(1)
  set accountId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearAccountId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get parentAccountId => $_getN(1);
  @$pb.TagNumber(2)
  set parentAccountId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasParentAccountId() => $_has(1);
  @$pb.TagNumber(2)
  void clearParentAccountId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get publicName => $_getSZ(2);
  @$pb.TagNumber(3)
  set publicName($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasPublicName() => $_has(2);
  @$pb.TagNumber(3)
  void clearPublicName() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get profileImageUrl => $_getSZ(3);
  @$pb.TagNumber(4)
  set profileImageUrl($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasProfileImageUrl() => $_has(3);
  @$pb.TagNumber(4)
  void clearProfileImageUrl() => clearField(4);

  @$pb.TagNumber(5)
  $core.String get code => $_getSZ(4);
  @$pb.TagNumber(5)
  set code($core.String v) { $_setString(4, v); }
  @$pb.TagNumber(5)
  $core.bool hasCode() => $_has(4);
  @$pb.TagNumber(5)
  void clearCode() => clearField(5);

  @$pb.TagNumber(6)
  $core.int get decimalPlaces => $_getIZ(5);
  @$pb.TagNumber(6)
  set decimalPlaces($core.int v) { $_setUnsignedInt32(5, v); }
  @$pb.TagNumber(6)
  $core.bool hasDecimalPlaces() => $_has(5);
  @$pb.TagNumber(6)
  void clearDecimalPlaces() => clearField(6);
}

class BankAccountRef extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'BankAccountRef', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.model'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountId', $pb.PbFieldType.OY)
    ..e<BankAccountRef_BankAccountType>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountType', $pb.PbFieldType.OE, defaultOrMaker: BankAccountRef_BankAccountType.CBDC, valueOf: BankAccountRef_BankAccountType.valueOf, enumValues: BankAccountRef_BankAccountType.values)
    ..hasRequiredFields = false
  ;

  BankAccountRef._() : super();
  factory BankAccountRef({
    $core.List<$core.int>? accountId,
    BankAccountRef_BankAccountType? accountType,
  }) {
    final _result = create();
    if (accountId != null) {
      _result.accountId = accountId;
    }
    if (accountType != null) {
      _result.accountType = accountType;
    }
    return _result;
  }
  factory BankAccountRef.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BankAccountRef.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BankAccountRef clone() => BankAccountRef()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BankAccountRef copyWith(void Function(BankAccountRef) updates) => super.copyWith((message) => updates(message as BankAccountRef)) as BankAccountRef; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static BankAccountRef create() => BankAccountRef._();
  BankAccountRef createEmptyInstance() => create();
  static $pb.PbList<BankAccountRef> createRepeated() => $pb.PbList<BankAccountRef>();
  @$core.pragma('dart2js:noInline')
  static BankAccountRef getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BankAccountRef>(create);
  static BankAccountRef? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get accountId => $_getN(0);
  @$pb.TagNumber(1)
  set accountId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearAccountId() => clearField(1);

  @$pb.TagNumber(2)
  BankAccountRef_BankAccountType get accountType => $_getN(1);
  @$pb.TagNumber(2)
  set accountType(BankAccountRef_BankAccountType v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasAccountType() => $_has(1);
  @$pb.TagNumber(2)
  void clearAccountType() => clearField(2);
}

class Bank extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Bank', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk.model'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'owner', $pb.PbFieldType.OY)
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'shortName')
    ..aOS(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'displayName')
    ..pc<BankAccountRef>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accounts', $pb.PbFieldType.PM, subBuilder: BankAccountRef.create)
    ..hasRequiredFields = false
  ;

  Bank._() : super();
  factory Bank({
    $core.List<$core.int>? id,
    $core.List<$core.int>? owner,
    $core.String? shortName,
    $core.String? displayName,
    $core.Iterable<BankAccountRef>? accounts,
  }) {
    final _result = create();
    if (id != null) {
      _result.id = id;
    }
    if (owner != null) {
      _result.owner = owner;
    }
    if (shortName != null) {
      _result.shortName = shortName;
    }
    if (displayName != null) {
      _result.displayName = displayName;
    }
    if (accounts != null) {
      _result.accounts.addAll(accounts);
    }
    return _result;
  }
  factory Bank.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Bank.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Bank clone() => Bank()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Bank copyWith(void Function(Bank) updates) => super.copyWith((message) => updates(message as Bank)) as Bank; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Bank create() => Bank._();
  Bank createEmptyInstance() => create();
  static $pb.PbList<Bank> createRepeated() => $pb.PbList<Bank>();
  @$core.pragma('dart2js:noInline')
  static Bank getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Bank>(create);
  static Bank? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get owner => $_getN(1);
  @$pb.TagNumber(2)
  set owner($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasOwner() => $_has(1);
  @$pb.TagNumber(2)
  void clearOwner() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get shortName => $_getSZ(2);
  @$pb.TagNumber(3)
  set shortName($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasShortName() => $_has(2);
  @$pb.TagNumber(3)
  void clearShortName() => clearField(3);

  @$pb.TagNumber(4)
  $core.String get displayName => $_getSZ(3);
  @$pb.TagNumber(4)
  set displayName($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasDisplayName() => $_has(3);
  @$pb.TagNumber(4)
  void clearDisplayName() => clearField(4);

  @$pb.TagNumber(5)
  $core.List<BankAccountRef> get accounts => $_getList(4);
}

