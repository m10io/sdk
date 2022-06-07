import 'dart:convert';
import 'dart:math';

import 'package:convert/convert.dart';
import 'package:fixnum/fixnum.dart';
import 'package:grpc/grpc.dart';
import 'package:m10_sdk/metadata.dart';
import 'package:m10_sdk/src/generated/sdk/document.pb.dart';
import 'package:m10_sdk/src/generated/directory/api.pb.dart' as directory;
import 'package:protobuf/protobuf.dart';
import 'package:uuid/uuid.dart';
import 'package:m10_sdk/ledger_services/document_store/builder.dart';
import 'package:m10_sdk/ledger_services/request_extensions.dart';
import 'package:m10_sdk/ledger_services/service_client.dart';
import 'package:m10_sdk/ledger_services/contract/builder.dart';
import 'package:m10_sdk/security/request_signing.dart';
import 'package:m10_sdk/security/security.dart';
import 'package:m10_sdk/library.dart';
import 'package:collection/collection.dart';

class M10Sdk {
  final Signing signer;
  final ChannelCredentials _credentials;
  final Random _random;

  Map<String, AccountInfo> _accountInfoCache;

  List<directory.Ledger> _ledgers;
  set ledgers(List<directory.Ledger> newValue) {
    _ledgers = newValue;
  }

  M10Sdk({
    required this.signer,
    required List<directory.Ledger> ledgers,
    bool disableTls = false,
  })  : _random = Random.secure(),
        _credentials = disableTls
            ? const ChannelCredentials.insecure()
            : const ChannelCredentials.secure(),
        _ledgers = ledgers,
        _accountInfoCache = Map<String, AccountInfo>();

  M10Sdk copyWithSigner(Signing signer) {
    return M10Sdk(signer: signer, ledgers: _ledgers);
  }

  final Map<String, ServiceClient> serviceClients = {};

  ClientChannel getChannel(String operator) {
    final ledger = _ledgers.firstWhere(
      (it) => it.id.toLowerCase() == operator.toLowerCase(),
      orElse: () => throw ArgumentError(
        'No ledger matching ID $operator\n'
        'Ledgers: ${_ledgers.map((it) => it.asMap())}',
      ),
    );
    return ClientChannel(
      ledger.host,
      options: ChannelOptions(credentials: _credentials),
    );
  }

  ServiceClient getServiceClient(String operator) {
    return serviceClients.putIfAbsent(
      operator,
      () => ServiceClient(getChannel(operator)),
    );
  }

  Future<RequestEnvelope> requestEnvelope({
    required GeneratedMessage request,
    List<int>? contextId,
  }) async {
    if (request is TransactionRequestPayload) {
      request.nonce =
          Int64.fromInts(_random.nextInt(1 << 32), _random.nextInt(1 << 32));
      request.timestamp = Int64(DateTime.now().microsecondsSinceEpoch);
      request.contextId = contextId ?? [];
    }
    final payload = request.writeToBuffer();
    final signature = await signer.signPayload(payload);
    return RequestEnvelope(payload: payload, signature: signature);
  }

  Future<String> createUser({
    required String operator,
    String? contextId,
    String? id,
  }) async {
    final uuid = Uuid();
    final userId =
        id != null ? Uuid.parse(id) : uuid.v4buffer(List.filled(16, 0));

    AccountSet user = AccountSet();
    user.id = userId;
    user.owner = await signer.publicKey();

    final request = user.createRequest();
    final client = getServiceClient(operator);
    final envelop = await requestEnvelope(
        request: request,
        contextId: contextId != null ? hex.decode(contextId) : null);
    await client.tx.createTransaction(envelop);
    return Uuid.unparse(userId);
  }

  Future<AccountSetDoc> getUser({
    required String userId,
    required String operator,
  }) async {
    final request = GetAccountSetRequest(id: Uuid.parse(userId));
    final envelop = await requestEnvelope(request: request);
    final user = await getServiceClient(operator).query.getAccountSet(envelop);
    return AccountSetDoc(user);
  }

