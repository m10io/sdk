import 'dart:typed_data';

import 'package:dio/dio.dart';

class ObjectClient {
  ObjectClient() : _dio = Dio();
  final Dio _dio;

  Future<void> putObject(
    String url,
    Uint8List imageBytes, {
    String? contentType,
  }) async {
    await _dio.put(
      url,
      data: Stream.fromIterable([imageBytes.toList()]),
      options: Options(
        headers: {
          Headers.contentLengthHeader: imageBytes.length,
          Headers.contentTypeHeader: contentType,
        },
      ),
    );
  }

  Future<Uint8List> getObject(String url) async {
    final response = await _dio.get(
      url,
      options: Options(responseType: ResponseType.stream),
    );
    final body = response.data;
    final imageBytes = BytesBuilder();
    await body.stream.forEach(imageBytes.add);
    return imageBytes.toBytes();
  }
}
