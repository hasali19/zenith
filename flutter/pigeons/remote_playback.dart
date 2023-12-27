import 'package:pigeon/pigeon.dart';

class MediaRoute {
  final String id;
  final String name;
  final String? description;

  MediaRoute({
    required this.id,
    required this.name,
    required this.description,
  });
}

enum RoutesScanningMode {
  none,
  passive,
  active,
}

class MediaRouterState {
  final List<MediaRoute?> routes;
  final MediaRoute? selected;

  MediaRouterState({
    required this.selected,
    required this.routes,
  });
}

@HostApi()
abstract class RemotePlaybackApi {
  MediaRouterState getRouterState();
  void registerRoutesListener(RoutesScanningMode mode);
  void unregisterRoutesListener(RoutesScanningMode mode);
  void selectRoute(String? id);
}

@FlutterApi()
abstract class RemotePlaybackEventsApi {
  void onRoutesChanged(List<MediaRoute> routes);
  void onSelectedMediaRouteChanged(MediaRoute? route);
}
