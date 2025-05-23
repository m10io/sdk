import 'package:grpc/grpc.dart';
import 'package:m10_sdk/library.dart';
import 'package:m10_sdk/src/generated/directory/api.pbgrpc.dart';
import 'package:m10_sdk/src/generated/google/protobuf/empty.pb.dart';
import 'package:uuid/uuid.dart';

/// Abstract base class for token providers.
///
/// Provides the subject of the token and a method to obtain the access token.
///
/// Properties:
/// * `subject`: The subject of the token.
///
/// Methods:
/// * `accessToken`: Returns a Future that completes with the access token as a
///   string.
abstract class TokenProvider {
  String get subject;
  Future<String> get accessToken;
}

/// Represents a client for interacting with the M10 directory service.
///
/// This class provides methods for various directory-related operations,
/// including listing ledgers, creating aliases, checking alias existence,
/// retrieving alias details, searching for aliases, and creating and
/// retrieving object URLs.
///
/// M10Directory establishes a connection to the directory service using
/// the provided `host`, `port`, and optional `tokenProvider` and
/// `disableTls`.
class M10Directory {
  /// Creates an [M10Directory] instance for interacting with the directory
  /// service.
  ///
  /// Parameters:
  /// * `host` (required): The hostname or IP address of the directory service.
  /// * `tokenProvider` (optional): A [TokenProvider] instance used to obtain
  ///   access tokens for authorized operations. If not provided, operations
  ///   requiring tokens will throw an exception.
  /// * `disableTls` (optional): A boolean flag indicating whether to disable
  ///   TLS security for the connection. Defaults to `false` (secure
  ///   connection).
  /// * `port` (optional): The port number to connect to on the directory
  ///   service host. Defaults to 443 for HTTPS connections.
  ///
  /// Throws:
  /// * An [ArgumentError] if an invalid port number is provided.
  M10Directory(
    String host, {
    TokenProvider? tokenProvider,
    bool disableTls = false,
    int port = 443,
  })  : _tokenProvider = tokenProvider,
        _client = DirectoryServiceClient(
          ClientChannel(
            host,
            port: port,
            options: ChannelOptions(
              credentials: disableTls
                  ? const ChannelCredentials.insecure()
                  : const ChannelCredentials.secure(),
            ),
          ),
        );

  final DirectoryServiceClient _client;
  final TokenProvider? _tokenProvider;

  /// Retrieves a list of available ledgers from the directory service.
  ///
  /// This method makes a gRPC call to the directory service to list all
  /// available ledgers.
  ///
  /// Returns:
  /// * A [Future] that completes with a list of [Ledger] objects.
  Future<List<Ledger>> listLedgers() async =>
      (await _client.listLedgers(Empty())).ledgers;

  /// Creates a new alias in the directory service.
  ///
  /// This method creates a new alias with the specified details.
  ///
  /// Parameters:
  /// * `handle` (required): The unique handle for the alias.
  /// * `displayName` (required): The display name for the alias.
  /// * `accountSetId` (required): The ID of the account set associated with the
  ///   alias.
  /// * `aliasType` (required): The type of the alias (e.g. `EMAIL`, `PHONE`).
  /// * `operator` (required): The platform operator (e.g. "m10").
  Future<void> createAlias({
    required String handle,
    required String displayName,
    required String accountSetId,
    required Alias_Type aliasType,
    required String operator,
  }) async {
    final request = Alias(
      handle: handle,
      displayName: displayName,
      accountSetId: Uuid.parse(accountSetId),
      aliasType: aliasType,
      operator: operator,
    );
    await _client.createAlias(request, options: await _fetchOptions());
  }

  /// Checks if an alias with the given handle exists.
  ///
  /// This method checks the existence of an alias based on its handle.
  ///
  /// Parameters:
  /// * `handle`: The handle of the alias to check.
  ///
  /// Returns:
  /// * A [Future] that completes with `true` if the alias exists, `false`
  ///   otherwise.
  Future<bool> checkAlias(String handle) async {
    try {
      await _client.checkAlias(CheckAliasRequest(handle: handle));
      return true;
    } on GrpcError {
      return false;
    }
  }

