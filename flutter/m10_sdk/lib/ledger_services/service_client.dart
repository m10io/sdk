import 'package:grpc/grpc.dart';

import 'package:m10_sdk/library.dart';

class DebugInterceptor extends ClientInterceptor {
  @override
  ResponseFuture<R> interceptUnary<Q, R>(
      ClientMethod<Q, R> method, Q request, CallOptions options, invoker) {
    print('service client request path: ${method.path}');
    return super.interceptUnary(method, request, options, invoker);
  }
}

class ServiceClient {
  final M10QueryServiceClient query;
  final M10TxServiceClient tx;

  ServiceClient(ClientChannel channel)
      : query =
            M10QueryServiceClient(channel, interceptors: [DebugInterceptor()]),
        tx = M10TxServiceClient(channel, interceptors: [DebugInterceptor()]) {}
}
