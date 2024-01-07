import 'package:cast_framework/cast_framework.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

part 'media_route_button_cubit.freezed.dart';

@freezed
class MediaRouteButtonState with _$MediaRouteButtonState {
  factory MediaRouteButtonState({required bool isConnected}) =
      _MediaRouteButtonState;
}

class MediaRouteButtonCubit extends Cubit<MediaRouteButtonState> {
  final MediaRouter _mediaRouter;

  MediaRouteButtonCubit()
      : _mediaRouter = CastFrameworkPlatform.instance.mediaRouter,
        super(MediaRouteButtonState(
            isConnected: CastFrameworkPlatform
                    .instance.mediaRouter.selectedRoute.value !=
                null)) {
    _mediaRouter.startRouteScanning(RoutesScanningMode.none);
    _mediaRouter.selectedRoute.addListener(_onSelectedRouteChanged);
  }

  @override
  Future<void> close() {
    _mediaRouter.stopRouteScanning(RoutesScanningMode.none);
    _mediaRouter.selectedRoute.removeListener(_onSelectedRouteChanged);
    return super.close();
  }

  void _onSelectedRouteChanged() {
    emit(state.copyWith(isConnected: _mediaRouter.selectedRoute.value != null));
  }
}
