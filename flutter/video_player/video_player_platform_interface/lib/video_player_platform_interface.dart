import 'package:flutter/widgets.dart';
import 'package:plugin_platform_interface/plugin_platform_interface.dart';

abstract class VideoPlayerPlatform extends PlatformInterface {
  VideoPlayerPlatform() : super(token: _token);

  static VideoPlayerPlatform? _instance;

  static final Object _token = Object();

  static VideoPlayerPlatform get instance => _instance!;

  static set instance(VideoPlayerPlatform instance) {
    PlatformInterface.verify(instance, _token);
    _instance = instance;
  }

  Future<VideoController> createController();
  Widget buildView(VideoController controller);

  bool get isWindowed;
  Future<void> enterFullscreen();
  Future<void> exitFullscreen();
  Future<void> toggleFullscreen();
}

enum VideoState { idle, active, ended }

class SubtitleTrack {
  String id;
  String src;
  String? title;
  String? language;
  String? displayLanguage;

  SubtitleTrack({
    required this.id,
    required this.src,
    this.title,
    this.language,
    this.displayLanguage,
  });
}

abstract class VideoController {
  VideoState get state;
  double get position;
  set position(double value);
  double get duration;
  bool get paused;
  bool get loading;
  bool get supportsAudioTrackSelection;

  void load(String url, List<SubtitleTrack> subtitles, double startPosition);
  void play();
  void pause();
  void setAudioTrack(int index);
  void setTextTrack(SubtitleTrack? track);
  void setFit(BoxFit fit);
  void addListener(void Function() listener);
  void removeListener(void Function() listener);
  void dispose();
}
