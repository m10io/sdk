///
//  Generated code. Do not modify.
//  source: directory/api.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:async' as $async;

import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'api.pb.dart' as $0;
import '../google/protobuf/empty.pb.dart' as $1;
export 'api.pb.dart';

class DirectoryServiceClient extends $grpc.Client {
  static final _$createLedger = $grpc.ClientMethod<$0.Ledger, $1.Empty>(
      '/m10.directory.DirectoryService/CreateLedger',
      ($0.Ledger value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $1.Empty.fromBuffer(value));
  static final _$listLedgers =
      $grpc.ClientMethod<$1.Empty, $0.ListLedgersResponse>(
          '/m10.directory.DirectoryService/ListLedgers',
          ($1.Empty value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.ListLedgersResponse.fromBuffer(value));
  static final _$checkAlias =
      $grpc.ClientMethod<$0.CheckAliasRequest, $1.Empty>(
          '/m10.directory.DirectoryService/CheckAlias',
          ($0.CheckAliasRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) => $1.Empty.fromBuffer(value));
  static final _$createAlias = $grpc.ClientMethod<$0.Alias, $1.Empty>(
      '/m10.directory.DirectoryService/CreateAlias',
      ($0.Alias value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $1.Empty.fromBuffer(value));
  static final _$searchAliases =
      $grpc.ClientMethod<$0.SearchAliasesRequest, $0.SearchAliasesResponse>(
          '/m10.directory.DirectoryService/SearchAliases',
          ($0.SearchAliasesRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.SearchAliasesResponse.fromBuffer(value));
  static final _$createObjectUrl =
      $grpc.ClientMethod<$1.Empty, $0.ObjectUrlResponse>(
          '/m10.directory.DirectoryService/CreateObjectUrl',
          ($1.Empty value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.ObjectUrlResponse.fromBuffer(value));
  static final _$getObjectUrl =
      $grpc.ClientMethod<$0.GetObjectUrlRequest, $0.ObjectUrlResponse>(
          '/m10.directory.DirectoryService/GetObjectUrl',
          ($0.GetObjectUrlRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.ObjectUrlResponse.fromBuffer(value));
  static final _$createImageUrl =
      $grpc.ClientMethod<$0.CreateImageUrlRequest, $0.ObjectUrlResponse>(
          '/m10.directory.DirectoryService/CreateImageUrl',
          ($0.CreateImageUrlRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.ObjectUrlResponse.fromBuffer(value));

  DirectoryServiceClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options, interceptors: interceptors);

  $grpc.ResponseFuture<$1.Empty> createLedger($0.Ledger request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$createLedger, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListLedgersResponse> listLedgers($1.Empty request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$listLedgers, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> checkAlias($0.CheckAliasRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$checkAlias, request, options: options);
  }

  $grpc.ResponseFuture<$1.Empty> createAlias($0.Alias request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$createAlias, request, options: options);
  }

  $grpc.ResponseFuture<$0.SearchAliasesResponse> searchAliases(
      $0.SearchAliasesRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$searchAliases, request, options: options);
  }

  $grpc.ResponseFuture<$0.ObjectUrlResponse> createObjectUrl($1.Empty request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$createObjectUrl, request, options: options);
  }

  $grpc.ResponseFuture<$0.ObjectUrlResponse> getObjectUrl(
      $0.GetObjectUrlRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getObjectUrl, request, options: options);
  }

  $grpc.ResponseFuture<$0.ObjectUrlResponse> createImageUrl(
      $0.CreateImageUrlRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$createImageUrl, request, options: options);
  }
}

abstract class DirectoryServiceBase extends $grpc.Service {
  $core.String get $name => 'm10.directory.DirectoryService';

  DirectoryServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.Ledger, $1.Empty>(
        'CreateLedger',
        createLedger_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.Ledger.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.Empty, $0.ListLedgersResponse>(
        'ListLedgers',
        listLedgers_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $1.Empty.fromBuffer(value),
        ($0.ListLedgersResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.CheckAliasRequest, $1.Empty>(
        'CheckAlias',
        checkAlias_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.CheckAliasRequest.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.Alias, $1.Empty>(
        'CreateAlias',
        createAlias_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.Alias.fromBuffer(value),
        ($1.Empty value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.SearchAliasesRequest, $0.SearchAliasesResponse>(
            'SearchAliases',
            searchAliases_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.SearchAliasesRequest.fromBuffer(value),
            ($0.SearchAliasesResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.Empty, $0.ObjectUrlResponse>(
        'CreateObjectUrl',
        createObjectUrl_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $1.Empty.fromBuffer(value),
        ($0.ObjectUrlResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.GetObjectUrlRequest, $0.ObjectUrlResponse>(
            'GetObjectUrl',
            getObjectUrl_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.GetObjectUrlRequest.fromBuffer(value),
            ($0.ObjectUrlResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.CreateImageUrlRequest, $0.ObjectUrlResponse>(
            'CreateImageUrl',
            createImageUrl_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.CreateImageUrlRequest.fromBuffer(value),
            ($0.ObjectUrlResponse value) => value.writeToBuffer()));
  }

  $async.Future<$1.Empty> createLedger_Pre(
      $grpc.ServiceCall call, $async.Future<$0.Ledger> request) async {
    return createLedger(call, await request);
  }

  $async.Future<$0.ListLedgersResponse> listLedgers_Pre(
      $grpc.ServiceCall call, $async.Future<$1.Empty> request) async {
    return listLedgers(call, await request);
  }

  $async.Future<$1.Empty> checkAlias_Pre($grpc.ServiceCall call,
      $async.Future<$0.CheckAliasRequest> request) async {
    return checkAlias(call, await request);
  }

  $async.Future<$1.Empty> createAlias_Pre(
      $grpc.ServiceCall call, $async.Future<$0.Alias> request) async {
    return createAlias(call, await request);
  }

  $async.Future<$0.SearchAliasesResponse> searchAliases_Pre(
      $grpc.ServiceCall call,
      $async.Future<$0.SearchAliasesRequest> request) async {
    return searchAliases(call, await request);
  }

  $async.Future<$0.ObjectUrlResponse> createObjectUrl_Pre(
      $grpc.ServiceCall call, $async.Future<$1.Empty> request) async {
    return createObjectUrl(call, await request);
  }

  $async.Future<$0.ObjectUrlResponse> getObjectUrl_Pre($grpc.ServiceCall call,
      $async.Future<$0.GetObjectUrlRequest> request) async {
    return getObjectUrl(call, await request);
  }

  $async.Future<$0.ObjectUrlResponse> createImageUrl_Pre($grpc.ServiceCall call,
      $async.Future<$0.CreateImageUrlRequest> request) async {
    return createImageUrl(call, await request);
  }

  $async.Future<$1.Empty> createLedger(
      $grpc.ServiceCall call, $0.Ledger request);
  $async.Future<$0.ListLedgersResponse> listLedgers(
      $grpc.ServiceCall call, $1.Empty request);
  $async.Future<$1.Empty> checkAlias(
      $grpc.ServiceCall call, $0.CheckAliasRequest request);
  $async.Future<$1.Empty> createAlias($grpc.ServiceCall call, $0.Alias request);
  $async.Future<$0.SearchAliasesResponse> searchAliases(
      $grpc.ServiceCall call, $0.SearchAliasesRequest request);
  $async.Future<$0.ObjectUrlResponse> createObjectUrl(
      $grpc.ServiceCall call, $1.Empty request);
  $async.Future<$0.ObjectUrlResponse> getObjectUrl(
      $grpc.ServiceCall call, $0.GetObjectUrlRequest request);
  $async.Future<$0.ObjectUrlResponse> createImageUrl(
      $grpc.ServiceCall call, $0.CreateImageUrlRequest request);
}
