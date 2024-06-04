import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:zenith/main.dart';
import 'package:zenith/router/pop_scope.dart';
import 'package:zenith/router/router_controller.dart';
import 'package:zenith/router/router_delegate.dart';

class StackRouter<T> extends StatefulWidget {
  final List<T> Function(RouteLocation location) onSetLocation;
  final Page<dynamic> Function(T route) buildPage;
  final String Function(T route) buildLocation;

  const StackRouter({
    super.key,
    required this.onSetLocation,
    required this.buildPage,
    required this.buildLocation,
  });

  static StackRouterController<T> of<T>(BuildContext context) {
    return _StackRouterScope.of<T>(context).controller;
  }

  static StackRouterController<dynamic>? anyOf(BuildContext context) {
    return _DynamicStackRouterScope.of(context)?.controller;
  }

  @override
  State<StackRouter<T>> createState() => StackRouterState<T>();
}

class StackRouterState<T> extends State<StackRouter<T>>
    with RouteAware
    implements StackRouterController<T>, PopHandler, PopController {
  final List<T> _stack = [];
  final GlobalKey<NavigatorState> _navigatorKey = GlobalKey<NavigatorState>();
  final List<PopHandler> _popHandlers = [];
  final List<(T, RouteAware)> _routeAwares = [];

  RouterController? _routerController;
  StackRouterController? _parentController;
  PopController? _popController;

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();

    _routerController?.removeLocationListener(_onLocationChanged);
    _routerController = RouterController.of(context);
    _routerController?.addLocationListener(_onLocationChanged);

    _parentController?.unsubscribe(this);
    _parentController = StackRouter.anyOf(context);
    _parentController?.subscribe(this);

    _popController?.removePopHandler(this);
    _popController = PopController.maybeOf(context);
    _popController?.addPopHandler(this);
  }

  @override
  void dispose() {
    super.dispose();
    _routerController?.removeLocationListener(_onLocationChanged);
    _parentController?.unsubscribe(this);
    _popController?.removePopHandler(this);
  }

  void _onLocationChanged(RouteLocation location) {
    setState(() {
      _stack.clear();
      _stack.addAll(widget.onSetLocation(location));
    });
  }

  @override
  Widget build(BuildContext context) {
    return _DynamicStackRouterScope(
      controller: this,
      child: _StackRouterScope(
        controller: this,
        child: PopControllerScope(
          controller: this,
          child: Navigator(
            key: _navigatorKey,
            pages: _stack.map(widget.buildPage).toList(),
            onPopPage: (route, result) {
              if (!route.didPop(result)) {
                return false;
              }

              if (route.settings.arguments is T) {
                final prevTop = _stack.remove(route.settings.arguments);

                for (final item in _routeAwares) {
                  if (_stack.last == item.$1) {
                    item.$2.didPopNext();
                  } else if (prevTop == item.$1) {
                    item.$2.didPop();
                  }
                }

                _updateRouterLocation();
              }

              route.onPopInvoked(true);

              return true;
            },
          ),
        ),
      ),
    );
  }

  @override
  T get currentRoute => _stack.last;

  @override
  void subscribe(RouteAware routeAware, {T? route}) {
    _routeAwares.add((route ?? _stack.last, routeAware));
  }

  @override
  void unsubscribe(RouteAware routeAware, {T? route}) {
    _routeAwares.removeWhere(
        (item) => item.$2 == routeAware && (route == null || route == item.$1));
  }

  @override
  void didPushNext() {
    for (final item in _routeAwares) {
      item.$2.didPushNext();
    }
  }

  @override
  void didPopNext() {
    for (final item in _routeAwares) {
      if (item.$1 == _stack.last) {
        item.$2.didPopNext();
      }
    }
  }

  @override
  void pop() {
    setState(() {
      final prevTop = _stack.removeLast();

      for (final item in _routeAwares) {
        if (_stack.last == item.$1) {
          item.$2.didPopNext();
        } else if (prevTop == item.$1) {
          item.$2.didPop();
        }
      }
    });

    _updateRouterLocation();
  }

  @override
  bool maybePopTop() {
    for (final handler in _popHandlers) {
      if (handler.maybePopTop()) {
        return true;
      }
    }

    if (_navigatorKey.currentState?.canPop() ?? false) {
      _navigatorKey.currentState?.pop();
      return true;
    }

    return false;
  }

  @override
  void push(T route) {
    final prevTop = _stack.last;

    setState(() {
      _stack.add(route);
    });

    for (final item in _routeAwares) {
      if (prevTop == item.$1) {
        item.$2.didPushNext();
      } else if (_stack.last == item.$1) {
        item.$2.didPush();
      }
    }

    _updateRouterLocation();
  }

  @override
  void replace(T route) {
    setState(() {
      _stack.removeLast();
      _stack.add(route);
    });
    _updateRouterLocation();
  }

  @override
  void replaceAll(T route) {
    setState(() {
      _stack.clear();
      _stack.add(route);
    });
    _updateRouterLocation();
  }

  @override
  void addPopHandler(PopHandler handler) {
    _popHandlers.add(handler);
  }

  @override
  void removePopHandler(PopHandler handler) {
    _popHandlers.remove(handler);
  }

  void _updateRouterLocation() {
    RouterController.of(context)
        .updateLocation(widget.buildLocation(currentRoute));
  }
}

class _StackRouterScope<T> extends InheritedWidget {
  final StackRouterController<T> controller;

  const _StackRouterScope({required this.controller, required super.child});

  static _StackRouterScope<T> of<T>(BuildContext context) {
    final router =
        context.dependOnInheritedWidgetOfExactType<_StackRouterScope<T>>();
    assert(router != null, 'No StackRouter found in context');
    return router!;
  }

  @override
  bool updateShouldNotify(covariant _StackRouterScope<T> oldWidget) {
    return controller != oldWidget.controller;
  }
}

class _DynamicStackRouterScope extends InheritedWidget {
  final StackRouterController controller;

  const _DynamicStackRouterScope({
    required this.controller,
    required super.child,
  });

  static _DynamicStackRouterScope? of<T>(BuildContext context) {
    return context
        .dependOnInheritedWidgetOfExactType<_DynamicStackRouterScope>();
  }

  @override
  bool updateShouldNotify(covariant _DynamicStackRouterScope oldWidget) {
    return controller != oldWidget.controller;
  }
}
