import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:zenith/remote_playback_api.g.dart' as api;

export 'remote_playback_api.g.dart' show RoutesScanningMode;

part 'remote_playback.freezed.dart';

@freezed
class MediaRoute with _$MediaRoute {
  const factory MediaRoute({
    required String id,
    required String name,
    required String? description,
  }) = _MediaRoute;

  factory MediaRoute._fromApi(api.MediaRoute route) => MediaRoute(
        id: route.id,
        name: route.name,
        description: route.description,
      );
}

typedef MediaRoutesListener = void Function(List<MediaRoute> routes);

final api.RemotePlaybackApi _api = api.RemotePlaybackApi();

class MediaRouter {
  final routes = ValueNotifier<List<MediaRoute>>([]);
  final selectedRoute = ValueNotifier<MediaRoute?>(null);
  final mediaStatus = ValueNotifier<api.MediaStatus?>(null);

  MediaRouter() {
    api.RemotePlaybackEventsApi.setup(_RemotePlaybackEventsApi(this));
  }

  Future<void> startRouteScanning(api.RoutesScanningMode mode) async {
    await _api.registerRoutesListener(mode);
  }

  Future<void> stopRoutesScanning(api.RoutesScanningMode mode) async {
    await _api.unregisterRoutesListener(mode);
  }

  Future<void> selectRoute(String? id) => _api.selectRoute(id);
}

class _RemotePlaybackEventsApi implements api.RemotePlaybackEventsApi {
  final MediaRouter _mediaRouter;

  _RemotePlaybackEventsApi(this._mediaRouter);

  @override
  void onRoutesChanged(List<api.MediaRoute?> routes) {
    _mediaRouter.routes.value =
        routes.map((e) => MediaRoute._fromApi(e!)).toList();
  }

  @override
  void onSelectedMediaRouteChanged(api.MediaRoute? route) {
    _mediaRouter.selectedRoute.value =
        route == null ? null : MediaRoute._fromApi(route);
  }

  @override
  void onStatusUpdated(api.MediaStatus status) {
    _mediaRouter.mediaStatus.value = status;
  }
}
