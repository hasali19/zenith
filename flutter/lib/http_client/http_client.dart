export 'http_client_stub.dart'
    if (dart.library.html) 'http_client_web.dart'
    if (dart.library.io) 'http_client_io.dart';