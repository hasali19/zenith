
import 'video_player_android_platform_interface.dart';

class VideoPlayerAndroid {
  Future<String?> getPlatformVersion() {
    return VideoPlayerAndroidPlatform.instance.getPlatformVersion();
  }
}
