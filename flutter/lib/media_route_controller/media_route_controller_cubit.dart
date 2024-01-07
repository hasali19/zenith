import 'package:cast_framework/cast_framework.dart' as cast;
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

part 'media_route_controller_cubit.freezed.dart';

typedef CastMediaStatus = cast.MediaStatus;

@freezed
class MediaRouteControllerState with _$MediaRouteControllerState {
  factory MediaRouteControllerState({
    required cast.MediaRoute? route,
    required CastMediaStatus? mediaStatus,
  }) = _MediaRouteControllerState;
}

class MediaRouteControllerCubit extends Cubit<MediaRouteControllerState> {
  final cast.MediaRouter _mediaRouter =
      cast.CastFrameworkPlatform.instance.mediaRouter;
  final cast.RemoteMediaClient _client =
      cast.CastFrameworkPlatform.instance.remoteMediaClient;

  MediaRouteControllerCubit()
      : super(MediaRouteControllerState(
          route: cast
              .CastFrameworkPlatform.instance.mediaRouter.selectedRoute.value,
          mediaStatus: cast.CastFrameworkPlatform.instance.remoteMediaClient
              .mediaStatus.value,
        )) {
    _mediaRouter.selectedRoute.addListener(_onSelectedRouteChanged);
    _client.mediaStatus.addListener(_onMediaStatusChanged);
    Future.microtask(
        () => _mediaRouter.startRouteScanning(cast.RoutesScanningMode.none));
  }

  void _onSelectedRouteChanged() {
    emit(state.copyWith(route: _mediaRouter.selectedRoute.value));
  }

  void _onMediaStatusChanged() {
    emit(state.copyWith(mediaStatus: _client.mediaStatus.value));
  }

  void deselectRoute() async {
    await _mediaRouter.selectRoute(null);
  }

  @override
  Future<void> close() async {
    _mediaRouter.selectedRoute.removeListener(_onSelectedRouteChanged);
    _client.mediaStatus.removeListener(_onMediaStatusChanged);
    await _mediaRouter.stopRouteScanning(cast.RoutesScanningMode.none);
    return super.close();
  }
}
