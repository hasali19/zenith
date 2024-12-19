import 'package:flutter/foundation.dart';
import 'package:plugin_platform_interface/plugin_platform_interface.dart';

import 'cast_api.g.dart';

abstract class CastFrameworkPlatform extends PlatformInterface {
  CastFrameworkPlatform() : super(token: _token);

  static final Object _token = Object();

  static CastFrameworkPlatform _instance = _CastFrameworkUnsupported();

  static CastFrameworkPlatform get instance => _instance;

  static set instance(CastFrameworkPlatform instance) {
    PlatformInterface.verifyToken(instance, _token);
    _instance = instance;
  }

  bool get isSupported => false;
  MediaRouter get mediaRouter;
  RemoteMediaClient get remoteMediaClient;
}

class _CastFrameworkUnsupported extends CastFrameworkPlatform {
  @override
  MediaRouter get mediaRouter => throw UnimplementedError();

  @override
  RemoteMediaClient get remoteMediaClient => throw UnimplementedError();
}

abstract interface class MediaRouter {
  ValueNotifier<List<MediaRoute>> get routes;
  ValueNotifier<MediaRoute?> get selectedRoute;

  Future<void> init({required String receiverAppId});
  Future<void> startRouteScanning(RoutesScanningMode mode);
  Future<void> stopRouteScanning(RoutesScanningMode mode);
  Future<void> selectRoute(String? id);
}

abstract interface class RemoteMediaClient {
  ValueNotifier<MediaStatus?> get mediaStatus;
  ValueNotifier<MediaInfo?> get mediaInfo;

  void load(MediaLoadRequestData request);
  void play();
  void pause();
  void stop();
  void seek(MediaSeekOptions options);
  void queueNext();
  void queuePrev();
  void setActiveMediaTracks(List<int> trackIds);
  void setPlaybackRate(double playbackRate);
}
