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

  ExternalSubtitleTrack({
    required this.id,
    required this.src,
    required this.mimeType,
    this.title,
    this.language,
  });
}

enum MediaType { movie, tvShow }

final class MediaMetadata {
  final MediaType? type;
  final String? title;
  final String? subtitle;
  final String? seriesTitle;
  final int? seasonNumber;
  final int? episodeNumber;
  final String? posterUrl;
  final String? backdropUrl;

  const MediaMetadata({
    this.type,
    this.title,
    this.subtitle,
    this.seriesTitle,
    this.seasonNumber,
    this.episodeNumber,
    this.posterUrl,
    this.backdropUrl,
  });
}

sealed class MediaSource {
  const MediaSource();
}

final class NetworkSource extends MediaSource {
  final String url;

  const NetworkSource(this.url);
}

final class LocalFileSource extends MediaSource {
  final String path;

  const LocalFileSource(this.path);
}

final class VideoItem {
  final MediaSource source;
  final List<ExternalSubtitleTrack> subtitles;
  final Rect? cropRect;
  final MediaMetadata metadata;
  final Object? extra;

  const VideoItem({
    required this.source,
    this.subtitles = const [],
    this.cropRect,
    this.metadata = const MediaMetadata(),
    this.extra,
  });
}

final class AudioTrack {
  final int index;
  final String language;
  final String codec;

  const AudioTrack({
    required this.index,
    required this.language,
    required this.codec,
  });
}

final class SubtitleTrack {
  final String id;
  final String? label;
  final String? language;

  const SubtitleTrack({
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

  bool get isUsingCropRects => false;
  set isUsingCropRects(bool value) {}

  Rect? get currentCropRect => null;

  List<AudioTrack> get availableAudioTracks;
  List<SubtitleTrack> get currentSubtitleTracks;

  String? get activeSubtitleTrackId;

  bool get supportsAudioTrackSelection;
  bool get supportsEmbeddedSubtitles;
  bool get supportsVideoFitting => false;
  bool get supportsCropRects => false;

  SubtitleStyleOptions? get subtitleStyle => null;

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

abstract class SubtitleStyleOptions implements Listenable {
  int get size;
  set size(int value);
}

class MediaPositionHandler {
  int _lastKnownPositionMs = 0;
  DateTime _lastKnownPositionTime = DateTime.now();

  double _playbackSpeed = 1.0;
  bool _isPlaying = false;

  double get positionMs {
    double position = _lastKnownPositionMs.toDouble();
    if (_isPlaying) {
      final msSinceLastPosition = DateTime.now()
          .difference(_lastKnownPositionTime)
          .inMilliseconds;
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
