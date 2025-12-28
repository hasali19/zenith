import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/painting.dart';
import 'package:flutter/services.dart';

final isInPipMode = ValueNotifier(false);
final stableSystemBarInsets = ValueNotifier(EdgeInsets.zero);

final _channel = const MethodChannel('zenith.hasali.dev/platform')
  ..setMethodCallHandler((call) async {
    if (call.method == 'setIsInPipMode') {
      isInPipMode.value = call.arguments;
    } else if (call.method == 'setStableSystemBarInsets') {
      stableSystemBarInsets.value = EdgeInsets.fromLTRB(
        call.arguments['left'].toDouble(),
        call.arguments['top'].toDouble(),
        call.arguments['right'].toDouble(),
        call.arguments['bottom'].toDouble(),
      );
    }
  });

Future<void> setPipEnabled(bool enabled) async {
  if (!kIsWeb && Platform.isAndroid) {
    return _channel.invokeMethod('setPipEnabled', enabled);
  }
}

Future<void> setExtendIntoCutout(bool extendIntoCutout) async {
  if (!kIsWeb && Platform.isAndroid) {
    return _channel.invokeMethod('setExtendIntoCutout', extendIntoCutout);
  }
}

Future<void> setSystemBarsVisible(bool visible) async {
  if (!kIsWeb && Platform.isAndroid) {
    return _channel.invokeMethod('setSystemBarsVisible', visible);
  }
}

Future<List<String>> getSupportedAbis() async {
  final List<Object?> res = await _channel.invokeMethod('getSupportedAbis');
  return res.where((v) => v != null).map((v) => v!.toString()).toList();
}
