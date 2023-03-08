import 'dart:typed_data';

import 'package:dio/dio.dart';

class ObjectClient {
  final Dio _dio;
  ObjectClient() : _dio = new Dio();

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
    final response = await _dio.get(url,
        options: Options(responseType: ResponseType.stream));
    ResponseBody body = response.data;
    var imageBytes = BytesBuilder();
    await body.stream.forEach((bytes) => imageBytes.add(bytes));
    return imageBytes.toBytes();
  }
}
