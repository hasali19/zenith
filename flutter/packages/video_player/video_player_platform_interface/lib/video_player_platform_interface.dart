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

  Future<VideoController> createController({Map<String, String> headers});
  Widget buildView(VideoController controller);
}

enum VideoState { idle, active, ended }

class ExternalSubtitleTrack {
  String id;
  String src;
  String mimeType;
  String? title;
  String? language;
  String? displayLanguage;

  ExternalSubtitleTrack({
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
  final List<ExternalSubtitleTrack> subtitles;

  VideoItem({
    required this.url,
    required this.subtitles,
    this.title,
    this.subtitle,
  });
}

class SubtitleTrack {
  String id;
  String? label;
  String? language;

  SubtitleTrack({
    required this.id,
    required this.label,
    required this.language,
  });
}

abstract class VideoController implements Listenable {
  VideoState get state;
  // TODO: Should this return int milliseconds instead?
  double get position;
  set position(double value);
  double get duration;
  bool get paused;
  bool get loading;
  int get currentItemIndex;
  BoxFit get fit;
  double get playbackSpeed;
  List<SubtitleTrack> get currentTextTracks;

  bool get supportsAudioTrackSelection;
  bool get supportsEmbeddedSubtitles;

  void load(List<VideoItem> items, int startIndex, double startPosition);
  void play();
  void pause();
  void seekToNextItem();
  void seekToPreviousItem();
  void setAudioTrack(int index);
  void setSubtitleTrack(String? trackId);
  void setFit(BoxFit fit);
  void setPlaybackSpeed(double speed);
  void dispose();
}

class MediaPositionHandler {
  int _lastKnownPositionMs = 0;
  DateTime _lastKnownPositionTime = DateTime.now();

  double _playbackSpeed = 1.0;
  bool _isPlaying = false;

  double get positionMs {
    double position = _lastKnownPositionMs.toDouble();
    if (_isPlaying) {
      final msSinceLastPosition =
          DateTime.now().difference(_lastKnownPositionTime).inMilliseconds;
      position += msSinceLastPosition.toDouble() * _playbackSpeed;
    }
    return position;
  }

  void update({
    required int positionMs,
    required bool isPlaying,
    required double speed,
  }) {
    _lastKnownPositionMs = positionMs;
    _lastKnownPositionTime = DateTime.now();
    _isPlaying = isPlaying;
    _playbackSpeed = speed;
  }
}
