import 'package:flutter/services.dart';

import 'windowing_platform_interface.dart';

const _channel = MethodChannel('zenith.hasali.uk/windowing');

class MethodChannelWindowing extends WindowingPlatform {
  @override
  Future<WindowController> createController() async {
    bool isWindowed = true;
    try {
      isWindowed = await _channel.invokeMethod('isWindowed');
    } catch (e) {
      print(
        'Failed to determine if application is windowed, defaulting to true',
      );
    }
    return MethodChannelWindowController(isWindowed: isWindowed);
  }
}

class MethodChannelWindowController extends WindowController {
  MethodChannelWindowController({required this.isWindowed});

  bool _isFullscreen = false;

  @override
  final bool isWindowed;

  @override
  bool get isFullscreen => _isFullscreen;

  @override
  Future<void> setFullscreen(bool value) async {
    try {
      await _channel.invokeMethod('setFullscreen', value);
      _isFullscreen = value;
    } catch (e) {
      print('Failed to set fullscreen mode: $e');
    }
  }
}
