///
//  Generated code. Do not modify.
//  source: sdk/api.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:async' as $async;

import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'api.pb.dart' as $0;
import 'transaction/transaction.pb.dart' as $1;
import 'model/model.pb.dart' as $2;
import '../google/protobuf/empty.pb.dart' as $3;
import 'rbac.pb.dart' as $4;
export 'api.pb.dart';

class M10TxServiceClient extends $grpc.Client {
  static final _$createTransaction =
      $grpc.ClientMethod<$0.RequestEnvelope, $1.TransactionResponse>(
          '/m10.sdk.M10TxService/CreateTransaction',
          ($0.RequestEnvelope value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $1.TransactionResponse.fromBuffer(value));

  M10TxServiceClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options, interceptors: interceptors);

  $grpc.ResponseFuture<$1.TransactionResponse> createTransaction(
      $0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$createTransaction, request, options: options);
  }
}

abstract class M10TxServiceBase extends $grpc.Service {
  $core.String get $name => 'm10.sdk.M10TxService';

  M10TxServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.RequestEnvelope, $1.TransactionResponse>(
        'CreateTransaction',
        createTransaction_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.RequestEnvelope.fromBuffer(value),
        ($1.TransactionResponse value) => value.writeToBuffer()));
  }

  $async.Future<$1.TransactionResponse> createTransaction_Pre(
      $grpc.ServiceCall call, $async.Future<$0.RequestEnvelope> request) async {
    return createTransaction(call, await request);
  }

  $async.Future<$1.TransactionResponse> createTransaction(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
}

