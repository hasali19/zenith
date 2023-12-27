import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:zenith/remote_playback.dart';

part 'media_route_chooser_cubit.freezed.dart';

@freezed
class MediaRouteChooserState with _$MediaRouteChooserState {
  factory MediaRouteChooserState({
    required List<MediaRoute> routes,
    required String? requestedId,
    required bool isConnected,
  }) = _MediaRouteChooserState;

  factory MediaRouteChooserState.initial() {
    return MediaRouteChooserState(
      routes: [],
      requestedId: null,
      isConnected: false,
    );
  }
}

class MediaRouteChooserCubit extends Cubit<MediaRouteChooserState> {
  final MediaRouter _mediaRouter;

  MediaRouteChooserCubit(this._mediaRouter)
      : super(MediaRouteChooserState.initial()) {
    _mediaRouter.routes.addListener(_onRoutesChanged);
    _mediaRouter.selectedRoute.addListener(_onSelectedRouteChanged);
    Future.microtask(
        () => _mediaRouter.startRouteScanning(RoutesScanningMode.active));
  }

  @override
  Future<void> close() async {
    await _mediaRouter.stopRoutesScanning(RoutesScanningMode.active);
    _mediaRouter.routes.removeListener(_onRoutesChanged);
    _mediaRouter.selectedRoute.removeListener(_onSelectedRouteChanged);
    return super.close();
  }

  void _onRoutesChanged() {
    emit(state.copyWith(routes: _mediaRouter.routes.value));
  }

  void _onSelectedRouteChanged() {
    emit(state.copyWith(isConnected: _mediaRouter.selectedRoute.value != null));
  }

  void selectRoute(MediaRoute route) async {
    await _mediaRouter.selectRoute(route.id);
    emit(state.copyWith(requestedId: route.id));
  }
}
