import 'package:flutter/widgets.dart';

abstract interface class RouterController {
  void updateLocation(String location);

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
