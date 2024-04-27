import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';
import 'package:zenith/router/pop_scope.dart';
import 'package:zenith/router/router_controller.dart';
import 'package:zenith/router/stack_router.dart';

class RouteConfig {
  final String location;

  const RouteConfig(this.location);
}

class ZenithRouterDelegate<T> extends RouterDelegate<RouteConfig>
    with ChangeNotifier
    implements RouterController, PopController {
  final List<T> Function(RouteConfig location) onSetLocation;
  final Page<T> Function(T route) buildPage;
  final String Function(T route) buildLocation;

  final _locationListeners = <LocationListener>[];
  final _popHandlers = <PopHandler>[];

  RouteConfig? _config;

  ZenithRouterDelegate({
    required this.onSetLocation,
    required this.buildPage,
    required this.buildLocation,
  });

  @override
  RouteConfig? get currentConfiguration => _config;

  @override
  Widget build(BuildContext context) {
    return RouterControllerScope(
      controller: this,
      child: PopControllerScope(
        controller: this,
        child: StackRouter<T>(
          onSetLocation: onSetLocation,
          buildPage: buildPage,
          buildLocation: buildLocation,
        ),
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
  Future<void> setNewRoutePath(RouteConfig configuration) {
    _config = configuration;
    for (final listener in _locationListeners) {
      listener(configuration);
    }
    return SynchronousFuture(null);
  }

  @override
  void updateLocation(String location) {
    _config = RouteConfig(location);
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