  Future<String> updateUser({
    required String userId,
    required String operator,
    String? defaultCurrency,
    String? contextId,
    List<AccountRef> accounts = const [],
  }) async {
    dynamic builder = DocumentUpdate(AccountSet());
    builder.id(Uuid.parse(userId));
    if (defaultCurrency != null) {
      builder.defaultCurrency(defaultCurrency);
    }
    if (accounts.isNotEmpty) {
      builder.accounts(accounts);
    }
    final request = AccountSetExt.updateRequest(builder);
    final client = getServiceClient(operator);
    final envelop = await requestEnvelope(
        request: request,
        contextId: contextId != null ? hex.decode(contextId) : null);
    await client.tx.createTransaction(envelop);
    return userId;
  }

  Future<String> deleteUser({
    required String userId,
    required String operator,
    String? contextId,
  }) async {
    final request = AccountSetExt.deleteRequest(Uuid.parse(userId));

    final client = getServiceClient(operator);
    final envelop = await requestEnvelope(
        request: request,
        contextId: contextId != null ? hex.decode(contextId) : null);
    await client.tx.createTransaction(envelop);
    return userId;
  }

  Future<String> createAccount({
    required String parentId,
    required String operator,
    String? name,
    String? publicName,
    String? owner,
    String? profileImageUrl,
    String? contextId,
  }) async {
    final ownerBytes = owner != null ? base64Decode(owner) : null;

    final accountId = await createLedgerAccount(
      parentId: parentId,
      operator: operator,
    );

    Account account = Account();
    account.id = hex.decode(accountId);
    account.owner = ownerBytes ?? await signer.publicKey();

    if (name != null) {
      account.name = name;
    }

    if (publicName != null) {
      account.publicName = publicName;
    }

    if (profileImageUrl != null) {
      account.profileImageUrl = profileImageUrl;
    }

    final request = account.createRequest();
    final client = getServiceClient(operator);
    final envelop = await requestEnvelope(
        request: request,
        contextId: contextId != null ? hex.decode(contextId) : null);
    await client.tx.createTransaction(envelop);

    return accountId;
  }

  Future<AccountDoc> getAccount({
    required String id,
    required String operator,
  }) async {
    final request = GetAccountRequest();
    request.id = hex.decode(id);
    final envelop = await requestEnvelope(request: request);
    final account = await getServiceClient(operator).query.getAccount(envelop);
    final indexedAccount =
        await _getLedgerAccount(accountId: id, operator: operator);

    return AccountDoc.fromModel(account, indexedAccount);
  }

  Future<AccountInfo> getAccountInfo({
    required String id,
    required String operator,
  }) async {
    final request = GetAccountRequest();
    request.id = hex.decode(id);
    final envelop = await requestEnvelope(request: request);
    return await getServiceClient(operator).query.getAccountInfo(envelop);
  }

  Future<AccountInfo> getAccountInfoCached({
    required String id,
    required String operator,
  }) async {
    if (_accountInfoCache.containsKey(id)) {
      return Future.value(_accountInfoCache[id]);
    }

    late AccountInfo accountInfo;
    try {
      accountInfo = await getAccountInfo(id: id, operator: operator);
    } catch (e) {
      // The account does not exist or we can't resolve it currently
      accountInfo =
          AccountInfo(accountId: hex.decode(id), publicName: 'Unknown');
    }

    _accountInfoCache[id] = accountInfo;
    return accountInfo;
  }

  Future<List<AccountDoc>> findAccountByOwner({
    required String owner,
    required String operator,
  }) async {
    final request = ListAccountsRequest();
    request.owner = base64Decode(owner);

    final envelop = await requestEnvelope(request: request);
    final documents =
        await getServiceClient(operator).query.listAccounts(envelop);

    return documents.accounts.map((account) => AccountDoc(account)).toList();
  }

  Future<int> updateAccount({
    required String id,
    required String operator,
    String? name,
    String? publicName,
    String? profileImageUrl,
    String? contextId,
  }) async {
    dynamic builder = DocumentUpdate(Account());
    builder.id(hex.decode(id));
    if (name != null) {
      builder.name(name);
    }
    if (publicName != null) {
      builder.publicName(publicName);
    }
    if (profileImageUrl != null) {
      builder.profileImageUrl(profileImageUrl);
    }
    final request = AccountExt.updateRequest(builder);
    final client = getServiceClient(operator);
    final envelop = await requestEnvelope(
        request: request,
        contextId: contextId != null ? hex.decode(contextId) : null);
    final response = await client.tx.createTransaction(envelop);
    if (response.hasError()) {
      throw response.error;
    }
    return response.txId.toInt();
  }

