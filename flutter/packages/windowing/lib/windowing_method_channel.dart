import 'package:flutter/services.dart';

import 'windowing_platform_interface.dart';

const _channel = MethodChannel('zenith.hasali.uk/windowing');

class MethodChannelWindowing extends WindowingPlatform {
  @override
  Future<WindowController> createController() async {
    final bool isWindowed = await _channel.invokeMethod('isWindowed');
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
    await _channel.invokeMethod('setFullscreen', value);
    _isFullscreen = value;
  }
}
