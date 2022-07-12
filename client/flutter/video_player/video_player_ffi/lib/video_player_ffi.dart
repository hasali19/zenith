import 'package:flutter/widgets.dart';
import 'package:video_player_platform_interface/video_player_platform_interface.dart';

class VideoPlayerPlugin extends VideoPlayerPlatform {
  static registerWith() {
    VideoPlayerPlatform.instance = VideoPlayerPlugin();
  }

  @override
  VideoController createController() {
    // TODO: implement createController
    throw UnimplementedError();
  }

  @override
  Widget createView(VideoController controller) {
    // TODO: implement createView
    throw UnimplementedError();
  }

  @override
  void toggleFullscreen() {
    // TODO: implement toggleFullscreen
  }
}