  Future<RoleBindingDoc> getRoleBinding({
    required String name,
    required String operator,
  }) async {
    final request = ListRoleBindingsRequest();
    request.name = name;

    final envelop = await requestEnvelope(request: request);
    final bindings =
        await getServiceClient(operator).query.listRoleBindings(envelop);

    return RoleBindingDoc(bindings.roleBindings.first);
  }

  Future updateRoleBinding({
    required String id,
    required String operator,
    List<String>? subjects,
    String? contextId,
  }) async {
    dynamic builder = DocumentUpdate(RoleBinding());
    builder.id(Uuid.parse(id));
    if (subjects != null) {
      final subjectIds = subjects.map((s) => base64.decode(s));
      builder.subjects(subjectIds);
    }

    final request = RoleBindingExt.updateRequest(builder);
    final client = getServiceClient(operator);
    final envelop = await requestEnvelope(
        request: request,
        contextId: contextId != null ? hex.decode(contextId) : null);
    await client.tx.createTransaction(envelop);
    return id;
  }

  Future<TransactionResponseDoc> transfer({
    required List<TransferStepDoc> steps,
    required String operator,
    String? contextId,
    bool commit = true,
  }) async {
    final transferSteps = steps.map((step) => step.model).toList();
    final transfer = CreateTransfer(transferSteps: transferSteps);

    final request = commit
        ? TransactionRequestPayload(data: TransactionData(transfer: transfer))
        : TransactionRequestPayload(
            data: TransactionData(initiateTransfer: transfer));
    final client = getServiceClient(operator);
    final envelop = await requestEnvelope(
        request: request,
        contextId: contextId != null ? hex.decode(contextId) : null);
    final response = await client.tx.createTransaction(envelop);

    return TransactionResponseDoc(response, contextId);
  }

  // Transfers
  Future<TransactionResponseDoc> createTransfer({
    required String fromAccountId,
    required String toAccountId,
    required int amount,
    required String operator,
    String memo = "",
    List<Attachment> attachments = const [],
    Contract? contract,
    String? contextId,
  }) async {
    final metadata = <Any>[];
    metadata.addAll(attachments.map(Metadata.attachment));
    if (memo.isNotEmpty) {
      metadata.add(Metadata.memo(memo));
    }
    if (contract != null) {
      metadata.add(Metadata.contract(contract));
      contextId = contract.contractId;
    }
    return await transfer(steps: [
      TransferStepDoc.fromFields(
          fromAccountId: fromAccountId,
          toAccountId: toAccountId,
          amount: amount,
          metadata: metadata)
    ], operator: operator, contextId: contextId);
  }

  Future<TransferDoc> getTransfer({
    required int txId,
    required String operator,
  }) async {
    final request = GetTransferRequest()..txId = Int64(txId);
    final envelop = await requestEnvelope(request: request);
    final transfer =
        await getServiceClient(operator).query.getTransfer(envelop);
    return TransferDoc(transfer, operator);
  }

  Future<List<EnhancedTransferStepDoc>> enhance(TransferDoc transfer) async {
    return await Stream.fromIterable(transfer.model.transferSteps)
        .asyncMap((step) => enhanceTransferStep(transfer.operator, step))
        .toList();
  }

  Future<EnhancedTransferStepDoc> enhanceTransferStep(
    String operator,
    TransferStep step,
  ) async {
    AccountInfo from = await getAccountInfoCached(
        id: hex.encode(step.fromAccountId), operator: operator);
    AccountInfo? fromBank = from.parentAccountId.isNotEmpty
        ? await getAccountInfoCached(
            id: hex.encode(from.parentAccountId), operator: operator)
        : null;
    AccountInfo to = await getAccountInfoCached(
        id: hex.encode(step.toAccountId), operator: operator);
    AccountInfo? toBank = to.parentAccountId.isNotEmpty
        ? await getAccountInfoCached(
            id: hex.encode(to.parentAccountId), operator: operator)
        : null;
    return EnhancedTransferStepDoc(
        step: step, from: from, to: to, fromBank: fromBank, toBank: toBank);
  }

