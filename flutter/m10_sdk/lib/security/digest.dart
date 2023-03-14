import 'dart:typed_data';
import 'package:m10_sdk/security/security.dart';
import 'package:cryptography/dart.dart';

class LocalDigest implements Digest {
  @override
  Uint8List hash(List<int> data) =>
      Uint8List.fromList(const DartSha256().hashSync(data).bytes);
}
