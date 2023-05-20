import 'package:plugin_platform_interface/plugin_platform_interface.dart';

import 'windowing_method_channel.dart';

abstract class WindowingPlatform extends PlatformInterface {
  WindowingPlatform() : super(token: _token);

  static final Object _token = Object();

  static WindowingPlatform _instance = MethodChannelWindowing();

  static WindowingPlatform get instance => _instance;

  static set instance(WindowingPlatform instance) {
    PlatformInterface.verifyToken(instance, _token);
    _instance = instance;
  }

  Future<WindowController> createController();
}

abstract class WindowController {
  static Future<WindowController> create() {
    return WindowingPlatform.instance.createController();
  }

  bool get isWindowed;
  bool get isFullscreen;

  Future<void> setFullscreen(bool value);

  Future<void> toggleFullscreen() {
    return setFullscreen(!isFullscreen);
  }
}