class M10QueryServiceClient extends $grpc.Client {
  static final _$getTransfer =
      $grpc.ClientMethod<$0.RequestEnvelope, $1.FinalizedTransfer>(
          '/m10.sdk.M10QueryService/GetTransfer',
          ($0.RequestEnvelope value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $1.FinalizedTransfer.fromBuffer(value));
  static final _$listTransfers =
      $grpc.ClientMethod<$0.RequestEnvelope, $1.FinalizedTransfers>(
          '/m10.sdk.M10QueryService/ListTransfers',
          ($0.RequestEnvelope value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $1.FinalizedTransfers.fromBuffer(value));
  static final _$observeTransfers =
      $grpc.ClientMethod<$0.RequestEnvelope, $0.FinalizedTransactions>(
          '/m10.sdk.M10QueryService/ObserveTransfers',
          ($0.RequestEnvelope value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.FinalizedTransactions.fromBuffer(value));
  static final _$getIndexedAccount =
      $grpc.ClientMethod<$0.RequestEnvelope, $1.IndexedAccount>(
          '/m10.sdk.M10QueryService/GetIndexedAccount',
          ($0.RequestEnvelope value) => value.writeToBuffer(),
          ($core.List<$core.int> value) => $1.IndexedAccount.fromBuffer(value));
  static final _$getAccount =
      $grpc.ClientMethod<$0.RequestEnvelope, $2.Account>(
          '/m10.sdk.M10QueryService/GetAccount',
          ($0.RequestEnvelope value) => value.writeToBuffer(),
          ($core.List<$core.int> value) => $2.Account.fromBuffer(value));
  static final _$getAccountInfo =
      $grpc.ClientMethod<$0.RequestEnvelope, $2.AccountInfo>(
          '/m10.sdk.M10QueryService/GetAccountInfo',
          ($0.RequestEnvelope value) => value.writeToBuffer(),
          ($core.List<$core.int> value) => $2.AccountInfo.fromBuffer(value));
  static final _$listAccounts =
      $grpc.ClientMethod<$0.RequestEnvelope, $0.ListAccountsResponse>(
          '/m10.sdk.M10QueryService/ListAccounts',
          ($0.RequestEnvelope value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.ListAccountsResponse.fromBuffer(value));
  static final _$observeAccounts =
      $grpc.ClientMethod<$0.RequestEnvelope, $0.FinalizedTransactions>(
          '/m10.sdk.M10QueryService/ObserveAccounts',
          ($0.RequestEnvelope value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.FinalizedTransactions.fromBuffer(value));
  static final _$getAction = $grpc.ClientMethod<$0.RequestEnvelope, $1.Action>(
      '/m10.sdk.M10QueryService/GetAction',
      ($0.RequestEnvelope value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $1.Action.fromBuffer(value));
  static final _$listActions =
      $grpc.ClientMethod<$0.RequestEnvelope, $1.Actions>(
          '/m10.sdk.M10QueryService/ListActions',
          ($0.RequestEnvelope value) => value.writeToBuffer(),
          ($core.List<$core.int> value) => $1.Actions.fromBuffer(value));
  static final _$observeActions =
      $grpc.ClientMethod<$0.RequestEnvelope, $0.FinalizedTransactions>(
          '/m10.sdk.M10QueryService/ObserveActions',
          ($0.RequestEnvelope value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.FinalizedTransactions.fromBuffer(value));
  static final _$getChainInfo = $grpc.ClientMethod<$3.Empty, $0.ChainInfo>(
      '/m10.sdk.M10QueryService/GetChainInfo',
      ($3.Empty value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.ChainInfo.fromBuffer(value));
  static final _$getTransaction =
      $grpc.ClientMethod<$0.RequestEnvelope, $0.FinalizedTransaction>(
          '/m10.sdk.M10QueryService/GetTransaction',
          ($0.RequestEnvelope value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.FinalizedTransaction.fromBuffer(value));
  static final _$listTransactions =
      $grpc.ClientMethod<$0.RequestEnvelope, $0.FinalizedTransactions>(
          '/m10.sdk.M10QueryService/ListTransactions',
          ($0.RequestEnvelope value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.FinalizedTransactions.fromBuffer(value));
  static final _$groupTransactions =
      $grpc.ClientMethod<$0.RequestEnvelope, $0.GroupedFinalizedTransactions>(
          '/m10.sdk.M10QueryService/GroupTransactions',
          ($0.RequestEnvelope value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.GroupedFinalizedTransactions.fromBuffer(value));
  static final _$getAccountSet =
      $grpc.ClientMethod<$0.RequestEnvelope, $2.AccountSet>(
          '/m10.sdk.M10QueryService/GetAccountSet',
          ($0.RequestEnvelope value) => value.writeToBuffer(),
          ($core.List<$core.int> value) => $2.AccountSet.fromBuffer(value));
  static final _$listAccountSets =
      $grpc.ClientMethod<$0.RequestEnvelope, $0.ListAccountSetsResponse>(
          '/m10.sdk.M10QueryService/ListAccountSets',
          ($0.RequestEnvelope value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.ListAccountSetsResponse.fromBuffer(value));
  static final _$getRoleBinding =
      $grpc.ClientMethod<$0.RequestEnvelope, $4.RoleBinding>(
          '/m10.sdk.M10QueryService/GetRoleBinding',
          ($0.RequestEnvelope value) => value.writeToBuffer(),
          ($core.List<$core.int> value) => $4.RoleBinding.fromBuffer(value));
  static final _$listRoleBindings =
      $grpc.ClientMethod<$0.RequestEnvelope, $0.ListRoleBindingsResponse>(
          '/m10.sdk.M10QueryService/ListRoleBindings',
          ($0.RequestEnvelope value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.ListRoleBindingsResponse.fromBuffer(value));
  static final _$getRole = $grpc.ClientMethod<$0.RequestEnvelope, $4.Role>(
      '/m10.sdk.M10QueryService/GetRole',
      ($0.RequestEnvelope value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $4.Role.fromBuffer(value));
  static final _$listRoles =
      $grpc.ClientMethod<$0.RequestEnvelope, $0.ListRolesResponse>(
          '/m10.sdk.M10QueryService/ListRoles',
          ($0.RequestEnvelope value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.ListRolesResponse.fromBuffer(value));
  static final _$observeResources =
      $grpc.ClientMethod<$0.RequestEnvelope, $0.FinalizedTransactions>(
          '/m10.sdk.M10QueryService/ObserveResources',
          ($0.RequestEnvelope value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.FinalizedTransactions.fromBuffer(value));
  static final _$observeMetrics =
      $grpc.ClientMethod<$0.RequestEnvelope, $0.TransactionMetrics>(
          '/m10.sdk.M10QueryService/ObserveMetrics',
          ($0.RequestEnvelope value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $0.TransactionMetrics.fromBuffer(value));

  M10QueryServiceClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options, interceptors: interceptors);

  $grpc.ResponseFuture<$1.FinalizedTransfer> getTransfer(
      $0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getTransfer, request, options: options);
  }

  $grpc.ResponseFuture<$1.FinalizedTransfers> listTransfers(
      $0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$listTransfers, request, options: options);
  }

  $grpc.ResponseStream<$0.FinalizedTransactions> observeTransfers(
      $0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createStreamingCall(
        _$observeTransfers, $async.Stream.fromIterable([request]),
        options: options);
  }

  $grpc.ResponseFuture<$1.IndexedAccount> getIndexedAccount(
      $0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getIndexedAccount, request, options: options);
  }

  $grpc.ResponseFuture<$2.Account> getAccount($0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getAccount, request, options: options);
  }

  $grpc.ResponseFuture<$2.AccountInfo> getAccountInfo(
      $0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getAccountInfo, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListAccountsResponse> listAccounts(
      $0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$listAccounts, request, options: options);
  }

  $grpc.ResponseStream<$0.FinalizedTransactions> observeAccounts(
      $0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createStreamingCall(
        _$observeAccounts, $async.Stream.fromIterable([request]),
        options: options);
  }

  $grpc.ResponseFuture<$1.Action> getAction($0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getAction, request, options: options);
  }

  $grpc.ResponseFuture<$1.Actions> listActions($0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$listActions, request, options: options);
  }

  $grpc.ResponseStream<$0.FinalizedTransactions> observeActions(
      $0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createStreamingCall(
        _$observeActions, $async.Stream.fromIterable([request]),
        options: options);
  }

  $grpc.ResponseFuture<$0.ChainInfo> getChainInfo($3.Empty request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getChainInfo, request, options: options);
  }

  $grpc.ResponseFuture<$0.FinalizedTransaction> getTransaction(
      $0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getTransaction, request, options: options);
  }

  $grpc.ResponseFuture<$0.FinalizedTransactions> listTransactions(
      $0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$listTransactions, request, options: options);
  }

  $grpc.ResponseFuture<$0.GroupedFinalizedTransactions> groupTransactions(
      $0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$groupTransactions, request, options: options);
  }

  $grpc.ResponseFuture<$2.AccountSet> getAccountSet($0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getAccountSet, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListAccountSetsResponse> listAccountSets(
      $0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$listAccountSets, request, options: options);
  }

  $grpc.ResponseFuture<$4.RoleBinding> getRoleBinding(
      $0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getRoleBinding, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListRoleBindingsResponse> listRoleBindings(
      $0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$listRoleBindings, request, options: options);
  }

  $grpc.ResponseFuture<$4.Role> getRole($0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$getRole, request, options: options);
  }

  $grpc.ResponseFuture<$0.ListRolesResponse> listRoles(
      $0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$listRoles, request, options: options);
  }

  $grpc.ResponseStream<$0.FinalizedTransactions> observeResources(
      $0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createStreamingCall(
        _$observeResources, $async.Stream.fromIterable([request]),
        options: options);
  }

  $grpc.ResponseStream<$0.TransactionMetrics> observeMetrics(
      $0.RequestEnvelope request,
      {$grpc.CallOptions? options}) {
    return $createStreamingCall(
        _$observeMetrics, $async.Stream.fromIterable([request]),
        options: options);
  }
}

abstract class M10QueryServiceBase extends $grpc.Service {
  $core.String get $name => 'm10.sdk.M10QueryService';

  M10QueryServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.RequestEnvelope, $1.FinalizedTransfer>(
        'GetTransfer',
        getTransfer_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.RequestEnvelope.fromBuffer(value),
        ($1.FinalizedTransfer value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RequestEnvelope, $1.FinalizedTransfers>(
        'ListTransfers',
        listTransfers_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.RequestEnvelope.fromBuffer(value),
        ($1.FinalizedTransfers value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.RequestEnvelope, $0.FinalizedTransactions>(
            'ObserveTransfers',
            observeTransfers_Pre,
            false,
            true,
            ($core.List<$core.int> value) =>
                $0.RequestEnvelope.fromBuffer(value),
            ($0.FinalizedTransactions value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RequestEnvelope, $1.IndexedAccount>(
        'GetIndexedAccount',
        getIndexedAccount_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.RequestEnvelope.fromBuffer(value),
        ($1.IndexedAccount value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RequestEnvelope, $2.Account>(
        'GetAccount',
        getAccount_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.RequestEnvelope.fromBuffer(value),
        ($2.Account value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RequestEnvelope, $2.AccountInfo>(
        'GetAccountInfo',
        getAccountInfo_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.RequestEnvelope.fromBuffer(value),
        ($2.AccountInfo value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RequestEnvelope, $0.ListAccountsResponse>(
        'ListAccounts',
        listAccounts_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.RequestEnvelope.fromBuffer(value),
        ($0.ListAccountsResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.RequestEnvelope, $0.FinalizedTransactions>(
            'ObserveAccounts',
            observeAccounts_Pre,
            false,
            true,
            ($core.List<$core.int> value) =>
                $0.RequestEnvelope.fromBuffer(value),
            ($0.FinalizedTransactions value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RequestEnvelope, $1.Action>(
        'GetAction',
        getAction_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.RequestEnvelope.fromBuffer(value),
        ($1.Action value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RequestEnvelope, $1.Actions>(
        'ListActions',
        listActions_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.RequestEnvelope.fromBuffer(value),
        ($1.Actions value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.RequestEnvelope, $0.FinalizedTransactions>(
            'ObserveActions',
            observeActions_Pre,
            false,
            true,
            ($core.List<$core.int> value) =>
                $0.RequestEnvelope.fromBuffer(value),
            ($0.FinalizedTransactions value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$3.Empty, $0.ChainInfo>(
        'GetChainInfo',
        getChainInfo_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $3.Empty.fromBuffer(value),
        ($0.ChainInfo value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RequestEnvelope, $0.FinalizedTransaction>(
        'GetTransaction',
        getTransaction_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.RequestEnvelope.fromBuffer(value),
        ($0.FinalizedTransaction value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.RequestEnvelope, $0.FinalizedTransactions>(
            'ListTransactions',
            listTransactions_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.RequestEnvelope.fromBuffer(value),
            ($0.FinalizedTransactions value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RequestEnvelope,
            $0.GroupedFinalizedTransactions>(
        'GroupTransactions',
        groupTransactions_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.RequestEnvelope.fromBuffer(value),
        ($0.GroupedFinalizedTransactions value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RequestEnvelope, $2.AccountSet>(
        'GetAccountSet',
        getAccountSet_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.RequestEnvelope.fromBuffer(value),
        ($2.AccountSet value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.RequestEnvelope, $0.ListAccountSetsResponse>(
            'ListAccountSets',
            listAccountSets_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.RequestEnvelope.fromBuffer(value),
            ($0.ListAccountSetsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RequestEnvelope, $4.RoleBinding>(
        'GetRoleBinding',
        getRoleBinding_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.RequestEnvelope.fromBuffer(value),
        ($4.RoleBinding value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.RequestEnvelope, $0.ListRoleBindingsResponse>(
            'ListRoleBindings',
            listRoleBindings_Pre,
            false,
            false,
            ($core.List<$core.int> value) =>
                $0.RequestEnvelope.fromBuffer(value),
            ($0.ListRoleBindingsResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RequestEnvelope, $4.Role>(
        'GetRole',
        getRole_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.RequestEnvelope.fromBuffer(value),
        ($4.Role value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RequestEnvelope, $0.ListRolesResponse>(
        'ListRoles',
        listRoles_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.RequestEnvelope.fromBuffer(value),
        ($0.ListRolesResponse value) => value.writeToBuffer()));
    $addMethod(
        $grpc.ServiceMethod<$0.RequestEnvelope, $0.FinalizedTransactions>(
            'ObserveResources',
            observeResources_Pre,
            false,
            true,
            ($core.List<$core.int> value) =>
                $0.RequestEnvelope.fromBuffer(value),
            ($0.FinalizedTransactions value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.RequestEnvelope, $0.TransactionMetrics>(
        'ObserveMetrics',
        observeMetrics_Pre,
        false,
        true,
        ($core.List<$core.int> value) => $0.RequestEnvelope.fromBuffer(value),
        ($0.TransactionMetrics value) => value.writeToBuffer()));
  }

  $async.Future<$1.FinalizedTransfer> getTransfer_Pre(
      $grpc.ServiceCall call, $async.Future<$0.RequestEnvelope> request) async {
    return getTransfer(call, await request);
  }

  $async.Future<$1.FinalizedTransfers> listTransfers_Pre(
      $grpc.ServiceCall call, $async.Future<$0.RequestEnvelope> request) async {
    return listTransfers(call, await request);
  }

  $async.Stream<$0.FinalizedTransactions> observeTransfers_Pre(
      $grpc.ServiceCall call,
      $async.Future<$0.RequestEnvelope> request) async* {
    yield* observeTransfers(call, await request);
  }

  $async.Future<$1.IndexedAccount> getIndexedAccount_Pre(
      $grpc.ServiceCall call, $async.Future<$0.RequestEnvelope> request) async {
    return getIndexedAccount(call, await request);
  }

  $async.Future<$2.Account> getAccount_Pre(
      $grpc.ServiceCall call, $async.Future<$0.RequestEnvelope> request) async {
    return getAccount(call, await request);
  }

  $async.Future<$2.AccountInfo> getAccountInfo_Pre(
      $grpc.ServiceCall call, $async.Future<$0.RequestEnvelope> request) async {
    return getAccountInfo(call, await request);
  }

  $async.Future<$0.ListAccountsResponse> listAccounts_Pre(
      $grpc.ServiceCall call, $async.Future<$0.RequestEnvelope> request) async {
    return listAccounts(call, await request);
  }

  $async.Stream<$0.FinalizedTransactions> observeAccounts_Pre(
      $grpc.ServiceCall call,
      $async.Future<$0.RequestEnvelope> request) async* {
    yield* observeAccounts(call, await request);
  }

  $async.Future<$1.Action> getAction_Pre(
      $grpc.ServiceCall call, $async.Future<$0.RequestEnvelope> request) async {
    return getAction(call, await request);
  }

  $async.Future<$1.Actions> listActions_Pre(
      $grpc.ServiceCall call, $async.Future<$0.RequestEnvelope> request) async {
    return listActions(call, await request);
  }

  $async.Stream<$0.FinalizedTransactions> observeActions_Pre(
      $grpc.ServiceCall call,
      $async.Future<$0.RequestEnvelope> request) async* {
    yield* observeActions(call, await request);
  }

  $async.Future<$0.ChainInfo> getChainInfo_Pre(
      $grpc.ServiceCall call, $async.Future<$3.Empty> request) async {
    return getChainInfo(call, await request);
  }

  $async.Future<$0.FinalizedTransaction> getTransaction_Pre(
      $grpc.ServiceCall call, $async.Future<$0.RequestEnvelope> request) async {
    return getTransaction(call, await request);
  }

  $async.Future<$0.FinalizedTransactions> listTransactions_Pre(
      $grpc.ServiceCall call, $async.Future<$0.RequestEnvelope> request) async {
    return listTransactions(call, await request);
  }

  $async.Future<$0.GroupedFinalizedTransactions> groupTransactions_Pre(
      $grpc.ServiceCall call, $async.Future<$0.RequestEnvelope> request) async {
    return groupTransactions(call, await request);
  }

  $async.Future<$2.AccountSet> getAccountSet_Pre(
      $grpc.ServiceCall call, $async.Future<$0.RequestEnvelope> request) async {
    return getAccountSet(call, await request);
  }

  $async.Future<$0.ListAccountSetsResponse> listAccountSets_Pre(
      $grpc.ServiceCall call, $async.Future<$0.RequestEnvelope> request) async {
    return listAccountSets(call, await request);
  }

  $async.Future<$4.RoleBinding> getRoleBinding_Pre(
      $grpc.ServiceCall call, $async.Future<$0.RequestEnvelope> request) async {
    return getRoleBinding(call, await request);
  }

  $async.Future<$0.ListRoleBindingsResponse> listRoleBindings_Pre(
      $grpc.ServiceCall call, $async.Future<$0.RequestEnvelope> request) async {
    return listRoleBindings(call, await request);
  }

  $async.Future<$4.Role> getRole_Pre(
      $grpc.ServiceCall call, $async.Future<$0.RequestEnvelope> request) async {
    return getRole(call, await request);
  }

  $async.Future<$0.ListRolesResponse> listRoles_Pre(
      $grpc.ServiceCall call, $async.Future<$0.RequestEnvelope> request) async {
    return listRoles(call, await request);
  }

  $async.Stream<$0.FinalizedTransactions> observeResources_Pre(
      $grpc.ServiceCall call,
      $async.Future<$0.RequestEnvelope> request) async* {
    yield* observeResources(call, await request);
  }

  $async.Stream<$0.TransactionMetrics> observeMetrics_Pre(
      $grpc.ServiceCall call,
      $async.Future<$0.RequestEnvelope> request) async* {
    yield* observeMetrics(call, await request);
  }

  $async.Future<$1.FinalizedTransfer> getTransfer(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
  $async.Future<$1.FinalizedTransfers> listTransfers(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
  $async.Stream<$0.FinalizedTransactions> observeTransfers(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
  $async.Future<$1.IndexedAccount> getIndexedAccount(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
  $async.Future<$2.Account> getAccount(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
  $async.Future<$2.AccountInfo> getAccountInfo(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
  $async.Future<$0.ListAccountsResponse> listAccounts(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
  $async.Stream<$0.FinalizedTransactions> observeAccounts(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
  $async.Future<$1.Action> getAction(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
  $async.Future<$1.Actions> listActions(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
  $async.Stream<$0.FinalizedTransactions> observeActions(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
  $async.Future<$0.ChainInfo> getChainInfo(
      $grpc.ServiceCall call, $3.Empty request);
  $async.Future<$0.FinalizedTransaction> getTransaction(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
  $async.Future<$0.FinalizedTransactions> listTransactions(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
  $async.Future<$0.GroupedFinalizedTransactions> groupTransactions(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
  $async.Future<$2.AccountSet> getAccountSet(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
  $async.Future<$0.ListAccountSetsResponse> listAccountSets(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
  $async.Future<$4.RoleBinding> getRoleBinding(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
  $async.Future<$0.ListRoleBindingsResponse> listRoleBindings(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
  $async.Future<$4.Role> getRole(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
  $async.Future<$0.ListRolesResponse> listRoles(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
  $async.Stream<$0.FinalizedTransactions> observeResources(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
  $async.Stream<$0.TransactionMetrics> observeMetrics(
      $grpc.ServiceCall call, $0.RequestEnvelope request);
}
