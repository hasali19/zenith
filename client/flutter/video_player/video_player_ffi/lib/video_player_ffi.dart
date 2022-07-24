import 'package:flutter/widgets.dart';
import 'package:video_player_platform_interface/video_player_platform_interface.dart';

class VideoPlayerFfi extends VideoPlayerPlatform {
  static registerWith() {
    VideoPlayerPlatform.instance = VideoPlayerFfi();
  }

  @override
  Future<VideoController> createController() {
    // TODO: implement createController
    throw UnimplementedError();
  }

  @override
  Widget buildView(VideoController controller) {
    // TODO: implement createView
    throw UnimplementedError();
  }

  @override
  void toggleFullscreen() {
    // TODO: implement toggleFullscreen
  }
}
