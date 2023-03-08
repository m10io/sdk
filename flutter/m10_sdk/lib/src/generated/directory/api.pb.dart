///
//  Generated code. Do not modify.
//  source: directory/api.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'api.pbenum.dart';

export 'api.pbenum.dart';

class Ledger extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Ledger', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.directory'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'operator')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'url')
    ..hasRequiredFields = false
  ;

  Ledger._() : super();
  factory Ledger({
    $core.String? operator,
    $core.String? url,
  }) {
    final _result = create();
    if (operator != null) {
      _result.operator = operator;
    }
    if (url != null) {
      _result.url = url;
    }
    return _result;
  }
  factory Ledger.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Ledger.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Ledger clone() => Ledger()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Ledger copyWith(void Function(Ledger) updates) => super.copyWith((message) => updates(message as Ledger)) as Ledger; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Ledger create() => Ledger._();
  Ledger createEmptyInstance() => create();
  static $pb.PbList<Ledger> createRepeated() => $pb.PbList<Ledger>();
  @$core.pragma('dart2js:noInline')
  static Ledger getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Ledger>(create);
  static Ledger? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get operator => $_getSZ(0);
  @$pb.TagNumber(1)
  set operator($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasOperator() => $_has(0);
  @$pb.TagNumber(1)
  void clearOperator() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get url => $_getSZ(1);
  @$pb.TagNumber(2)
  set url($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasUrl() => $_has(1);
  @$pb.TagNumber(2)
  void clearUrl() => clearField(2);
}

class ListLedgersResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ListLedgersResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.directory'), createEmptyInstance: create)
    ..pc<Ledger>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'ledgers', $pb.PbFieldType.PM, subBuilder: Ledger.create)
    ..hasRequiredFields = false
  ;

  ListLedgersResponse._() : super();
  factory ListLedgersResponse({
    $core.Iterable<Ledger>? ledgers,
  }) {
    final _result = create();
    if (ledgers != null) {
      _result.ledgers.addAll(ledgers);
    }
    return _result;
  }
  factory ListLedgersResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListLedgersResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListLedgersResponse clone() => ListLedgersResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListLedgersResponse copyWith(void Function(ListLedgersResponse) updates) => super.copyWith((message) => updates(message as ListLedgersResponse)) as ListLedgersResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ListLedgersResponse create() => ListLedgersResponse._();
  ListLedgersResponse createEmptyInstance() => create();
  static $pb.PbList<ListLedgersResponse> createRepeated() => $pb.PbList<ListLedgersResponse>();
  @$core.pragma('dart2js:noInline')
  static ListLedgersResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListLedgersResponse>(create);
  static ListLedgersResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<Ledger> get ledgers => $_getList(0);
}

