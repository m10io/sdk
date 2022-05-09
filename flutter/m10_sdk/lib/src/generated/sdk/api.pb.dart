///
//  Generated code. Do not modify.
//  source: sdk/api.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'model/model.pb.dart' as $2;
import 'rbac.pb.dart' as $4;
import 'document.pb.dart' as $7;
import 'transaction/transaction.pb.dart' as $1;

import 'api.pbenum.dart';

export 'api.pbenum.dart';

class RequestEnvelope extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'RequestEnvelope', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'payload', $pb.PbFieldType.OY)
    ..aOM<Signature>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signature', subBuilder: Signature.create)
    ..hasRequiredFields = false
  ;

  RequestEnvelope._() : super();
  factory RequestEnvelope({
    $core.List<$core.int>? payload,
    Signature? signature,
  }) {
    final _result = create();
    if (payload != null) {
      _result.payload = payload;
    }
    if (signature != null) {
      _result.signature = signature;
    }
    return _result;
  }
  factory RequestEnvelope.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RequestEnvelope.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RequestEnvelope clone() => RequestEnvelope()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RequestEnvelope copyWith(void Function(RequestEnvelope) updates) => super.copyWith((message) => updates(message as RequestEnvelope)) as RequestEnvelope; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RequestEnvelope create() => RequestEnvelope._();
  RequestEnvelope createEmptyInstance() => create();
  static $pb.PbList<RequestEnvelope> createRepeated() => $pb.PbList<RequestEnvelope>();
  @$core.pragma('dart2js:noInline')
  static RequestEnvelope getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RequestEnvelope>(create);
  static RequestEnvelope? _defaultInstance;

  @$pb.TagNumber(2)
  $core.List<$core.int> get payload => $_getN(0);
  @$pb.TagNumber(2)
  set payload($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(2)
  $core.bool hasPayload() => $_has(0);
  @$pb.TagNumber(2)
  void clearPayload() => clearField(2);

  @$pb.TagNumber(3)
  Signature get signature => $_getN(1);
  @$pb.TagNumber(3)
  set signature(Signature v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasSignature() => $_has(1);
  @$pb.TagNumber(3)
  void clearSignature() => clearField(3);
  @$pb.TagNumber(3)
  Signature ensureSignature() => $_ensure(1);
}

class Signature extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Signature', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'publicKey', $pb.PbFieldType.OY)
    ..a<$core.List<$core.int>>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'signature', $pb.PbFieldType.OY)
    ..e<Signature_Algorithm>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'algorithm', $pb.PbFieldType.OE, defaultOrMaker: Signature_Algorithm.P256_SHA256_ASN1, valueOf: Signature_Algorithm.valueOf, enumValues: Signature_Algorithm.values)
    ..hasRequiredFields = false
  ;

  Signature._() : super();
  factory Signature({
    $core.List<$core.int>? publicKey,
    $core.List<$core.int>? signature,
    Signature_Algorithm? algorithm,
  }) {
    final _result = create();
    if (publicKey != null) {
      _result.publicKey = publicKey;
    }
    if (signature != null) {
      _result.signature = signature;
    }
    if (algorithm != null) {
      _result.algorithm = algorithm;
    }
    return _result;
  }
  factory Signature.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Signature.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Signature clone() => Signature()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Signature copyWith(void Function(Signature) updates) => super.copyWith((message) => updates(message as Signature)) as Signature; // ignore: deprecated_member_use
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

class Page extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Page', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$core.int>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'limit', $pb.PbFieldType.OU3)
    ..a<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'lastId', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  Page._() : super();
  factory Page({
    $core.int? limit,
    $core.List<$core.int>? lastId,
  }) {
    final _result = create();
    if (limit != null) {
      _result.limit = limit;
    }
    if (lastId != null) {
      _result.lastId = lastId;
    }
    return _result;
  }
  factory Page.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Page.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Page clone() => Page()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Page copyWith(void Function(Page) updates) => super.copyWith((message) => updates(message as Page)) as Page; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Page create() => Page._();
  Page createEmptyInstance() => create();
  static $pb.PbList<Page> createRepeated() => $pb.PbList<Page>();
  @$core.pragma('dart2js:noInline')
  static Page getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Page>(create);
  static Page? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get limit => $_getIZ(0);
  @$pb.TagNumber(1)
  set limit($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasLimit() => $_has(0);
  @$pb.TagNumber(1)
  void clearLimit() => clearField(1);

  @$pb.TagNumber(2)
  $core.List<$core.int> get lastId => $_getN(1);
  @$pb.TagNumber(2)
  set lastId($core.List<$core.int> v) { $_setBytes(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasLastId() => $_has(1);
  @$pb.TagNumber(2)
  void clearLastId() => clearField(2);
}

class GetAccountSetRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetAccountSetRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  GetAccountSetRequest._() : super();
  factory GetAccountSetRequest({
    $core.List<$core.int>? id,
  }) {
    final _result = create();
    if (id != null) {
      _result.id = id;
    }
    return _result;
  }
  factory GetAccountSetRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetAccountSetRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetAccountSetRequest clone() => GetAccountSetRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetAccountSetRequest copyWith(void Function(GetAccountSetRequest) updates) => super.copyWith((message) => updates(message as GetAccountSetRequest)) as GetAccountSetRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetAccountSetRequest create() => GetAccountSetRequest._();
  GetAccountSetRequest createEmptyInstance() => create();
  static $pb.PbList<GetAccountSetRequest> createRepeated() => $pb.PbList<GetAccountSetRequest>();
  @$core.pragma('dart2js:noInline')
  static GetAccountSetRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetAccountSetRequest>(create);
  static GetAccountSetRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);
}

