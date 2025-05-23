//
//  Generated code. Do not modify.
//  source: sdk/api.proto
//
// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'document.pb.dart' as $7;
import 'model/model.pb.dart' as $2;
import 'rbac.pb.dart' as $4;
import 'transaction/transaction.pb.dart' as $1;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

/// / A batch of transactions to be created.
class BulkTransactions extends $pb.GeneratedMessage {
  factory BulkTransactions({
    $core.Iterable<RequestEnvelope>? transactions,
  }) {
    final $result = create();
    if (transactions != null) {
      $result.transactions.addAll(transactions);
    }
    return $result;
  }
  BulkTransactions._() : super();
  factory BulkTransactions.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BulkTransactions.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'BulkTransactions', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..pc<RequestEnvelope>(1, _omitFieldNames ? '' : 'transactions', $pb.PbFieldType.PM, subBuilder: RequestEnvelope.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BulkTransactions clone() => BulkTransactions()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BulkTransactions copyWith(void Function(BulkTransactions) updates) => super.copyWith((message) => updates(message as BulkTransactions)) as BulkTransactions;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BulkTransactions create() => BulkTransactions._();
  BulkTransactions createEmptyInstance() => create();
  static $pb.PbList<BulkTransactions> createRepeated() => $pb.PbList<BulkTransactions>();
  @$core.pragma('dart2js:noInline')
  static BulkTransactions getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BulkTransactions>(create);
  static BulkTransactions? _defaultInstance;

  /// / List of transaction requests.
  @$pb.TagNumber(1)
  $pb.PbList<RequestEnvelope> get transactions => $_getList(0);
}

/// / Responses for a bulk transaction creation request.
class BulkTransactionsResponse extends $pb.GeneratedMessage {
  factory BulkTransactionsResponse({
    $core.Iterable<$1.TransactionResponse>? responses,
  }) {
    final $result = create();
    if (responses != null) {
      $result.responses.addAll(responses);
    }
    return $result;
  }
  BulkTransactionsResponse._() : super();
  factory BulkTransactionsResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory BulkTransactionsResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'BulkTransactionsResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..pc<$1.TransactionResponse>(1, _omitFieldNames ? '' : 'responses', $pb.PbFieldType.PM, subBuilder: $1.TransactionResponse.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  BulkTransactionsResponse clone() => BulkTransactionsResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  BulkTransactionsResponse copyWith(void Function(BulkTransactionsResponse) updates) => super.copyWith((message) => updates(message as BulkTransactionsResponse)) as BulkTransactionsResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static BulkTransactionsResponse create() => BulkTransactionsResponse._();
  BulkTransactionsResponse createEmptyInstance() => create();
  static $pb.PbList<BulkTransactionsResponse> createRepeated() => $pb.PbList<BulkTransactionsResponse>();
  @$core.pragma('dart2js:noInline')
  static BulkTransactionsResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<BulkTransactionsResponse>(create);
  static BulkTransactionsResponse? _defaultInstance;