class Alias extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Alias', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.directory'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'handle')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'displayName')
    ..a<$core.List<$core.int>>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountSetId', $pb.PbFieldType.OY)
    ..aOS(8, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'operator')
    ..e<Alias_Type>(10, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'aliasType', $pb.PbFieldType.OE, defaultOrMaker: Alias_Type.HANDLE, valueOf: Alias_Type.valueOf, enumValues: Alias_Type.values)
    ..hasRequiredFields = false
  ;

  Alias._() : super();
  factory Alias({
    $core.String? handle,
    $core.String? displayName,
    $core.List<$core.int>? accountSetId,
    $core.String? operator,
    Alias_Type? aliasType,
  }) {
    final _result = create();
    if (handle != null) {
      _result.handle = handle;
    }
    if (displayName != null) {
      _result.displayName = displayName;
    }
    if (accountSetId != null) {
      _result.accountSetId = accountSetId;
    }
    if (operator != null) {
      _result.operator = operator;
    }
    if (aliasType != null) {
      _result.aliasType = aliasType;
    }
    return _result;
  }
  factory Alias.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Alias.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Alias clone() => Alias()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Alias copyWith(void Function(Alias) updates) => super.copyWith((message) => updates(message as Alias)) as Alias; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Alias create() => Alias._();
  Alias createEmptyInstance() => create();
  static $pb.PbList<Alias> createRepeated() => $pb.PbList<Alias>();
  @$core.pragma('dart2js:noInline')
  static Alias getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Alias>(create);
  static Alias? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get handle => $_getSZ(0);
  @$pb.TagNumber(1)
  set handle($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasHandle() => $_has(0);
  @$pb.TagNumber(1)
  void clearHandle() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get displayName => $_getSZ(1);
  @$pb.TagNumber(2)
  set displayName($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasDisplayName() => $_has(1);
  @$pb.TagNumber(2)
  void clearDisplayName() => clearField(2);

  @$pb.TagNumber(5)
  $core.List<$core.int> get accountSetId => $_getN(2);
  @$pb.TagNumber(5)
  set accountSetId($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(5)
  $core.bool hasAccountSetId() => $_has(2);
  @$pb.TagNumber(5)
  void clearAccountSetId() => clearField(5);

  @$pb.TagNumber(8)
  $core.String get operator => $_getSZ(3);
  @$pb.TagNumber(8)
  set operator($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(8)
  $core.bool hasOperator() => $_has(3);
  @$pb.TagNumber(8)
  void clearOperator() => clearField(8);

  @$pb.TagNumber(10)
  Alias_Type get aliasType => $_getN(4);
  @$pb.TagNumber(10)
  set aliasType(Alias_Type v) { setField(10, v); }
  @$pb.TagNumber(10)
  $core.bool hasAliasType() => $_has(4);
  @$pb.TagNumber(10)
  void clearAliasType() => clearField(10);
}

class CheckAliasRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'CheckAliasRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.directory'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'handle')
    ..hasRequiredFields = false
  ;

  CheckAliasRequest._() : super();
  factory CheckAliasRequest({
    $core.String? handle,
  }) {
    final _result = create();
    if (handle != null) {
      _result.handle = handle;
    }
    return _result;
  }
  factory CheckAliasRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CheckAliasRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CheckAliasRequest clone() => CheckAliasRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CheckAliasRequest copyWith(void Function(CheckAliasRequest) updates) => super.copyWith((message) => updates(message as CheckAliasRequest)) as CheckAliasRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static CheckAliasRequest create() => CheckAliasRequest._();
  CheckAliasRequest createEmptyInstance() => create();
  static $pb.PbList<CheckAliasRequest> createRepeated() => $pb.PbList<CheckAliasRequest>();
  @$core.pragma('dart2js:noInline')
  static CheckAliasRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CheckAliasRequest>(create);
  static CheckAliasRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get handle => $_getSZ(0);
  @$pb.TagNumber(1)
  set handle($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasHandle() => $_has(0);
  @$pb.TagNumber(1)
  void clearHandle() => clearField(1);
}

class SearchAliasesRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SearchAliasesRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.directory'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'handlePrefix')
    ..a<$core.int>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'pageSize', $pb.PbFieldType.O3)
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'pageToken')
    ..aOS(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'subject')
    ..hasRequiredFields = false
  ;

  SearchAliasesRequest._() : super();
  factory SearchAliasesRequest({
    $core.String? handlePrefix,
    $core.int? pageSize,
    $core.String? pageToken,
    $core.String? subject,
  }) {
    final _result = create();
    if (handlePrefix != null) {
      _result.handlePrefix = handlePrefix;
    }
    if (pageSize != null) {
      _result.pageSize = pageSize;
    }
    if (pageToken != null) {
      _result.pageToken = pageToken;
    }
    if (subject != null) {
      _result.subject = subject;
    }
    return _result;
  }
  factory SearchAliasesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SearchAliasesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SearchAliasesRequest clone() => SearchAliasesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SearchAliasesRequest copyWith(void Function(SearchAliasesRequest) updates) => super.copyWith((message) => updates(message as SearchAliasesRequest)) as SearchAliasesRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SearchAliasesRequest create() => SearchAliasesRequest._();
  SearchAliasesRequest createEmptyInstance() => create();
  static $pb.PbList<SearchAliasesRequest> createRepeated() => $pb.PbList<SearchAliasesRequest>();
  @$core.pragma('dart2js:noInline')
  static SearchAliasesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SearchAliasesRequest>(create);
  static SearchAliasesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get handlePrefix => $_getSZ(0);
  @$pb.TagNumber(1)
  set handlePrefix($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasHandlePrefix() => $_has(0);
  @$pb.TagNumber(1)
  void clearHandlePrefix() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get pageSize => $_getIZ(1);
  @$pb.TagNumber(2)
  set pageSize($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasPageSize() => $_has(1);
  @$pb.TagNumber(2)
  void clearPageSize() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get pageToken => $_getSZ(2);
  @$pb.TagNumber(3)
  set pageToken($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasPageToken() => $_has(2);
  @$pb.TagNumber(3)
  void clearPageToken() => clearField(3);

  @$pb.TagNumber(5)
  $core.String get subject => $_getSZ(3);
  @$pb.TagNumber(5)
  set subject($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(5)
  $core.bool hasSubject() => $_has(3);
  @$pb.TagNumber(5)
  void clearSubject() => clearField(5);
}

class SearchAliasesResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'SearchAliasesResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.directory'), createEmptyInstance: create)
    ..pc<Alias>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'aliases', $pb.PbFieldType.PM, subBuilder: Alias.create)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'nextPageToken')
    ..hasRequiredFields = false
  ;

  SearchAliasesResponse._() : super();
  factory SearchAliasesResponse({
    $core.Iterable<Alias>? aliases,
    $core.String? nextPageToken,
  }) {
    final _result = create();
    if (aliases != null) {
      _result.aliases.addAll(aliases);
    }
    if (nextPageToken != null) {
      _result.nextPageToken = nextPageToken;
    }
    return _result;
  }
  factory SearchAliasesResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SearchAliasesResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SearchAliasesResponse clone() => SearchAliasesResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SearchAliasesResponse copyWith(void Function(SearchAliasesResponse) updates) => super.copyWith((message) => updates(message as SearchAliasesResponse)) as SearchAliasesResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static SearchAliasesResponse create() => SearchAliasesResponse._();
  SearchAliasesResponse createEmptyInstance() => create();
  static $pb.PbList<SearchAliasesResponse> createRepeated() => $pb.PbList<SearchAliasesResponse>();
  @$core.pragma('dart2js:noInline')
  static SearchAliasesResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SearchAliasesResponse>(create);
  static SearchAliasesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<Alias> get aliases => $_getList(0);

  @$pb.TagNumber(2)
  $core.String get nextPageToken => $_getSZ(1);
  @$pb.TagNumber(2)
  set nextPageToken($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasNextPageToken() => $_has(1);
  @$pb.TagNumber(2)
  void clearNextPageToken() => clearField(2);
}

class GetObjectUrlRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetObjectUrlRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.directory'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'objectId')
    ..hasRequiredFields = false
  ;

  GetObjectUrlRequest._() : super();
  factory GetObjectUrlRequest({
    $core.String? objectId,
  }) {
    final _result = create();
    if (objectId != null) {
      _result.objectId = objectId;
    }
    return _result;
  }
  factory GetObjectUrlRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetObjectUrlRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetObjectUrlRequest clone() => GetObjectUrlRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetObjectUrlRequest copyWith(void Function(GetObjectUrlRequest) updates) => super.copyWith((message) => updates(message as GetObjectUrlRequest)) as GetObjectUrlRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetObjectUrlRequest create() => GetObjectUrlRequest._();
  GetObjectUrlRequest createEmptyInstance() => create();
  static $pb.PbList<GetObjectUrlRequest> createRepeated() => $pb.PbList<GetObjectUrlRequest>();
  @$core.pragma('dart2js:noInline')
  static GetObjectUrlRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetObjectUrlRequest>(create);
  static GetObjectUrlRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get objectId => $_getSZ(0);
  @$pb.TagNumber(1)
  set objectId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasObjectId() => $_has(0);
  @$pb.TagNumber(1)
  void clearObjectId() => clearField(1);
}

class ObjectUrlResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ObjectUrlResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.directory'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'presignedUrl')
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'objectId')
    ..aOS(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'url')
    ..hasRequiredFields = false
  ;

  ObjectUrlResponse._() : super();
  factory ObjectUrlResponse({
    $core.String? presignedUrl,
    $core.String? objectId,
    $core.String? url,
  }) {
    final _result = create();
    if (presignedUrl != null) {
      _result.presignedUrl = presignedUrl;
    }
    if (objectId != null) {
      _result.objectId = objectId;
    }
    if (url != null) {
      _result.url = url;
    }
    return _result;
  }
  factory ObjectUrlResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ObjectUrlResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ObjectUrlResponse clone() => ObjectUrlResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ObjectUrlResponse copyWith(void Function(ObjectUrlResponse) updates) => super.copyWith((message) => updates(message as ObjectUrlResponse)) as ObjectUrlResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ObjectUrlResponse create() => ObjectUrlResponse._();
  ObjectUrlResponse createEmptyInstance() => create();
  static $pb.PbList<ObjectUrlResponse> createRepeated() => $pb.PbList<ObjectUrlResponse>();
  @$core.pragma('dart2js:noInline')
  static ObjectUrlResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ObjectUrlResponse>(create);
  static ObjectUrlResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get presignedUrl => $_getSZ(0);
  @$pb.TagNumber(1)
  set presignedUrl($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasPresignedUrl() => $_has(0);
  @$pb.TagNumber(1)
  void clearPresignedUrl() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get objectId => $_getSZ(1);
  @$pb.TagNumber(2)
  set objectId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasObjectId() => $_has(1);
  @$pb.TagNumber(2)
  void clearObjectId() => clearField(2);

  @$pb.TagNumber(3)
  $core.String get url => $_getSZ(2);
  @$pb.TagNumber(3)
  set url($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasUrl() => $_has(2);
  @$pb.TagNumber(3)
  void clearUrl() => clearField(3);
}

class CreateImageUrlRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'CreateImageUrlRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.directory'), createEmptyInstance: create)
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'mimeType')
    ..hasRequiredFields = false
  ;

  CreateImageUrlRequest._() : super();
  factory CreateImageUrlRequest({
    $core.String? mimeType,
  }) {
    final _result = create();
    if (mimeType != null) {
      _result.mimeType = mimeType;
    }
    return _result;
  }
  factory CreateImageUrlRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CreateImageUrlRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CreateImageUrlRequest clone() => CreateImageUrlRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CreateImageUrlRequest copyWith(void Function(CreateImageUrlRequest) updates) => super.copyWith((message) => updates(message as CreateImageUrlRequest)) as CreateImageUrlRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static CreateImageUrlRequest create() => CreateImageUrlRequest._();
  CreateImageUrlRequest createEmptyInstance() => create();
  static $pb.PbList<CreateImageUrlRequest> createRepeated() => $pb.PbList<CreateImageUrlRequest>();
  @$core.pragma('dart2js:noInline')
  static CreateImageUrlRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CreateImageUrlRequest>(create);
  static CreateImageUrlRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get mimeType => $_getSZ(0);
  @$pb.TagNumber(1)
  set mimeType($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasMimeType() => $_has(0);
  @$pb.TagNumber(1)
  void clearMimeType() => clearField(1);
}

