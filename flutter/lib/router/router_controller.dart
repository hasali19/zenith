import 'package:flutter/widgets.dart';
import 'package:zenith/router/router_delegate.dart';

typedef LocationListener = void Function(RouteConfig location);

abstract interface class RouterController {
  void updateLocation(String location);

  void addLocationListener(LocationListener listener);
  void removeLocationListener(LocationListener listener);

  static RouterController of(BuildContext context) {
    final widget =
        context.dependOnInheritedWidgetOfExactType<RouterControllerScope>();
    assert(widget != null, 'No StackRouter found in context');
    return widget!.controller;
  }
}

class RouterControllerScope extends InheritedWidget {
  final RouterController controller;

  const RouterControllerScope({
    super.key,
    required this.controller,
    required super.child,
  });

  @override
  bool updateShouldNotify(covariant RouterControllerScope oldWidget) {
    return controller != oldWidget.controller;
  }
}
