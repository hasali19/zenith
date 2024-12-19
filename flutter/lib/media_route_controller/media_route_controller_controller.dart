import 'package:cast_framework/cast_framework.dart' as cast;
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'media_route_controller_controller.freezed.dart';
part 'media_route_controller_controller.g.dart';

typedef CastMediaStatus = cast.MediaStatus;

@freezed
class MediaRouteControllerState with _$MediaRouteControllerState {
  factory MediaRouteControllerState({
    required cast.MediaRoute? route,
    required CastMediaStatus? mediaStatus,
    required cast.MediaInfo? mediaInfo,
  }) = _MediaRouteControllerState;
}

@riverpod
// FIXME: naming 🤮
class MediaRouteControllerController extends _$MediaRouteControllerController {
  final _mediaRouter = cast.CastFrameworkPlatform.instance.mediaRouter;
  final _client = cast.CastFrameworkPlatform.instance.remoteMediaClient;

  @override
  MediaRouteControllerState build() {
    _mediaRouter.selectedRoute.addListener(_onSelectedRouteChanged);
    _client.mediaStatus.addListener(_onMediaStatusChanged);
    _client.mediaInfo.addListener(_onMediaInfoChanged);

    Future.microtask(
        () => _mediaRouter.startRouteScanning(cast.RoutesScanningMode.none));

    ref.onDispose(() async {
      _mediaRouter.selectedRoute.removeListener(_onSelectedRouteChanged);
      _client.mediaStatus.removeListener(_onMediaStatusChanged);
      _client.mediaInfo.removeListener(_onMediaInfoChanged);
      await _mediaRouter.stopRouteScanning(cast.RoutesScanningMode.none);
    });

    return MediaRouteControllerState(
      route: _mediaRouter.selectedRoute.value,
      mediaStatus: _client.mediaStatus.value,
      mediaInfo: _client.mediaInfo.value,
    );
  }

  void _onSelectedRouteChanged() {
    state = state.copyWith(route: _mediaRouter.selectedRoute.value);
  }

  void _onMediaStatusChanged() {
    state = state.copyWith(mediaStatus: _client.mediaStatus.value);
  }

  void _onMediaInfoChanged() {
    state = state.copyWith(mediaInfo: _client.mediaInfo.value);
  }

  Future<void> deselectRoute() async {
    await _mediaRouter.selectRoute(null);
  }
}
