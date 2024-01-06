import 'dart:async';

import 'package:cast_framework/cast_framework.dart' as cast;
import 'package:flutter/foundation.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

export 'package:cast_framework/cast_framework.dart' show RoutesScanningMode;

part 'remote_playback.freezed.dart';

@freezed
class MediaRoute with _$MediaRoute {
  const factory MediaRoute({
    required String id,
    required String name,
    required String? description,
    required bool isSelected,
  }) = _MediaRoute;

  factory MediaRoute._fromApi(cast.MediaRoute route) => MediaRoute(
        id: route.id,
        name: route.name,
        description: route.description,
        isSelected: route.isSelected,
      );
}

typedef MediaRoutesListener = void Function(List<MediaRoute> routes);

final cast.CastApi _api = cast.CastApi();

class MediaRouter {
  final routes = ValueNotifier<List<MediaRoute>>([]);
  final selectedRoute = ValueNotifier<MediaRoute?>(null);
  final mediaStatus = ValueNotifier<cast.MediaStatus?>(null);

  MediaRouter() {
    cast.CastEventsApi.setup(_RemotePlaybackEventsApi(this));
  }

  Future<void> startRouteScanning(cast.RoutesScanningMode mode) async {
    await _api.registerRoutesListener(mode);
  }

  Future<void> stopRoutesScanning(cast.RoutesScanningMode mode) async {
    await _api.unregisterRoutesListener(mode);
  }

  Future<void> selectRoute(String? id) => _api.selectRoute(id);
}

class _RemotePlaybackEventsApi implements cast.CastEventsApi {
  final MediaRouter _mediaRouter;

  _RemotePlaybackEventsApi(this._mediaRouter);

  @override
  void onRoutesChanged(List<cast.MediaRoute?> routes) {
    _mediaRouter.routes.value =
        routes.map((e) => MediaRoute._fromApi(e!)).toList();
    _mediaRouter.selectedRoute.value = _mediaRouter.routes.value
        .where((element) => element.isSelected)
        .firstOrNull;
  }

  @override
  void onStatusUpdated(cast.MediaStatus? status) {
    _mediaRouter.mediaStatus.value = status;
  }
}
