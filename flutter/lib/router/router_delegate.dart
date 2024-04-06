import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';
import 'package:zenith/router/pop_scope.dart';
import 'package:zenith/router/stack_router.dart';

class RouteConfig {}

class ZenithRouterDelegate<T> extends RouterDelegate<RouteConfig>
    with ChangeNotifier
    implements PopController {
  final T initial;
  final Page<T> Function(T route) buildPage;

  final _navigatorKey = GlobalKey<StackRouterState>();
  final _popHandlers = <PopHandler>[];

  ZenithRouterDelegate({
    required this.initial,
    required this.buildPage,
  });

  @override
  Widget build(BuildContext context) {
    return PopControllerScope(
      controller: this,
      child: StackRouter<T>(
        key: _navigatorKey,
        initial: initial,
        buildPage: buildPage,
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
    throw UnimplementedError();
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
