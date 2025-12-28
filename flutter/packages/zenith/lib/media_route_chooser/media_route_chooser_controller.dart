import 'package:cast_framework/cast_framework.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'media_route_chooser_controller.freezed.dart';
part 'media_route_chooser_controller.g.dart';

@freezed
abstract class MediaRouteChooserState with _$MediaRouteChooserState {
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

@riverpod
class MediaRouteChooserController extends _$MediaRouteChooserController {
  final _mediaRouter = CastFrameworkPlatform.instance.mediaRouter;

  @override
  MediaRouteChooserState build() {
    _mediaRouter.routes.addListener(_onRoutesChanged);
    _mediaRouter.selectedRoute.addListener(_onSelectedRouteChanged);

    Future.microtask(
        () => _mediaRouter.startRouteScanning(RoutesScanningMode.active));

    ref.onDispose(() async {
      _mediaRouter.routes.removeListener(_onRoutesChanged);
      _mediaRouter.selectedRoute.removeListener(_onSelectedRouteChanged);
      await _mediaRouter.stopRouteScanning(RoutesScanningMode.active);
    });

    return MediaRouteChooserState.initial();
  }

  void _onRoutesChanged() {
    state = state.copyWith(routes: _mediaRouter.routes.value);
  }

  void _onSelectedRouteChanged() {
    state =
        state.copyWith(isConnected: _mediaRouter.selectedRoute.value != null);
  }

  Future<void> selectRoute(MediaRoute route) async {
    await _mediaRouter.selectRoute(route.id);
    state = state.copyWith(requestedId: route.id);
  }
}
