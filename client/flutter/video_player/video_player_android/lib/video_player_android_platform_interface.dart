import 'package:plugin_platform_interface/plugin_platform_interface.dart';

import 'video_player_android_method_channel.dart';

abstract class VideoPlayerAndroidPlatform extends PlatformInterface {
  /// Constructs a VideoPlayerAndroidPlatform.
  VideoPlayerAndroidPlatform() : super(token: _token);

  static final Object _token = Object();

  static VideoPlayerAndroidPlatform _instance = MethodChannelVideoPlayerAndroid();

  /// The default instance of [VideoPlayerAndroidPlatform] to use.
  ///
  /// Defaults to [MethodChannelVideoPlayerAndroid].
  static VideoPlayerAndroidPlatform get instance => _instance;

  /// Platform-specific implementations should set this with their own
  /// platform-specific class that extends [VideoPlayerAndroidPlatform] when
  /// they register themselves.
  static set instance(VideoPlayerAndroidPlatform instance) {
    PlatformInterface.verifyToken(instance, _token);
    _instance = instance;
  }

  Future<String?> getPlatformVersion() {
    throw UnimplementedError('platformVersion() has not been implemented.');
  }
}
