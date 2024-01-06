import 'package:cast_framework/cast_framework.dart' as cast;
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:zenith/remote_playback.dart';

part 'media_route_controller_cubit.freezed.dart';

typedef CastMediaStatus = cast.MediaStatus;

@freezed
class MediaRouteControllerState with _$MediaRouteControllerState {
  factory MediaRouteControllerState({
    required MediaRoute? route,
    required CastMediaStatus? mediaStatus,
  }) = _MediaRouteControllerState;
}

class MediaRouteControllerCubit extends Cubit<MediaRouteControllerState> {
  final MediaRouter _mediaRouter;

  MediaRouteControllerCubit(this._mediaRouter)
      : super(MediaRouteControllerState(
          route: _mediaRouter.selectedRoute.value,
          mediaStatus: _mediaRouter.mediaStatus.value,
        )) {
    _mediaRouter.selectedRoute.addListener(_onSelectedRouteChanged);
    _mediaRouter.mediaStatus.addListener(_onMediaStatusChanged);
    Future.microtask(
        () => _mediaRouter.startRouteScanning(RoutesScanningMode.none));
  }

  void _onSelectedRouteChanged() {
    emit(state.copyWith(route: _mediaRouter.selectedRoute.value));
  }

  void _onMediaStatusChanged() {
    emit(state.copyWith(mediaStatus: _mediaRouter.mediaStatus.value));
  }

  void deselectRoute() async {
    await _mediaRouter.selectRoute(null);
  }

  @override
  Future<void> close() async {
    _mediaRouter.selectedRoute.removeListener(_onSelectedRouteChanged);
    _mediaRouter.mediaStatus.removeListener(_onMediaStatusChanged);
    await _mediaRouter.stopRoutesScanning(RoutesScanningMode.none);
    return super.close();
  }
}
