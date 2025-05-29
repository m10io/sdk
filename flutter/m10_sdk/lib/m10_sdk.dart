import 'dart:convert';
import 'dart:math';
import 'dart:typed_data';

import 'package:convert/convert.dart';
import 'package:fixnum/fixnum.dart';
import 'package:grpc/grpc.dart';
import 'package:m10_sdk/metadata.dart';
import 'package:m10_sdk/oauth/oauth.dart';
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
import 'package:xid/xid.dart';

///
/// M10Sdk provides access to the M10 platform's ledger service APIs.
///
class M10Sdk {
  /// Creates a new M10Sdk instance.
  ///
  /// Parameters:
  /// * `signer` (required): The signer used to sign requests sent to the
  ///   platform.
  /// * `ledgers` (required): List of ledgers to connect to.
  /// * `disableTls` (optional): Whether to disable TLS connections.
  ///   Defaults to false.
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

  /// The signer used to sign requests sent to the platform.
  final Signing signer;

  final ChannelCredentials _credentials;
  final Random _random;
  final Map<String, AccountInfo> _accountInfoCache;

  late final _oauthManager = OauthManager();

  List<directory.Ledger> _ledgers;

  /// Updates the list of ledgers to connect to.
  set ledgers(List<directory.Ledger> newValue) {
    _ledgers = newValue;
  }

  /// Creates a copy of the current M10Sdk instance with a new signer.
  ///
  /// Parameters:
  /// * `signer` (required): The new signer to use.
  ///
  /// Returns:
  /// * A new [M10Sdk] instance with the provided signer.
  M10Sdk copyWithSigner(Signing signer) =>
      M10Sdk(signer: signer, ledgers: _ledgers);

  /// Cached service clients for interacting with the gRPC API.
  final Map<String, ServiceClient> serviceClients = {};

  /// Retrieves a [ClientChannel] instance for a specific operator.
  ///
  /// Searches for a ledger within the internal [_ledgers] list whose ID
  /// (case-insensitive) matches the provided `operator` string. If a matching
  /// ledger is found, creates a [ClientChannel] instance using the ledger's
  /// host address and configured credentials.
  ///
  /// Parameters:
  /// * `operator` (required): The platform operator (e.g. "m10").
  ///
  /// Returns:
  /// * A [ClientChannel] instance for interacting with the specified ledger.
  ///
  /// Throws:
  /// * An [ArgumentError] if no ledger matching the provided `operator` ID
  ///   is found. The error message provides details about the searched
  ///   ledgers and the missing ID.
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

  /// Retrieves a [ServiceClient] instance for a specific operator.
  ///
  /// Checks if a [ServiceClient] already exists for the given `operator`. If
  /// found, returns the existing instance. Otherwise, creates a new
  /// [ServiceClient] using a [ClientChannel] for the specified operator and
  /// caches it.
  ///
  /// Parameters:
  /// * `operator` (required): The platform operator (e.g. "m10").
  ///
  /// Returns:
  /// * The [ServiceClient] instance for the specified operator.
  ServiceClient getServiceClient(String operator) => serviceClients.putIfAbsent(
        operator,
        () => ServiceClient(getChannel(operator), [_oauthManager]),
      );

  /// Creates a [RequestEnvelope] containing a signed request message.
  ///
  /// This function takes a required `request` message (of type
  /// [GeneratedMessage]) and an optional `contextId` list. If the request is a
  /// [TransactionRequestPayload], it automatically sets the `nonce` and
  /// `timestamp` fields and populates the `contextId` with the provided list or
  /// an empty list if not provided.
  /// The function then serializes the request message and signs the payload
  /// using the configured signer. Finally, it creates and returns a
  /// [RequestEnvelope] containing the signed payload and signature.
  ///
  /// Parameters:
  /// * `request` (required): The [GeneratedMessage] object representing the
  ///   request.
  /// * `contextId` (optional): A list of integers representing the context ID.
  ///
  /// Returns:
  /// * A [Future<RequestEnvelope>] that completes with the constructed
  ///   [RequestEnvelope].
  ///
  /// Throws:
  /// * Any exceptions thrown by the signer during the signing process.
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

  /// Creates a new user account.
  ///
  /// Creates a new user account with the specified operator,
  /// optional context ID, and optional user ID. If no user
  /// ID is provided, a new one is generated.
  ///
  /// Parameters:
  /// * `operator` (required): The operator performing
  ///   the user creation.
  /// * `contextId` (optional): An optional context ID
  ///   for the transaction.
  /// * `id` (optional): An optional user ID. If provided,
  ///   it must be a valid UUID string.
  ///
  /// Returns:
  /// * A Future that completes with the created user ID
  ///   as a string.
  ///
  /// Throws:
  /// * An Exception if user creation fails.
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

  /// Retrieves information about an existing user account.
  ///
  /// Fetches details about a user account using the provided
  /// user ID and operator.
  ///
  /// Parameters:
  /// * `userId` (required): The user ID of the account
  ///   to retrieve. Must be a valid UUID string.
  /// * `operator` (required): The operator performing
  ///   the user retrieval.
  ///
  /// Returns:
  /// * A Future that completes with an `AccountSetDoc`
  ///   object containing the user information.
  ///
  /// Throws:
  /// * An Exception if user retrieval fails.
  Future<AccountSetDoc> getUser({
    required String userId,
    required String operator,
  }) async {
    final request = GetAccountSetRequest()..id = Uuid.parse(userId);
    final envelop = await requestEnvelope(request: request);
    final user = await getServiceClient(operator).query.getAccountSet(envelop);
    return AccountSetDoc(user);
  }

