import 'package:cast_framework/cast_framework.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'media_route_button_controller.freezed.dart';
part 'media_route_button_controller.g.dart';

@freezed
abstract class MediaRouteButtonState with _$MediaRouteButtonState {
  factory MediaRouteButtonState({required bool isConnected}) =
      _MediaRouteButtonState;
}

@riverpod
class MediaRouteButtonController extends _$MediaRouteButtonController {
  final _mediaRouter = CastFrameworkPlatform.instance.mediaRouter;

  @override
  MediaRouteButtonState build() {
    _mediaRouter.startRouteScanning(RoutesScanningMode.none);
    _mediaRouter.selectedRoute.addListener(_onSelectedRouteChanged);

    ref.onDispose(() {
      _mediaRouter.stopRouteScanning(RoutesScanningMode.none);
      _mediaRouter.selectedRoute.removeListener(_onSelectedRouteChanged);
    });

    return MediaRouteButtonState(
      isConnected:
          CastFrameworkPlatform.instance.mediaRouter.selectedRoute.value !=
          null,
    );
  }

  void _onSelectedRouteChanged() {
    state = state.copyWith(
      isConnected: _mediaRouter.selectedRoute.value != null,
    );
  }
}
