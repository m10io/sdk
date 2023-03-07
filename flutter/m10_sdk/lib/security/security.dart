import 'dart:typed_data';

enum Algorithm { P256, Ed25519 }

abstract class Signing {
  Future<List<int>> sign(List<int> data);
  Future<List<int>> publicKey();
  Future<List<int>?> existingPublicKey();
  Algorithm get algorithm;
}

extension AlgorithmName on Algorithm {
  String get name {
    switch (this) {
      case Algorithm.P256:
        return "ecdsaP256Sha256";
      case Algorithm.Ed25519:
        return "ed25519";
    }
  }
}

abstract class Digest {
  Uint8List hash(List<int> data);
}