  Future<List<TransferDoc>> listTransfers({
    required String operator,
    String? accountId,
    String? contextId,
    int minTxId = 0,
    int limit = 10,
    bool includeChildAccounts = false,
  }) async {
    final filterCount =
        (accountId == null ? 0 : 1) + (contextId == null ? 0 : 1);
    if (filterCount != 1) {
      throw ArgumentError('Invalid filter');
    }
    ListTransferRequest request = ListTransferRequest()
      ..minTxId = Int64(minTxId)
      ..limit = Int64(limit)
      ..includeChildAccounts = includeChildAccounts;

    if (accountId != null) {
      request.accountId = hex.decode(accountId);
    }
    if (contextId != null) {
      request.contextId = hex.decode(contextId);
    }

    final envelop = await requestEnvelope(request: request);
    FinalizedTransfers response =
        await getServiceClient(operator).query.listTransfers(envelop);
    return response.transfers
        .map((transfer) => TransferDoc(transfer, operator))
        .toList();
  }

  // Context
  Future<List<TransactionDoc>> listTransactions({
    required String operator,
    required String contextId,
    int minTxId = 0,
    int maxTxId = 0,
    int limit = 10,
  }) async {
    ListTransactionsRequest request = ListTransactionsRequest(
        contextId: hex.decode(contextId),
        minTxId: Int64(minTxId),
        maxTxId: Int64(maxTxId),
        limit: Int64(limit));

    final envelop = await requestEnvelope(request: request);
    FinalizedTransactions response =
        await getServiceClient(operator).query.listTransactions(envelop);
    return response.transactions.map((tx) => TransactionDoc(tx)).toList();
  }

  Future<List<List<TransactionDoc>>> groupTransactions({
    required String operator,
    required String accountId,
    int minTxId = 0,
    int maxTxId = 0,
    int limitGroups = 10,
  }) async {
    GroupTransactionsRequest request = GroupTransactionsRequest(
        accountId: hex.decode(accountId),
        minTxId: Int64(minTxId),
        maxTxId: Int64(maxTxId),
        limitGroups: Int64(limitGroups));

    final envelop = await requestEnvelope(request: request);
    GroupedFinalizedTransactions response =
        await getServiceClient(operator).query.groupTransactions(envelop);
    return response.groups
        .map((group) =>
            group.transactions.map((tx) => TransactionDoc(tx)).toList())
        .toList();
  }

  // Actions
  Future<TransactionResponseDoc> invokeAction({
    required String operator,
    required String name,
    required String fromAccountId,
    required String targetAccountId,
    List<int> payload = const [],
    String? contextId,
  }) async {
    InvokeAction action = InvokeAction(
        name: name,
        fromAccount: hex.decode(fromAccountId),
        target: Target(accountId: hex.decode(targetAccountId)),
        payload: payload);

    final request =
        TransactionRequestPayload(data: TransactionData(invokeAction: action));
    final client = getServiceClient(operator);
    final envelop = await requestEnvelope(
        request: request,
        contextId: contextId != null ? hex.decode(contextId) : null);
    final response = await client.tx.createTransaction(envelop);

    return TransactionResponseDoc(response, contextId);
  }

  Future<ActionDoc> getAction({
    required int txId,
    required String operator,
  }) async {
    final request = GetActionRequest(txId: Int64(txId));
    final envelop = await requestEnvelope(request: request);
    final action = await getServiceClient(operator).query.getAction(envelop);

    return ActionDoc(action);
  }

  Future<List<ActionDoc>> listActions({
    required String operator,
    required String name,
    required String accountId,
    int minTxId = 0,
    int maxTxId = 0,
    int limit = 10,
  }) async {
    ListActionsRequest request = ListActionsRequest(
        name: name,
        accountId: hex.decode(accountId),
        minTxId: Int64(minTxId),
        maxTxId: Int64(maxTxId),
        limit: Int64(limit));

    final envelop = await requestEnvelope(request: request);
    final response =
        await getServiceClient(operator).query.listActions(envelop);

    return response.actions.map((tx) => ActionDoc(tx)).toList();
  }