  /// Retrieves an alias by its handle.
  ///
  /// This method searches for an alias with the given handle and returns its
  /// details.
  ///
  /// Parameters:
  /// * `handle`: The handle of the alias to retrieve.
  ///
  /// Returns:
  /// * A [Future] that completes with the [AliasDoc] object if found, or `null`
  ///   if not found.
  Future<AliasDoc?> getAlias(String handle) async {
    final aliases = await searchAliases(handle, 1);
    try {
      return aliases.firstWhere((alias) => alias.handle == handle);
    } catch (e) {
      return null;
    }
  }

  /// Searches for aliases associated with the current user.
  ///
  /// This method retrieves a list of aliases associated with the current user
  /// (identified by the subject from the token provider).
  ///
  /// Parameters:
  /// * `pageSize` (optional): The maximum number of aliases to return in the
  ///   response. Defaults to 20.
  ///
  /// Returns:
  /// * A [Future] that completes with an [Iterable] containing AliasDoc objects
  ///   for the found aliases.
  ///
  /// Throws:
  /// * An [Exception] if the token provider is missing.
  Future<Iterable<AliasDoc>> findAliasesOfUser([int pageSize = 20]) async {
    if (_tokenProvider == null) throw Exception('missing token provider');
    final subject = _tokenProvider.subject;
    final request = SearchAliasesRequest(pageSize: pageSize, subject: subject);
    final response =
        await _client.searchAliases(request, options: await _fetchOptions());
    final entries = response.aliases.map(AliasDoc.new);
    return entries;
  }

  /// Searches for aliases based on a handle prefix.
  ///
  /// This method retrieves a list of aliases whose handles start with the
  /// provided prefix.
  ///
  /// Parameters:
  /// * `handlePrefix` (required): The prefix string to search for in alias
  ///   handles.
  /// * `pageSize` (optional): The maximum number of aliases to return in the
  ///   response. Defaults to 20.
  ///
  /// Returns:
  /// * A [Future] that completes with an [Iterable] containing [AliasDoc]
  ///   objects for the found aliases.
  Future<Iterable<AliasDoc>> searchAliases(
    String handlePrefix, [
    int pageSize = 20,
  ]) async {
    final request =
        SearchAliasesRequest(handlePrefix: handlePrefix, pageSize: pageSize);
    final response =
        await _client.searchAliases(request, options: await _fetchOptions());
    final entries = response.aliases.map(AliasDoc.new);
    return entries;
  }

  /// Creates a URL for uploading an object.
  ///
  /// This method generates a pre-signed URL for uploading an object to the
  /// specified storage location.
  ///
  /// Returns:
  /// * A [Future] that completes with an [ObjectUrlResponse] containing the
  ///   generated URL and other metadata.
  Future<ObjectUrlResponse> createObjectUrl() async =>
      await _client.createObjectUrl(
        Empty(),
        options: await _fetchOptions(),
      );

  /// Creates a URL for uploading an image.
  ///
  /// This method generates a pre-signed URL for uploading an image to the
  /// specified storage location.
  ///
  /// Parameters:
  /// * `mimeType`: The MIME type of the image to be uploaded.
  ///
  /// Returns:
  /// * A [Future] that completes with an [ObjectUrlResponse] containing the
  /// generated URL and other metadata.
  Future<ObjectUrlResponse> createImageUrl(String mimeType) async =>
      await _client.createImageUrl(
        CreateImageUrlRequest(mimeType: mimeType),
        options: await _fetchOptions(),
      );

  /// Retrieves a URL for downloading an object.
  ///
  /// This method fetches a pre-signed URL for downloading an object from the
  /// specified storage location.
  ///
  /// Parameters:
  /// * `objectId`: The ID of the object to download.
  ///
  /// Returns:
  /// * A [Future] that completes with an [ObjectUrlResponse] containing the
  ///   object URL and other metadata.
  Future<ObjectUrlResponse> getObjectUrl(String objectId) async =>
      await _client.getObjectUrl(
        GetObjectUrlRequest(objectId: objectId),
        options: await _fetchOptions(),
      );

  Future<CallOptions> _fetchOptions() async {
    if (_tokenProvider == null) throw Exception('missing token provider');
    final accessToken = await _tokenProvider.accessToken;
    return CallOptions(metadata: {'Authorization': 'Bearer $accessToken'});
  }
}
