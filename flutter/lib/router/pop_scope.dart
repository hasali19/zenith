import 'package:flutter/widgets.dart';

abstract class PopHandler {
  bool maybePopTop();
}

abstract interface class PopController {
  void addPopHandler(PopHandler handler);
  void removePopHandler(PopHandler handler);

  static PopController? maybeOf(BuildContext context) {
    return context
        .dependOnInheritedWidgetOfExactType<PopControllerScope>()
        ?.controller;
  }
}

class PopControllerScope extends InheritedWidget {
  final PopController controller;

  const PopControllerScope({
    super.key,
    required this.controller,
    required super.child,
  });

  @override
  bool updateShouldNotify(covariant PopControllerScope oldWidget) {
    return controller != oldWidget.controller;
  }
}