enum ListAccountSetsRequest_Filter {
  owner, 
  name, 
  notSet
}

class ListAccountSetsRequest extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, ListAccountSetsRequest_Filter> _ListAccountSetsRequest_FilterByTag = {
    1 : ListAccountSetsRequest_Filter.owner,
    2 : ListAccountSetsRequest_Filter.name,
    0 : ListAccountSetsRequest_Filter.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ListAccountSetsRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'owner', $pb.PbFieldType.OY)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..aOM<Page>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'page', subBuilder: Page.create)
    ..hasRequiredFields = false
  ;

  ListAccountSetsRequest._() : super();
  factory ListAccountSetsRequest({
    $core.List<$core.int>? owner,
    $core.String? name,
    Page? page,
  }) {
    final _result = create();
    if (owner != null) {
      _result.owner = owner;
    }
    if (name != null) {
      _result.name = name;
    }
    if (page != null) {
      _result.page = page;
    }
    return _result;
  }
  factory ListAccountSetsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListAccountSetsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListAccountSetsRequest clone() => ListAccountSetsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListAccountSetsRequest copyWith(void Function(ListAccountSetsRequest) updates) => super.copyWith((message) => updates(message as ListAccountSetsRequest)) as ListAccountSetsRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ListAccountSetsRequest create() => ListAccountSetsRequest._();
  ListAccountSetsRequest createEmptyInstance() => create();
  static $pb.PbList<ListAccountSetsRequest> createRepeated() => $pb.PbList<ListAccountSetsRequest>();
  @$core.pragma('dart2js:noInline')
  static ListAccountSetsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListAccountSetsRequest>(create);
  static ListAccountSetsRequest? _defaultInstance;

  ListAccountSetsRequest_Filter whichFilter() => _ListAccountSetsRequest_FilterByTag[$_whichOneof(0)]!;
  void clearFilter() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.List<$core.int> get owner => $_getN(0);
  @$pb.TagNumber(1)
  set owner($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasOwner() => $_has(0);
  @$pb.TagNumber(1)
  void clearOwner() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => clearField(2);

  @$pb.TagNumber(4)
  Page get page => $_getN(2);
  @$pb.TagNumber(4)
  set page(Page v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasPage() => $_has(2);
  @$pb.TagNumber(4)
  void clearPage() => clearField(4);
  @$pb.TagNumber(4)
  Page ensurePage() => $_ensure(2);
}

class ListAccountSetsResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ListAccountSetsResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..pc<$2.AccountSet>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountSets', $pb.PbFieldType.PM, subBuilder: $2.AccountSet.create)
    ..aOM<ListAccountSetsRequest>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'nextRequest', subBuilder: ListAccountSetsRequest.create)
    ..hasRequiredFields = false
  ;

  ListAccountSetsResponse._() : super();
  factory ListAccountSetsResponse({
    $core.Iterable<$2.AccountSet>? accountSets,
    ListAccountSetsRequest? nextRequest,
  }) {
    final _result = create();
    if (accountSets != null) {
      _result.accountSets.addAll(accountSets);
    }
    if (nextRequest != null) {
      _result.nextRequest = nextRequest;
    }
    return _result;
  }
  factory ListAccountSetsResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListAccountSetsResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListAccountSetsResponse clone() => ListAccountSetsResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListAccountSetsResponse copyWith(void Function(ListAccountSetsResponse) updates) => super.copyWith((message) => updates(message as ListAccountSetsResponse)) as ListAccountSetsResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ListAccountSetsResponse create() => ListAccountSetsResponse._();
  ListAccountSetsResponse createEmptyInstance() => create();
  static $pb.PbList<ListAccountSetsResponse> createRepeated() => $pb.PbList<ListAccountSetsResponse>();
  @$core.pragma('dart2js:noInline')
  static ListAccountSetsResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListAccountSetsResponse>(create);
  static ListAccountSetsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$2.AccountSet> get accountSets => $_getList(0);

  @$pb.TagNumber(2)
  ListAccountSetsRequest get nextRequest => $_getN(1);
  @$pb.TagNumber(2)
  set nextRequest(ListAccountSetsRequest v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasNextRequest() => $_has(1);
  @$pb.TagNumber(2)
  void clearNextRequest() => clearField(2);
  @$pb.TagNumber(2)
  ListAccountSetsRequest ensureNextRequest() => $_ensure(1);
}

enum ListAccountsRequest_Filter {
  owner, 
  notSet
}

