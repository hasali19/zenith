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
  // TODO: implement isWindowed
  bool get isWindowed => throw UnimplementedError();

  @override
  Future<void> enterFullscreen() {
    // TODO: implement enterFullscreen
    throw UnimplementedError();
  }

  @override
  Future<void> exitFullscreen() {
    // TODO: implement exitFullscreen
    throw UnimplementedError();
  }

  @override
  Future<void> toggleFullscreen() {
    // TODO: implement toggleFullscreen
    throw UnimplementedError();
  }
}
