import 'dart:convert';
import 'dart:math';
import 'dart:typed_data';

import 'package:convert/convert.dart';
import 'package:fixnum/fixnum.dart';
import 'package:grpc/grpc.dart';
import 'package:m10_sdk/metadata.dart';
import 'package:m10_sdk/src/generated/google/protobuf/empty.pb.dart';
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
import 'package:xid/xid.dart';

class M10Sdk {
  M10Sdk({
    required this.signer,
    required List<directory.Ledger> ledgers,
    bool disableTls = false,
  })  : _random = Random.secure(),
        _credentials = disableTls
            ? const ChannelCredentials.insecure()
            : const ChannelCredentials.secure(),
        _ledgers = ledgers,
        _accountInfoCache = <String, AccountInfo>{};

  final Signing signer;
  final ChannelCredentials _credentials;
  final Random _random;
  final Map<String, AccountInfo> _accountInfoCache;

  List<directory.Ledger> _ledgers;
  set ledgers(List<directory.Ledger> newValue) {
    _ledgers = newValue;
  }

  M10Sdk copyWithSigner(Signing signer) =>
      M10Sdk(signer: signer, ledgers: _ledgers);

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

  ServiceClient getServiceClient(String operator) => serviceClients.putIfAbsent(
        operator,
        () => ServiceClient(getChannel(operator)),
      );

  Future<RequestEnvelope> requestEnvelope({
    required GeneratedMessage request,
    List<int>? contextId,
  }) async {
    if (request is TransactionRequestPayload) {
      request
        ..nonce =
            Int64.fromInts(_random.nextInt(1 << 32), _random.nextInt(1 << 32))
        ..timestamp = Int64(DateTime.now().microsecondsSinceEpoch)
        ..contextId = contextId ?? [];
    }
    final payload = request.writeToBuffer();
    final signature = await signer.signPayload(payload);
    return RequestEnvelope()
      ..payload = payload
      ..signature = signature;
  }

  Future<String> createUser({
    required String operator,
    String? contextId,
    String? id,
  }) async {
    final uuid = Uuid();
    final userId =
        id != null ? Uuid.parse(id) : uuid.v4buffer(List.filled(16, 0));
    final user = AccountSet()
      ..id = userId
      ..owner = await signer.publicKey();

    final request = user.createRequest();
    final client = getServiceClient(operator);
    final envelop = await requestEnvelope(
      request: request,
      contextId: contextId != null ? hex.decode(contextId) : null,
    );
    await client.tx.createTransaction(envelop);
    return Uuid.unparse(userId);
  }