class ListAccountsRequest extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, ListAccountsRequest_Filter> _ListAccountsRequest_FilterByTag = {
    1 : ListAccountsRequest_Filter.owner,
    0 : ListAccountsRequest_Filter.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ListAccountsRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..oo(0, [1])
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'owner', $pb.PbFieldType.OY)
    ..aOM<Page>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'page', subBuilder: Page.create)
    ..hasRequiredFields = false
  ;

  ListAccountsRequest._() : super();
  factory ListAccountsRequest({
    $core.List<$core.int>? owner,
    Page? page,
  }) {
    final _result = create();
    if (owner != null) {
      _result.owner = owner;
    }
    if (page != null) {
      _result.page = page;
    }
    return _result;
  }
  factory ListAccountsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListAccountsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListAccountsRequest clone() => ListAccountsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListAccountsRequest copyWith(void Function(ListAccountsRequest) updates) => super.copyWith((message) => updates(message as ListAccountsRequest)) as ListAccountsRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ListAccountsRequest create() => ListAccountsRequest._();
  ListAccountsRequest createEmptyInstance() => create();
  static $pb.PbList<ListAccountsRequest> createRepeated() => $pb.PbList<ListAccountsRequest>();
  @$core.pragma('dart2js:noInline')
  static ListAccountsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListAccountsRequest>(create);
  static ListAccountsRequest? _defaultInstance;

  ListAccountsRequest_Filter whichFilter() => _ListAccountsRequest_FilterByTag[$_whichOneof(0)]!;
  void clearFilter() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.List<$core.int> get owner => $_getN(0);
  @$pb.TagNumber(1)
  set owner($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasOwner() => $_has(0);
  @$pb.TagNumber(1)
  void clearOwner() => clearField(1);

  @$pb.TagNumber(4)
  Page get page => $_getN(1);
  @$pb.TagNumber(4)
  set page(Page v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasPage() => $_has(1);
  @$pb.TagNumber(4)
  void clearPage() => clearField(4);
  @$pb.TagNumber(4)
  Page ensurePage() => $_ensure(1);
}

class ListAccountsResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ListAccountsResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..pc<$2.Account>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accounts', $pb.PbFieldType.PM, subBuilder: $2.Account.create)
    ..aOM<ListAccountsRequest>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'nextRequest', subBuilder: ListAccountsRequest.create)
    ..hasRequiredFields = false
  ;

  ListAccountsResponse._() : super();
  factory ListAccountsResponse({
    $core.Iterable<$2.Account>? accounts,
    ListAccountsRequest? nextRequest,
  }) {
    final _result = create();
    if (accounts != null) {
      _result.accounts.addAll(accounts);
    }
    if (nextRequest != null) {
      _result.nextRequest = nextRequest;
    }
    return _result;
  }
  factory ListAccountsResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListAccountsResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListAccountsResponse clone() => ListAccountsResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListAccountsResponse copyWith(void Function(ListAccountsResponse) updates) => super.copyWith((message) => updates(message as ListAccountsResponse)) as ListAccountsResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ListAccountsResponse create() => ListAccountsResponse._();
  ListAccountsResponse createEmptyInstance() => create();
  static $pb.PbList<ListAccountsResponse> createRepeated() => $pb.PbList<ListAccountsResponse>();
  @$core.pragma('dart2js:noInline')
  static ListAccountsResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListAccountsResponse>(create);
  static ListAccountsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$2.Account> get accounts => $_getList(0);

  @$pb.TagNumber(2)
  ListAccountsRequest get nextRequest => $_getN(1);
  @$pb.TagNumber(2)
  set nextRequest(ListAccountsRequest v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasNextRequest() => $_has(1);
  @$pb.TagNumber(2)
  void clearNextRequest() => clearField(2);
  @$pb.TagNumber(2)
  ListAccountsRequest ensureNextRequest() => $_ensure(1);
}

class GetRoleBindingRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetRoleBindingRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  GetRoleBindingRequest._() : super();
  factory GetRoleBindingRequest({
    $core.List<$core.int>? id,
  }) {
    final _result = create();
    if (id != null) {
      _result.id = id;
    }
    return _result;
  }
  factory GetRoleBindingRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetRoleBindingRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetRoleBindingRequest clone() => GetRoleBindingRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetRoleBindingRequest copyWith(void Function(GetRoleBindingRequest) updates) => super.copyWith((message) => updates(message as GetRoleBindingRequest)) as GetRoleBindingRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetRoleBindingRequest create() => GetRoleBindingRequest._();
  GetRoleBindingRequest createEmptyInstance() => create();
  static $pb.PbList<GetRoleBindingRequest> createRepeated() => $pb.PbList<GetRoleBindingRequest>();
  @$core.pragma('dart2js:noInline')
  static GetRoleBindingRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetRoleBindingRequest>(create);
  static GetRoleBindingRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);
}

enum ListRoleBindingsRequest_Filter {
  name, 
  notSet
}

class ListRoleBindingsRequest extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, ListRoleBindingsRequest_Filter> _ListRoleBindingsRequest_FilterByTag = {
    1 : ListRoleBindingsRequest_Filter.name,
    0 : ListRoleBindingsRequest_Filter.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ListRoleBindingsRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..oo(0, [1])
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..aOM<Page>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'page', subBuilder: Page.create)
    ..hasRequiredFields = false
  ;

  ListRoleBindingsRequest._() : super();
  factory ListRoleBindingsRequest({
    $core.String? name,
    Page? page,
  }) {
    final _result = create();
    if (name != null) {
      _result.name = name;
    }
    if (page != null) {
      _result.page = page;
    }
    return _result;
  }
  factory ListRoleBindingsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListRoleBindingsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListRoleBindingsRequest clone() => ListRoleBindingsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListRoleBindingsRequest copyWith(void Function(ListRoleBindingsRequest) updates) => super.copyWith((message) => updates(message as ListRoleBindingsRequest)) as ListRoleBindingsRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ListRoleBindingsRequest create() => ListRoleBindingsRequest._();
  ListRoleBindingsRequest createEmptyInstance() => create();
  static $pb.PbList<ListRoleBindingsRequest> createRepeated() => $pb.PbList<ListRoleBindingsRequest>();
  @$core.pragma('dart2js:noInline')
  static ListRoleBindingsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListRoleBindingsRequest>(create);
  static ListRoleBindingsRequest? _defaultInstance;

  ListRoleBindingsRequest_Filter whichFilter() => _ListRoleBindingsRequest_FilterByTag[$_whichOneof(0)]!;
  void clearFilter() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => clearField(1);

  @$pb.TagNumber(4)
  Page get page => $_getN(1);
  @$pb.TagNumber(4)
  set page(Page v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasPage() => $_has(1);
  @$pb.TagNumber(4)
  void clearPage() => clearField(4);
  @$pb.TagNumber(4)
  Page ensurePage() => $_ensure(1);
}

class ListRoleBindingsResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ListRoleBindingsResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..pc<$4.RoleBinding>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'roleBindings', $pb.PbFieldType.PM, subBuilder: $4.RoleBinding.create)
    ..aOM<ListRoleBindingsRequest>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'nextRequest', subBuilder: ListRoleBindingsRequest.create)
    ..hasRequiredFields = false
  ;

  ListRoleBindingsResponse._() : super();
  factory ListRoleBindingsResponse({
    $core.Iterable<$4.RoleBinding>? roleBindings,
    ListRoleBindingsRequest? nextRequest,
  }) {
    final _result = create();
    if (roleBindings != null) {
      _result.roleBindings.addAll(roleBindings);
    }
    if (nextRequest != null) {
      _result.nextRequest = nextRequest;
    }
    return _result;
  }
  factory ListRoleBindingsResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListRoleBindingsResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListRoleBindingsResponse clone() => ListRoleBindingsResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListRoleBindingsResponse copyWith(void Function(ListRoleBindingsResponse) updates) => super.copyWith((message) => updates(message as ListRoleBindingsResponse)) as ListRoleBindingsResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ListRoleBindingsResponse create() => ListRoleBindingsResponse._();
  ListRoleBindingsResponse createEmptyInstance() => create();
  static $pb.PbList<ListRoleBindingsResponse> createRepeated() => $pb.PbList<ListRoleBindingsResponse>();
  @$core.pragma('dart2js:noInline')
  static ListRoleBindingsResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListRoleBindingsResponse>(create);
  static ListRoleBindingsResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$4.RoleBinding> get roleBindings => $_getList(0);

  @$pb.TagNumber(2)
  ListRoleBindingsRequest get nextRequest => $_getN(1);
  @$pb.TagNumber(2)
  set nextRequest(ListRoleBindingsRequest v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasNextRequest() => $_has(1);
  @$pb.TagNumber(2)
  void clearNextRequest() => clearField(2);
  @$pb.TagNumber(2)
  ListRoleBindingsRequest ensureNextRequest() => $_ensure(1);
}

class GetRoleRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetRoleRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'id', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  GetRoleRequest._() : super();
  factory GetRoleRequest({
    $core.List<$core.int>? id,
  }) {
    final _result = create();
    if (id != null) {
      _result.id = id;
    }
    return _result;
  }
  factory GetRoleRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetRoleRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetRoleRequest clone() => GetRoleRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetRoleRequest copyWith(void Function(GetRoleRequest) updates) => super.copyWith((message) => updates(message as GetRoleRequest)) as GetRoleRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetRoleRequest create() => GetRoleRequest._();
  GetRoleRequest createEmptyInstance() => create();
  static $pb.PbList<GetRoleRequest> createRepeated() => $pb.PbList<GetRoleRequest>();
  @$core.pragma('dart2js:noInline')
  static GetRoleRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetRoleRequest>(create);
  static GetRoleRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get id => $_getN(0);
  @$pb.TagNumber(1)
  set id($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);
}

enum ListRolesRequest_Filter {
  name, 
  notSet
}

