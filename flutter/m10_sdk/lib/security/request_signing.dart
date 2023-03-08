import 'package:m10_sdk/library.dart';
import 'package:m10_sdk/security/security.dart';

extension RequestSigning on Signing {
  Future<Signature> signPayload(List<int> payload) async {
    final signature = Signature();
    signature.publicKey = await publicKey();
    switch (algorithm) {
      case Algorithm.P256:
        signature.algorithm = Signature_Algorithm.P256_SHA256_ASN1;
        break;
      case Algorithm.Ed25519:
        signature.algorithm = Signature_Algorithm.ED25519;
        break;
    }
    signature.signature = await sign(payload);
    return signature;
  }
}
