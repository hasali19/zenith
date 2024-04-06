import 'package:flutter/widgets.dart';
import 'package:zenith/router/stack_router.dart';

class RouteConfig {}

class ZenithRouterDelegate<T> extends RouterDelegate<RouteConfig>
    with ChangeNotifier, PopNavigatorRouterDelegateMixin<RouteConfig> {
  final T initial;
  final Page<T> Function(T route) buildPage;

  ZenithRouterDelegate({
    required this.initial,
    required this.buildPage,
  });

  @override
  final navigatorKey = GlobalKey<NavigatorState>();

  @override
  Widget build(BuildContext context) {
    return StackRouter<T>(
      navigatorKey: navigatorKey,
      initial: initial,
      buildPage: buildPage,
    );
  }

  @override
  Future<void> setNewRoutePath(RouteConfig configuration) {
    // TODO: implement setNewRoutePath
    throw UnimplementedError();
  }
}
