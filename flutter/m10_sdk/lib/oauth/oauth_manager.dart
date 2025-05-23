import 'dart:io';

import 'package:dio/dio.dart';
import 'package:grpc/grpc.dart';
import 'package:yaml/yaml.dart';

const _baseUrl = 'https://login-uat.fisglobal.com/idp/tikkabank243/rest/1.0';
const _configFilePath = 'oauth_config.yaml';

///
/// Interceptor which handles fetching an oauth access token and adding them to
/// gRPC requests.
///
/// Config defining client_id and client_secret must be added at
/// oauth_config.yaml, or this interceptor will do nothing.
/// See oauth_config.example.yaml for a template.
///
class OauthManager implements ClientInterceptor {
  OauthManager({
    Dio? dio,
  }) : _dio = dio ?? Dio()
          ..options = BaseOptions(
            baseUrl: _baseUrl,
            headers: {
              'X-SunGard-IdP-API-Key': 'SunGard-IdP-UI',
              'Accept': Headers.jsonContentType,
            },
          ) {
    _init();
  }

  final Dio _dio;
  String? _clientId;
  String? _clientSecret;
  String? _accessToken;

  /// Passes streaming requests without modification.
  @override
  ResponseStream<R> interceptStreaming<Q, R>(
    ClientMethod<Q, R> method,
    Stream<Q> requests,
    CallOptions options,
    ClientStreamingInvoker<Q, R> invoker,
  ) =>
      invoker(method, requests, options);

  /// Adds the access token to the gRPC call's metadata if the token is present.
  @override
  ResponseFuture<R> interceptUnary<Q, R>(
    ClientMethod<Q, R> method,
    Q request,
    CallOptions options,
    ClientUnaryInvoker<Q, R> invoker,
  ) {
    Future<void> addAccessToken(Map<String, String> metadata, String _) async {
      final token = await _getAccessToken();
      if (token != null) {
        metadata['access_token'] = token;
      }
    }

    return invoker(
      method,
      request,
      options.mergedWith(CallOptions(providers: [addAccessToken])),
    );
  }

  Future<void> _init() async {
    final parsedConfig = await _loadConfig();
    if (parsedConfig == null) return;

    final details = _parseClientDetails(parsedConfig);
    _clientId = details?.$1;
    _clientSecret = details?.$2;
  }

  Future<YamlMap?> _loadConfig() async {
    try {
      final file = File(_configFilePath);
      final yamlStr = await file.readAsString();
      return loadYaml(yamlStr);
    } catch (e) {
      return null;
    }
  }

  (String clientId, String clientSecret)? _parseClientDetails(YamlMap config) {
    try {
      return (config['client_id'], config['client_secret']);
    } catch (e) {
      return null;
    }
  }

  Future<String?> _getAccessToken() async {
    if (_accessToken case final String token) return token;

    final clientId = _clientId;
    final clientSecret = _clientSecret;
    if (clientId == null || clientSecret == null) {
      return null;
    }

    try {
      final response = await _dio.post<Map<String, dynamic>>(
        '/accesstoken',
        data: {
          'client_id': clientId,
          'client_secret': clientSecret,
          'grant_type': 'client_credentials',
          'scope': 'openid',
        },
        options: Options(contentType: Headers.formUrlEncodedContentType),
      );

      final data = response.data;
      if (data == null) throw Exception('response data was null');

      final accessToken = data['access_token'];
      if (accessToken == null || accessToken is! String) {
        throw Exception('access token could not be parsed from response');
      }
      _accessToken = accessToken;

      print('successfully fetched access token');
      return accessToken;
    } catch (e) {
      // ignored, interceptor will not add access token
    }

    return null;
  }
}
