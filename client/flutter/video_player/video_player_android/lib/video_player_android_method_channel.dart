import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

import 'video_player_android_platform_interface.dart';

/// An implementation of [VideoPlayerAndroidPlatform] that uses method channels.
class MethodChannelVideoPlayerAndroid extends VideoPlayerAndroidPlatform {
  /// The method channel used to interact with the native platform.
  @visibleForTesting
  final methodChannel = const MethodChannel('video_player_android');

  @override
  Future<String?> getPlatformVersion() async {
    final version = await methodChannel.invokeMethod<String>('getPlatformVersion');
    return version;
  }
}