  /// / List of transaction responses.
  @$pb.TagNumber(1)
  $pb.PbList<$1.TransactionResponse> get responses => $_getList(0);
}

/// Wraps a payload with a signature for authentication and authorization.
class RequestEnvelope extends $pb.GeneratedMessage {
  factory RequestEnvelope({
    $core.List<$core.int>? payload,
    $1.Signature? signature,
  }) {
    final $result = create();
    if (payload != null) {
      $result.payload = payload;
    }
    if (signature != null) {
      $result.signature = signature;
    }
    return $result;
  }
  RequestEnvelope._() : super();
  factory RequestEnvelope.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RequestEnvelope.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'RequestEnvelope', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'payload', $pb.PbFieldType.OY)
    ..aOM<$1.Signature>(3, _omitFieldNames ? '' : 'signature', subBuilder: $1.Signature.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RequestEnvelope clone() => RequestEnvelope()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RequestEnvelope copyWith(void Function(RequestEnvelope) updates) => super.copyWith((message) => updates(message as RequestEnvelope)) as RequestEnvelope;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static RequestEnvelope create() => RequestEnvelope._();
  RequestEnvelope createEmptyInstance() => create();
  static $pb.PbList<RequestEnvelope> createRepeated() => $pb.PbList<RequestEnvelope>();
  @$core.pragma('dart2js:noInline')
  static RequestEnvelope getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RequestEnvelope>(create);
  static RequestEnvelope? _defaultInstance;

  /// / Payload data.
  @$pb.TagNumber(2)
  $core.List<$core.int> get payload => $_getN(0);
  @$pb.TagNumber(2)
  set payload($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(2)
  $core.bool hasPayload() => $_has(0);
  @$pb.TagNumber(2)
  void clearPayload() => $_clearField(2);

  /// / Signature for the payload.
  @$pb.TagNumber(3)
  $1.Signature get signature => $_getN(1);
  @$pb.TagNumber(3)
  set signature($1.Signature v) { $_setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasSignature() => $_has(1);
  @$pb.TagNumber(3)
  void clearSignature() => $_clearField(3);
  @$pb.TagNumber(3)
  $1.Signature ensureSignature() => $_ensure(1);
}

/// A page of results for paginated queries.
class Page extends $pb.GeneratedMessage {
  factory Page({
    $core.int? limit,
    $core.List<$core.int>? lastId,
  }) {
    final $result = create();
    if (limit != null) {
      $result.limit = limit;
    }
    if (lastId != null) {
      $result.lastId = lastId;
    }
    return $result;
  }
  Page._() : super();
  factory Page.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Page.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Page', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'limit', $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'lastId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Page clone() => Page()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Page copyWith(void Function(Page) updates) => super.copyWith((message) => updates(message as Page)) as Page;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Page create() => Page._();
  Page createEmptyInstance() => create();
  static $pb.PbList<Page> createRepeated() => $pb.PbList<Page>();
  @$core.pragma('dart2js:noInline')
  static Page getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Page>(create);
  static Page? _defaultInstance;

  /// / Limit of results per page.
  @$pb.TagNumber(1)
  $core.int get limit => $_getIZ(0);
  @$pb.TagNumber(1)
  set limit($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasLimit() => $_has(0);
  @$pb.TagNumber(1)
  void clearLimit() => $_clearField(1);

  /// / Last ID from previous page.
  @$pb.TagNumber(2)
  $core.List<$core.int> get lastId => $_getN(1);
  @$pb.TagNumber(2)
  set lastId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasLastId() => $_has(1);
  @$pb.TagNumber(2)
  void clearLastId() => $_clearField(2);
}

/// Retrieve a specific AccountSet.
class GetAccountSetRequest extends $pb.GeneratedMessage {
  factory GetAccountSetRequest({
    $core.List<$core.int>? id,
  }) {
    final $result = create();
    if (id != null) {
      $result.id = id;
    }
    return $result;
  }
  GetAccountSetRequest._() : super();
  factory GetAccountSetRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetAccountSetRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetAccountSetRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetAccountSetRequest clone() => GetAccountSetRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetAccountSetRequest copyWith(void Function(GetAccountSetRequest) updates) => super.copyWith((message) => updates(message as GetAccountSetRequest)) as GetAccountSetRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetAccountSetRequest create() => GetAccountSetRequest._();
  GetAccountSetRequest createEmptyInstance() => create();
  static $pb.PbList<GetAccountSetRequest> createRepeated() => $pb.PbList<GetAccountSetRequest>();
  @$core.pragma('dart2js:noInline')
  static GetAccountSetRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetAccountSetRequest>(create);
  static GetAccountSetRequest? _defaultInstance;

  /// / ID of the AccountSet.
  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

enum ListAccountSetsRequest_Filter {
  owner, 
  name, 
  notSet
}

/// ListAccountSetsRequest is the request message for listing account sets.
class ListAccountSetsRequest extends $pb.GeneratedMessage {
  factory ListAccountSetsRequest({
    $core.List<$core.int>? owner,
    $core.String? name,
    Page? page,
  }) {
    final $result = create();
    if (owner != null) {
      $result.owner = owner;
    }
    if (name != null) {
      $result.name = name;
    }
    if (page != null) {
      $result.page = page;
    }
    return $result;
  }
  ListAccountSetsRequest._() : super();
  factory ListAccountSetsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListAccountSetsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, ListAccountSetsRequest_Filter> _ListAccountSetsRequest_FilterByTag = {
    1 : ListAccountSetsRequest_Filter.owner,
    2 : ListAccountSetsRequest_Filter.name,
    0 : ListAccountSetsRequest_Filter.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ListAccountSetsRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'owner', $pb.PbFieldType.OY)
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOM<Page>(4, _omitFieldNames ? '' : 'page', subBuilder: Page.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListAccountSetsRequest clone() => ListAccountSetsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListAccountSetsRequest copyWith(void Function(ListAccountSetsRequest) updates) => super.copyWith((message) => updates(message as ListAccountSetsRequest)) as ListAccountSetsRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListAccountSetsRequest create() => ListAccountSetsRequest._();
  ListAccountSetsRequest createEmptyInstance() => create();
  static $pb.PbList<ListAccountSetsRequest> createRepeated() => $pb.PbList<ListAccountSetsRequest>();
  @$core.pragma('dart2js:noInline')
  static ListAccountSetsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListAccountSetsRequest>(create);
  static ListAccountSetsRequest? _defaultInstance;

  ListAccountSetsRequest_Filter whichFilter() => _ListAccountSetsRequest_FilterByTag[$_whichOneof(0)]!;
  void clearFilter() => $_clearField($_whichOneof(0));

  /// / Filter by owner.
  @$pb.TagNumber(1)
  $core.List<$core.int> get owner => $_getN(0);
  @$pb.TagNumber(1)
  set owner($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasOwner() => $_has(0);
  @$pb.TagNumber(1)
  void clearOwner() => $_clearField(1);

  /// / Filter by name.
  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => $_clearField(2);

  /// / Pagination options.
  @$pb.TagNumber(4)
  Page get page => $_getN(2);
  @$pb.TagNumber(4)
  set page(Page v) { $_setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasPage() => $_has(2);
  @$pb.TagNumber(4)
  void clearPage() => $_clearField(4);
  @$pb.TagNumber(4)
  Page ensurePage() => $_ensure(2);
}

/// The response message for listing account sets.
class ListAccountSetsResponse extends $pb.GeneratedMessage {
  factory ListAccountSetsResponse({
    $core.Iterable<$2.AccountSet>? accountSets,
    ListAccountSetsRequest? nextRequest,
  }) {
    final $result = create();
    if (accountSets != null) {
      $result.accountSets.addAll(accountSets);
    }
    if (nextRequest != null) {
      $result.nextRequest = nextRequest;
    }
    return $result;
  }
  ListAccountSetsResponse._() : super();
  factory ListAccountSetsResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListAccountSetsResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ListAccountSetsResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..pc<$2.AccountSet>(1, _omitFieldNames ? '' : 'accountSets', $pb.PbFieldType.PM, subBuilder: $2.AccountSet.create)
    ..aOM<ListAccountSetsRequest>(2, _omitFieldNames ? '' : 'nextRequest', subBuilder: ListAccountSetsRequest.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListAccountSetsResponse clone() => ListAccountSetsResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListAccountSetsResponse copyWith(void Function(ListAccountSetsResponse) updates) => super.copyWith((message) => updates(message as ListAccountSetsResponse)) as ListAccountSetsResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListAccountSetsResponse create() => ListAccountSetsResponse._();
  ListAccountSetsResponse createEmptyInstance() => create();
  static $pb.PbList<ListAccountSetsResponse> createRepeated() => $pb.PbList<ListAccountSetsResponse>();
  @$core.pragma('dart2js:noInline')
  static ListAccountSetsResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListAccountSetsResponse>(create);
  static ListAccountSetsResponse? _defaultInstance;

  /// / List of account sets.
  @$pb.TagNumber(1)
  $pb.PbList<$2.AccountSet> get accountSets => $_getList(0);

  /// / Request for next page of results.
  @$pb.TagNumber(2)
  ListAccountSetsRequest get nextRequest => $_getN(1);
  @$pb.TagNumber(2)
  set nextRequest(ListAccountSetsRequest v) { $_setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasNextRequest() => $_has(1);
  @$pb.TagNumber(2)
  void clearNextRequest() => $_clearField(2);
  @$pb.TagNumber(2)
  ListAccountSetsRequest ensureNextRequest() => $_ensure(1);
}

enum ListAccountMetadataRequest_Filter {
  owner, 
  name, 
  notSet
}

/// The request message for listing account metadata.
class ListAccountMetadataRequest extends $pb.GeneratedMessage {
  factory ListAccountMetadataRequest({
    $core.List<$core.int>? owner,
    $core.String? name,
    Page? page,
  }) {
    final $result = create();
    if (owner != null) {
      $result.owner = owner;
    }
    if (name != null) {
      $result.name = name;
    }
    if (page != null) {
      $result.page = page;
    }
    return $result;
  }
  ListAccountMetadataRequest._() : super();
  factory ListAccountMetadataRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListAccountMetadataRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, ListAccountMetadataRequest_Filter> _ListAccountMetadataRequest_FilterByTag = {
    1 : ListAccountMetadataRequest_Filter.owner,
    2 : ListAccountMetadataRequest_Filter.name,
    0 : ListAccountMetadataRequest_Filter.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ListAccountMetadataRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'owner', $pb.PbFieldType.OY)
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..aOM<Page>(4, _omitFieldNames ? '' : 'page', subBuilder: Page.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListAccountMetadataRequest clone() => ListAccountMetadataRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListAccountMetadataRequest copyWith(void Function(ListAccountMetadataRequest) updates) => super.copyWith((message) => updates(message as ListAccountMetadataRequest)) as ListAccountMetadataRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListAccountMetadataRequest create() => ListAccountMetadataRequest._();
  ListAccountMetadataRequest createEmptyInstance() => create();
  static $pb.PbList<ListAccountMetadataRequest> createRepeated() => $pb.PbList<ListAccountMetadataRequest>();
  @$core.pragma('dart2js:noInline')
  static ListAccountMetadataRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListAccountMetadataRequest>(create);
  static ListAccountMetadataRequest? _defaultInstance;

  ListAccountMetadataRequest_Filter whichFilter() => _ListAccountMetadataRequest_FilterByTag[$_whichOneof(0)]!;
  void clearFilter() => $_clearField($_whichOneof(0));

  /// / Filter by owner.
  @$pb.TagNumber(1)
  $core.List<$core.int> get owner => $_getN(0);
  @$pb.TagNumber(1)
  set owner($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasOwner() => $_has(0);
  @$pb.TagNumber(1)
  void clearOwner() => $_clearField(1);

  /// / Filter by name.
  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => $_clearField(2);

  /// / Pagination options.
  @$pb.TagNumber(4)
  Page get page => $_getN(2);
  @$pb.TagNumber(4)
  set page(Page v) { $_setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasPage() => $_has(2);
  @$pb.TagNumber(4)
  void clearPage() => $_clearField(4);
  @$pb.TagNumber(4)
  Page ensurePage() => $_ensure(2);
}

/// The response message for listing account metadata.
class ListAccountMetadataResponse extends $pb.GeneratedMessage {
  factory ListAccountMetadataResponse({
    $core.Iterable<$2.AccountMetadata>? accounts,
    ListAccountMetadataRequest? nextRequest,
  }) {
    final $result = create();
    if (accounts != null) {
      $result.accounts.addAll(accounts);
    }
    if (nextRequest != null) {
      $result.nextRequest = nextRequest;
    }
    return $result;
  }
  ListAccountMetadataResponse._() : super();
  factory ListAccountMetadataResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListAccountMetadataResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ListAccountMetadataResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..pc<$2.AccountMetadata>(1, _omitFieldNames ? '' : 'accounts', $pb.PbFieldType.PM, subBuilder: $2.AccountMetadata.create)
    ..aOM<ListAccountMetadataRequest>(2, _omitFieldNames ? '' : 'nextRequest', subBuilder: ListAccountMetadataRequest.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListAccountMetadataResponse clone() => ListAccountMetadataResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListAccountMetadataResponse copyWith(void Function(ListAccountMetadataResponse) updates) => super.copyWith((message) => updates(message as ListAccountMetadataResponse)) as ListAccountMetadataResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListAccountMetadataResponse create() => ListAccountMetadataResponse._();
  ListAccountMetadataResponse createEmptyInstance() => create();
  static $pb.PbList<ListAccountMetadataResponse> createRepeated() => $pb.PbList<ListAccountMetadataResponse>();
  @$core.pragma('dart2js:noInline')
  static ListAccountMetadataResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListAccountMetadataResponse>(create);
  static ListAccountMetadataResponse? _defaultInstance;

  /// / List of account metadata.
  @$pb.TagNumber(1)
  $pb.PbList<$2.AccountMetadata> get accounts => $_getList(0);

  /// / Request for the next page.
  @$pb.TagNumber(2)
  ListAccountMetadataRequest get nextRequest => $_getN(1);
  @$pb.TagNumber(2)
  set nextRequest(ListAccountMetadataRequest v) { $_setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasNextRequest() => $_has(1);
  @$pb.TagNumber(2)
  void clearNextRequest() => $_clearField(2);
  @$pb.TagNumber(2)
  ListAccountMetadataRequest ensureNextRequest() => $_ensure(1);
}

/// The request message for retrieving a specific role binding.
class GetRoleBindingRequest extends $pb.GeneratedMessage {
  factory GetRoleBindingRequest({
    $core.List<$core.int>? id,
  }) {
    final $result = create();
    if (id != null) {
      $result.id = id;
    }
    return $result;
  }
  GetRoleBindingRequest._() : super();
  factory GetRoleBindingRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetRoleBindingRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetRoleBindingRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetRoleBindingRequest clone() => GetRoleBindingRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetRoleBindingRequest copyWith(void Function(GetRoleBindingRequest) updates) => super.copyWith((message) => updates(message as GetRoleBindingRequest)) as GetRoleBindingRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetRoleBindingRequest create() => GetRoleBindingRequest._();
  GetRoleBindingRequest createEmptyInstance() => create();
  static $pb.PbList<GetRoleBindingRequest> createRepeated() => $pb.PbList<GetRoleBindingRequest>();
  @$core.pragma('dart2js:noInline')
  static GetRoleBindingRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetRoleBindingRequest>(create);
  static GetRoleBindingRequest? _defaultInstance;

  /// / ID of the role binding.
  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

enum ListRoleBindingsRequest_Filter {
  name, 
  notSet
}

/// The request message for listing role bindings.
class ListRoleBindingsRequest extends $pb.GeneratedMessage {
  factory ListRoleBindingsRequest({
    $core.String? name,
    Page? page,
  }) {
    final $result = create();
    if (name != null) {
      $result.name = name;
    }
    if (page != null) {
      $result.page = page;
    }
    return $result;
  }
  ListRoleBindingsRequest._() : super();
  factory ListRoleBindingsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListRoleBindingsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, ListRoleBindingsRequest_Filter> _ListRoleBindingsRequest_FilterByTag = {
    1 : ListRoleBindingsRequest_Filter.name,
    0 : ListRoleBindingsRequest_Filter.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ListRoleBindingsRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..oo(0, [1])
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOM<Page>(4, _omitFieldNames ? '' : 'page', subBuilder: Page.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListRoleBindingsRequest clone() => ListRoleBindingsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListRoleBindingsRequest copyWith(void Function(ListRoleBindingsRequest) updates) => super.copyWith((message) => updates(message as ListRoleBindingsRequest)) as ListRoleBindingsRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListRoleBindingsRequest create() => ListRoleBindingsRequest._();
  ListRoleBindingsRequest createEmptyInstance() => create();
  static $pb.PbList<ListRoleBindingsRequest> createRepeated() => $pb.PbList<ListRoleBindingsRequest>();
  @$core.pragma('dart2js:noInline')
  static ListRoleBindingsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListRoleBindingsRequest>(create);
  static ListRoleBindingsRequest? _defaultInstance;

  ListRoleBindingsRequest_Filter whichFilter() => _ListRoleBindingsRequest_FilterByTag[$_whichOneof(0)]!;
  void clearFilter() => $_clearField($_whichOneof(0));

  /// / Filter by name.
  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  /// / Pagination options.
  @$pb.TagNumber(4)
  Page get page => $_getN(1);
  @$pb.TagNumber(4)
  set page(Page v) { $_setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasPage() => $_has(1);
  @$pb.TagNumber(4)
  void clearPage() => $_clearField(4);
  @$pb.TagNumber(4)
  Page ensurePage() => $_ensure(1);
}

/// The response message for listing role bindings.
class ListRoleBindingsResponse extends $pb.GeneratedMessage {
  factory ListRoleBindingsResponse({
    $core.Iterable<$4.RoleBinding>? roleBindings,
    ListRoleBindingsRequest? nextRequest,
  }) {
    final $result = create();
    if (roleBindings != null) {
      $result.roleBindings.addAll(roleBindings);
    }
    if (nextRequest != null) {
      $result.nextRequest = nextRequest;
    }
    return $result;
  }
  ListRoleBindingsResponse._() : super();
  factory ListRoleBindingsResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListRoleBindingsResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ListRoleBindingsResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..pc<$4.RoleBinding>(1, _omitFieldNames ? '' : 'roleBindings', $pb.PbFieldType.PM, subBuilder: $4.RoleBinding.create)
    ..aOM<ListRoleBindingsRequest>(2, _omitFieldNames ? '' : 'nextRequest', subBuilder: ListRoleBindingsRequest.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListRoleBindingsResponse clone() => ListRoleBindingsResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListRoleBindingsResponse copyWith(void Function(ListRoleBindingsResponse) updates) => super.copyWith((message) => updates(message as ListRoleBindingsResponse)) as ListRoleBindingsResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListRoleBindingsResponse create() => ListRoleBindingsResponse._();
  ListRoleBindingsResponse createEmptyInstance() => create();
  static $pb.PbList<ListRoleBindingsResponse> createRepeated() => $pb.PbList<ListRoleBindingsResponse>();
  @$core.pragma('dart2js:noInline')
  static ListRoleBindingsResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListRoleBindingsResponse>(create);
  static ListRoleBindingsResponse? _defaultInstance;

  /// / List of role bindings.
  @$pb.TagNumber(1)
  $pb.PbList<$4.RoleBinding> get roleBindings => $_getList(0);

  /// / Request for next page of results.
  @$pb.TagNumber(2)
  ListRoleBindingsRequest get nextRequest => $_getN(1);
  @$pb.TagNumber(2)
  set nextRequest(ListRoleBindingsRequest v) { $_setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasNextRequest() => $_has(1);
  @$pb.TagNumber(2)
  void clearNextRequest() => $_clearField(2);
  @$pb.TagNumber(2)
  ListRoleBindingsRequest ensureNextRequest() => $_ensure(1);
}

/// The request message for retrieving a specific role.
class GetRoleRequest extends $pb.GeneratedMessage {
  factory GetRoleRequest({
    $core.List<$core.int>? id,
  }) {
    final $result = create();
    if (id != null) {
      $result.id = id;
    }
    return $result;
  }
  GetRoleRequest._() : super();
  factory GetRoleRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetRoleRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetRoleRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetRoleRequest clone() => GetRoleRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetRoleRequest copyWith(void Function(GetRoleRequest) updates) => super.copyWith((message) => updates(message as GetRoleRequest)) as GetRoleRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetRoleRequest create() => GetRoleRequest._();
  GetRoleRequest createEmptyInstance() => create();
  static $pb.PbList<GetRoleRequest> createRepeated() => $pb.PbList<GetRoleRequest>();
  @$core.pragma('dart2js:noInline')
  static GetRoleRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetRoleRequest>(create);
  static GetRoleRequest? _defaultInstance;

  /// / ID of the role.
  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

enum ListRolesRequest_Filter {
  name, 
  notSet
}

/// The request message for listing roles.
class ListRolesRequest extends $pb.GeneratedMessage {
  factory ListRolesRequest({
    $core.String? name,
    Page? page,
  }) {
    final $result = create();
    if (name != null) {
      $result.name = name;
    }
    if (page != null) {
      $result.page = page;
    }
    return $result;
  }
  ListRolesRequest._() : super();
  factory ListRolesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListRolesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, ListRolesRequest_Filter> _ListRolesRequest_FilterByTag = {
    1 : ListRolesRequest_Filter.name,
    0 : ListRolesRequest_Filter.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ListRolesRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..oo(0, [1])
    ..aOS(1, _omitFieldNames ? '' : 'name')
    ..aOM<Page>(4, _omitFieldNames ? '' : 'page', subBuilder: Page.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListRolesRequest clone() => ListRolesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListRolesRequest copyWith(void Function(ListRolesRequest) updates) => super.copyWith((message) => updates(message as ListRolesRequest)) as ListRolesRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListRolesRequest create() => ListRolesRequest._();
  ListRolesRequest createEmptyInstance() => create();
  static $pb.PbList<ListRolesRequest> createRepeated() => $pb.PbList<ListRolesRequest>();
  @$core.pragma('dart2js:noInline')
  static ListRolesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListRolesRequest>(create);
  static ListRolesRequest? _defaultInstance;

  ListRolesRequest_Filter whichFilter() => _ListRolesRequest_FilterByTag[$_whichOneof(0)]!;
  void clearFilter() => $_clearField($_whichOneof(0));

  /// / Filter by name.
  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => $_clearField(1);

  /// / Pagination options.
  @$pb.TagNumber(4)
  Page get page => $_getN(1);
  @$pb.TagNumber(4)
  set page(Page v) { $_setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasPage() => $_has(1);
  @$pb.TagNumber(4)
  void clearPage() => $_clearField(4);
  @$pb.TagNumber(4)
  Page ensurePage() => $_ensure(1);
}

/// The response message for listing roles.
class ListRolesResponse extends $pb.GeneratedMessage {
  factory ListRolesResponse({
    $core.Iterable<$4.Role>? roles,
    ListRolesRequest? nextRequest,
  }) {
    final $result = create();
    if (roles != null) {
      $result.roles.addAll(roles);
    }
    if (nextRequest != null) {
      $result.nextRequest = nextRequest;
    }
    return $result;
  }
  ListRolesResponse._() : super();
  factory ListRolesResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListRolesResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ListRolesResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..pc<$4.Role>(1, _omitFieldNames ? '' : 'roles', $pb.PbFieldType.PM, subBuilder: $4.Role.create)
    ..aOM<ListRolesRequest>(2, _omitFieldNames ? '' : 'nextRequest', subBuilder: ListRolesRequest.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListRolesResponse clone() => ListRolesResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListRolesResponse copyWith(void Function(ListRolesResponse) updates) => super.copyWith((message) => updates(message as ListRolesResponse)) as ListRolesResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListRolesResponse create() => ListRolesResponse._();
  ListRolesResponse createEmptyInstance() => create();
  static $pb.PbList<ListRolesResponse> createRepeated() => $pb.PbList<ListRolesResponse>();
  @$core.pragma('dart2js:noInline')
  static ListRolesResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListRolesResponse>(create);
  static ListRolesResponse? _defaultInstance;

  /// / List of roles.
  @$pb.TagNumber(1)
  $pb.PbList<$4.Role> get roles => $_getList(0);

  /// / Request for next page of results.
  @$pb.TagNumber(2)
  ListRolesRequest get nextRequest => $_getN(1);
  @$pb.TagNumber(2)
  set nextRequest(ListRolesRequest v) { $_setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasNextRequest() => $_has(1);
  @$pb.TagNumber(2)
  void clearNextRequest() => $_clearField(2);
  @$pb.TagNumber(2)
  ListRolesRequest ensureNextRequest() => $_ensure(1);
}

/// The request message for retrieving a specific transaction.
class GetTransactionRequest extends $pb.GeneratedMessage {
  factory GetTransactionRequest({
    $fixnum.Int64? txId,
  }) {
    final $result = create();
    if (txId != null) {
      $result.txId = txId;
    }
    return $result;
  }
  GetTransactionRequest._() : super();
  factory GetTransactionRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetTransactionRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetTransactionRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'txId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetTransactionRequest clone() => GetTransactionRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetTransactionRequest copyWith(void Function(GetTransactionRequest) updates) => super.copyWith((message) => updates(message as GetTransactionRequest)) as GetTransactionRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetTransactionRequest create() => GetTransactionRequest._();
  GetTransactionRequest createEmptyInstance() => create();
  static $pb.PbList<GetTransactionRequest> createRepeated() => $pb.PbList<GetTransactionRequest>();
  @$core.pragma('dart2js:noInline')
  static GetTransactionRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetTransactionRequest>(create);
  static GetTransactionRequest? _defaultInstance;

  /// / ID of the transaction.
  @$pb.TagNumber(1)
  $fixnum.Int64 get txId => $_getI64(0);
  @$pb.TagNumber(1)
  set txId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTxId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTxId() => $_clearField(1);
}

/// The request message for listing transactions.
class ListTransactionsRequest extends $pb.GeneratedMessage {
  factory ListTransactionsRequest({
    $core.List<$core.int>? contextId,
    $fixnum.Int64? limit,
    $fixnum.Int64? minTxId,
    $fixnum.Int64? maxTxId,
  }) {
    final $result = create();
    if (contextId != null) {
      $result.contextId = contextId;
    }
    if (limit != null) {
      $result.limit = limit;
    }
    if (minTxId != null) {
      $result.minTxId = minTxId;
    }
    if (maxTxId != null) {
      $result.maxTxId = maxTxId;
    }
    return $result;
  }
  ListTransactionsRequest._() : super();
  factory ListTransactionsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListTransactionsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ListTransactionsRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'contextId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(3, _omitFieldNames ? '' : 'limit', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(4, _omitFieldNames ? '' : 'minTxId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(5, _omitFieldNames ? '' : 'maxTxId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListTransactionsRequest clone() => ListTransactionsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListTransactionsRequest copyWith(void Function(ListTransactionsRequest) updates) => super.copyWith((message) => updates(message as ListTransactionsRequest)) as ListTransactionsRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListTransactionsRequest create() => ListTransactionsRequest._();
  ListTransactionsRequest createEmptyInstance() => create();
  static $pb.PbList<ListTransactionsRequest> createRepeated() => $pb.PbList<ListTransactionsRequest>();
  @$core.pragma('dart2js:noInline')
  static ListTransactionsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListTransactionsRequest>(create);
  static ListTransactionsRequest? _defaultInstance;

  /// / Context ID to filter by.
  @$pb.TagNumber(1)
  $core.List<$core.int> get contextId => $_getN(0);
  @$pb.TagNumber(1)
  set contextId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasContextId() => $_has(0);
  @$pb.TagNumber(1)
  void clearContextId() => $_clearField(1);

  /// / Maximum number of transactions to return.
  @$pb.TagNumber(3)
  $fixnum.Int64 get limit => $_getI64(1);
  @$pb.TagNumber(3)
  set limit($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(3)
  $core.bool hasLimit() => $_has(1);
  @$pb.TagNumber(3)
  void clearLimit() => $_clearField(3);

  /// / Minimum transaction ID.
  @$pb.TagNumber(4)
  $fixnum.Int64 get minTxId => $_getI64(2);
  @$pb.TagNumber(4)
  set minTxId($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(4)
  $core.bool hasMinTxId() => $_has(2);
  @$pb.TagNumber(4)
  void clearMinTxId() => $_clearField(4);

  /// / Maximum transaction ID.
  @$pb.TagNumber(5)
  $fixnum.Int64 get maxTxId => $_getI64(3);
  @$pb.TagNumber(5)
  set maxTxId($fixnum.Int64 v) { $_setInt64(3, v); }
  @$pb.TagNumber(5)
  $core.bool hasMaxTxId() => $_has(3);
  @$pb.TagNumber(5)
  void clearMaxTxId() => $_clearField(5);
}

/// The request message for grouping transactions by context ID.
class GroupTransactionsRequest extends $pb.GeneratedMessage {
  factory GroupTransactionsRequest({
    $core.List<$core.int>? accountId,
    $fixnum.Int64? limitGroups,
    $fixnum.Int64? minTxId,
    $fixnum.Int64? maxTxId,
  }) {
    final $result = create();
    if (accountId != null) {
      $result.accountId = accountId;
    }
    if (limitGroups != null) {
      $result.limitGroups = limitGroups;
    }
    if (minTxId != null) {
      $result.minTxId = minTxId;
    }
    if (maxTxId != null) {
      $result.maxTxId = maxTxId;
    }
    return $result;
  }
  GroupTransactionsRequest._() : super();
  factory GroupTransactionsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupTransactionsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GroupTransactionsRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'accountId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(2, _omitFieldNames ? '' : 'limitGroups', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(3, _omitFieldNames ? '' : 'minTxId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(4, _omitFieldNames ? '' : 'maxTxId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupTransactionsRequest clone() => GroupTransactionsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupTransactionsRequest copyWith(void Function(GroupTransactionsRequest) updates) => super.copyWith((message) => updates(message as GroupTransactionsRequest)) as GroupTransactionsRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupTransactionsRequest create() => GroupTransactionsRequest._();
  GroupTransactionsRequest createEmptyInstance() => create();
  static $pb.PbList<GroupTransactionsRequest> createRepeated() => $pb.PbList<GroupTransactionsRequest>();
  @$core.pragma('dart2js:noInline')
  static GroupTransactionsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupTransactionsRequest>(create);
  static GroupTransactionsRequest? _defaultInstance;

  /// / Account ID to filter by.
  @$pb.TagNumber(1)
  $core.List<$core.int> get accountId => $_getN(0);
  @$pb.TagNumber(1)
  set accountId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearAccountId() => $_clearField(1);

  /// / Maximum number of groups to return.
  @$pb.TagNumber(2)
  $fixnum.Int64 get limitGroups => $_getI64(1);
  @$pb.TagNumber(2)
  set limitGroups($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasLimitGroups() => $_has(1);
  @$pb.TagNumber(2)
  void clearLimitGroups() => $_clearField(2);

  /// / Minimum transaction ID.
  @$pb.TagNumber(3)
  $fixnum.Int64 get minTxId => $_getI64(2);
  @$pb.TagNumber(3)
  set minTxId($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasMinTxId() => $_has(2);
  @$pb.TagNumber(3)
  void clearMinTxId() => $_clearField(3);

  /// / Maximum transaction ID.
  @$pb.TagNumber(4)
  $fixnum.Int64 get maxTxId => $_getI64(3);
  @$pb.TagNumber(4)
  set maxTxId($fixnum.Int64 v) { $_setInt64(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasMaxTxId() => $_has(3);
  @$pb.TagNumber(4)
  void clearMaxTxId() => $_clearField(4);
}

/// The request message for observing account changes.
class ObserveAccountsRequest extends $pb.GeneratedMessage {
  factory ObserveAccountsRequest({
    TxId? startingFrom,
    $core.Iterable<$core.List<$core.int>>? involvedAccounts,
  }) {
    final $result = create();
    if (startingFrom != null) {
      $result.startingFrom = startingFrom;
    }
    if (involvedAccounts != null) {
      $result.involvedAccounts.addAll(involvedAccounts);
    }
    return $result;
  }
  ObserveAccountsRequest._() : super();
  factory ObserveAccountsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ObserveAccountsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ObserveAccountsRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..aOM<TxId>(1, _omitFieldNames ? '' : 'startingFrom', subBuilder: TxId.create)
    ..p<$core.List<$core.int>>(2, _omitFieldNames ? '' : 'involvedAccounts', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ObserveAccountsRequest clone() => ObserveAccountsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ObserveAccountsRequest copyWith(void Function(ObserveAccountsRequest) updates) => super.copyWith((message) => updates(message as ObserveAccountsRequest)) as ObserveAccountsRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ObserveAccountsRequest create() => ObserveAccountsRequest._();
  ObserveAccountsRequest createEmptyInstance() => create();
  static $pb.PbList<ObserveAccountsRequest> createRepeated() => $pb.PbList<ObserveAccountsRequest>();
  @$core.pragma('dart2js:noInline')
  static ObserveAccountsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ObserveAccountsRequest>(create);
  static ObserveAccountsRequest? _defaultInstance;

  /// / Starting transaction ID for observation.
  @$pb.TagNumber(1)
  TxId get startingFrom => $_getN(0);
  @$pb.TagNumber(1)
  set startingFrom(TxId v) { $_setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasStartingFrom() => $_has(0);
  @$pb.TagNumber(1)
  void clearStartingFrom() => $_clearField(1);
  @$pb.TagNumber(1)
  TxId ensureStartingFrom() => $_ensure(0);

  /// / Account IDs to observe.
  @$pb.TagNumber(2)
  $pb.PbList<$core.List<$core.int>> get involvedAccounts => $_getList(1);
}

/// ObserveResourcesRequest is the request message for observing resource changes.
class ObserveResourcesRequest extends $pb.GeneratedMessage {
  factory ObserveResourcesRequest({
    $7.Exp? expression,
    $core.String? collection,
    TxId? startingFrom,
  }) {
    final $result = create();
    if (expression != null) {
      $result.expression = expression;
    }
    if (collection != null) {
      $result.collection = collection;
    }
    if (startingFrom != null) {
      $result.startingFrom = startingFrom;
    }
    return $result;
  }
  ObserveResourcesRequest._() : super();
  factory ObserveResourcesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ObserveResourcesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ObserveResourcesRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..aOM<$7.Exp>(1, _omitFieldNames ? '' : 'expression', subBuilder: $7.Exp.create)
    ..aOS(2, _omitFieldNames ? '' : 'collection')
    ..aOM<TxId>(3, _omitFieldNames ? '' : 'startingFrom', subBuilder: TxId.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ObserveResourcesRequest clone() => ObserveResourcesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ObserveResourcesRequest copyWith(void Function(ObserveResourcesRequest) updates) => super.copyWith((message) => updates(message as ObserveResourcesRequest)) as ObserveResourcesRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ObserveResourcesRequest create() => ObserveResourcesRequest._();
  ObserveResourcesRequest createEmptyInstance() => create();
  static $pb.PbList<ObserveResourcesRequest> createRepeated() => $pb.PbList<ObserveResourcesRequest>();
  @$core.pragma('dart2js:noInline')
  static ObserveResourcesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ObserveResourcesRequest>(create);
  static ObserveResourcesRequest? _defaultInstance;

  /// / Expression to filter resources.
  @$pb.TagNumber(1)
  $7.Exp get expression => $_getN(0);
  @$pb.TagNumber(1)
  set expression($7.Exp v) { $_setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasExpression() => $_has(0);
  @$pb.TagNumber(1)
  void clearExpression() => $_clearField(1);
  @$pb.TagNumber(1)
  $7.Exp ensureExpression() => $_ensure(0);

  /// / Collection to observe.
  @$pb.TagNumber(2)
  $core.String get collection => $_getSZ(1);
  @$pb.TagNumber(2)
  set collection($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasCollection() => $_has(1);
  @$pb.TagNumber(2)
  void clearCollection() => $_clearField(2);

  /// / Starting transaction ID for observation.
  @$pb.TagNumber(3)
  TxId get startingFrom => $_getN(2);
  @$pb.TagNumber(3)
  set startingFrom(TxId v) { $_setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasStartingFrom() => $_has(2);
  @$pb.TagNumber(3)
  void clearStartingFrom() => $_clearField(3);
  @$pb.TagNumber(3)
  TxId ensureStartingFrom() => $_ensure(2);
}

/// TxId represents a transaction ID.
class TxId extends $pb.GeneratedMessage {
  factory TxId({
    $fixnum.Int64? txId,
  }) {
    final $result = create();
    if (txId != null) {
      $result.txId = txId;
    }
    return $result;
  }
  TxId._() : super();
  factory TxId.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TxId.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'TxId', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'txId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TxId clone() => TxId()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TxId copyWith(void Function(TxId) updates) => super.copyWith((message) => updates(message as TxId)) as TxId;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TxId create() => TxId._();
  TxId createEmptyInstance() => create();
  static $pb.PbList<TxId> createRepeated() => $pb.PbList<TxId>();
  @$core.pragma('dart2js:noInline')
  static TxId getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TxId>(create);
  static TxId? _defaultInstance;

  /// / ID of the transaction.
  @$pb.TagNumber(1)
  $fixnum.Int64 get txId => $_getI64(0);
  @$pb.TagNumber(1)
  set txId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTxId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTxId() => $_clearField(1);
}

/// Request message to observe actions.
class ObserveActionsRequest extends $pb.GeneratedMessage {
  factory ObserveActionsRequest({
    TxId? startingFrom,
    $core.String? name,
    $core.Iterable<$core.List<$core.int>>? involvesAccounts,
  }) {
    final $result = create();
    if (startingFrom != null) {
      $result.startingFrom = startingFrom;
    }
    if (name != null) {
      $result.name = name;
    }
    if (involvesAccounts != null) {
      $result.involvesAccounts.addAll(involvesAccounts);
    }
    return $result;
  }
  ObserveActionsRequest._() : super();
  factory ObserveActionsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ObserveActionsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ObserveActionsRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..aOM<TxId>(1, _omitFieldNames ? '' : 'startingFrom', subBuilder: TxId.create)
    ..aOS(2, _omitFieldNames ? '' : 'name')
    ..p<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'involvesAccounts', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ObserveActionsRequest clone() => ObserveActionsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ObserveActionsRequest copyWith(void Function(ObserveActionsRequest) updates) => super.copyWith((message) => updates(message as ObserveActionsRequest)) as ObserveActionsRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ObserveActionsRequest create() => ObserveActionsRequest._();
  ObserveActionsRequest createEmptyInstance() => create();
  static $pb.PbList<ObserveActionsRequest> createRepeated() => $pb.PbList<ObserveActionsRequest>();
  @$core.pragma('dart2js:noInline')
  static ObserveActionsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ObserveActionsRequest>(create);
  static ObserveActionsRequest? _defaultInstance;

  /// / Starting transaction ID for observation.
  @$pb.TagNumber(1)
  TxId get startingFrom => $_getN(0);
  @$pb.TagNumber(1)
  set startingFrom(TxId v) { $_setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasStartingFrom() => $_has(0);
  @$pb.TagNumber(1)
  void clearStartingFrom() => $_clearField(1);
  @$pb.TagNumber(1)
  TxId ensureStartingFrom() => $_ensure(0);

  /// / Name of the action to observe.
  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => $_clearField(2);

  /// / Account IDs involved in the action.
  @$pb.TagNumber(3)
  $pb.PbList<$core.List<$core.int>> get involvesAccounts => $_getList(2);
}

/// A finalized transaction.
class FinalizedTransaction extends $pb.GeneratedMessage {
  factory FinalizedTransaction({
    $1.TransactionRequestPayload? request,
    $1.TransactionResponse? response,
  }) {
    final $result = create();
    if (request != null) {
      $result.request = request;
    }
    if (response != null) {
      $result.response = response;
    }
    return $result;
  }
  FinalizedTransaction._() : super();
  factory FinalizedTransaction.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FinalizedTransaction.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'FinalizedTransaction', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..aOM<$1.TransactionRequestPayload>(1, _omitFieldNames ? '' : 'request', subBuilder: $1.TransactionRequestPayload.create)
    ..aOM<$1.TransactionResponse>(2, _omitFieldNames ? '' : 'response', subBuilder: $1.TransactionResponse.create)
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FinalizedTransaction clone() => FinalizedTransaction()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FinalizedTransaction copyWith(void Function(FinalizedTransaction) updates) => super.copyWith((message) => updates(message as FinalizedTransaction)) as FinalizedTransaction;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FinalizedTransaction create() => FinalizedTransaction._();
  FinalizedTransaction createEmptyInstance() => create();
  static $pb.PbList<FinalizedTransaction> createRepeated() => $pb.PbList<FinalizedTransaction>();
  @$core.pragma('dart2js:noInline')
  static FinalizedTransaction getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FinalizedTransaction>(create);
  static FinalizedTransaction? _defaultInstance;

  /// / Transaction request.
  @$pb.TagNumber(1)
  $1.TransactionRequestPayload get request => $_getN(0);
  @$pb.TagNumber(1)
  set request($1.TransactionRequestPayload v) { $_setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearRequest() => $_clearField(1);
  @$pb.TagNumber(1)
  $1.TransactionRequestPayload ensureRequest() => $_ensure(0);

  /// / Transaction response.
  @$pb.TagNumber(2)
  $1.TransactionResponse get response => $_getN(1);
  @$pb.TagNumber(2)
  set response($1.TransactionResponse v) { $_setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasResponse() => $_has(1);
  @$pb.TagNumber(2)
  void clearResponse() => $_clearField(2);
  @$pb.TagNumber(2)
  $1.TransactionResponse ensureResponse() => $_ensure(1);
}

/// A list of finalized transactions.
class FinalizedTransactions extends $pb.GeneratedMessage {
  factory FinalizedTransactions({
    $core.Iterable<FinalizedTransaction>? transactions,
  }) {
    final $result = create();
    if (transactions != null) {
      $result.transactions.addAll(transactions);
    }
    return $result;
  }
  FinalizedTransactions._() : super();
  factory FinalizedTransactions.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FinalizedTransactions.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'FinalizedTransactions', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..pc<FinalizedTransaction>(1, _omitFieldNames ? '' : 'transactions', $pb.PbFieldType.PM, subBuilder: FinalizedTransaction.create)
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FinalizedTransactions clone() => FinalizedTransactions()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FinalizedTransactions copyWith(void Function(FinalizedTransactions) updates) => super.copyWith((message) => updates(message as FinalizedTransactions)) as FinalizedTransactions;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static FinalizedTransactions create() => FinalizedTransactions._();
  FinalizedTransactions createEmptyInstance() => create();
  static $pb.PbList<FinalizedTransactions> createRepeated() => $pb.PbList<FinalizedTransactions>();
  @$core.pragma('dart2js:noInline')
  static FinalizedTransactions getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FinalizedTransactions>(create);
  static FinalizedTransactions? _defaultInstance;

  /// / List of finalized transactions.
  @$pb.TagNumber(1)
  $pb.PbList<FinalizedTransaction> get transactions => $_getList(0);
}

/// A list of grouped finalized transactions.
class GroupedFinalizedTransactions extends $pb.GeneratedMessage {
  factory GroupedFinalizedTransactions({
    $core.Iterable<FinalizedTransactions>? groups,
  }) {
    final $result = create();
    if (groups != null) {
      $result.groups.addAll(groups);
    }
    return $result;
  }
  GroupedFinalizedTransactions._() : super();
  factory GroupedFinalizedTransactions.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupedFinalizedTransactions.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GroupedFinalizedTransactions', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..pc<FinalizedTransactions>(1, _omitFieldNames ? '' : 'groups', $pb.PbFieldType.PM, subBuilder: FinalizedTransactions.create)
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupedFinalizedTransactions clone() => GroupedFinalizedTransactions()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupedFinalizedTransactions copyWith(void Function(GroupedFinalizedTransactions) updates) => super.copyWith((message) => updates(message as GroupedFinalizedTransactions)) as GroupedFinalizedTransactions;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GroupedFinalizedTransactions create() => GroupedFinalizedTransactions._();
  GroupedFinalizedTransactions createEmptyInstance() => create();
  static $pb.PbList<GroupedFinalizedTransactions> createRepeated() => $pb.PbList<GroupedFinalizedTransactions>();
  @$core.pragma('dart2js:noInline')
  static GroupedFinalizedTransactions getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupedFinalizedTransactions>(create);
  static GroupedFinalizedTransactions? _defaultInstance;

  /// / List of grouped transactions.
  @$pb.TagNumber(1)
  $pb.PbList<FinalizedTransactions> get groups => $_getList(0);
}

/// Information about the blockchain.
class ChainInfo extends $pb.GeneratedMessage {
  factory ChainInfo({
    $fixnum.Int64? blockHeight,
  }) {
    final $result = create();
    if (blockHeight != null) {
      $result.blockHeight = blockHeight;
    }
    return $result;
  }
  ChainInfo._() : super();
  factory ChainInfo.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChainInfo.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChainInfo', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'blockHeight', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChainInfo clone() => ChainInfo()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChainInfo copyWith(void Function(ChainInfo) updates) => super.copyWith((message) => updates(message as ChainInfo)) as ChainInfo;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChainInfo create() => ChainInfo._();
  ChainInfo createEmptyInstance() => create();
  static $pb.PbList<ChainInfo> createRepeated() => $pb.PbList<ChainInfo>();
  @$core.pragma('dart2js:noInline')
  static ChainInfo getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChainInfo>(create);
  static ChainInfo? _defaultInstance;

  /// / Current block height.
  @$pb.TagNumber(1)
  $fixnum.Int64 get blockHeight => $_getI64(0);
  @$pb.TagNumber(1)
  set blockHeight($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasBlockHeight() => $_has(0);
  @$pb.TagNumber(1)
  void clearBlockHeight() => $_clearField(1);
}

/// / Transaction metrics.
class TransactionMetrics extends $pb.GeneratedMessage {
  factory TransactionMetrics({
    $fixnum.Int64? transferVolume,
    $fixnum.Int64? transferCount,
    $fixnum.Int64? transferErrors,
    $fixnum.Int64? accountsCreated,
  }) {
    final $result = create();
    if (transferVolume != null) {
      $result.transferVolume = transferVolume;
    }
    if (transferCount != null) {
      $result.transferCount = transferCount;
    }
    if (transferErrors != null) {
      $result.transferErrors = transferErrors;
    }
    if (accountsCreated != null) {
      $result.accountsCreated = accountsCreated;
    }
    return $result;
  }
  TransactionMetrics._() : super();
  factory TransactionMetrics.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TransactionMetrics.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'TransactionMetrics', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'transferVolume', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(2, _omitFieldNames ? '' : 'transferCount', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(3, _omitFieldNames ? '' : 'transferErrors', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(4, _omitFieldNames ? '' : 'accountsCreated', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TransactionMetrics clone() => TransactionMetrics()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TransactionMetrics copyWith(void Function(TransactionMetrics) updates) => super.copyWith((message) => updates(message as TransactionMetrics)) as TransactionMetrics;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static TransactionMetrics create() => TransactionMetrics._();
  TransactionMetrics createEmptyInstance() => create();
  static $pb.PbList<TransactionMetrics> createRepeated() => $pb.PbList<TransactionMetrics>();
  @$core.pragma('dart2js:noInline')
  static TransactionMetrics getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TransactionMetrics>(create);
  static TransactionMetrics? _defaultInstance;

  /// / Total transfer volume.
  @$pb.TagNumber(1)
  $fixnum.Int64 get transferVolume => $_getI64(0);
  @$pb.TagNumber(1)
  set transferVolume($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTransferVolume() => $_has(0);
  @$pb.TagNumber(1)
  void clearTransferVolume() => $_clearField(1);

  /// / Number of transfers.
  @$pb.TagNumber(2)
  $fixnum.Int64 get transferCount => $_getI64(1);
  @$pb.TagNumber(2)
  set transferCount($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasTransferCount() => $_has(1);
  @$pb.TagNumber(2)
  void clearTransferCount() => $_clearField(2);

  /// / Number of transfer errors.
  @$pb.TagNumber(3)
  $fixnum.Int64 get transferErrors => $_getI64(2);
  @$pb.TagNumber(3)
  set transferErrors($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasTransferErrors() => $_has(2);
  @$pb.TagNumber(3)
  void clearTransferErrors() => $_clearField(3);

  /// / Number of accounts created.
  @$pb.TagNumber(4)
  $fixnum.Int64 get accountsCreated => $_getI64(3);
  @$pb.TagNumber(4)
  set accountsCreated($fixnum.Int64 v) { $_setInt64(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasAccountsCreated() => $_has(3);
  @$pb.TagNumber(4)
  void clearAccountsCreated() => $_clearField(4);
}

/// / Request list of banks.
class ListBanksRequest extends $pb.GeneratedMessage {
  factory ListBanksRequest({
    Page? page,
  }) {
    final $result = create();
    if (page != null) {
      $result.page = page;
    }
    return $result;
  }
  ListBanksRequest._() : super();
  factory ListBanksRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListBanksRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ListBanksRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..aOM<Page>(1, _omitFieldNames ? '' : 'page', subBuilder: Page.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListBanksRequest clone() => ListBanksRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListBanksRequest copyWith(void Function(ListBanksRequest) updates) => super.copyWith((message) => updates(message as ListBanksRequest)) as ListBanksRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListBanksRequest create() => ListBanksRequest._();
  ListBanksRequest createEmptyInstance() => create();
  static $pb.PbList<ListBanksRequest> createRepeated() => $pb.PbList<ListBanksRequest>();
  @$core.pragma('dart2js:noInline')
  static ListBanksRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListBanksRequest>(create);
  static ListBanksRequest? _defaultInstance;

  /// / Pagination options.
  @$pb.TagNumber(1)
  Page get page => $_getN(0);
  @$pb.TagNumber(1)
  set page(Page v) { $_setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasPage() => $_has(0);
  @$pb.TagNumber(1)
  void clearPage() => $_clearField(1);
  @$pb.TagNumber(1)
  Page ensurePage() => $_ensure(0);
}

/// / Receive list of banks.
class ListBanksResponse extends $pb.GeneratedMessage {
  factory ListBanksResponse({
    $core.Iterable<$2.Bank>? banks,
  }) {
    final $result = create();
    if (banks != null) {
      $result.banks.addAll(banks);
    }
    return $result;
  }
  ListBanksResponse._() : super();
  factory ListBanksResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListBanksResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ListBanksResponse', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..pc<$2.Bank>(1, _omitFieldNames ? '' : 'banks', $pb.PbFieldType.PM, subBuilder: $2.Bank.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListBanksResponse clone() => ListBanksResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListBanksResponse copyWith(void Function(ListBanksResponse) updates) => super.copyWith((message) => updates(message as ListBanksResponse)) as ListBanksResponse;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListBanksResponse create() => ListBanksResponse._();
  ListBanksResponse createEmptyInstance() => create();
  static $pb.PbList<ListBanksResponse> createRepeated() => $pb.PbList<ListBanksResponse>();
  @$core.pragma('dart2js:noInline')
  static ListBanksResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListBanksResponse>(create);
  static ListBanksResponse? _defaultInstance;

  /// / List of banks.
  @$pb.TagNumber(1)
  $pb.PbList<$2.Bank> get banks => $_getList(0);
}

/// Retrieve a specific bank.
class GetBankRequest extends $pb.GeneratedMessage {
  factory GetBankRequest({
    $core.List<$core.int>? id,
  }) {
    final $result = create();
    if (id != null) {
      $result.id = id;
    }
    return $result;
  }
  GetBankRequest._() : super();
  factory GetBankRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetBankRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'GetBankRequest', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetBankRequest clone() => GetBankRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetBankRequest copyWith(void Function(GetBankRequest) updates) => super.copyWith((message) => updates(message as GetBankRequest)) as GetBankRequest;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GetBankRequest create() => GetBankRequest._();
  GetBankRequest createEmptyInstance() => create();
  static $pb.PbList<GetBankRequest> createRepeated() => $pb.PbList<GetBankRequest>();
  @$core.pragma('dart2js:noInline')
  static GetBankRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetBankRequest>(create);
  static GetBankRequest? _defaultInstance;

  /// / ID of the bank to retrieve.
  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);
}

/// / Offline key.
class OfflineKey extends $pb.GeneratedMessage {
  factory OfflineKey({
    $core.List<$core.int>? offlinePk,
  }) {
    final $result = create();
    if (offlinePk != null) {
      $result.offlinePk = offlinePk;
    }
    return $result;
  }
  OfflineKey._() : super();
  factory OfflineKey.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory OfflineKey.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'OfflineKey', package: const $pb.PackageName(_omitMessageNames ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'offlinePk', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  OfflineKey clone() => OfflineKey()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  OfflineKey copyWith(void Function(OfflineKey) updates) => super.copyWith((message) => updates(message as OfflineKey)) as OfflineKey;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static OfflineKey create() => OfflineKey._();
  OfflineKey createEmptyInstance() => create();
  static $pb.PbList<OfflineKey> createRepeated() => $pb.PbList<OfflineKey>();
  @$core.pragma('dart2js:noInline')
  static OfflineKey getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<OfflineKey>(create);
  static OfflineKey? _defaultInstance;

  /// The offline public key.
  @$pb.TagNumber(1)
  $core.List<$core.int> get offlinePk => $_getN(0);
  @$pb.TagNumber(1)
  set offlinePk($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasOfflinePk() => $_has(0);
  @$pb.TagNumber(1)
  void clearOfflinePk() => $_clearField(1);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
