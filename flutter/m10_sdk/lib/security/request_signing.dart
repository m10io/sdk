import 'package:m10_sdk/library.dart';
import 'package:m10_sdk/security/security.dart';

extension RequestSigning on Signing {
  Future<Signature> signPayload(List<int> payload) async {
    final signature = Signature()..publicKey = await publicKey();
    switch (algorithm) {
      case Algorithm.p256:
        signature.algorithm = Signature_Algorithm.P256_SHA256_ASN1;
        break;
      case Algorithm.ed25519:
        signature.algorithm = Signature_Algorithm.ED25519;
        break;
    }
    signature.signature = await sign(payload);
    return signature;
  }
}
