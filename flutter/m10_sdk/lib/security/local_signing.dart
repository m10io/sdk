import 'dart:io';

import 'package:m10_sdk/security/security.dart';
import 'package:cryptography/cryptography.dart';
import 'package:collection/collection.dart';

class LocalSigning implements Signing {
  LocalSigning(this._keyPair);

  static const ed25519Asn1Prefix = [
    0x30,
    0x53,
    0x02,
    0x01,
    0x01,
    0x30,
    0x05,
    0x06,
    0x03,
    0x2B,
    0x65,
    0x70,
    0x04,
    0x22,
    0x04,
    0x20
  ];
  static const ed25519Length = 32;

  final KeyPair _keyPair;

  static Future<LocalSigning> ed25519(List<int> pkcs8Bytes) async {
    if (!ListEquality().equals(
      pkcs8Bytes.sublist(0, ed25519Asn1Prefix.length),
      ed25519Asn1Prefix,
    )) {
      throw Exception('invalid Ed25519 PKCS #8 ASN.1 DER prefix');
    }
    final bytes = pkcs8Bytes.sublist(
      ed25519Asn1Prefix.length,
      ed25519Asn1Prefix.length + ed25519Length,
    );
    return LocalSigning(await Ed25519().newKeyPairFromSeed(bytes));
  }

  static Future<LocalSigning> generateKeyPair() async => LocalSigning(
        await Ed25519().newKeyPair(),
      );

  static Future<LocalSigning> loadKeyPair(String path) async {
    final file = File(path);
    final pkcs8Bytes = file.readAsBytesSync().toList();
    return LocalSigning.ed25519(pkcs8Bytes);
  }

  @override
  Future<List<int>> publicKey() async =>
      (await _keyPair.extractPublicKey() as SimplePublicKey).bytes;

  @override
  Future<List<int>?> existingPublicKey() async => publicKey();

  @override
  Future<List<int>> sign(List<int> data) async {
    final signature = await Ed25519().sign(data, keyPair: _keyPair);
    return signature.bytes;
  }

  @override
  Algorithm get algorithm => Algorithm.ed25519;
}
