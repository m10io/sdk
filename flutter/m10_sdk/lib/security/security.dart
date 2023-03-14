import 'dart:typed_data';

enum Algorithm { p256, ed25519 }

abstract class Signing {
  Future<List<int>> sign(List<int> data);
  Future<List<int>> publicKey();
  Future<List<int>?> existingPublicKey();
  Algorithm get algorithm;
}

extension AlgorithmName on Algorithm {
  String get name {
    switch (this) {
      case Algorithm.p256:
        return 'ecdsaP256Sha256';
      case Algorithm.ed25519:
        return 'ed25519';
    }
  }
}

abstract class Digest {
  Uint8List hash(List<int> data);
}
