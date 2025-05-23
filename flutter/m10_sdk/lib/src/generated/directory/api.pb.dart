//
//  Generated code. Do not modify.
//  source: directory/api.proto
//
// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'api.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'api.pbenum.dart';

/// Ledger messages
class Ledger extends $pb.GeneratedMessage {
  factory Ledger({
    $core.String? operator,
    $core.String? url,
  }) {
    final $result = create();
    if (operator != null) {
      $result.operator = operator;
    }
    if (url != null) {
      $result.url = url;
    }
    return $result;
  }
  Ledger._() : super();
  factory Ledger.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Ledger.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Ledger', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.directory'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'operator')
    ..aOS(2, _omitFieldNames ? '' : 'url')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Ledger clone() => Ledger()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Ledger copyWith(void Function(Ledger) updates) => super.copyWith((message) => updates(message as Ledger)) as Ledger;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Ledger create() => Ledger._();
  Ledger createEmptyInstance() => create();
  static $pb.PbList<Ledger> createRepeated() => $pb.PbList<Ledger>();
  @$core.pragma('dart2js:noInline')
  static Ledger getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Ledger>(create);
  static Ledger? _defaultInstance;

  /// Operator of the ledger.
  @$pb.TagNumber(1)
  $core.String get operator => $_getSZ(0);
  @$pb.TagNumber(1)
  set operator($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasOperator() => $_has(0);
  @$pb.TagNumber(1)
  void clearOperator() => $_clearField(1);

  /// URL of the ledger.
  @$pb.TagNumber(2)
  $core.String get url => $_getSZ(1);
  @$pb.TagNumber(2)
  set url($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasUrl() => $_has(1);
  @$pb.TagNumber(2)
  void clearUrl() => $_clearField(2);
}

/// Response message for listing ledgers.
class ListLedgersResponse extends $pb.GeneratedMessage {
  factory ListLedgersResponse({
    $core.Iterable<Ledger>? ledgers,
  }) {
    final $result = create();
    if (ledgers != null) {
      $result.ledgers.addAll(ledgers);
    }
    return $result;
  }
  ListLedgersResponse._() : super();
  factory ListLedgersResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListLedgersResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ListLedgersResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.directory'), createEmptyInstance: create)
    ..pc<Ledger>(1, _omitFieldNames ? '' : 'ledgers', $pb.PbFieldType.PM, subBuilder: Ledger.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListLedgersResponse clone() => ListLedgersResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListLedgersResponse copyWith(void Function(ListLedgersResponse) updates) => super.copyWith((message) => updates(message as ListLedgersResponse)) as ListLedgersResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListLedgersResponse create() => ListLedgersResponse._();
  ListLedgersResponse createEmptyInstance() => create();
  static $pb.PbList<ListLedgersResponse> createRepeated() => $pb.PbList<ListLedgersResponse>();
  @$core.pragma('dart2js:noInline')
  static ListLedgersResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListLedgersResponse>(create);
  static ListLedgersResponse? _defaultInstance;

  /// List of ledgers.
  @$pb.TagNumber(1)
  $pb.PbList<Ledger> get ledgers => $_getList(0);
}

/// Alias messages
class Alias extends $pb.GeneratedMessage {
  factory Alias({
    $core.String? handle,
    $core.String? displayName,
    $core.List<$core.int>? accountSetId,
    $core.String? operator,
    Alias_Type? aliasType,
  }) {
    final $result = create();
    if (handle != null) {
      $result.handle = handle;
    }
    if (displayName != null) {
      $result.displayName = displayName;
    }
    if (accountSetId != null) {
      $result.accountSetId = accountSetId;
    }
    if (operator != null) {
      $result.operator = operator;
    }
    if (aliasType != null) {
      $result.aliasType = aliasType;
    }
    return $result;
  }
  Alias._() : super();
  factory Alias.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Alias.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Alias', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.directory'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'handle')
    ..aOS(2, _omitFieldNames ? '' : 'displayName')
    ..a<$core.List<$core.int>>(5, _omitFieldNames ? '' : 'accountSetId', $pb.PbFieldType.OY)
    ..aOS(8, _omitFieldNames ? '' : 'operator')
    ..e<Alias_Type>(10, _omitFieldNames ? '' : 'aliasType', $pb.PbFieldType.OE, defaultOrMaker: Alias_Type.HANDLE, valueOf: Alias_Type.valueOf, enumValues: Alias_Type.values)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Alias clone() => Alias()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Alias copyWith(void Function(Alias) updates) => super.copyWith((message) => updates(message as Alias)) as Alias;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Alias create() => Alias._();
  Alias createEmptyInstance() => create();
  static $pb.PbList<Alias> createRepeated() => $pb.PbList<Alias>();
  @$core.pragma('dart2js:noInline')
  static Alias getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Alias>(create);
  static Alias? _defaultInstance;

  /// Handle of the alias.
  @$pb.TagNumber(1)
  $core.String get handle => $_getSZ(0);
  @$pb.TagNumber(1)
  set handle($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasHandle() => $_has(0);
  @$pb.TagNumber(1)
  void clearHandle() => $_clearField(1);

  /// Display name of the alias.
  @$pb.TagNumber(2)
  $core.String get displayName => $_getSZ(1);
  @$pb.TagNumber(2)
  set displayName($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasDisplayName() => $_has(1);
  @$pb.TagNumber(2)
  void clearDisplayName() => $_clearField(2);

  /// AccountSet ID associated with the alias.
  @$pb.TagNumber(5)
  $core.List<$core.int> get accountSetId => $_getN(2);
  @$pb.TagNumber(5)
  set accountSetId($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(5)
  $core.bool hasAccountSetId() => $_has(2);
  @$pb.TagNumber(5)
  void clearAccountSetId() => $_clearField(5);

  /// Operator of the alias.
  @$pb.TagNumber(8)
  $core.String get operator => $_getSZ(3);
  @$pb.TagNumber(8)
  set operator($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(8)
  $core.bool hasOperator() => $_has(3);
  @$pb.TagNumber(8)
  void clearOperator() => $_clearField(8);

  /// Type of the alias.
  @$pb.TagNumber(10)
  Alias_Type get aliasType => $_getN(4);
  @$pb.TagNumber(10)
  set aliasType(Alias_Type v) { $_setField(10, v); }
  @$pb.TagNumber(10)
  $core.bool hasAliasType() => $_has(4);
  @$pb.TagNumber(10)
  void clearAliasType() => $_clearField(10);
}

/// For checking if an alias exist.
class CheckAliasRequest extends $pb.GeneratedMessage {
  factory CheckAliasRequest({
    $core.String? handle,
  }) {
    final $result = create();
    if (handle != null) {
      $result.handle = handle;
    }
    return $result;
  }
  CheckAliasRequest._() : super();
  factory CheckAliasRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CheckAliasRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'CheckAliasRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.directory'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'handle')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CheckAliasRequest clone() => CheckAliasRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CheckAliasRequest copyWith(void Function(CheckAliasRequest) updates) => super.copyWith((message) => updates(message as CheckAliasRequest)) as CheckAliasRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CheckAliasRequest create() => CheckAliasRequest._();
  CheckAliasRequest createEmptyInstance() => create();
  static $pb.PbList<CheckAliasRequest> createRepeated() => $pb.PbList<CheckAliasRequest>();
  @$core.pragma('dart2js:noInline')
  static CheckAliasRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CheckAliasRequest>(create);
  static CheckAliasRequest? _defaultInstance;

  /// Handle of the alias to check.
  @$pb.TagNumber(1)
  $core.String get handle => $_getSZ(0);
  @$pb.TagNumber(1)
  set handle($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasHandle() => $_has(0);
  @$pb.TagNumber(1)
  void clearHandle() => $_clearField(1);
}

/// The request message for searching aliases.
class SearchAliasesRequest extends $pb.GeneratedMessage {
  factory SearchAliasesRequest({
    $core.String? handlePrefix,
    $core.int? pageSize,
    $core.String? pageToken,
    $core.String? subject,
  }) {
    final $result = create();
    if (handlePrefix != null) {
      $result.handlePrefix = handlePrefix;
    }
    if (pageSize != null) {
      $result.pageSize = pageSize;
    }
    if (pageToken != null) {
      $result.pageToken = pageToken;
    }
    if (subject != null) {
      $result.subject = subject;
    }
    return $result;
  }
  SearchAliasesRequest._() : super();
  factory SearchAliasesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SearchAliasesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'SearchAliasesRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.directory'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'handlePrefix')
    ..a<$core.int>(2, _omitFieldNames ? '' : 'pageSize', $pb.PbFieldType.O3)
    ..aOS(3, _omitFieldNames ? '' : 'pageToken')
    ..aOS(5, _omitFieldNames ? '' : 'subject')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SearchAliasesRequest clone() => SearchAliasesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SearchAliasesRequest copyWith(void Function(SearchAliasesRequest) updates) => super.copyWith((message) => updates(message as SearchAliasesRequest)) as SearchAliasesRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SearchAliasesRequest create() => SearchAliasesRequest._();
  SearchAliasesRequest createEmptyInstance() => create();
  static $pb.PbList<SearchAliasesRequest> createRepeated() => $pb.PbList<SearchAliasesRequest>();
  @$core.pragma('dart2js:noInline')
  static SearchAliasesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SearchAliasesRequest>(create);
  static SearchAliasesRequest? _defaultInstance;

  /// Prefix of the handle to search for.
  @$pb.TagNumber(1)
  $core.String get handlePrefix => $_getSZ(0);
  @$pb.TagNumber(1)
  set handlePrefix($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasHandlePrefix() => $_has(0);
  @$pb.TagNumber(1)
  void clearHandlePrefix() => $_clearField(1);

  /// Maximum number of results to return.
  @$pb.TagNumber(2)
  $core.int get pageSize => $_getIZ(1);
  @$pb.TagNumber(2)
  set pageSize($core.int v) { $_setSignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasPageSize() => $_has(1);
  @$pb.TagNumber(2)
  void clearPageSize() => $_clearField(2);

  /// Token for pagination.
  @$pb.TagNumber(3)
  $core.String get pageToken => $_getSZ(2);
  @$pb.TagNumber(3)
  set pageToken($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasPageToken() => $_has(2);
  @$pb.TagNumber(3)
  void clearPageToken() => $_clearField(3);

  /// Subject string
  @$pb.TagNumber(5)
  $core.String get subject => $_getSZ(3);
  @$pb.TagNumber(5)
  set subject($core.String v) { $_setString(3, v); }
  @$pb.TagNumber(5)
  $core.bool hasSubject() => $_has(3);
  @$pb.TagNumber(5)
  void clearSubject() => $_clearField(5);
}

/// The response message for searching aliases.
class SearchAliasesResponse extends $pb.GeneratedMessage {
  factory SearchAliasesResponse({
    $core.Iterable<Alias>? aliases,
    $core.String? nextPageToken,
  }) {
    final $result = create();
    if (aliases != null) {
      $result.aliases.addAll(aliases);
    }
    if (nextPageToken != null) {
      $result.nextPageToken = nextPageToken;
    }
    return $result;
  }
  SearchAliasesResponse._() : super();
  factory SearchAliasesResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SearchAliasesResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'SearchAliasesResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.directory'), createEmptyInstance: create)
    ..pc<Alias>(1, _omitFieldNames ? '' : 'aliases', $pb.PbFieldType.PM, subBuilder: Alias.create)
    ..aOS(2, _omitFieldNames ? '' : 'nextPageToken')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SearchAliasesResponse clone() => SearchAliasesResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SearchAliasesResponse copyWith(void Function(SearchAliasesResponse) updates) => super.copyWith((message) => updates(message as SearchAliasesResponse)) as SearchAliasesResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SearchAliasesResponse create() => SearchAliasesResponse._();
  SearchAliasesResponse createEmptyInstance() => create();
  static $pb.PbList<SearchAliasesResponse> createRepeated() => $pb.PbList<SearchAliasesResponse>();
  @$core.pragma('dart2js:noInline')
  static SearchAliasesResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SearchAliasesResponse>(create);
  static SearchAliasesResponse? _defaultInstance;

  /// List of aliases found.
  @$pb.TagNumber(1)
  $pb.PbList<Alias> get aliases => $_getList(0);

  /// Token for the next page of results.
  @$pb.TagNumber(2)
  $core.String get nextPageToken => $_getSZ(1);
  @$pb.TagNumber(2)
  set nextPageToken($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasNextPageToken() => $_has(1);
  @$pb.TagNumber(2)
  void clearNextPageToken() => $_clearField(2);
}

/// Document messages
/// Function for uploading an object.
class GetObjectUrlRequest extends $pb.GeneratedMessage {
  factory GetObjectUrlRequest({
    $core.String? objectId,
  }) {
    final $result = create();
    if (objectId != null) {
      $result.objectId = objectId;
    }
    return $result;
  }
  GetObjectUrlRequest._() : super();
  factory GetObjectUrlRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetObjectUrlRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetObjectUrlRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.directory'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'objectId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetObjectUrlRequest clone() => GetObjectUrlRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetObjectUrlRequest copyWith(void Function(GetObjectUrlRequest) updates) => super.copyWith((message) => updates(message as GetObjectUrlRequest)) as GetObjectUrlRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetObjectUrlRequest create() => GetObjectUrlRequest._();
  GetObjectUrlRequest createEmptyInstance() => create();
  static $pb.PbList<GetObjectUrlRequest> createRepeated() => $pb.PbList<GetObjectUrlRequest>();
  @$core.pragma('dart2js:noInline')
  static GetObjectUrlRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetObjectUrlRequest>(create);
  static GetObjectUrlRequest? _defaultInstance;

  /// ID of the object.
  @$pb.TagNumber(1)
  $core.String get objectId => $_getSZ(0);
  @$pb.TagNumber(1)
  set objectId($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasObjectId() => $_has(0);
  @$pb.TagNumber(1)
  void clearObjectId() => $_clearField(1);
}

/// The response message for object URL requests.
class ObjectUrlResponse extends $pb.GeneratedMessage {
  factory ObjectUrlResponse({
    $core.String? presignedUrl,
    $core.String? objectId,
    $core.String? url,
  }) {
    final $result = create();
    if (presignedUrl != null) {
      $result.presignedUrl = presignedUrl;
    }
    if (objectId != null) {
      $result.objectId = objectId;
    }
    if (url != null) {
      $result.url = url;
    }
    return $result;
  }
  ObjectUrlResponse._() : super();
  factory ObjectUrlResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ObjectUrlResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ObjectUrlResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.directory'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'presignedUrl')
    ..aOS(2, _omitFieldNames ? '' : 'objectId')
    ..aOS(3, _omitFieldNames ? '' : 'url')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ObjectUrlResponse clone() => ObjectUrlResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ObjectUrlResponse copyWith(void Function(ObjectUrlResponse) updates) => super.copyWith((message) => updates(message as ObjectUrlResponse)) as ObjectUrlResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ObjectUrlResponse create() => ObjectUrlResponse._();
  ObjectUrlResponse createEmptyInstance() => create();
  static $pb.PbList<ObjectUrlResponse> createRepeated() => $pb.PbList<ObjectUrlResponse>();
  @$core.pragma('dart2js:noInline')
  static ObjectUrlResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ObjectUrlResponse>(create);
  static ObjectUrlResponse? _defaultInstance;

  /// Pre-signed URL for the object.
  @$pb.TagNumber(1)
  $core.String get presignedUrl => $_getSZ(0);
  @$pb.TagNumber(1)
  set presignedUrl($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasPresignedUrl() => $_has(0);
  @$pb.TagNumber(1)
  void clearPresignedUrl() => $_clearField(1);

  /// ID of the object.
  @$pb.TagNumber(2)
  $core.String get objectId => $_getSZ(1);
  @$pb.TagNumber(2)
  set objectId($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasObjectId() => $_has(1);
  @$pb.TagNumber(2)
  void clearObjectId() => $_clearField(2);

  /// URL of the object
  @$pb.TagNumber(3)
  $core.String get url => $_getSZ(2);
  @$pb.TagNumber(3)
  set url($core.String v) { $_setString(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasUrl() => $_has(2);
  @$pb.TagNumber(3)
  void clearUrl() => $_clearField(3);
}

/// The request message for creating a pre-signed URL for uploading an image.
class CreateImageUrlRequest extends $pb.GeneratedMessage {
  factory CreateImageUrlRequest({
    $core.String? mimeType,
  }) {
    final $result = create();
    if (mimeType != null) {
      $result.mimeType = mimeType;
    }
    return $result;
  }
  CreateImageUrlRequest._() : super();
  factory CreateImageUrlRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CreateImageUrlRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'CreateImageUrlRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.directory'), createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'mimeType')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CreateImageUrlRequest clone() => CreateImageUrlRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CreateImageUrlRequest copyWith(void Function(CreateImageUrlRequest) updates) => super.copyWith((message) => updates(message as CreateImageUrlRequest)) as CreateImageUrlRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CreateImageUrlRequest create() => CreateImageUrlRequest._();
  CreateImageUrlRequest createEmptyInstance() => create();
  static $pb.PbList<CreateImageUrlRequest> createRepeated() => $pb.PbList<CreateImageUrlRequest>();
  @$core.pragma('dart2js:noInline')
  static CreateImageUrlRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CreateImageUrlRequest>(create);
  static CreateImageUrlRequest? _defaultInstance;

  /// Optional MIME type of the image.
  @$pb.TagNumber(1)
  $core.String get mimeType => $_getSZ(0);
  @$pb.TagNumber(1)
  set mimeType($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasMimeType() => $_has(0);
  @$pb.TagNumber(1)
  void clearMimeType() => $_clearField(1);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