class ListRolesRequest extends $pb.GeneratedMessage {
  static const $core.Map<$core.int, ListRolesRequest_Filter> _ListRolesRequest_FilterByTag = {
    1 : ListRolesRequest_Filter.name,
    0 : ListRolesRequest_Filter.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ListRolesRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..oo(0, [1])
    ..aOS(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..aOM<Page>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'page', subBuilder: Page.create)
    ..hasRequiredFields = false
  ;

  ListRolesRequest._() : super();
  factory ListRolesRequest({
    $core.String? name,
    Page? page,
  }) {
    final _result = create();
    if (name != null) {
      _result.name = name;
    }
    if (page != null) {
      _result.page = page;
    }
    return _result;
  }
  factory ListRolesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListRolesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListRolesRequest clone() => ListRolesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListRolesRequest copyWith(void Function(ListRolesRequest) updates) => super.copyWith((message) => updates(message as ListRolesRequest)) as ListRolesRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ListRolesRequest create() => ListRolesRequest._();
  ListRolesRequest createEmptyInstance() => create();
  static $pb.PbList<ListRolesRequest> createRepeated() => $pb.PbList<ListRolesRequest>();
  @$core.pragma('dart2js:noInline')
  static ListRolesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListRolesRequest>(create);
  static ListRolesRequest? _defaultInstance;

  ListRolesRequest_Filter whichFilter() => _ListRolesRequest_FilterByTag[$_whichOneof(0)]!;
  void clearFilter() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.String get name => $_getSZ(0);
  @$pb.TagNumber(1)
  set name($core.String v) { $_setString(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasName() => $_has(0);
  @$pb.TagNumber(1)
  void clearName() => clearField(1);

  @$pb.TagNumber(4)
  Page get page => $_getN(1);
  @$pb.TagNumber(4)
  set page(Page v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasPage() => $_has(1);
  @$pb.TagNumber(4)
  void clearPage() => clearField(4);
  @$pb.TagNumber(4)
  Page ensurePage() => $_ensure(1);
}

class ListRolesResponse extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ListRolesResponse', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..pc<$4.Role>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'roles', $pb.PbFieldType.PM, subBuilder: $4.Role.create)
    ..aOM<ListRolesRequest>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'nextRequest', subBuilder: ListRolesRequest.create)
    ..hasRequiredFields = false
  ;

  ListRolesResponse._() : super();
  factory ListRolesResponse({
    $core.Iterable<$4.Role>? roles,
    ListRolesRequest? nextRequest,
  }) {
    final _result = create();
    if (roles != null) {
      _result.roles.addAll(roles);
    }
    if (nextRequest != null) {
      _result.nextRequest = nextRequest;
    }
    return _result;
  }
  factory ListRolesResponse.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListRolesResponse.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListRolesResponse clone() => ListRolesResponse()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListRolesResponse copyWith(void Function(ListRolesResponse) updates) => super.copyWith((message) => updates(message as ListRolesResponse)) as ListRolesResponse; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ListRolesResponse create() => ListRolesResponse._();
  ListRolesResponse createEmptyInstance() => create();
  static $pb.PbList<ListRolesResponse> createRepeated() => $pb.PbList<ListRolesResponse>();
  @$core.pragma('dart2js:noInline')
  static ListRolesResponse getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListRolesResponse>(create);
  static ListRolesResponse? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$4.Role> get roles => $_getList(0);

  @$pb.TagNumber(2)
  ListRolesRequest get nextRequest => $_getN(1);
  @$pb.TagNumber(2)
  set nextRequest(ListRolesRequest v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasNextRequest() => $_has(1);
  @$pb.TagNumber(2)
  void clearNextRequest() => clearField(2);
  @$pb.TagNumber(2)
  ListRolesRequest ensureNextRequest() => $_ensure(1);
}

class GetTransactionRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GetTransactionRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'txId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  GetTransactionRequest._() : super();
  factory GetTransactionRequest({
    $fixnum.Int64? txId,
  }) {
    final _result = create();
    if (txId != null) {
      _result.txId = txId;
    }
    return _result;
  }
  factory GetTransactionRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GetTransactionRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GetTransactionRequest clone() => GetTransactionRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GetTransactionRequest copyWith(void Function(GetTransactionRequest) updates) => super.copyWith((message) => updates(message as GetTransactionRequest)) as GetTransactionRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GetTransactionRequest create() => GetTransactionRequest._();
  GetTransactionRequest createEmptyInstance() => create();
  static $pb.PbList<GetTransactionRequest> createRepeated() => $pb.PbList<GetTransactionRequest>();
  @$core.pragma('dart2js:noInline')
  static GetTransactionRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GetTransactionRequest>(create);
  static GetTransactionRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get txId => $_getI64(0);
  @$pb.TagNumber(1)
  set txId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTxId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTxId() => clearField(1);
}

class ListTransactionsRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ListTransactionsRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'contextId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'limit', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'minTxId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'maxTxId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  ListTransactionsRequest._() : super();
  factory ListTransactionsRequest({
    $core.List<$core.int>? contextId,
    $fixnum.Int64? limit,
    $fixnum.Int64? minTxId,
    $fixnum.Int64? maxTxId,
  }) {
    final _result = create();
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
  factory ListTransactionsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListTransactionsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListTransactionsRequest clone() => ListTransactionsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListTransactionsRequest copyWith(void Function(ListTransactionsRequest) updates) => super.copyWith((message) => updates(message as ListTransactionsRequest)) as ListTransactionsRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ListTransactionsRequest create() => ListTransactionsRequest._();
  ListTransactionsRequest createEmptyInstance() => create();
  static $pb.PbList<ListTransactionsRequest> createRepeated() => $pb.PbList<ListTransactionsRequest>();
  @$core.pragma('dart2js:noInline')
  static ListTransactionsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListTransactionsRequest>(create);
  static ListTransactionsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get contextId => $_getN(0);
  @$pb.TagNumber(1)
  set contextId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasContextId() => $_has(0);
  @$pb.TagNumber(1)
  void clearContextId() => clearField(1);

  @$pb.TagNumber(3)
  $fixnum.Int64 get limit => $_getI64(1);
  @$pb.TagNumber(3)
  set limit($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(3)
  $core.bool hasLimit() => $_has(1);
  @$pb.TagNumber(3)
  void clearLimit() => clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get minTxId => $_getI64(2);
  @$pb.TagNumber(4)
  set minTxId($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(4)
  $core.bool hasMinTxId() => $_has(2);
  @$pb.TagNumber(4)
  void clearMinTxId() => clearField(4);

  @$pb.TagNumber(5)
  $fixnum.Int64 get maxTxId => $_getI64(3);
  @$pb.TagNumber(5)
  set maxTxId($fixnum.Int64 v) { $_setInt64(3, v); }
  @$pb.TagNumber(5)
  $core.bool hasMaxTxId() => $_has(3);
  @$pb.TagNumber(5)
  void clearMaxTxId() => clearField(5);
}

class GroupTransactionsRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupTransactionsRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'accountId', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'limitGroups', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'minTxId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..a<$fixnum.Int64>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'maxTxId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  GroupTransactionsRequest._() : super();
  factory GroupTransactionsRequest({
    $core.List<$core.int>? accountId,
    $fixnum.Int64? limitGroups,
    $fixnum.Int64? minTxId,
    $fixnum.Int64? maxTxId,
  }) {
    final _result = create();
    if (accountId != null) {
      _result.accountId = accountId;
    }
    if (limitGroups != null) {
      _result.limitGroups = limitGroups;
    }
    if (minTxId != null) {
      _result.minTxId = minTxId;
    }
    if (maxTxId != null) {
      _result.maxTxId = maxTxId;
    }
    return _result;
  }
  factory GroupTransactionsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupTransactionsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupTransactionsRequest clone() => GroupTransactionsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupTransactionsRequest copyWith(void Function(GroupTransactionsRequest) updates) => super.copyWith((message) => updates(message as GroupTransactionsRequest)) as GroupTransactionsRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupTransactionsRequest create() => GroupTransactionsRequest._();
  GroupTransactionsRequest createEmptyInstance() => create();
  static $pb.PbList<GroupTransactionsRequest> createRepeated() => $pb.PbList<GroupTransactionsRequest>();
  @$core.pragma('dart2js:noInline')
  static GroupTransactionsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupTransactionsRequest>(create);
  static GroupTransactionsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get accountId => $_getN(0);
  @$pb.TagNumber(1)
  set accountId($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasAccountId() => $_has(0);
  @$pb.TagNumber(1)
  void clearAccountId() => clearField(1);

  @$pb.TagNumber(2)
  $fixnum.Int64 get limitGroups => $_getI64(1);
  @$pb.TagNumber(2)
  set limitGroups($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasLimitGroups() => $_has(1);
  @$pb.TagNumber(2)
  void clearLimitGroups() => clearField(2);

  @$pb.TagNumber(3)
  $fixnum.Int64 get minTxId => $_getI64(2);
  @$pb.TagNumber(3)
  set minTxId($fixnum.Int64 v) { $_setInt64(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasMinTxId() => $_has(2);
  @$pb.TagNumber(3)
  void clearMinTxId() => clearField(3);

  @$pb.TagNumber(4)
  $fixnum.Int64 get maxTxId => $_getI64(3);
  @$pb.TagNumber(4)
  set maxTxId($fixnum.Int64 v) { $_setInt64(3, v); }
  @$pb.TagNumber(4)
  $core.bool hasMaxTxId() => $_has(3);
  @$pb.TagNumber(4)
  void clearMaxTxId() => clearField(4);
}

class ObserveAccountsRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ObserveAccountsRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..aOM<TxId>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'startingFrom', subBuilder: TxId.create)
    ..p<$core.List<$core.int>>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'involvedAccounts', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  ObserveAccountsRequest._() : super();
  factory ObserveAccountsRequest({
    TxId? startingFrom,
    $core.Iterable<$core.List<$core.int>>? involvedAccounts,
  }) {
    final _result = create();
    if (startingFrom != null) {
      _result.startingFrom = startingFrom;
    }
    if (involvedAccounts != null) {
      _result.involvedAccounts.addAll(involvedAccounts);
    }
    return _result;
  }
  factory ObserveAccountsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ObserveAccountsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ObserveAccountsRequest clone() => ObserveAccountsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ObserveAccountsRequest copyWith(void Function(ObserveAccountsRequest) updates) => super.copyWith((message) => updates(message as ObserveAccountsRequest)) as ObserveAccountsRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ObserveAccountsRequest create() => ObserveAccountsRequest._();
  ObserveAccountsRequest createEmptyInstance() => create();
  static $pb.PbList<ObserveAccountsRequest> createRepeated() => $pb.PbList<ObserveAccountsRequest>();
  @$core.pragma('dart2js:noInline')
  static ObserveAccountsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ObserveAccountsRequest>(create);
  static ObserveAccountsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  TxId get startingFrom => $_getN(0);
  @$pb.TagNumber(1)
  set startingFrom(TxId v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasStartingFrom() => $_has(0);
  @$pb.TagNumber(1)
  void clearStartingFrom() => clearField(1);
  @$pb.TagNumber(1)
  TxId ensureStartingFrom() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.List<$core.List<$core.int>> get involvedAccounts => $_getList(1);
}

class ObserveResourcesRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ObserveResourcesRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..aOM<$7.Exp>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'expression', subBuilder: $7.Exp.create)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'collection')
    ..aOM<TxId>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'startingFrom', subBuilder: TxId.create)
    ..hasRequiredFields = false
  ;

  ObserveResourcesRequest._() : super();
  factory ObserveResourcesRequest({
    $7.Exp? expression,
    $core.String? collection,
    TxId? startingFrom,
  }) {
    final _result = create();
    if (expression != null) {
      _result.expression = expression;
    }
    if (collection != null) {
      _result.collection = collection;
    }
    if (startingFrom != null) {
      _result.startingFrom = startingFrom;
    }
    return _result;
  }
  factory ObserveResourcesRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ObserveResourcesRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ObserveResourcesRequest clone() => ObserveResourcesRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ObserveResourcesRequest copyWith(void Function(ObserveResourcesRequest) updates) => super.copyWith((message) => updates(message as ObserveResourcesRequest)) as ObserveResourcesRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ObserveResourcesRequest create() => ObserveResourcesRequest._();
  ObserveResourcesRequest createEmptyInstance() => create();
  static $pb.PbList<ObserveResourcesRequest> createRepeated() => $pb.PbList<ObserveResourcesRequest>();
  @$core.pragma('dart2js:noInline')
  static ObserveResourcesRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ObserveResourcesRequest>(create);
  static ObserveResourcesRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $7.Exp get expression => $_getN(0);
  @$pb.TagNumber(1)
  set expression($7.Exp v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasExpression() => $_has(0);
  @$pb.TagNumber(1)
  void clearExpression() => clearField(1);
  @$pb.TagNumber(1)
  $7.Exp ensureExpression() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.String get collection => $_getSZ(1);
  @$pb.TagNumber(2)
  set collection($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasCollection() => $_has(1);
  @$pb.TagNumber(2)
  void clearCollection() => clearField(2);

  @$pb.TagNumber(3)
  TxId get startingFrom => $_getN(2);
  @$pb.TagNumber(3)
  set startingFrom(TxId v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasStartingFrom() => $_has(2);
  @$pb.TagNumber(3)
  void clearStartingFrom() => clearField(3);
  @$pb.TagNumber(3)
  TxId ensureStartingFrom() => $_ensure(2);
}

class TxId extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'TxId', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'txId', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  TxId._() : super();
  factory TxId({
    $fixnum.Int64? txId,
  }) {
    final _result = create();
    if (txId != null) {
      _result.txId = txId;
    }
    return _result;
  }
  factory TxId.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory TxId.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  TxId clone() => TxId()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  TxId copyWith(void Function(TxId) updates) => super.copyWith((message) => updates(message as TxId)) as TxId; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static TxId create() => TxId._();
  TxId createEmptyInstance() => create();
  static $pb.PbList<TxId> createRepeated() => $pb.PbList<TxId>();
  @$core.pragma('dart2js:noInline')
  static TxId getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<TxId>(create);
  static TxId? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get txId => $_getI64(0);
  @$pb.TagNumber(1)
  set txId($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasTxId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTxId() => clearField(1);
}

class ObserveActionsRequest extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ObserveActionsRequest', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..aOM<TxId>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'startingFrom', subBuilder: TxId.create)
    ..aOS(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'name')
    ..p<$core.List<$core.int>>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'involvesAccounts', $pb.PbFieldType.PY)
    ..hasRequiredFields = false
  ;

  ObserveActionsRequest._() : super();
  factory ObserveActionsRequest({
    TxId? startingFrom,
    $core.String? name,
    $core.Iterable<$core.List<$core.int>>? involvesAccounts,
  }) {
    final _result = create();
    if (startingFrom != null) {
      _result.startingFrom = startingFrom;
    }
    if (name != null) {
      _result.name = name;
    }
    if (involvesAccounts != null) {
      _result.involvesAccounts.addAll(involvesAccounts);
    }
    return _result;
  }
  factory ObserveActionsRequest.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ObserveActionsRequest.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ObserveActionsRequest clone() => ObserveActionsRequest()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ObserveActionsRequest copyWith(void Function(ObserveActionsRequest) updates) => super.copyWith((message) => updates(message as ObserveActionsRequest)) as ObserveActionsRequest; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ObserveActionsRequest create() => ObserveActionsRequest._();
  ObserveActionsRequest createEmptyInstance() => create();
  static $pb.PbList<ObserveActionsRequest> createRepeated() => $pb.PbList<ObserveActionsRequest>();
  @$core.pragma('dart2js:noInline')
  static ObserveActionsRequest getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ObserveActionsRequest>(create);
  static ObserveActionsRequest? _defaultInstance;

  @$pb.TagNumber(1)
  TxId get startingFrom => $_getN(0);
  @$pb.TagNumber(1)
  set startingFrom(TxId v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasStartingFrom() => $_has(0);
  @$pb.TagNumber(1)
  void clearStartingFrom() => clearField(1);
  @$pb.TagNumber(1)
  TxId ensureStartingFrom() => $_ensure(0);

  @$pb.TagNumber(2)
  $core.String get name => $_getSZ(1);
  @$pb.TagNumber(2)
  set name($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasName() => $_has(1);
  @$pb.TagNumber(2)
  void clearName() => clearField(2);

  @$pb.TagNumber(3)
  $core.List<$core.List<$core.int>> get involvesAccounts => $_getList(2);
}

class FinalizedTransaction extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FinalizedTransaction', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..aOM<$1.TransactionRequestPayload>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'request', subBuilder: $1.TransactionRequestPayload.create)
    ..aOM<$1.TransactionResponse>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'response', subBuilder: $1.TransactionResponse.create)
  ;

  FinalizedTransaction._() : super();
  factory FinalizedTransaction({
    $1.TransactionRequestPayload? request,
    $1.TransactionResponse? response,
  }) {
    final _result = create();
    if (request != null) {
      _result.request = request;
    }
    if (response != null) {
      _result.response = response;
    }
    return _result;
  }
  factory FinalizedTransaction.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FinalizedTransaction.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FinalizedTransaction clone() => FinalizedTransaction()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FinalizedTransaction copyWith(void Function(FinalizedTransaction) updates) => super.copyWith((message) => updates(message as FinalizedTransaction)) as FinalizedTransaction; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FinalizedTransaction create() => FinalizedTransaction._();
  FinalizedTransaction createEmptyInstance() => create();
  static $pb.PbList<FinalizedTransaction> createRepeated() => $pb.PbList<FinalizedTransaction>();
  @$core.pragma('dart2js:noInline')
  static FinalizedTransaction getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FinalizedTransaction>(create);
  static FinalizedTransaction? _defaultInstance;

  @$pb.TagNumber(1)
  $1.TransactionRequestPayload get request => $_getN(0);
  @$pb.TagNumber(1)
  set request($1.TransactionRequestPayload v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearRequest() => clearField(1);
  @$pb.TagNumber(1)
  $1.TransactionRequestPayload ensureRequest() => $_ensure(0);

  @$pb.TagNumber(2)
  $1.TransactionResponse get response => $_getN(1);
  @$pb.TagNumber(2)
  set response($1.TransactionResponse v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasResponse() => $_has(1);
  @$pb.TagNumber(2)
  void clearResponse() => clearField(2);
  @$pb.TagNumber(2)
  $1.TransactionResponse ensureResponse() => $_ensure(1);
}

class FinalizedTransactions extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'FinalizedTransactions', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..pc<FinalizedTransaction>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'transactions', $pb.PbFieldType.PM, subBuilder: FinalizedTransaction.create)
  ;

  FinalizedTransactions._() : super();
  factory FinalizedTransactions({
    $core.Iterable<FinalizedTransaction>? transactions,
  }) {
    final _result = create();
    if (transactions != null) {
      _result.transactions.addAll(transactions);
    }
    return _result;
  }
  factory FinalizedTransactions.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory FinalizedTransactions.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  FinalizedTransactions clone() => FinalizedTransactions()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  FinalizedTransactions copyWith(void Function(FinalizedTransactions) updates) => super.copyWith((message) => updates(message as FinalizedTransactions)) as FinalizedTransactions; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static FinalizedTransactions create() => FinalizedTransactions._();
  FinalizedTransactions createEmptyInstance() => create();
  static $pb.PbList<FinalizedTransactions> createRepeated() => $pb.PbList<FinalizedTransactions>();
  @$core.pragma('dart2js:noInline')
  static FinalizedTransactions getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<FinalizedTransactions>(create);
  static FinalizedTransactions? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<FinalizedTransaction> get transactions => $_getList(0);
}

class GroupedFinalizedTransactions extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'GroupedFinalizedTransactions', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..pc<FinalizedTransactions>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'groups', $pb.PbFieldType.PM, subBuilder: FinalizedTransactions.create)
  ;

  GroupedFinalizedTransactions._() : super();
  factory GroupedFinalizedTransactions({
    $core.Iterable<FinalizedTransactions>? groups,
  }) {
    final _result = create();
    if (groups != null) {
      _result.groups.addAll(groups);
    }
    return _result;
  }
  factory GroupedFinalizedTransactions.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory GroupedFinalizedTransactions.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  GroupedFinalizedTransactions clone() => GroupedFinalizedTransactions()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  GroupedFinalizedTransactions copyWith(void Function(GroupedFinalizedTransactions) updates) => super.copyWith((message) => updates(message as GroupedFinalizedTransactions)) as GroupedFinalizedTransactions; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static GroupedFinalizedTransactions create() => GroupedFinalizedTransactions._();
  GroupedFinalizedTransactions createEmptyInstance() => create();
  static $pb.PbList<GroupedFinalizedTransactions> createRepeated() => $pb.PbList<GroupedFinalizedTransactions>();
  @$core.pragma('dart2js:noInline')
  static GroupedFinalizedTransactions getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<GroupedFinalizedTransactions>(create);
  static GroupedFinalizedTransactions? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<FinalizedTransactions> get groups => $_getList(0);
}

class ChainInfo extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'ChainInfo', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'm10.sdk'), createEmptyInstance: create)
    ..a<$fixnum.Int64>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'blockHeight', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  ChainInfo._() : super();
  factory ChainInfo({
    $fixnum.Int64? blockHeight,
  }) {
    final _result = create();
    if (blockHeight != null) {
      _result.blockHeight = blockHeight;
    }
    return _result;
  }
  factory ChainInfo.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChainInfo.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChainInfo clone() => ChainInfo()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChainInfo copyWith(void Function(ChainInfo) updates) => super.copyWith((message) => updates(message as ChainInfo)) as ChainInfo; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static ChainInfo create() => ChainInfo._();
  ChainInfo createEmptyInstance() => create();
  static $pb.PbList<ChainInfo> createRepeated() => $pb.PbList<ChainInfo>();
  @$core.pragma('dart2js:noInline')
  static ChainInfo getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChainInfo>(create);
  static ChainInfo? _defaultInstance;

  @$pb.TagNumber(1)
  $fixnum.Int64 get blockHeight => $_getI64(0);
  @$pb.TagNumber(1)
  set blockHeight($fixnum.Int64 v) { $_setInt64(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasBlockHeight() => $_has(0);
  @$pb.TagNumber(1)
  void clearBlockHeight() => clearField(1);
}

