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
  RemoteMediaClient get remoteMediaClient;
}

abstract interface class MediaRouter {
  ValueNotifier<List<MediaRoute>> get routes;
  ValueNotifier<MediaRoute?> get selectedRoute;

  Future<void> startRouteScanning(RoutesScanningMode mode);
  Future<void> stopRouteScanning(RoutesScanningMode mode);
  Future<void> selectRoute(String? id);
}

abstract interface class RemoteMediaClient {
  ValueNotifier<MediaStatus?> get mediaStatus;

  void load(MediaLoadRequestData request);
  void play();
  void pause();
  void stop();
  void seek(MediaSeekOptions options);
  void setActiveMediaTracks(List<int> trackIds);
  void setPlaybackRate(double playbackRate);
}
