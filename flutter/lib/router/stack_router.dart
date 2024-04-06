import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:zenith/main.dart';

class StackRouter<T> extends StatefulWidget {
  final T initial;
  final GlobalKey<NavigatorState>? navigatorKey;
  final Page<T> Function(T route) buildPage;

  const StackRouter({
    super.key,
    this.navigatorKey,
    required this.initial,
    required this.buildPage,
  });

  static StackRouterController<T> of<T>(BuildContext context) {
    return _StackRouterScope.of<T>(context).controller;
  }

  @override
  State<StackRouter<T>> createState() => _StackRouterState<T>();
}

class _StackRouterState<T> extends State<StackRouter<T>>
    implements StackRouterController<T> {
  final List<T> _stack = [];

  @override
  void initState() {
    super.initState();
    _stack.add(widget.initial);
  }

  @override
  Widget build(BuildContext context) {
    return _StackRouterScope(
      controller: this,
      child: Navigator(
        key: widget.navigatorKey,
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
