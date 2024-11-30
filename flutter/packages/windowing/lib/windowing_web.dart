import 'package:flutter_web_plugins/flutter_web_plugins.dart';
import 'package:web/web.dart';

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
  bool get isFullscreen => document.fullscreenElement != null;

  @override
  Future<void> setFullscreen(bool value) async {
    if (value) {
      document.documentElement?.requestFullscreen();
    } else {
      document.exitFullscreen();
    }
  }
}
