import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:zenith/remote_playback.dart';

part 'media_route_button_cubit.freezed.dart';

@freezed
class MediaRouteButtonState with _$MediaRouteButtonState {
  factory MediaRouteButtonState({required bool isConnected}) =
      _MediaRouteButtonState;
}

class MediaRouteButtonCubit extends Cubit<MediaRouteButtonState> {
  final MediaRouter _mediaRouter;

  MediaRouteButtonCubit(this._mediaRouter)
      : super(MediaRouteButtonState(isConnected: false)) {
    _mediaRouter.startRouteScanning(RoutesScanningMode.none);
    _mediaRouter.selectedRoute.addListener(_onSelectedRouteChanged);
  }

  @override
  Future<void> close() {
    _mediaRouter.stopRoutesScanning(RoutesScanningMode.none);
    return super.close();
  }

  void _onSelectedRouteChanged() {
    emit(state.copyWith(isConnected: _mediaRouter.selectedRoute.value != null));
  }
}
