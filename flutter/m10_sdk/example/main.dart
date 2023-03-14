import 'package:cryptography/cryptography.dart';
import 'package:m10_sdk/m10_directory.dart';
import 'package:m10_sdk/m10_sdk.dart';
import 'package:m10_sdk/security/local_signing.dart';

Future<void> main() async {
  // Connect to the M10 directory
  final m10Directory = M10Directory(
    'https://example-host.io',
    tokenProvider: ExampleTokenProvider(),
  );

  // Create an instance of the SDK
  final m10 = M10Sdk(
    signer: await LocalSigning.loadKeyPair('/path/to/example_key'),
    ledgers: await m10Directory.listLedgers(),
  );

  // Create a user
  await m10.createUser(operator: 'Example operator');
}

class ExampleTokenProvider implements TokenProvider {
  @override
  Future<String> get accessToken async => 'Example token';

  @override
  String get subject => 'Example subject';
}
