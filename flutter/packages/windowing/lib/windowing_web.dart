// In order to *not* need this ignore, consider extracting the "web" version
// of your plugin as a separate package, instead of inlining it in the same
// package as the core of your plugin.
// ignore: avoid_web_libraries_in_flutter
import 'dart:html' as html;

import 'package:flutter_web_plugins/flutter_web_plugins.dart';

import 'windowing_platform_interface.dart';

class WindowingWeb extends WindowingPlatform {
  WindowingWeb();

  static void registerWith(Registrar registrar) {
    WindowingPlatform.instance = WindowingWeb();
  }

  @override
  Future<WindowController> createController() {
    return Future.value(WindowControllerWeb());
  }
}

class WindowControllerWeb extends WindowController {
  @override
  bool get isWindowed => true;

  @override
  bool get isFullscreen => html.document.fullscreenElement != null;

  @override
  Future<void> setFullscreen(bool value) async {
    if (value) {
      html.document.documentElement?.requestFullscreen();
    } else {
      html.document.exitFullscreen();
    }
  }
}
