import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';
import 'package:zenith/router/pop_scope.dart';
import 'package:zenith/router/router_controller.dart';

class RouteLocation {
  final Uri uri;
  final List<String> pathSegments;

  const RouteLocation({required this.uri, required this.pathSegments});

  factory RouteLocation.uri(Uri uri) {
    final start = uri.path.startsWith('/') ? 1 : 0;
    final end =
        uri.path.length - (uri.path != '/' && uri.path.endsWith('/') ? 1 : 0);
    return RouteLocation(
      uri: uri,
      pathSegments: uri.path.substring(start, end).split('/'),
    );
  }
}

class ZenithRouterDelegate extends RouterDelegate<RouteLocation>
    with ChangeNotifier
    implements RouterController, PopController {
  final Widget Function(BuildContext context) builder;

  final _locationListeners = <LocationListener>[];
  final _popHandlers = <PopHandler>[];

  RouteLocation? _config;

  ZenithRouterDelegate({required this.builder});

  @override
  RouteLocation? get currentConfiguration => _config;

  @override
  Widget build(BuildContext context) {
    return RouterControllerScope(
      controller: this,
      child: PopControllerScope(
        controller: this,
        child: builder(context),
      ),
    );
  }

  @override
  Future<bool> popRoute() {
    for (final handler in _popHandlers) {
      if (handler.maybePopTop()) {
        return SynchronousFuture(true);
      }
    }
    return SynchronousFuture(false);
  }

  @override
  Future<void> setNewRoutePath(RouteLocation configuration) {
    _config = configuration;
    for (final listener in _locationListeners) {
      listener(configuration);
    }
    return SynchronousFuture(null);
  }

  @override
  void updateLocation(String location) {
    _config = RouteLocation.uri(Uri.parse(location));
    notifyListeners();
  }

  /// Regisers a listener to be invoked when the operating system updates the
  /// current location.
  ///
  /// Upon registration, this will be invoked immediately with the current
  /// location.
  @override
  void addLocationListener(LocationListener listener) {
    _locationListeners.add(listener);

    final currentConfig = _config;
    if (currentConfig != null) {
      listener(currentConfig);
    }
  }

  /// Unregisters a location listener.
  ///
  /// See also:
  /// * [addLocationListener]
  @override
  void removeLocationListener(LocationListener listener) {
    _locationListeners.remove(listener);
  }

  @override
  void addPopHandler(PopHandler handler) {
    _popHandlers.add(handler);
  }

  @override
  void removePopHandler(PopHandler handler) {
    _popHandlers.remove(handler);
  }
}