  /// Retrieves a list of existing users.
  ///
  /// Parameters:
  /// * `operator` (required): The operator performing
  ///   the user retrieval.
  /// * `queryFilter` (required): Filters user query by owner
  /// or name.
  ///
  /// Returns:
  /// * A Future that completes with a list of `AccountSetDoc`
  ///   objects containing the user information.
  ///
  /// Throws:
  /// * An Exception if user retrieval fails.
  Future<List<AccountSetDoc>> listUsers({
    required String operator,
    required QueryFilter filter,
  }) async {
    final request = ListAccountSetsRequest();
    switch (filter) {
      case final NameFilter nf:
        request.name = nf.value;
        break;
      case final OwnerFilter of:
        request.owner = base64Decode(of.value);
        break;
    }

    final envelope = await requestEnvelope(request: request);
    final response =
        await getServiceClient(operator).query.listAccountSets(envelope);
    return response.accountSets.map(AccountSetDoc.new).toList();
  }

  /// Updates an existing user account.
  ///
  /// Updates an existing user account with the specified
  /// user ID, operator, and optional properties.
  ///
  /// Parameters:
  /// * `userId` (required): The user ID of the account
  ///   to update. Must be a valid UUID string.
  /// * `operator` (required): The operator performing
  ///   the user update.
  /// * `defaultCurrency` (optional): The new default
  ///   currency for the user account.
  /// * `contextId` (optional): An optional context ID
  ///   for the transaction.
  /// * `accounts` (optional): A list of hex-encoded account
  ///   IDs as strings. Defaults to an empty list.
  ///
  /// Returns:
  /// * A Future that completes with the updated user ID
  ///   as a string.
  ///
  /// Throws:
  /// * An Exception if user update fails.
  Future<String> updateUser({
    required String userId,
    required String operator,
    String? defaultCurrency,
    String? contextId,
    List<String> accounts = const [],
  }) async {
    final dynamic builder = DocumentUpdate(AccountSet());
    builder.id(Uuid.parse(userId));
    if (defaultCurrency != null) {
      builder.defaultCurrency(defaultCurrency);
    }
    if (accounts.isNotEmpty) {
      builder.accounts(accounts.map(hex.decode).toList());
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

  /// Deletes an existing user account.
  ///
  /// Deletes a user account with the specified user ID
  /// and operator.
  ///
  /// Parameters:
  /// * `userId` (required): The user ID of the account
  ///   to delete. Must be a valid UUID string.
  /// * `operator` (required): The operator performing
  ///   the user deletion.
  /// * `contextId` (optional): An optional context ID
  ///   for the transaction.
  ///
  /// Returns:
  /// * A Future that completes with the deleted user ID
  ///   as a string.
  ///
  /// Throws:
  /// * An Exception if user deletion fails.
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

  /// Creates a new account.
  ///
  /// Creates a new account under the specified parent account,
  /// with the provided operator. Optional parameters allow
  /// setting the account name, public name, owner, profile
  /// image URL, and balance limit.
  ///
  /// Parameters:
  /// * `parentId` (required): The ID of the parent account.
  /// * `operator` (required): The operator performing the
  ///   account creation.
  /// * `name` (optional): The name of the account.
  /// * `publicName` (optional): The public name of the account.
  /// * `owner` (optional): The owner of the account (base64 encoded).
  /// * `profileImageUrl` (optional): The profile image URL of the account.
  /// * `balanceLimit` (optional): The balance limit for the account.
  /// * `contextId` (optional): An optional context ID for the transaction.
  ///
  /// Returns:
  /// * A Future that completes with the ID of the created account
  ///   as a string.
  ///
  /// Throws:
  /// * An Exception if account creation fails.
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

  /// Retrieves metadata for an existing account.
  ///
  /// Fetches details about an account using the provided ID and
  /// operator.
  ///
  /// Parameters:
  /// * `id` (required): The ID of the account to retrieve.
  /// * `operator` (required): The operator performing the account
  ///   metadata retrieval.
  ///
  /// Returns:
  /// * A Future that completes with an `AccountMetadataDoc` object
  ///   containing the account information.
  ///
  /// Throws:
  /// * An Exception if account metadata retrieval fails.
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

  /// Retrieves information about an existing account.
  ///
  /// Fetches information about an account using the provided ID and
  /// operator.
  ///
  /// Parameters:
  /// * `id` (required): The ID of the account to retrieve.
  /// * `operator` (required): The operator performing the account
  ///   information retrieval.
  ///
  /// Returns:
  /// * A Future that completes with an `AccountInfo` object containing
  ///   the account information.
  ///
  /// Throws:
  /// * An Exception if account information retrieval fails.
  Future<AccountInfo> getAccountInfo({
    required String id,
    required String operator,
  }) async {
    final request = GetAccountRequest()..id = hex.decode(id);
    final envelop = await requestEnvelope(request: request);
    return await getServiceClient(operator).query.getAccountInfo(envelop);
  }

  /// Retrieves information about an existing account, using cache
  /// if available.
  ///
  /// Attempts to retrieve account information from the cache. If not
  /// found, fetches information from the server and caches the result
  /// for future lookups.
  ///
  /// Parameters:
  /// * `id` (required): The ID of the account to retrieve.
  /// * `operator` (required): The operator performing the account
  ///   information retrieval.
  ///
  /// Returns:
  /// * A Future that completes with an `AccountInfo` object containing
  ///   the account information.
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

  /// Finds accounts owned by a specific public key.
  ///
  /// Retrieves a list of accounts owned by the public key provided in
  /// base64-encoded format. The platform operator performing the search
  /// is also required.
  ///
  /// Parameters:
  /// * `owner` (required): The base64-encoded public key of the accounts
  ///   to find.
  /// * `operator` (required): The platform operator (e.g. `"m10"`).
  ///
  /// Returns:
  /// * A Future that completes with a list of `AccountMetadataDoc` objects
  ///   representing the found accounts.
  ///
  /// Throws:
  /// * An Exception if account search fails.
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

  /// Finds accounts by name.
  ///
  /// Retrieves a list of accounts that match the provided name. The platform
  /// operator performing the search is also required.
  ///
  /// Parameters:
  /// * `name` (required): The name of the accounts to find.
  /// * `operator` (required): The platform operator (e.g. `"m10"`).
  ///
  /// Returns:
  /// * A Future that completes with a list of `AccountMetadataDoc` objects
  ///   representing the found accounts.
  ///
  /// Throws:
  /// * An Exception if account search fails.
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

  /// Updates an existing account.
  ///
  /// Updates an existing account with the specified ID, platform operator,
  /// and optional properties.
  ///
  /// Parameters:
  /// * `id` (required): The ID of the account to update.
  /// * `operator` (required): The platform operator (e.g. `"m10"`).
  /// * `name` (optional): The new name for the account.
  /// * `publicName` (optional): The new public name for the account.
  /// * `profileImageUrl` (optional): The new profile image URL for the
  ///   account.
  /// * `contextId` (optional): An optional context ID for the transaction.
  ///
  /// Returns:
  /// * A Future that completes with the transaction ID of the update
  ///   operation as an integer.
  ///
  /// Throws:
  /// * An Exception if account update fails.
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

  /// Retrieves a list of roles.
  ///
  /// Fetches a list of roles that match the provided name and
  /// platform operator.
  ///
  /// Parameters:
  /// * `name` (required): The name of the roles to find.
  /// * `operator` (required): The platform operator (e.g. "m10").
  ///
  /// Returns:
  /// * A Future that completes with a list of `RoleDoc` objects representing
  ///   the found roles.
  ///
  /// Throws:
  /// * An Exception if role search fails.
  Future<List<RoleDoc>> listRole({
    required String name,
    required String operator,
  }) async {
    final request = ListRolesRequest()..name = name;
    final envelop = await requestEnvelope(request: request);
    final bindings = await getServiceClient(operator).query.listRoles(envelop);

    return bindings.roles.map(RoleDoc.new).toList();
  }

  /// Retrieves information about a specific role.
  ///
  /// Fetches details about a role using the provided ID and platform
  /// operator.
  ///
  /// Parameters:
  /// * `id` (required): The ID of the role to retrieve. Must be a valid
  ///   UUID string.
  /// * `operator` (required): The platform operator (e.g. "m10").
  ///
  /// Returns:
  /// * A Future that completes with a `RoleDoc` object containing the role
  ///   information.
  ///
  /// Throws:
  /// * An Exception if role retrieval fails.
  Future<RoleDoc> getRole({
    required String id,
    required String operator,
  }) async {
    final request = GetRoleRequest()..id = hex.decode(id);
    final envelop = await requestEnvelope(request: request);
    final role = await getServiceClient(operator).query.getRole(envelop);

    return RoleDoc(role);
  }

  /// Creates a new role.
  ///
  /// Creates a new role with the specified name, platform operator,
  /// and optional properties. If no ID is provided, a new one is generated.
  /// The role owner can be either explicitly set or defaults to the platform
  /// operator's public key.
  ///
  /// Parameters:
  /// * `operator` (required): The platform operator (e.g. "m10").
  /// * `name` (required): The name of the role.
  /// * `id` (optional): An optional ID for the role. If provided, it must be a
  ///   valid UUID string.
  /// * `owner` (optional): The public key of the role owner in base64-encoded
  ///   format.
  /// * `rules` (optional): A list of `RuleDoc`
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

  /// Updates an existing role.
  ///
  /// Updates an existing role with the specified ID, platform operator,
  /// and optional properties.
  ///
  /// Parameters:
  /// * `id` (required): The ID of the role to update. Must be a valid UUID
  ///   string.
  /// * `operator` (required): The platform operator (e.g. "m10").
  /// * `name` (optional): The new name for the role.
  /// * `owner` (optional): The public key of the role owner in base64-encoded
  ///   format.
  /// * `rules` (optional): A list of `RuleDoc` objects defining the role's
  ///   permissions.
  /// * `contextId` (optional): An optional context ID for the transaction.
  ///
  /// Returns:
  /// * A Future that completes with the updated role ID as a string.
  ///
  /// Throws:
  /// * An Exception if role update fails.
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

  /// Deletes an existing role.
  ///
  /// Deletes a role with the specified ID and platform operator.
  ///
  /// Parameters:
  /// * `id` (required): The ID of the role to delete. Must be a valid UUID
  ///   string.
  /// * `operator` (required): The platform operator (e.g. "m10").
  /// * `contextId` (optional): An optional context ID for the transaction.
  ///
  /// Returns:
  /// * A Future that completes with the deleted role ID as a string.
  ///
  /// Throws:
  /// * An Exception if role deletion fails.
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

  /// Retrieves a list of role bindings.
  ///
  /// Fetches a list of role bindings that match the provided name and
  /// platform operator.
  ///
  /// Parameters:
  /// * `name` (required): The name of the role bindings to find.
  /// * `operator` (required): The platform operator.
  ///
  /// Returns:
  /// * A Future that completes with a list of `RoleBindingDoc` objects
  ///   representing the found role bindings.
  ///
  /// Throws:
  /// * An Exception if role binding search fails.
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

  /// Retrieves information about a specific role binding.
  ///
  /// Fetches details about a role binding using the provided ID and platform
  /// operator.
  ///
  /// Parameters:
  /// * `id` (required): The ID of the role binding to retrieve. Must be a valid
  ///   UUID string.
  /// * `operator` (required): The platform operator.
  ///
  /// Returns:
  /// * A Future that completes with a `RoleBindingDoc` object containing the
  ///   role binding information.
  ///
  /// Throws:
  /// * An Exception if role binding retrieval fails.
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

  /// Creates a new role binding.
  ///
  /// Creates a new role binding with the specified name, platform operator,
  /// role, and optional properties. If no ID is provided, a new one is
  /// generated.
  /// Subjects can be specified as a list of base64-encoded strings, and
  /// expressions can be provided for more granular permission control.
  ///
  /// Parameters:
  /// * `operator` (required): The platform operator (e.g., "m10").
  /// * `role` (required): The name of the role to bind.
  /// * `name` (required): The name of the role binding.
  /// * `id` (optional): An optional ID for the role binding. If provided, it
  ///   must be a valid UUID string.
  /// * `subjects` (optional): A list of base64-encoded subject identifiers to
  ///   associate with the role binding.
  /// * `expressions` (optional): A list of expressions defining additional
  ///   permissions for the role binding.
  /// * `contextId` (optional): An optional context ID for the transaction.
  ///
  /// Returns:
  /// * A Future that completes with the ID of the created role binding as a
  ///   string.
  ///
  /// Throws:
  /// * An Exception if role binding creation fails.
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

  /// Updates an existing role binding.
  ///
  /// Updates an existing role binding with the specified ID, platform operator,
  /// and optional properties.
  ///
  /// Parameters:
  /// * `id` (required): The ID of the role binding to update. Must be a valid
  ///   UUID string.
  /// * `operator` (required): The platform operator (e.g. "m10").
  /// * `subjects` (optional): A list of base64-encoded subject identifiers to
  ///   associate with the role binding.
  /// * `contextId` (optional): An optional context ID for the transaction.
  ///
  /// Returns:
  /// * A Future that completes with the updated role binding ID as a string.
  ///
  /// Throws:
  /// * An Exception if role binding update fails.
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

  /// Initiates or commits a transfer.
  ///
  /// This function allows you to initiate or commit a transfer
  /// consisting of multiple steps defined in a list of `TransferStepDoc`
  /// objects.
  ///
  /// Parameters:
  /// * `steps` (required): A list of `TransferStepDoc` objects defining the
  ///   transfer steps.
  /// * `operator` (required): The platform operator (e.g., "m10").
  /// * `contextId` (optional): An optional context ID for the transaction.
  /// * `commit` (optional, defaults to true): If true, the transfer is
  ///   committed.
  ///   If false, only the transfer is initiated and requires further steps for
  ///   completion.
  ///
  /// Returns:
  /// * A Future that completes with a `TransactionResponseDoc` object
  ///   containing information about the transaction.
  ///
  /// Throws:
  /// * An Exception if transfer initiation or commit fails.
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

  /// Creates a simplified transfer between accounts.
  ///
  /// This function provides a convenient way to create a transfer between two
  /// accounts with a specified amount, platform operator, and
  /// optional properties. It is equivalent to calling `transfer` with a single
  /// transfer step.
  ///
  /// Parameters:
  /// * `fromAccountId` (required): The ID of the account to transfer funds
  ///   from.
  /// * `toAccountId` (required): The ID of the account to transfer funds to.
  /// * `amount` (required): The amount of funds to transfer.
  /// * `operator` (required): The platform operator (e.g. "m10").
  /// * `memo` (optional): An optional memo attached to the transfer (text).
  /// * `attachments` (optional): A list of `Attachment` objects to associate
  ///   with the transfer.
  /// * `contract` (optional): A `Contract` object defining a related contract.
  ///   Sets the context ID to the contract ID if provided.
  /// * `contextId` (optional): An optional context ID for the transaction.
  /// * `selfTransfer` (optional): A `SelfTransfer` object.
  ///
  /// Returns:
  /// * A Future that completes with a `TransactionResponseDoc` object
  ///   containing information about the transfer transaction.
  ///
  /// Throws:
  /// * An Exception if transfer creation fails.
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

  /// Retrieves information about a transfer.
  ///
  /// Fetches details about a transfer using the provided transaction ID and
  /// platform operator.
  ///
  /// Parameters:
  /// * `txId` (required): The ID of the transfer transaction to retrieve.
  /// * `operator` (required): The platform operator (e.g. "m10").
  ///
  /// Returns:
  /// * A Future that completes with a `TransferDoc` object containing the
  ///   transfer information.
  ///
  /// Throws:
  /// * An Exception if transfer retrieval fails.
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

  /// Creates an encoded transfer envelope for an offline transfer.
  ///
  /// This function allows you to create an envelope representing a transfer
  /// that can be signed and sent offline. The envelope can then be submitted by
  /// the receiver with `submitSignedTransferEnvelope`.
  ///
  /// Parameters:
  /// * `fromAccountId` (required): The ID of the account to transfer funds
  ///   from.
  /// * `toAccountId` (required): The ID of the account to transfer funds to.
  /// * `amount` (required): The amount of funds to transfer.
  /// * `operator` (required): The platform operator (e.g. "m10").
  /// * `memo` (optional): An optional memo attached to the transfer (text).
  /// * `attachments` (optional): A list of `Attachment` objects to associate
  ///   with the transfer.
  /// * `contract` (optional): A `Contract` object defining a related contract.
  ///   Sets the context ID to the contract ID if provided.
  /// * `contextId` (optional): An optional context ID for the transaction.
  /// * `selfTransfer` (optional): A `SelfTransfer` object.
  ///
  /// Returns:
  /// * A Future that completes with a string containing the base64-encoded
  ///   transfer envelope.
  ///
  /// Throws:
  /// * An Exception if transfer envelope creation fails.
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

  /// Creates an encoded transfer envelope from a CreateTransfer object.
  ///
  /// This function takes a `CreateTransfer` object and creates an encoded
  /// envelope for an offline transfer. The envelope can be submitted by the
  /// receiver with `submitSignedTransferEnvelope`.
  ///
  /// Parameters:
  /// * `createTransfer` (required): The `CreateTransfer` object representing
  ///   the transfer to be encoded.
  /// * `contextId` (optional): An optional context ID for the transaction.
  ///
  /// Returns:
  /// * A Future that completes with a string containing the base64-encoded
  ///   transfer envelope.
  ///
  /// Throws:
  /// * An Exception if transfer envelope creation fails.
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

  /// Enhances transfer steps with additional account information.
  ///
  /// This function enriches a transfer by fetching additional details about
  /// the accounts involved in each transfer step. The enhanced information
  /// includes account details for both the sender and recipient, as well as
  /// their respective parent accounts (if applicable).
  ///
  /// Parameters:
  /// * `transfer` (required): The `TransferDoc` object representing the
  ///   transfer to enhance.
  ///
  /// Returns:
  /// * A Future that completes with a list of `EnhancedTransferStepDoc` objects
  ///   containing the enhanced transfer steps.
  Future<List<EnhancedTransferStepDoc>> enhance(TransferDoc transfer) async =>
      enhanceTransferSteps(
        transfer.model.transferSteps,
        operator: transfer.operator,
      );

  /// Enhances a list of transfer steps with additional account information.
  ///
  /// This function enriches a list of transfer steps by fetching additional
  /// details about the accounts involved in each step. The enhanced information
  /// includes account details for both the sender and recipient, as well as
  /// their respective parent accounts (if applicable).
  ///
  /// Parameters:
  /// * `steps` (required): A list of `TransferStep` objects to enhance.
  /// * `operator` (required): The platform operator performing the account
  ///   lookups.
  ///
  /// Returns:
  /// * A Future that completes with a list of `EnhancedTransferStepDoc` objects
  ///   containing the enhanced transfer steps.
  Future<List<EnhancedTransferStepDoc>> enhanceTransferSteps(
    List<TransferStep> steps, {
    required String operator,
  }) async =>
      Stream.fromIterable(steps)
          .asyncMap((step) => enhanceTransferStep(operator, step))
          .toList();

  /// Enhances a single transfer step with additional account information.
  ///
  /// This function enriches a transfer step by fetching details about the
  /// sender and recipient accounts, as well as their respective parent
  /// accounts (if applicable).
  ///
  /// Parameters:
  /// * `operator` (required): The platform operator (e.g. "m10").
  /// * `step` (required): The `TransferStep` object to enhance.
  ///
  /// Returns:
  /// * A Future that completes with an `EnhancedTransferStepDoc` object
  ///   containing the enhanced transfer step information.
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

  /// Retrieves a list of transfers.
  ///
  /// Fetches a list of transfers based on specified filters and pagination
  /// parameters.
  /// You can filter by account ID or context ID, but not both simultaneously.
  /// The results are paginated with `minTxId`, `maxTxId`, and `limit`
  /// parameters.
  ///
  /// Parameters:
  /// * `operator` (required): The platform operator (e.g. "m10").
  /// * `accountId` (optional): The ID of the account to filter transfers by.
  /// * `contextId` (optional): The context ID to filter transfers by.
  /// * `maxTxId` (optional): The maximum transaction ID to include in the
  ///   results.
  /// * `minTxId` (optional, default: 0): The minimum transaction ID to include
  ///   in the results.
  /// * `limit` (optional, default: 10): The maximum number of transfers to
  ///   return.
  /// * `includeChildAccounts` (optional, default: false): Whether to include
  ///   transfers from child accounts.
  ///
  /// Returns:
  /// * A Future that completes with a list of `TransferDoc` objects
  ///   representing the retrieved transfers.
  ///
  /// Throws:
  /// * An ArgumentError if both `accountId` and `contextId` are provided.
  /// * An Exception if transfer listing fails.
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

  /// Retrieves information about a transaction.
  ///
  /// Fetches details about a transaction using the provided transaction ID and
  /// platform operator.
  ///
  /// Parameters:
  /// * `operator` (required): The platform operator (e.g. "m10").
  /// * `txId` (required): The ID of the transfer transaction to retrieve.
  ///
  /// Returns:
  /// * A Future that completes with a `transactionDoc` object containing the
  ///   transaction information.
  ///
  /// Throws:
  /// * An Exception if transaction retrieval fails.
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

  /// Retrieves a list of transactions for a specific context.
  ///
  /// Fetches a list of transactions associated with a given context ID.
  /// Pagination is supported through `minTxId`, `maxTxId`, and `limit`
  /// parameters.
  ///
  /// Parameters:
  /// * `operator` (required): The platform operator (e.g. "m10").
  /// * `contextId` (required): The context ID to filter transactions by.
  /// * `minTxId` (optional, default: 0): The minimum transaction ID to include
  ///   in the results.
  /// * `maxTxId` (optional, default: 0): The maximum transaction ID to include
  ///   in the results.
  /// * `limit` (optional, default: 10): The maximum number of transactions to
  ///   return.
  ///
  /// Returns:
  /// * A Future that completes with a list of `TransactionDoc` objects
  ///   representing the retrieved transactions.
  ///
  /// Throws:
  /// * An Exception if transaction listing fails.
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

  /// Groups transactions for a specific account.
  ///
  /// Fetches transactions for a given account ID and groups them. Pagination
  /// is controlled by `limitGroups` parameter.
  ///
  /// Parameters:
  /// * `operator` (required): The platform operator (e.g., "m10").
  /// * `accountId` (required): The ID of the account to group transactions for.
  /// * `minTxId` (optional, default: 0): The minimum transaction ID to include
  ///   in the results.
  /// * `maxTxId` (optional, default: 0): The maximum transaction ID to include
  ///   in the results.
  /// * `limitGroups` (optional, default: 10): The maximum number of transaction
  ///   groups to return.
  ///
  /// Returns:
  /// * A Future that completes with a list of lists of `TransactionDoc`
  ///   objects.
  ///   The outer list represents transaction groups, and the inner list
  ///   contains transactions within each group.
  ///
  /// Throws:
  /// * An Exception if transaction grouping fails.
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

  /// Initiates or commits an action on an account.
  ///
  /// This function allows you to initiate or commit an action on a specified
  /// account. The action is identified by its name and requires a source
  /// account and a target (either another account or all accounts). An
  /// optional payload can be provided for the action. The context ID can be
  /// used for transaction tracking.
  ///
  /// Parameters:
  /// * `operator` (required): The platform operator (e.g. "m10").
  /// * `name` (required): The name of the action to invoke.
  /// * `fromAccountId` (required): The ID of the account initiating the action.
  /// * `targetAccountId` (optional): The ID of the target account for the
  ///   action.
  /// * `targetAll` (optional, default: false): Whether to target all accounts.
  ///   If true, `targetAccountId` must be omitted.
  /// * `payload` (optional, default: []): An optional list of bytes
  ///   representing the action payload.
  /// * `contextId` (optional): An optional context ID for the transaction.
  ///
  /// Returns:
  /// * A Future that completes with a `TransactionResponseDoc` object
  ///   containing information about the action transaction.
  ///
  /// Throws:
  /// * An Exception if a required parameter is missing (e.g., target
  ///   information).
  /// * An Exception if action invocation fails.
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

  /// Retrieves information about an action.
  ///
  /// Fetches details about an action using the provided transaction ID and
  /// platform operator.
  ///
  /// Parameters:
  /// * `txId` (required): The ID of the transaction associated with the action.
  /// * `operator` (required): The platform operator (e.g., "m10").
  ///
  /// Returns:
  /// * A Future that completes with an `ActionDoc` object containing the action
  ///   information.
  ///
  /// Throws:
  /// * An Exception if action retrieval fails.
  Future<ActionDoc> getAction({
    required int txId,
    required String operator,
  }) async {
    final request = GetActionRequest()..txId = Int64(txId);
    final envelop = await requestEnvelope(request: request);
    final action = await getServiceClient(operator).query.getAction(envelop);

    return ActionDoc(action);
  }

  /// Retrieves a list of actions.
  ///
  /// Fetches a list of actions based on specified filters and pagination
  /// parameters.
  /// You can filter by action name and either account ID or context ID, but not
  /// both simultaneously. The results are paginated with `minTxId`, `maxTxId`,
  /// and `limit` parameters.
  ///
  /// Parameters:
  /// * `operator` (required): The platform operator (e.g. "m10").
  /// * `name` (required): The name of the action to filter by.
  /// * `accountId` (optional): The ID of the account to filter actions by.
  /// * `contextId` (optional): The context ID to filter actions by.
  /// * `minTxId` (optional, default: 0): The minimum transaction ID to include
  ///   in the results.
  /// * `maxTxId` (optional, default: 0): The maximum transaction ID to include
  ///   in the results.
  /// * `limit` (optional, default: 10): The maximum number of actions to
  ///   return.
  ///
  /// Returns:
  /// * A Future that completes with a list of `ActionDoc` objects representing
  ///   the retrieved actions.
  ///
  /// Throws:
  /// * An Exception if both `accountId` and `contextId` are provided.
  /// * An Exception if a required filter (either `accountId` or `contextId`) is
  ///   missing.
  /// * An Exception if action listing fails.
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

  /// Creates a new ledger account.
  ///
  /// Creates a new ledger account under a specified parent account, with an
  /// optional balance limit.
  ///
  /// Parameters:
  /// * `parentId` (required): The ID of the parent account.
  /// * `operator` (required): The platform operator (e.g. "m10").
  /// * `balanceLimit` (optional): The optional balance limit for the account.
  /// * `contextId` (optional): An optional context ID for the transaction.
  ///
  /// Returns:
  /// * A Future that completes with the ID of the created ledger account as a
  ///   hex-encoded string.
  ///
  /// Throws:
  /// * An Exception if account creation fails.
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

  /// Retrieves indexed account information.
  ///
  /// Fetches detailed information about a ledger account, including its indexed
  /// properties.
  ///
  /// Parameters:
  /// * `accountId` (required): The ID of the account to retrieve.
  /// * `operator` (required): The platform operator (e.g. "m10").
  ///
  /// Returns:
  /// * A Future that completes with an `IndexedAccount` object containing the
  ///   account information.
  ///
  /// Throws:
  /// * An Exception if account retrieval fails.
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

  /// Observes transfers for a list of accounts.
  ///
  /// Establishes a stream of transfer updates for the specified accounts.
  /// Optionally, you can provide a starting point (`startingFrom`) to resume
  /// the stream from a specific transaction ID.
  ///
  /// Parameters:
  /// * `operator` (required): The platform operator (e.g. "m10").
  /// * `accounts` (required): A list of account IDs to observe transfers for.
  /// * `startingFrom` (optional): An optional starting transaction ID to resume
  ///   the stream from.
  ///
  /// Returns:
  /// * A Future that completes with a Stream of lists of `TransferResultDoc`
  ///   objects. Each list represents a batch of received transfer updates.
  ///
  /// Throws:
  /// * An Exception if transfer observation fails.
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

  /// Observes actions for a specific action name and accounts.
  ///
  /// Establishes a stream of updates for a particular action identified by its
  /// name. You can specify a list of accounts to filter the observed actions.
  /// Optionally, you can provide a starting point (`startingFrom`) to resume
  /// the stream from a specific transaction ID.
  ///
  /// Parameters:
  /// * `operator` (required): The platform operator (e.g., "m10").
  /// * `name` (required): The name of the action to observe.
  /// * `accounts` (required): A list of account IDs to filter the observed
  ///   actions for.
  /// * `startingFrom` (optional): An optional starting transaction ID to resume
  ///   the stream from.
  ///
  /// Returns:
  /// * A Future that completes with a Stream of lists of `ActionDoc` objects.
  ///   Each list represents a batch of received action updates.
  ///
  /// Throws:
  /// * An Exception if action observation fails.
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

  /// Creates a transferable offline token.
  ///
  /// This function creates a transferable offline token by transferring funds
  /// from the specified ledger account. The token can be later used for offline
  /// payments or transfers.
  ///
  /// Parameters:
  /// * `accountId` (required): The ID of the ledger account to transfer funds
  ///   from.
  /// * `value` (required): The value of the token.
  /// * `operator` (required): The platform operator (e.g. "m10").
  /// * `address` (optional): An optional address for the token recipient.
  ///   If not provided, the local public key will be used.
  /// * `contextId` (optional): An optional context ID for the transaction.
  ///
  /// Returns:
  /// * A tuple containing the `TransactionResponse` and the created
  ///   `OfflineToken` object.
  ///
  /// Throws:
  /// * An Exception if token creation fails.
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

  /// Credits a redeemable offline token to a ledger account.
  ///
  /// This function credits the value of a redeemable offline token to the
  /// specified ledger account.
  ///
  /// Parameters:
  /// * `token` (required): The redeemable token to be credited.
  /// * `accountId` (required): The ID of the ledger account to credit the
  ///   token to.
  /// * `operator` (required): The platform operator (e.g. "m10").
  /// * `contextId` (optional): An optional context ID for the transaction.
  ///
  /// Returns:
  /// * The transaction ID of the token redemption.
  ///
  /// Throws:
  /// * An Exception if token redemption fails.
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

  /// Issues a signed redeemable token.
  ///
  /// Creates a signed redeemable token from an existing transferable token. The
  /// token is issued to a specified address with a given currency code and
  /// value. Multiple input tokens can be used as sources for the new token.
  ///
  /// Parameters:
  /// * `toAddress` (required): The address of the token recipient.
  /// * `currencyCode` (required): The currency code of the token.
  /// * `value` (required): The value of the token.
  /// * `tokenInputs` (required): A list of `RedeemableToken_TokenInput` objects
  ///   representing the input tokens used for issuing the new token.
  ///
  /// Returns:
  /// * The newly issued `RedeemableToken` object.
  ///
  /// Throws:
  /// * An Exception if token issuance fails.
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

  /// Observes payment requests for a list of accounts.
  ///
  /// Establishes a stream of payment request updates for the specified
  /// accounts.
  /// Optionally, you can provide a starting point (`startingFrom`) to resume
  /// the stream from a specific transaction ID.
  ///
  /// Parameters:
  /// * `operator` (required): The platform operator (e.g. "m10").
  /// * `accounts` (required): A list of account IDs to observe requests for.
  /// * `startingFrom` (optional): An optional starting transaction ID to resume
  ///   the stream from.
  ///
  /// Returns:
  /// * A Future that completes with a Stream of lists of `PaymentRequestDoc`
  ///   objects.
  ///   Each list represents a batch of received payment request updates.
  ///
  /// Throws:
  /// * An Exception if request observation fails.
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
        final requests = [txDocs].map(PaymentRequestDoc.from).nonNulls.toList();
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

  /// Observes resources based on a query expression.
  ///
  /// Establishes a stream of updates for resources matching the provided
  /// collection name, expression, and optional variables. Optionally, you can
  /// specify a starting point (`startingFrom`) to resume the stream from a
  /// specific transaction ID.
  ///
  /// Parameters:
  /// * `operator` (required): The platform operator (e.g. "m10").
  /// * `expression` (required): The query expression to filter resources.
  /// * `collection` (required): The name of the resource collection to observe.
  /// * `variables` (optional, default: empty map): An optional map of variables
  ///   to be used in the expression.
  /// * `startingFrom` (optional): An optional starting transaction ID to resume
  ///   the stream from.
  ///
  /// Returns:
  /// * A Future that completes with a Stream of lists of `ResourceResultDoc`
  ///   objects.
  ///   Each list represents a batch of updates for matching resources.
  ///
  /// Throws:
  /// * An Exception if resource observation fails.
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

  /// Observes transaction metrics for a list of accounts.
  ///
  /// Establishes a stream of transaction metric updates for the specified
  /// accounts.
  /// Optionally, you can provide a starting point (`startingFrom`) to resume
  /// the stream from a specific transaction ID.
  ///
  /// Parameters:
  /// * `operator` (required): The platform operator (e.g., "m10").
  /// * `accounts` (required): A list of account IDs to observe metrics for.
  /// * `startingFrom` (optional): An optional starting transaction ID to resume
  ///   the stream from.
  ///
  /// Returns:
  /// * A Future that completes with a Stream of `TransactionMetricsDoc`
  ///   objects.
  ///   Each object represents a batch of transaction metric updates.
  ///
  /// Throws:
  /// * An Exception if metric observation fails.
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

  /// Creates a new contract builder instance.
  ///
  /// This function returns a `ContractBuilder` object which can be used to
  /// construct contract definitions for various purposes.
  ///
  /// Returns:
  /// * A new `ContractBuilder` instance.
  ContractBuilder createContract() => ContractBuilder();

  /// Updates the status of a ledger account.
  ///
  /// This function updates the frozen status of a ledger account. If the
  /// current frozen status differs from the requested one, a transaction is
  /// initiated to change the status.
  ///
  /// Parameters:
  /// * `id` (required): The ID of the ledger account to update.
  /// * `operator` (required): The platform operator (e.g. "m10").
  /// * `frozen` (required): The new frozen status for the account.
  /// * `contextId` (optional): An optional context ID for the transaction.
  ///
  /// Throws:
  /// * An Exception if account status update fails.
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

  /// Generates a random context ID.
  ///
  /// This function generates a random hex-encoded string of length 32 bytes,
  /// which can be used as a context ID for transactions.
  ///
  /// Returns:
  /// * A random hex-encoded string representing the context ID.
  String get randomContextId {
    final random = Random.secure();
    return hex.encode(List<int>.generate(32, (i) => random.nextInt(256)));
  }
}

/// Represents different resource types within the system.
enum Resource { user, id }

extension ResourceExt on Resource {
  /// Returns the string value of the resource type.
  String get value => toString().split('.').last;
}

extension M10Actions on M10Sdk {
  static const requestActionName = 'm10.Request';

  /// Initiates a payment request.
  ///
  /// Creates a payment request by invoking the `'m10.Request'` action with
  /// the specified transfer request. The request is sent from the recipient
  /// account to the sender account. A pending status is set for the payment
  /// request.
  ///
  /// Parameters:
  /// * `operator` (required): The platform operator (e.g. "m10").
  /// * `transferRequest` (required): The transfer object to be included in the
  ///   payment request.
  /// * `contextId` (optional): An optional context ID for the transaction.
  ///
  /// Returns:
  /// * A `TransactionResponseDoc` object representing the created payment
  ///   request transaction.
  ///
  /// Throws:
  /// * An Exception if payment request creation fails.
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

  /// Cancels a payment request.
  ///
  /// Cancels an existing payment request by invoking the `'m10.Request'` action
  /// with a `CANCELED` status.
  ///
  /// Parameters:
  /// * `operator` (required): The platform operator (e.g. "m10").
  /// * `fromAccountId` (required): The ID of the from account of the request.
  /// * `targetAccountId` (required): The ID of the target account of the
  ///   request.
  /// * `contextId` (required): The context ID of the original payment request.
  ///
  /// Returns:
  /// * A `TransactionResponseDoc` object representing the cancellation
  ///   transaction.
  ///
  /// Throws:
  /// * An Exception if payment request cancellation fails.
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

  /// Declines a payment request.
  ///
  /// Declines an existing payment request by invoking the `'m10.Request'`
  /// action with a `DECLINED` status.
  ///
  /// Parameters:
  /// * `operator` (required): The platform operator (e.g. "m10").
  /// * `fromAccountId` (required): The ID of the from account of the request.
  /// * `targetAccountId` (required): The ID of the target account of the
  ///   request.
  /// * `contextId` (required): The context ID of the original payment request.
  ///
  /// Returns:
  /// * A `TransactionResponseDoc` object representing the decline transaction.
  ///
  /// Throws:
  /// * An Exception if payment request decline fails.
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

  /// Retrieves a list of payment requests for a specific account.
  ///
  /// Fetches a paginated list of payment requests associated with a given
  /// account.
  /// Optionally, you can specify a range of transaction IDs (`minTxId` and
  /// `maxTxId`) and a maximum number of requests to retrieve (`limit`).
  ///
  /// This function iterates through transaction groups until the specified
  /// limit is reached or there are no more requests within the provided range.
  ///
  /// Parameters:
  /// * `operator` (required): The platform operator (e.g., "m10").
  /// * `accountId` (required): The ID of the account to retrieve requests for.
  /// * `minTxId` (optional, default: 0): The minimum transaction ID to start
  ///   the search from.
  /// * `maxTxId` (optional, default: 0): The maximum transaction ID to limit
  ///   the search to (exclusive).
  ///   Set to 0 to include all requests up to the highest ID.
  /// * `limit` (optional, default: 10): The maximum number of payment requests
  ///   to retrieve.
  ///
  /// Returns:
  /// * A list of `PaymentRequestDoc` objects representing the retrieved payment
  ///   requests. The list is sorted by timestamp in descending order (newest
  ///   first).
  ///
  /// Throws:
  /// * An Exception if payment request list retrieval fails.
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
      final requests = txGroups.map(PaymentRequestDoc.from).nonNulls.toList();

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

  /// Retrieves a list of payment requests for a specific account.
  ///
  /// Returns:
  /// * [ChainInfo] object containing current block height.
  ///
  /// Throws:
  /// * An Exception if chain info retrieval fails.
  Future<ChainInfo> getChainInfo({required String operator}) =>
      getServiceClient(operator).query.getChainInfo(Empty());
}
