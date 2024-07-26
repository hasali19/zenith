import 'package:flutter/material.dart';

class ZenithPage extends MaterialPage<dynamic> {
  ZenithPage({
    LocalKey? key,
    required dynamic route,
    required super.child,
  }) : super(
          key: key ?? ValueKey(route),
          arguments: route,
        );
}
