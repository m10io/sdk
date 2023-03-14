import 'package:grpc/grpc.dart';

import 'package:m10_sdk/library.dart';

class DebugInterceptor extends ClientInterceptor {
  @override
  ResponseFuture<R> interceptUnary<Q, R>(
    ClientMethod<Q, R> method,
    Q request,
    CallOptions options,
    ClientUnaryInvoker<Q, R> invoker,
  ) {
    print('service client request path: ${method.path}');
    return super.interceptUnary(method, request, options, invoker);
  }
}

class ServiceClient {
  ServiceClient(ClientChannel channel)
      : query =
            M10QueryServiceClient(channel, interceptors: [DebugInterceptor()]),
        tx = M10TxServiceClient(channel, interceptors: [DebugInterceptor()]);

  final M10QueryServiceClient query;
  final M10TxServiceClient tx;
}
