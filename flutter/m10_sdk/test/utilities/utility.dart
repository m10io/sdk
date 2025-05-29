import 'dart:convert';
import 'dart:io';

import 'package:dio/dio.dart';
import 'package:m10_sdk/library.dart';
import 'package:m10_sdk/m10_sdk.dart';
import 'package:m10_sdk/m10_directory.dart';
import 'package:m10_sdk/security/local_signing.dart';
import 'package:random_string/random_string.dart';
import 'package:uuid/uuid.dart';

final uuid = Uuid();

final authClientId = 'r1iPtmb6Mf6g0P7RvELXhRAWwFZViPHP';
final testUsername = 'ops+e2etest@m10.io';
final testPassword = 'n@R*88JxsccRpuw';

final _directoryHost =
    Platform.environment['DIRECTORY_HOST'] ?? 'app.dev.m10.net';
final _ledgerHost =
    Platform.environment['LEDGER_API_HOST'] ?? 'app.dev.m10.net';
final _disableTls = Platform.environment['DISABLE_TLS'] == 'true';
final operator = 'm10';
final instrument = Platform.environment['CURRENCY'] ?? 'ttt';
final ledgerId = '$instrument.m10';
final _ledgers = [
  Ledger()
    ..operator = operator
    ..url = 'https://$_ledgerHost/',
];

void printLedgerInfo() {
  print('Directory host: $_directoryHost');
  print('Ledger host: $_ledgerHost');
  print('Disable TLS: $_disableTls');
  print('Instrument: $instrument');
  final ledgerStr =
      _ledgers.map((l) => 'operator: ${l.operator}, url: ${l.url}').join(';');
  print('Ledgers: $ledgerStr');
}

class UsernamePasswordAuth implements TokenProvider {
  UsernamePasswordAuth(this.username, this.password);

  final Dio _client = Dio(BaseOptions(baseUrl: 'https://$_directoryHost'));
  final String username;
  final String password;
  final String _subject = uuid.v4();
  String? _accessToken;

  @override
  Future<String> get accessToken async {
    if (_accessToken == null) {
      final response = await _client.post(
        '/oauth/token',
        data: {
          'client_id': 'bank-emulator',
          'grant_type': 'password',
          'username': username,
          'password': password,
          'audience': 'https://api.m10.net',
          'scope': 'offline_access openid',
        },
      );
      _accessToken = response.data['access_token'] as String;
    }
    return _accessToken!;
  }

  @override
  String get subject => _subject;
}

final directory = M10Directory(
  _directoryHost,
  tokenProvider: UsernamePasswordAuth(testUsername, testPassword),
  disableTls: _disableTls,
);

// Test Bank
late final M10Sdk bankAdmin;
late final String parentAccountId;

// Test User "Alice"
late final M10Sdk alice;
late final String aliceId;
late final String aliceAccountId;
late final String aliceName;

// Test User "Bob"
late final M10Sdk bob;
late final String bobId;
late final String bobsAccountId;
late final String bobsName;

class Utility {
  ///
  /// This method MUST be called to initialize local signing before starting any
  /// tests that use the preset SDKs (alice, bob, etc.).
  ///
  static Future<void> init() async {
    // Setup bank admin
    bankAdmin = M10Sdk(
      signer: await LocalSigning.loadKeyPair('test/keys/test_bank_admin.pkcs8'),
      ledgers: _ledgers,
      disableTls: _disableTls,
    );

    final owner = base64Encode(await bankAdmin.signer.publicKey());
    final issuanceAccount =
        (await bankAdmin.findAccountByOwner(owner: owner, operator: operator))
            .first;
    parentAccountId = issuanceAccount.id;

    // Setup test user "Alice"
    alice = M10Sdk(
      signer: await LocalSigning.generateKeyPair(),
      ledgers: _ledgers,
      disableTls: _disableTls,
    );

    aliceName = 'alice ${randomAlpha(8)}';
    aliceId = await createUser(sdk: alice);
    aliceAccountId = await createAccount(alice, publicName: aliceName);

    // Setup test user "Bob"
    bob = M10Sdk(
      signer: await LocalSigning.generateKeyPair(),
      ledgers: _ledgers,
      disableTls: _disableTls,
    );

    bobsName = 'bob ${randomAlpha(8)}';
    bobId = await createUser(sdk: bob);
    bobsAccountId = await createAccount(bob, publicName: bobsName);
  }

  static Future<M10Sdk> newUser() async => M10Sdk(
        signer: await LocalSigning.generateKeyPair(),
        ledgers: _ledgers,
        disableTls: _disableTls,
      );

  static Future<String> createUser({
    required M10Sdk sdk,
  }) async {
    final userId = await sdk.createUser(
      operator: operator,
    );
    final roleBindings = await bankAdmin.listRoleBindings(
      name: 'dart-test-customer',
      operator: operator,
    );
    final roleBinding = roleBindings.first;
    final userKey = await sdk.signer.publicKey();
    if (!roleBinding.subjects.contains(base64Encode(userKey))) {
      final subject = base64.encode(userKey);
      await bankAdmin.updateRoleBinding(
        id: roleBinding.id,
        operator: operator,
        subjects: [subject],
      );
    }
    return userId;
  }

  static Future<String> createAccount(M10Sdk sdk, {String? publicName}) async =>
      sdk.createAccount(
        parentId: parentAccountId,
        name: 'Dart SDK Test Account',
        publicName: publicName,
        operator: operator,
      );
}
