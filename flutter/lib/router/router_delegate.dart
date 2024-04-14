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
  final T initial;
  final Page<T> Function(T route) buildPage;
  final String Function(T route) mapToLocation;

  final _popHandlers = <PopHandler>[];

  RouteConfig? _config;

  ZenithRouterDelegate({
    required this.initial,
    required this.buildPage,
    required this.mapToLocation,
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
          initial: initial,
          buildPage: buildPage,
          mapToLocation: mapToLocation,
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
    // TODO: implement setNewRoutePath
    return SynchronousFuture(null);
  }

  @override
  void updateLocation(String location) {
    _config = RouteConfig(location);
    notifyListeners();
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