  Future<AccountSetDoc> getUser({
    required String userId,
    required String operator,
  }) async {
    final request = GetAccountSetRequest()..id = Uuid.parse(userId);
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
    final dynamic builder = DocumentUpdate(AccountSet());
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
      contextId: contextId != null ? hex.decode(contextId) : null,
    );
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
      contextId: contextId != null ? hex.decode(contextId) : null,
    );
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
    int? balanceLimit,
    String? contextId,
  }) async {
    final ownerBytes = owner != null ? base64Decode(owner) : null;
    final accountId = await createLedgerAccount(
      parentId: parentId,
      operator: operator,
      balanceLimit: balanceLimit,
    );
    final account = AccountMetadata()
      ..id = hex.decode(accountId)
      ..owner = ownerBytes ?? await signer.publicKey();

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
      contextId: contextId != null ? hex.decode(contextId) : null,
    );
    await client.tx.createTransaction(envelop);

    return accountId;
  }

  Future<AccountMetadataDoc> getAccountMetadata({
    required String id,
    required String operator,
  }) async {
    final request = GetAccountRequest()..id = hex.decode(id);
    final envelop = await requestEnvelope(request: request);
    final account =
        await getServiceClient(operator).query.getAccountMetadata(envelop);
    final indexedAccount =
        await _getLedgerAccount(accountId: id, operator: operator);

    return AccountMetadataDoc.fromModel(account, indexedAccount);
  }

  Future<AccountInfo> getAccountInfo({
    required String id,
    required String operator,
  }) async {
    final request = GetAccountRequest()..id = hex.decode(id);
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

  Future<List<AccountMetadataDoc>> findAccountByOwner({
    required String owner,
    required String operator,
  }) async {
    final request = ListAccountMetadataRequest()..owner = base64Decode(owner);
    final envelop = await requestEnvelope(request: request);
    final documents =
        await getServiceClient(operator).query.listAccountMetadata(envelop);

    return documents.accounts.map(AccountMetadataDoc.new).toList();
  }

  Future<List<AccountMetadataDoc>> findAccountByName({
    required String name,
    required String operator,
  }) async {
    final request = ListAccountMetadataRequest()..name = name;
    final envelop = await requestEnvelope(request: request);
    final documents =
        await getServiceClient(operator).query.listAccountMetadata(envelop);

    return documents.accounts.map(AccountMetadataDoc.new).toList();
  }

  Future<int> updateAccount({
    required String id,
    required String operator,
    String? name,
    String? publicName,
    String? profileImageUrl,
    String? contextId,
  }) async {
    final dynamic builder = DocumentUpdate(AccountMetadata());
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
    final request = AccountMetadataExt.updateRequest(builder);
    final client = getServiceClient(operator);
    final envelop = await requestEnvelope(
      request: request,
      contextId: contextId != null ? hex.decode(contextId) : null,
    );
    final response = await client.tx.createTransaction(envelop);
    if (response.hasError()) {
      throw response.error;
    }
    return response.txId.toInt();
  }

  Future<List<RoleDoc>> listRole({
    required String name,
    required String operator,
  }) async {
    final request = ListRolesRequest()..name = name;
    final envelop = await requestEnvelope(request: request);
    final bindings = await getServiceClient(operator).query.listRoles(envelop);

    return bindings.roles.map(RoleDoc.new).toList();
  }

  Future<RoleDoc> getRole({
    required String id,
    required String operator,
  }) async {
    final request = GetRoleRequest()..id = hex.decode(id);
    final envelop = await requestEnvelope(request: request);
    final role = await getServiceClient(operator).query.getRole(envelop);

    return RoleDoc(role);
  }

  Future createRole({
    required String operator,
    required String name,
    String? id,
    String? owner,
    List<RuleDoc>? rules,
    String? contextId,
  }) async {
    final uuid = Uuid();
    final ownerBytes = owner != null ? base64Decode(owner) : null;

    final role = Role()
      ..id = (id != null ? hex.decode(id) : uuid.v4buffer(List.filled(16, 0)))
      ..name = name
      ..owner = ownerBytes ?? await signer.publicKey();

    role.rules.clear();
    if (rules != null) {
      role.rules.addAll(rules.map((r) => r.asRule()));
    }

    final request = role.createRequest();
    final client = getServiceClient(operator);
    final envelop = await requestEnvelope(
      request: request,
      contextId: contextId != null ? hex.decode(contextId) : null,
    );
    await client.tx.createTransaction(envelop);
    return id;
  }

  Future updateRole({
    required String id,
    required String operator,
    String? name,
    String? owner,
    List<RuleDoc>? rules,
    String? contextId,
  }) async {
    final dynamic builder = DocumentUpdate(Role());
    builder.id(Uuid.parse(id));
    if (rules != null) {
      builder.rules(rules.map((r) => r.asRule()));
    }

    if (name != null) {
      builder.name(name);
    }

    final request = RoleExt.updateRequest(builder);
    final client = getServiceClient(operator);
    final envelop = await requestEnvelope(
      request: request,
      contextId: contextId != null ? hex.decode(contextId) : null,
    );
    await client.tx.createTransaction(envelop);
    return id;
  }

  Future deleteRole({
    required String id,
    required String operator,
    String? contextId,
  }) async {
    final request = RoleExt.deleteRequest(Uuid.parse(id));
    final client = getServiceClient(operator);
    final envelop = await requestEnvelope(
      request: request,
      contextId: contextId != null ? hex.decode(contextId) : null,
    );
    await client.tx.createTransaction(envelop);
    return id;
  }

  Future<List<RoleBindingDoc>> listRoleBindings({
    required String name,
    required String operator,
  }) async {
    final request = ListRoleBindingsRequest()..name = name;
    final envelop = await requestEnvelope(request: request);
    final bindings =
        await getServiceClient(operator).query.listRoleBindings(envelop);

    return bindings.roleBindings.map(RoleBindingDoc.new).toList();
  }

  Future<RoleBindingDoc> getRoleBinding({
    required String id,
    required String operator,
  }) async {
    final request = GetRoleBindingRequest()..id = hex.decode(id);
    final envelop = await requestEnvelope(request: request);
    final roleBinding =
        await getServiceClient(operator).query.getRoleBinding(envelop);

    return RoleBindingDoc(roleBinding);
  }

  Future createRoleBinding({
    required String operator,
    required String role,
    required String name,
    String? id,
    List<String>? subjects,
    List<Expression>? expressions,
    String? contextId,
  }) async {
    final uuid = Uuid();
    final roleBinding = RoleBinding()
      ..id = (id != null ? hex.decode(id) : uuid.v4buffer(List.filled(16, 0)))
      ..name = name;

    roleBinding.expressions.clear();
    if (expressions != null) {
      roleBinding.expressions.addAll(expressions);
    }
    roleBinding.subjects.clear();
    if (subjects != null) {
      roleBinding.subjects.addAll(subjects.map(base64.decode));
    }

    final request = roleBinding.createRequest();
    final client = getServiceClient(operator);
    final envelop = await requestEnvelope(
      request: request,
      contextId: contextId != null ? hex.decode(contextId) : null,
    );
    await client.tx.createTransaction(envelop);
    return id;
  }

  Future updateRoleBinding({
    required String id,
    required String operator,
    List<String>? subjects,
    String? contextId,
  }) async {
    final dynamic builder = DocumentUpdate(RoleBinding());
    builder.id(Uuid.parse(id));
    if (subjects != null) {
      final subjectIds = subjects.map((s) => base64.decode(s));
      builder.subjects(subjectIds);
    }

    final request = RoleBindingExt.updateRequest(builder);
    final client = getServiceClient(operator);
    final envelop = await requestEnvelope(
      request: request,
      contextId: contextId != null ? hex.decode(contextId) : null,
    );
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
    final transfer = CreateTransfer()..setSteps(transferSteps);

    final request = TransactionRequestPayload();
    if (commit) {
      request.data = (TransactionData()..transfer = transfer);
    } else {
      request.data = (TransactionData()..initiateTransfer = transfer);
    }

    final client = getServiceClient(operator);
    final envelop = await requestEnvelope(
      request: request,
      contextId: contextId != null ? hex.decode(contextId) : null,
    );
    final response = await client.tx.createTransaction(envelop);

    return TransactionResponseDoc(response, contextId);
  }

  // Transfers
  Future<TransactionResponseDoc> createTransfer({
    required String fromAccountId,
    required String toAccountId,
    required int amount,
    required String operator,
    String memo = '',
    List<Attachment> attachments = const [],
    Contract? contract,
    String? contextId,
    SelfTransfer? selfTransfer,
  }) async {
    final metadata = <Any>[...attachments.map(Metadata.attachment)];
    if (memo.isNotEmpty) {
      metadata.add(Metadata.memo(memo));
    }
    if (contract != null) {
      metadata.add(Metadata.contract(contract));
      contextId = contract.contractId;
    }
    if (selfTransfer != null) {
      metadata.add(Metadata.selfTransfer(selfTransfer));
    }

    return transfer(
      steps: [
        TransferStepDoc.fromFields(
          fromAccountId: fromAccountId,
          toAccountId: toAccountId,
          amount: amount,
          metadata: metadata,
        ),
      ],
      operator: operator,
      contextId: contextId,
    );
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

  Future<String> createEncodedTransferEnvelope({
    required String fromAccountId,
    required String toAccountId,
    required int amount,
    required String operator,
    String memo = '',
    List<Attachment> attachments = const [],
    Contract? contract,
    String? contextId,
    SelfTransfer? selfTransfer,
  }) async {
    final metadata = <Any>[...attachments.map(Metadata.attachment)];
    if (memo.isNotEmpty) {
      metadata.add(Metadata.memo(memo));
    }
    if (contract != null) {
      metadata.add(Metadata.contract(contract));
      contextId = contract.contractId;
    }
    if (selfTransfer != null) {
      metadata.add(Metadata.selfTransfer(selfTransfer));
    }

    final transfer = CreateTransfer();
    transfer.transferSteps
      ..clear()
      ..add(
        TransferStepDoc.fromFields(
          fromAccountId: fromAccountId,
          toAccountId: toAccountId,
          amount: amount,
          metadata: metadata,
        ).model,
      );

    return createEncodedTransferEnvelopeFromCreateTransfer(
      createTransfer: transfer,
    );
  }

  Future<String> createEncodedTransferEnvelopeFromCreateTransfer({
    required CreateTransfer createTransfer,
    String? contextId,
  }) async {
    final request = TransactionRequestPayload()
      ..data = (TransactionData()..initiateTransfer = createTransfer);
    final envelope = await requestEnvelope(
      request: request,
      contextId: contextId != null ? hex.decode(contextId) : null,
    );

    return base64Encode(envelope.writeToBuffer());
  }

  Future<List<EnhancedTransferStepDoc>> enhance(TransferDoc transfer) async =>
      enhanceTransferSteps(
        transfer.model.transferSteps,
        operator: transfer.operator,
      );

  Future<List<EnhancedTransferStepDoc>> enhanceTransferSteps(
    List<TransferStep> steps, {
    required String operator,
  }) async =>
      Stream.fromIterable(steps)
          .asyncMap((step) => enhanceTransferStep(operator, step))
          .toList();

  Future<EnhancedTransferStepDoc> enhanceTransferStep(
    String operator,
    TransferStep step,
  ) async {
    final from = await getAccountInfoCached(
      id: hex.encode(step.fromAccountId),
      operator: operator,
    );
    final fromBank = from.parentAccountId.isNotEmpty
        ? await getAccountInfoCached(
            id: hex.encode(from.parentAccountId),
            operator: operator,
          )
        : null;
    final to = await getAccountInfoCached(
      id: hex.encode(step.toAccountId),
      operator: operator,
    );
    final toBank = to.parentAccountId.isNotEmpty
        ? await getAccountInfoCached(
            id: hex.encode(to.parentAccountId),
            operator: operator,
          )
        : null;

    return EnhancedTransferStepDoc(
      step: step,
      from: from,
      to: to,
      fromBank: fromBank,
      toBank: toBank,
    );
  }

  Future<List<TransferDoc>> listTransfers({
    required String operator,
    String? accountId,
    String? contextId,
    int? maxTxId,
    int minTxId = 0,
    int limit = 10,
    bool includeChildAccounts = false,
  }) async {
    final filterCount =
        (accountId == null ? 0 : 1) + (contextId == null ? 0 : 1);
    if (filterCount != 1) {
      throw ArgumentError('Invalid filter');
    }
    final request = ListTransferRequest()
      ..minTxId = Int64(minTxId)
      ..limit = Int64(limit)
      ..includeChildAccounts = includeChildAccounts;

    if (accountId != null) {
      request.accountId = hex.decode(accountId);
    }
    if (contextId != null) {
      request.contextId = hex.decode(contextId);
    }
    if (maxTxId != null) {
      request.maxTxId = Int64(maxTxId);
    }

    final envelop = await requestEnvelope(request: request);
    final response =
        await getServiceClient(operator).query.listTransfers(envelop);
    return response.transfers
        .map((transfer) => TransferDoc(transfer, operator))
        .toList();
  }

  Future<TransactionDoc> getTransaction({
    required String operator,
    required int txId,
  }) async {
    final envelop = await requestEnvelope(
      request: GetTransactionRequest()..txId = Int64(txId),
    );
    final response =
        await getServiceClient(operator).query.getTransaction(envelop);
    return TransactionDoc(response);
  }

  // Context
  Future<List<TransactionDoc>> listTransactions({
    required String operator,
    required String contextId,
    int minTxId = 0,
    int maxTxId = 0,
    int limit = 10,
  }) async {
    final request = ListTransactionsRequest()
      ..contextId = hex.decode(contextId)
      ..minTxId = Int64(minTxId)
      ..maxTxId = Int64(maxTxId)
      ..limit = Int64(limit);

    final envelop = await requestEnvelope(request: request);
    final response =
        await getServiceClient(operator).query.listTransactions(envelop);
    return response.transactions.map(TransactionDoc.new).toList();
  }

  Future<List<List<TransactionDoc>>> groupTransactions({
    required String operator,
    required String accountId,
    int minTxId = 0,
    int maxTxId = 0,
    int limitGroups = 10,
  }) async {
    final request = GroupTransactionsRequest()
      ..accountId = hex.decode(accountId)
      ..minTxId = Int64(minTxId)
      ..maxTxId = Int64(maxTxId)
      ..limitGroups = Int64(limitGroups);
    final envelop = await requestEnvelope(request: request);
    final response =
        await getServiceClient(operator).query.groupTransactions(envelop);

    return response.groups
        .map(
          (group) => group.transactions.map(TransactionDoc.new).toList(),
        )
        .toList();
  }

  // Actions
  Future<TransactionResponseDoc> invokeAction({
    required String operator,
    required String name,
    required String fromAccountId,
    String? targetAccountId,
    bool targetAll = false,
    List<int> payload = const [],
    String? contextId,
  }) async {
    final Target target;
    if (targetAccountId != null) {
      target = Target()..accountId = hex.decode(targetAccountId);
    } else if (targetAll) {
      target = Target()..anyAccount = Empty();
    } else {
      throw 'Missing action target';
    }

    final action = InvokeAction()
      ..name = name
      ..fromAccount = hex.decode(fromAccountId)
      ..target = target
      ..payload = payload;

    final request = TransactionRequestPayload()
      ..data = (TransactionData()..invokeAction = action);
    final client = getServiceClient(operator);
    final envelop = await requestEnvelope(
      request: request,
      contextId: contextId != null ? hex.decode(contextId) : null,
    );
    final response = await client.tx.createTransaction(envelop);

    return TransactionResponseDoc(response, contextId);
  }

  Future<ActionDoc> getAction({
    required int txId,
    required String operator,
  }) async {
    final request = GetActionRequest()..txId = Int64(txId);
    final envelop = await requestEnvelope(request: request);
    final action = await getServiceClient(operator).query.getAction(envelop);

    return ActionDoc(action);
  }

  Future<List<ActionDoc>> listActions({
    required String operator,
    required String name,
    String? accountId,
    Uint8List? contextId,
    int minTxId = 0,
    int maxTxId = 0,
    int limit = 10,
  }) async {
    final request = ListActionsRequest()
      ..name = name
      ..minTxId = Int64(minTxId)
      ..maxTxId = Int64(maxTxId)
      ..limit = Int64(limit);
    if (accountId != null) {
      request.accountId = hex.decode(accountId);
    } else if (contextId != null) {
      request.contextId = contextId;
    } else {
      throw Exception(
        'Missing filter, one of `accountId` or `contextId` must be non-null',
      );
    }

    final envelop = await requestEnvelope(request: request);
    final response =
        await getServiceClient(operator).query.listActions(envelop);

    return response.actions.map(ActionDoc.new).toList();
  }

  // Accounts
  Future<String> createLedgerAccount({
    required String parentId,
    required String operator,
    int? balanceLimit,
    String? contextId,
  }) async {
    final account = CreateLedgerAccount()
      ..parentId = hex.decode(parentId)
      ..balanceLimit = Int64(balanceLimit ?? 0);

    final request = TransactionRequestPayload()
      ..data = (TransactionData()..createLedgerAccount = account);
    final client = getServiceClient(operator);
    final envelop = await requestEnvelope(
      request: request,
      contextId: contextId != null ? hex.decode(contextId) : null,
    );
    final response = await client.tx.createTransaction(envelop);
    return hex.encode(response.accountCreated);
  }

  Future<IndexedAccount> _getLedgerAccount({
    required String accountId,
    required String operator,
  }) async {
    final request = GetAccountRequest()..id = hex.decode(accountId);
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
    final request = ObserveAccountsRequest();
    request.involvedAccounts.addAll(accounts.map((a) => hex.decode(a)));

    if (startingFrom != null) {
      request.startingFrom = TxId()..txId = startingFrom;
    }

    final envelop = await requestEnvelope(request: request);
    final stream = getServiceClient(operator).query.observeTransfers(envelop);
    return stream.map(
      (transactions) =>
          transactions.transactions.map(TransferResultDoc.fromModel).toList(),
    );
  }

  Future<Stream<List<ActionDoc>>> observeActions({
    required String operator,
    required String name,
    required List<String> accounts,
    Int64? startingFrom,
  }) async {
    final request = ObserveActionsRequest()..name = name;
    request.involvesAccounts
      ..clear()
      ..addAll(accounts.map(hex.decode));

    if (startingFrom != null) {
      request.startingFrom = TxId()..txId = startingFrom;
    }

    final envelop = await requestEnvelope(request: request);
    final stream = getServiceClient(operator).query.observeActions(envelop);
    return stream.map(
      (transactions) => transactions.transactions.map((tx) {
        final action = tx.request.data.invokeAction;
        return ActionDoc(
          Action()
            ..txId = tx.response.txId
            ..name = action.name
            ..contextId = tx.request.contextId
            ..fromAccount = action.fromAccount
            ..target = action.target
            ..payload = action.payload,
        );
      }).toList(),
    );
  }

  //
  // Offline tokens
  //

  ///
  /// Creates a transferable offline token by transfering
  /// from the specified ledger account.
  ///
  Future<(TransactionResponse, OfflineToken)> createToken({
    required String accountId,
    required Int64 value,
    required String operator,
    String? address,
    String? contextId,
  }) async {
    final createToken = CreateToken()
      ..accountId = hex.decode(accountId)
      ..value = value
      ..address =
          address != null ? base64.decode(address) : await signer.publicKey();

    final client = getServiceClient(operator);
    final request = TransactionRequestPayload()
      ..data = (TransactionData()..createToken = createToken);
    final envelope = await requestEnvelope(
      request: request,
      contextId: contextId != null ? hex.decode(contextId) : null,
    );

    final response = await client.tx.createTransaction(envelope);
    return (response, response.token);
  }

  ///
  /// Credits a redeemable offline token to the specified
  /// ledger account. Returns the transaction ID.
  ///
  Future<Int64> redeemToken({
    required RedeemableToken token,
    required String accountId,
    required String operator,
    String? contextId,
  }) async {
    final redeemToken = RedeemToken()
      ..accountId = hex.decode(accountId)
      ..token = token;

    final client = getServiceClient(operator);
    final request = TransactionRequestPayload()
      ..data = (TransactionData()..redeemToken = redeemToken);
    final envelope = await requestEnvelope(
      request: request,
      contextId: contextId != null ? hex.decode(contextId) : null,
    );

    final response = await client.tx.createTransaction(envelope);
    return response.txId;
  }

  ///
  /// Issues a signed redeemable token from an existing
  /// transferable token.
  ///
  Future<RedeemableToken> issueToken({
    required String toAddress,
    required String currencyCode,
    required Int64 value,
    required List<RedeemableToken_TokenInput> tokenInputs,
  }) async {
    final tokenData = RedeemableToken_Data()
      ..id = Xid().toBytes()
      ..address = base64.decode(toAddress)
      ..currency = currencyCode;
    tokenData.inputs
      ..clear()
      ..addAll(tokenInputs);

    final redeemableToken = RedeemableToken()
      ..data = tokenData
      ..signature = await signer.signPayload(tokenData.writeToBuffer());

    return redeemableToken;
  }

  Future<Stream<List<PaymentRequestDoc>>> observeRequests({
    required String operator,
    required List<String> accounts,
    Int64? startingFrom,
  }) async {
    final request = ObserveActionsRequest()..name = 'm10.Request';
    request.involvesAccounts
      ..clear()
      ..addAll(accounts.map(hex.decode));

    if (startingFrom != null) {
      request.startingFrom = TxId()..txId = startingFrom;
    }

    final envelop = await requestEnvelope(request: request);
    final stream = getServiceClient(operator).query.observeActions(envelop);
    return stream.asyncMap(
      (FinalizedTransactions transactions) async {
        final txDocs =
            transactions.transactions.map(TransactionDoc.new).toList();
        final requests =
            [txDocs].map(PaymentRequestDoc.from).whereNotNull().toList();
        final requestDocs = requests.map(
          (r) async => r
            ..fromAccount = await getAccountInfoCached(
              id: r.fromAccountId,
              operator: operator,
            )
            ..toAccount = await getAccountInfoCached(
              id: r.toAccountId,
              operator: operator,
            ),
        );
        return Future.wait(requestDocs);
      },
    );
  }

  Future<Stream<List<ResourceResultDoc>>> observeResources({
    required String operator,
    required String expression,
    required String collection,
    Map<String, Value> variables = const {},
    Int64? startingFrom,
  }) async {
    final exp = Exp()..exp = expression;
    exp.vars.addAll(variables.map(MapEntry.new));
    final request = ObserveResourcesRequest()
      ..collection = collection
      ..expression = exp;

    if (startingFrom != null) {
      request.startingFrom = TxId()..txId = startingFrom;
    }

    final envelop = await requestEnvelope(request: request);
    final stream = getServiceClient(operator).query.observeResources(envelop);
    return stream.map(
      (transactions) =>
          transactions.transactions.map(ResourceResultDoc.fromModel).toList(),
    );
  }

  Future<Stream<TransactionMetricsDoc>> observeMetrics({
    required String operator,
    required List<String> accounts,
    Int64? startingFrom,
  }) async {
    final request = ObserveAccountsRequest();
    request.involvedAccounts.addAll(accounts.map((a) => hex.decode(a)));

    if (startingFrom != null) {
      request.startingFrom = TxId()..txId = startingFrom;
    }

    final envelop = await requestEnvelope(request: request);
    final stream = getServiceClient(operator).query.observeMetrics(envelop);
    return stream.map(TransactionMetricsDoc.fromModel);
  }

  // Creates a builder object for a Ledger Contract
  ContractBuilder createContract() => ContractBuilder();

  /// A helper function that updates the ledger-account status
  Future<void> updateAccountStatus({
    required String id,
    required String operator,
    required bool frozen,
    String? contextId,
  }) async {
    final account = await _getLedgerAccount(accountId: id, operator: operator);
    final freezeState = SetFreezeState()..accountId = hex.decode(id);

    if (account.frozen != frozen) {
      freezeState.frozen = frozen;
      final request = TransactionRequestPayload()
        ..data = (TransactionData()..setFreezeState = freezeState);
      final envelop = await requestEnvelope(
        request: request,
        contextId: contextId != null ? hex.decode(contextId) : null,
      );
      final client = getServiceClient(operator);
      await client.tx.createTransaction(envelop);
    }
  }

  String get randomContextId {
    final random = Random.secure();
    return hex.encode(List<int>.generate(32, (i) => random.nextInt(256)));
  }
}

enum Resource { user, id }

extension ResourceExt on Resource {
  String get value => toString().split('.').last;
}

extension M10Actions on M10Sdk {
  static const requestActionName = 'm10.Request';

  Future<TransactionResponseDoc> request({
    required String operator,
    required CreateTransfer transferRequest,
    String? contextId,
  }) async =>
      invokeAction(
        operator: operator,
        name: requestActionName,
        fromAccountId: hex.encode(transferRequest.transferSteps[0].toAccountId),
        targetAccountId:
            hex.encode(transferRequest.transferSteps[0].fromAccountId),
        payload: (PaymentRequest()
              ..transfer = transferRequest
              ..status = PaymentRequest_PaymentRequestStatus.PENDING)
            .writeToBuffer(),
        contextId: contextId ?? randomContextId,
      );

  Future<TransactionResponseDoc> cancel({
    required String operator,
    required String fromAccountId,
    required String targetAccountId,
    required String contextId,
  }) async =>
      invokeAction(
        operator: operator,
        name: requestActionName,
        fromAccountId: fromAccountId,
        targetAccountId: targetAccountId,
        payload: (PaymentRequest()
              ..status = PaymentRequest_PaymentRequestStatus.CANCELED)
            .writeToBuffer(),
        contextId: contextId,
      );

  Future<TransactionResponseDoc> decline({
    required String operator,
    required String fromAccountId,
    required String targetAccountId,
    required String contextId,
  }) async =>
      invokeAction(
        operator: operator,
        name: requestActionName,
        fromAccountId: fromAccountId,
        targetAccountId: targetAccountId,
        payload: (PaymentRequest()
              ..status = PaymentRequest_PaymentRequestStatus.DECLINED)
            .writeToBuffer(),
        contextId: contextId,
      );

  Future<List<PaymentRequestDoc>> listRequests({
    required String operator,
    required String accountId,
    int minTxId = 0,
    int maxTxId = 0,
    int limit = 10,
  }) async {
    final result = <PaymentRequestDoc>[];
    // Setting maxRequestTxId to max int if '0'
    var maxRequestTxId = maxTxId == 0 ? ~(-1 << 63) : maxTxId;

    do {
      final txGroups = await groupTransactions(
        operator: operator,
        accountId: accountId,
        minTxId: minTxId,
        maxTxId: maxRequestTxId,
        limitGroups: limit,
      );

      if (txGroups.isEmpty) {
        // Exist loop if there is no request in the given range
        break;
      }

      // Update the max request ID
      maxRequestTxId = txGroups
              .map(
                (group) => group.map((tx) => tx.txId).fold(maxRequestTxId, min),
              )
              .reduce(min) -
          1;

      // Filter for groups with actions of type 'Request'
      final requests =
          txGroups.map(PaymentRequestDoc.from).whereNotNull().toList();

      // Augment the account info fields
      final requestDocs = requests.map(
        (request) async => request
          ..fromAccount = await getAccountInfoCached(
            id: request.fromAccountId,
            operator: operator,
          )
          ..toAccount = await getAccountInfoCached(
            id: request.toAccountId,
            operator: operator,
          ),
      );

      final ar = await Future.wait(requestDocs);
      result.addAll(ar);
    } while (result.length < limit && maxRequestTxId != 0);
    if (result.length > limit) {
      // Potentially the result list can be longer than the limit, so it needs
      // to be trimmed. This removes from the end, since sorted that will be the
      // older requests
      result.removeRange(limit, result.length);
    }

    result.sort((a, b) => b.timestamp.compareTo(a.timestamp));
    return result;
  }
}
