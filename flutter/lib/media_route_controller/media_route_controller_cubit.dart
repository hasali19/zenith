import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:zenith/remote_playback.dart';

part 'media_route_controller_cubit.freezed.dart';

@freezed
class MediaRouteControllerState with _$MediaRouteControllerState {
  factory MediaRouteControllerState({required MediaRoute? route}) =
      _MediaRouteControllerState;
}

class MediaRouteControllerCubit extends Cubit<MediaRouteControllerState> {
  final MediaRouter _mediaRouter;

  MediaRouteControllerCubit(this._mediaRouter)
      : super(MediaRouteControllerState(
            route: _mediaRouter.selectedRoute.value)) {
    _mediaRouter.selectedRoute.addListener(_onSelectedRouteChanged);
    Future.microtask(
        () => _mediaRouter.startRouteScanning(RoutesScanningMode.none));
  }

  void _onSelectedRouteChanged() {
    emit(state.copyWith(route: _mediaRouter.selectedRoute.value));
  }

  void deselectRoute() async {
    await _mediaRouter.selectRoute(null);
  }

  @override
  Future<void> close() async {
    await _mediaRouter.stopRoutesScanning(RoutesScanningMode.none);
    _mediaRouter.selectedRoute.removeListener(_onSelectedRouteChanged);
    return super.close();
  }
}