  // Accounts
  Future<String> createLedgerAccount({
    required String parentId,
    required String operator,
    String? contextId,
  }) async {
    CreateLedgerAccount account = CreateLedgerAccount();
    account.parentId = hex.decode(parentId);

    final request = TransactionRequestPayload(
        data: TransactionData(createLedgerAccount: account));
    final client = getServiceClient(operator);
    final envelop = await requestEnvelope(
        request: request,
        contextId: contextId != null ? hex.decode(contextId) : null);
    final response = await client.tx.createTransaction(envelop);
    return hex.encode(response.accountCreated);
  }

  Future<IndexedAccount> _getLedgerAccount({
    required String accountId,
    required String operator,
  }) async {
    GetAccountRequest request = GetAccountRequest();
    request.id = hex.decode(accountId);
    final envelop = await requestEnvelope(request: request);
    final response =
        await getServiceClient(operator).query.getIndexedAccount(envelop);
    return response;
  }

  Future<Stream<List<TransferResultDoc>>> observeTransfers({
    required String operator,
    required List<String> accounts,
    Int64? startingFrom,
  }) async {
    ObserveAccountsRequest request = ObserveAccountsRequest();
    request.involvedAccounts.addAll(accounts.map((a) => hex.decode(a)));

    if (startingFrom != null) {
      request.startingFrom = TxId()..txId = startingFrom;
    }

    final envelop = await requestEnvelope(request: request);
    final stream =
        await getServiceClient(operator).query.observeTransfers(envelop);
    return stream.map((transactions) => transactions.transactions
        .map((result) => TransferResultDoc.fromModel(result))
        .toList());
  }

  Future<Stream<List<ActionDoc>>> observeActions({
    required String operator,
    required String name,
    required List<String> accounts,
    Int64? startingFrom,
  }) async {
    ObserveActionsRequest request = ObserveActionsRequest(
        name: name,
        involvesAccounts: accounts.map((a) => hex.decode(a)).toList());

    if (startingFrom != null) {
      request.startingFrom = TxId()..txId = startingFrom;
    }

    final envelop = await requestEnvelope(request: request);
    final stream =
        await getServiceClient(operator).query.observeActions(envelop);
    return stream.map((transactions) => transactions.transactions.map((tx) {
          final action = tx.request.data.invokeAction;
          return ActionDoc(Action(
              txId: tx.response.txId,
              name: action.name,
              contextId: tx.request.contextId,
              fromAccount: action.fromAccount,
              target: action.target,
              payload: action.payload));
        }).toList());
  }

  Future<Stream<List<ResourceResultDoc>>> observeResources({
    required String operator,
    required String expression,
    required String collection,
    Map<String, Value> variables = const {},
    Int64? startingFrom,
  }) async {
    Exp exp = Exp()..exp = expression;
    exp.vars.addAll(variables.map((key, value) => MapEntry(key, value)));
    ObserveResourcesRequest request = ObserveResourcesRequest()
      ..collection = collection
      ..expression = exp;

    if (startingFrom != null) {
      request.startingFrom = TxId()..txId = startingFrom;
    }

    final envelop = await requestEnvelope(request: request);
    final stream =
        await getServiceClient(operator).query.observeResources(envelop);
    return stream.map((transactions) => transactions.transactions
        .map((result) => ResourceResultDoc.fromModel(result))
        .toList());
  }

  // Creates a builder object for a Ledger Contract
  ContractBuilder createContract() {
    return ContractBuilder();
  }

  /// A helper function that updates the ledger-account status
  Future<void> updateAccountStatus({
    required String id,
    required String operator,
    required bool frozen,
    String? contextId,
  }) async {
    final account = await _getLedgerAccount(
      accountId: id,
      operator: operator,
    );

    SetFreezeState freezeState = SetFreezeState();
    freezeState.accountId = hex.decode(id);

    if (account.frozen != frozen) {
      freezeState.frozen = frozen;
      final request = TransactionRequestPayload(
          data: TransactionData(setFreezeState: freezeState));
      final envelop = await requestEnvelope(
          request: request,
          contextId: contextId != null ? hex.decode(contextId) : null);
      final client = getServiceClient(operator);
      await client.tx.createTransaction(envelop);
    }
  }

