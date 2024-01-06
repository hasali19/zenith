import 'package:flutter/foundation.dart';

import 'cast_api.g.dart' as cast;
import 'cast_framework_platform_interface.dart';

final _api = cast.CastApi();

class CastFrameworkAndroid extends CastFrameworkPlatform {
  static registerWith() {
    CastFrameworkPlatform.instance = CastFrameworkAndroid();
  }

  bool _isInit = false;

  final _mediaRouter = _MediaRouter();
  final _remoteMediaClient = _RemoteMediaClient();

  @override
  bool get isSupported => true;

  @override
  MediaRouter get mediaRouter {
    _ensureInitialized();
    return _mediaRouter;
  }

  @override
  RemoteMediaClient get remoteMediaClient {
    _ensureInitialized();
    return _remoteMediaClient;
  }

  void _ensureInitialized() {
    if (!_isInit) {
      cast.CastEventsApi.setup(_RemotePlaybackEventsApi(this));
      _isInit = true;
    }
  }
}

class _MediaRouter implements MediaRouter {
  @override
  final routes = ValueNotifier<List<cast.MediaRoute>>([]);

  @override
  final selectedRoute = ValueNotifier<cast.MediaRoute?>(null);

  @override
  Future<void> startRouteScanning(cast.RoutesScanningMode mode) async =>
      await _api.registerRoutesListener(mode);

  @override
  Future<void> stopRouteScanning(cast.RoutesScanningMode mode) async =>
      await _api.unregisterRoutesListener(mode);

  @override
  Future<void> selectRoute(String? id) async => await _api.selectRoute(id);
}

class _RemoteMediaClient implements RemoteMediaClient {
  @override
  final mediaStatus = ValueNotifier<cast.MediaStatus?>(null);

  @override
  void load(cast.MediaLoadRequestData request) {
    _api.load(request);
  }

  @override
  void pause() {
    _api.pause();
  }

  @override
  void play() {
    _api.play();
  }

  @override
  void seek(cast.MediaSeekOptions options) {
    _api.seek(options);
  }

  @override
  void setActiveMediaTracks(List<int> trackIds) {
    _api.setActiveMediaTracks(trackIds);
  }

  @override
  void setPlaybackRate(double playbackRate) {
    _api.setPlaybackRate(playbackRate);
  }

  @override
  void stop() {
    _api.stop();
  }
}

class _RemotePlaybackEventsApi implements cast.CastEventsApi {
  final CastFrameworkAndroid _plugin;

  _RemotePlaybackEventsApi(this._plugin);

  @override
  void onRoutesChanged(List<cast.MediaRoute?> routes) {
    _plugin.mediaRouter.routes.value = routes.map((route) => route!).toList();
    _plugin.mediaRouter.selectedRoute.value = _plugin.mediaRouter.routes.value
        .where((route) => route.isSelected)
        .firstOrNull;
  }

  @override
  void onStatusUpdated(cast.MediaStatus? status) {
    _plugin.remoteMediaClient.mediaStatus.value = status;
  }
}
