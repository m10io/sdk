import 'package:grpc/grpc.dart';
import 'package:m10_sdk/library.dart';
import 'package:m10_sdk/src/generated/directory/api.pbgrpc.dart';
import 'package:m10_sdk/src/generated/google/protobuf/empty.pb.dart';
import 'package:uuid/uuid.dart';

abstract class TokenProvider {
  String get subject;
  Future<String> get accessToken;
}

class M10Directory {
  M10Directory(String host,
      {TokenProvider? tokenProvider, bool disableTls = false, int port = 443})
      : _tokenProvider = tokenProvider,
        _client = DirectoryServiceClient(ClientChannel(host,
            port: port,
            options: ChannelOptions(
                credentials: disableTls
                    ? const ChannelCredentials.insecure()
                    : const ChannelCredentials.secure())));

  final DirectoryServiceClient _client;
  final TokenProvider? _tokenProvider;

  Future<List<Ledger>> listLedgers() async =>
      (await _client.listLedgers(Empty())).ledgers;

  createAlias({
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
    await _client.createAlias(request, options: await options);
  }

  Future<bool> checkAlias(String handle) async {
    try {
      await _client.checkAlias(CheckAliasRequest(handle: handle));
      return true;
    } on GrpcError {
      return false;
    }
  }

  Future<AliasDoc?> getAlias(String handle) async {
    final aliases = await searchAliases(handle, 1);
    try {
      return aliases.firstWhere((alias) => alias.handle == handle);
    } catch (e) {
      return null;
    }
  }

  Future<Iterable<AliasDoc>> findAliasesOfUser([int pageSize = 20]) async {
    if (_tokenProvider == null) throw Exception("missing token provider");
    final subject = _tokenProvider!.subject;
    final request = SearchAliasesRequest(pageSize: pageSize, subject: subject);
    final response =
        await _client.searchAliases(request, options: await options);
    final entries = response.aliases.map((alias) {
      return AliasDoc(alias);
    });
    return entries;
  }

  Future<Iterable<AliasDoc>> searchAliases(
    String handlePrefix, [
    int pageSize = 20,
  ]) async {
    final request =
        SearchAliasesRequest(handlePrefix: handlePrefix, pageSize: pageSize);
    final response =
        await _client.searchAliases(request, options: await options);
    final entries = response.aliases.map((alias) {
      return AliasDoc(alias);
    });
    return entries;
  }

  Future<ObjectUrlResponse> createObjectUrl() async {
    return await _client.createObjectUrl(Empty(), options: await options);
  }

  Future<ObjectUrlResponse> createImageUrl(String mimeType) async {
    return await _client
        .createImageUrl(CreateImageUrlRequest(mimeType: mimeType));
  }

  Future<ObjectUrlResponse> getObjectUrl(String objectId) async {
    return await _client.getObjectUrl(GetObjectUrlRequest(objectId: objectId),
        options: await options);
  }

  Future<CallOptions> get options async {
    if (_tokenProvider == null) throw Exception("missing token provider");
    final accessToken = await _tokenProvider!.accessToken;
    return CallOptions(metadata: {'Authorization': 'Bearer ${accessToken}'});
  }
}
