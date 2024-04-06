import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:zenith/main.dart';
import 'package:zenith/router/pop_scope.dart';

class StackRouter<T> extends StatefulWidget {
  final T initial;
  final Page<T> Function(T route) buildPage;

  const StackRouter({
    super.key,
    required this.initial,
    required this.buildPage,
  });

  static StackRouterController<T> of<T>(BuildContext context) {
    return _StackRouterScope.of<T>(context).controller;
  }

  @override
  State<StackRouter<T>> createState() => StackRouterState<T>();
}

class StackRouterState<T> extends State<StackRouter<T>>
    implements StackRouterController<T>, PopHandler, PopController {
  final List<T> _stack = [];
  final GlobalKey<NavigatorState> _navigatorKey = GlobalKey<NavigatorState>();
  final List<PopHandler> _popHandlers = [];

  PopController? _popController;

  @override
  void initState() {
    super.initState();
    _stack.add(widget.initial);
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    _popController?.removePopHandler(this);
    _popController = PopController.maybeOf(context);
    _popController?.addPopHandler(this);
  }

  @override
  void dispose() {
    super.dispose();
    _popController?.removePopHandler(this);
  }

  @override
  Widget build(BuildContext context) {
    return _StackRouterScope(
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
              _stack.remove(route.settings.arguments);
            }
            route.onPopInvoked(true);
            return true;
          },
        ),
      ),
    );
  }

  @override
  T get currentRoute => _stack.last;

  @override
  void pop() {
    setState(() {
      _stack.removeLast();
    });
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
    setState(() {
      _stack.add(route);
    });
  }

  @override
  void replace(T route) {
    setState(() {
      _stack.removeLast();
      _stack.add(route);
    });
  }

  @override
  void replaceAll(T route) {
    setState(() {
      _stack.clear();
      _stack.add(route);
    });
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