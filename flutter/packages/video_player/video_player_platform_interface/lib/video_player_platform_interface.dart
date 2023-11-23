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
}

enum VideoState { idle, active, ended }

class SubtitleTrack {
  String id;
  String src;
  String mimeType;
  String? title;
  String? language;
  String? displayLanguage;

  SubtitleTrack({
    required this.id,
    required this.src,
    required this.mimeType,
    this.title,
    this.language,
    this.displayLanguage,
  });
}

class VideoItem {
  final String? title;
  final String? subtitle;
  final String url;
  final List<SubtitleTrack> subtitles;

  VideoItem({
    required this.url,
    required this.subtitles,
    this.title,
    this.subtitle,
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
  int get currentItemIndex;
  BoxFit get fit;
  double get playbackSpeed;

  void load(List<VideoItem> items, int startIndex, double startPosition);
  void play();
  void pause();
  void seekToNextItem();
  void seekToPreviousItem();
  void setAudioTrack(int index);
  void setTextTrack(SubtitleTrack? track);
  void setFit(BoxFit fit);
  void setPlaybackSpeed(double speed);
  void addListener(void Function() listener);
  void removeListener(void Function() listener);
  void dispose();
}
