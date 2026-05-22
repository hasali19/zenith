import 'package:flutter/widgets.dart';
import 'package:media_kit/media_kit.dart' as mk;
import 'package:media_kit_video/media_kit_video.dart' as mkv;

import '../video_player_platform_interface.dart';

class VideoPlayerLinux extends VideoPlayerPlatform {
  static void registerWith() {
    VideoPlayerPlatform.instance = VideoPlayerLinux();
  }

  @override
  Widget buildView(VideoController controller) {
    if (controller is _VideoControllerLinux) {
      return mkv.Video(controller: controller._controller);
    } else {
      throw ArgumentError();
    }
  }

  @override
  Future<VideoController> createController({
    Map<String, String>? headers,
  }) async {
    mk.MediaKit.ensureInitialized();
    return _VideoControllerLinux(mk.Player(), headers ?? {});
  }
}

class _VideoControllerLinux extends VideoController with ChangeNotifier {
  final mk.Player _player;
  final mkv.VideoController _controller;
  final Map<String, String> _headers;

  _VideoControllerLinux(this._player, this._headers)
    : _controller = mkv.VideoController(_player) {
    _player.stream.duration.listen((event) => notifyListeners());
    _player.stream.playing.listen((event) => notifyListeners());
    _player.stream.playlist.listen((event) => notifyListeners());
    _player.stream.completed.listen((event) => notifyListeners());
    _player.stream.buffering.listen((event) => notifyListeners());
    _player.stream.rate.listen((event) => notifyListeners());
    _player.stream.track.listen((event) => notifyListeners());
    _player.stream.tracks.listen((event) => notifyListeners());
  }

  @override
  double get position =>
      _player.state.position.inMilliseconds.toDouble() / 1000;

  @override
  set position(double value) {
    _player.seek(Duration(seconds: value.toInt()));
  }

  @override
  // TODO: implement activeSubtitleTrackId
  String? get activeSubtitleTrackId => null;

  @override
  List<AudioTrack> get availableAudioTracks => _player
      .state
      .tracks
      .audio
      .indexed
      .map(
        (track) => AudioTrack(
          index: track.$1,
          language: track.$2.language ?? 'Unknown',
          codec: track.$2.codec ?? 'Unknown',
        ),
      )
      .toList();

  @override
  int get currentItemIndex => _player.state.playlist.index;

  @override
  // TODO: implement currentSubtitleTracks
  List<SubtitleTrack> get currentSubtitleTracks => [];

  @override
  void dispose() {
    super.dispose();
    _player.dispose();
  }

  @override
  double get duration =>
      _player.state.duration.inMilliseconds.toDouble() / 1000;

  @override
  // TODO: implement fit
  BoxFit get fit => BoxFit.contain;

  @override
  void load(List<VideoItem> items, int startIndex, double startPosition) {
    _player.open(
      mk.Playlist(
        items.indexed.map((e) {
          final (i, item) = e;
          final source = switch (item.source) {
            NetworkSource(:final url) => url,
            LocalFileSource(:final path) => path,
          };
          final start = startIndex == i
              ? Duration(seconds: startPosition.toInt())
              : null;
          return mk.Media(source, httpHeaders: _headers, start: start);
        }).toList(),
        index: startIndex,
      ),
    );
  }

  @override
  bool get loading => _player.state.buffering;

  @override
  void pause() {
    _player.pause();
  }

  @override
  bool get paused => !_player.state.playing;

  @override
  void play() {
    _player.play();
  }

  @override
  double get playbackSpeed => _player.state.rate;

  @override
  void seekToNextItem() {
    _player.next();
  }

  @override
  void seekToPreviousItem() {
    _player.previous();
  }

  @override
  void setAudioTrack(int index) {
    _player.setAudioTrack(_player.state.tracks.audio[index]);
  }

  @override
  void setFit(BoxFit fit) {
    // TODO: implement setFit
  }

  @override
  void setPlaybackSpeed(double speed) {
    _player.setRate(speed);
  }

  @override
  void setSubtitleTrack(String? trackId) {
    // TODO: implement setSubtitleTrack
  }

  @override
  VideoState get state {
    if (_player.state.completed) {
      return VideoState.ended;
    }

    if (_player.state.playlist.medias.isEmpty) {
      return VideoState.idle;
    }

    return VideoState.active;
  }

  @override
  bool get supportsAudioTrackSelection => true;

  @override
  // TODO: implement supportsEmbeddedSubtitles
  bool get supportsEmbeddedSubtitles => false;
}
