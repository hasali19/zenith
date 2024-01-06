import 'package:flutter/foundation.dart';
import 'package:plugin_platform_interface/plugin_platform_interface.dart';

import 'cast_api.g.dart';

abstract class CastFrameworkPlatform extends PlatformInterface {
  CastFrameworkPlatform() : super(token: _token);

  static final Object _token = Object();

  static CastFrameworkPlatform? _instance;

  static CastFrameworkPlatform get instance => _instance!;

  static set instance(CastFrameworkPlatform instance) {
    PlatformInterface.verifyToken(instance, _token);
    _instance = instance;
  }

  MediaRouter get mediaRouter;
}

abstract interface class MediaRouter {
  ValueNotifier<List<MediaRoute>> get routes;
  ValueNotifier<MediaRoute?> get selectedRoute;
  ValueNotifier<MediaStatus?> get mediaStatus;

  Future<void> startRouteScanning(RoutesScanningMode mode);
  Future<void> stopRouteScanning(RoutesScanningMode mode);
  Future<void> selectRoute(String? id);
}