  String get randomContextId {
    final random = Random.secure();
    return hex.encode(List<int>.generate(32, (i) => random.nextInt(256)));
  }
}

enum Resource { User, Id }

extension ResourceExt on Resource {
  String get value => this.toString().split('.').last;
}

extension M10Actions on M10Sdk {
  static const requestActionName = "m10.Request";

  Future<TransactionResponseDoc> request({
    required String operator,
    required CreateTransfer transferRequest,
    String? contextId,
  }) async {
    return await invokeAction(
      operator: operator,
      name: requestActionName,
      fromAccountId: hex.encode(transferRequest.transferSteps[0].toAccountId),
      targetAccountId:
          hex.encode(transferRequest.transferSteps[0].fromAccountId),
      payload: PaymentRequest(
              transfer: transferRequest,
              status: PaymentRequest_PaymentRequestStatus.PENDING)
          .writeToBuffer(),
      contextId: contextId ?? randomContextId,
    );
  }

  Future<TransactionResponseDoc> cancel({
    required String operator,
    required String fromAccountId,
    required String targetAccountId,
    required String contextId,
  }) async {
    return await invokeAction(
      operator: operator,
      name: requestActionName,
      fromAccountId: fromAccountId,
      targetAccountId: targetAccountId,
      payload:
          PaymentRequest(status: PaymentRequest_PaymentRequestStatus.CANCELED)
              .writeToBuffer(),
      contextId: contextId,
    );
  }

  Future<TransactionResponseDoc> decline({
    required String operator,
    required String fromAccountId,
    required String targetAccountId,
    required String contextId,
  }) async {
    return await invokeAction(
      operator: operator,
      name: requestActionName,
      fromAccountId: fromAccountId,
      targetAccountId: targetAccountId,
      payload:
          PaymentRequest(status: PaymentRequest_PaymentRequestStatus.DECLINED)
              .writeToBuffer(),
      contextId: contextId,
    );
  }

  Future<List<PaymentRequestDoc>> listRequests({
    required String operator,
    required String accountId,
    int minTxId = 0,
    int maxTxId = 0,
    int limit = 10,
  }) async {
    List<PaymentRequestDoc> result = [];
    // Setting maxRequestTxId to max int if '0'
    int maxRequestTxId = maxTxId == 0 ? ~(-1 << 63) : maxTxId;

    do {
      List<List<TransactionDoc>> txGroups = await groupTransactions(
          operator: operator,
          accountId: accountId,
          minTxId: minTxId,
          maxTxId: maxRequestTxId,
          limitGroups: limit);

      if (txGroups.isEmpty) {
        // Exist loop if there is no request in the given range
        break;
      }

      // Update the max request ID
      maxRequestTxId = txGroups
              .map((group) =>
                  group.map((tx) => tx.txId).fold(maxRequestTxId, min))
              .reduce(min) -
          1;

      // Filter for groups with actions of type 'Request'
      List<PaymentRequestDoc> requests = txGroups
          .map((group) => PaymentRequestDoc.from(group))
          .whereNotNull()
          .toList();

      // Augment the account info fields
      final requestDocs = requests.map((request) async {
        request.fromAccount = await getAccountInfoCached(
            id: request.fromAccountId, operator: operator);
        request.toAccount = await getAccountInfoCached(
            id: request.toAccountId, operator: operator);
        return request;
      });

      final ar = await Future.wait(requestDocs);
      result.addAll(ar);
    } while (result.length < limit && maxRequestTxId != 0);
    if (result.length > limit) {
      // Potentially the result list can be longer than the limit, so it needs to be trimmed.
      // this removes from the end, since sorted that will be the older
      // requests
      result.removeRange(limit, result.length);
    }

    result.sort((a, b) => b.timestamp.compareTo(a.timestamp));
    return result;
  }
}
