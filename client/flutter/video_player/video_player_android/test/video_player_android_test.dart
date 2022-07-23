import 'package:flutter_test/flutter_test.dart';
import 'package:video_player_android/video_player_android.dart';
import 'package:video_player_android/video_player_android_platform_interface.dart';
import 'package:video_player_android/video_player_android_method_channel.dart';
import 'package:plugin_platform_interface/plugin_platform_interface.dart';

class MockVideoPlayerAndroidPlatform
    with MockPlatformInterfaceMixin
    implements VideoPlayerAndroidPlatform {

  @override
  Future<String?> getPlatformVersion() => Future.value('42');
}

void main() {
  final VideoPlayerAndroidPlatform initialPlatform = VideoPlayerAndroidPlatform.instance;

  test('$MethodChannelVideoPlayerAndroid is the default instance', () {
    expect(initialPlatform, isInstanceOf<MethodChannelVideoPlayerAndroid>());
  });

  test('getPlatformVersion', () async {
    VideoPlayerAndroid videoPlayerAndroidPlugin = VideoPlayerAndroid();
    MockVideoPlayerAndroidPlatform fakePlatform = MockVideoPlayerAndroidPlatform();
    VideoPlayerAndroidPlatform.instance = fakePlatform;

    expect(await videoPlayerAndroidPlugin.getPlatformVersion(), '42');
  });
}
