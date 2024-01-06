import 'package:cast_framework/src/cast_framework_platform_interface.dart';
import 'package:flutter/foundation.dart';

import 'cast_api.g.dart' as cast;

final _api = cast.CastApi();

class AndroidCastFramework extends CastFrameworkPlatform {
  final _mediaRouter = _MediaRouter();

  @override
  MediaRouter get mediaRouter => _mediaRouter;
}

class _MediaRouter implements MediaRouter {
  _MediaRouter() {
    cast.CastEventsApi.setup(_RemotePlaybackEventsApi(this));
  }

  @override
  final routes = ValueNotifier<List<cast.MediaRoute>>([]);

  @override
  final selectedRoute = ValueNotifier<cast.MediaRoute?>(null);

  @override
  final mediaStatus = ValueNotifier<cast.MediaStatus?>(null);

  @override
  Future<void> startRouteScanning(cast.RoutesScanningMode mode) async =>
      await _api.registerRoutesListener(mode);

  @override
  Future<void> stopRouteScanning(cast.RoutesScanningMode mode) async =>
      await _api.unregisterRoutesListener(mode);

  @override
  Future<void> selectRoute(String? id) async => await _api.selectRoute(id);
}

class _RemotePlaybackEventsApi implements cast.CastEventsApi {
  final _MediaRouter _mediaRouter;

  _RemotePlaybackEventsApi(this._mediaRouter);

  @override
  void onRoutesChanged(List<cast.MediaRoute?> routes) {
    _mediaRouter.routes.value = routes.map((route) => route!).toList();
    _mediaRouter.selectedRoute.value = _mediaRouter.routes.value
        .where((route) => route.isSelected)
        .firstOrNull;
  }

  @override
  void onStatusUpdated(cast.MediaStatus? status) {
    _mediaRouter.mediaStatus.value = status;
  }
}
