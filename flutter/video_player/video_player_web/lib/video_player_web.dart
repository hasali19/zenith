import 'dart:html';

import 'package:flutter/widgets.dart';
import 'package:flutter_web_plugins/flutter_web_plugins.dart';
import 'package:video_player_platform_interface/video_player_platform_interface.dart';

import "shims/dart_ui.dart" as ui;

class VideoPlayerWeb extends VideoPlayerPlatform {
  static void registerWith(Registrar registrar) {
    VideoPlayerPlatform.instance = VideoPlayerWeb();
  }

  int nextId = 1;

  @override
  Future<VideoController> createController() async {
    final id = nextId++;
    final element = VideoElement()
      ..autoplay = true
      ..disableRemotePlayback = true
      ..style.width = "100%"
      ..style.height = "100%"
      ..style.background = "black";

    ui.platformViewRegistry
        .registerViewFactory("videoplayer-$id", (viewId) => element);

    return VideoControllerWeb(id, element);
  }

  @override
  Widget buildView(VideoController controller) {
    if (controller is VideoControllerWeb) {
      return HtmlElementView(viewType: "videoplayer-${controller.id}");
    } else {
      throw ArgumentError(
          "controller must be an instance of VideoControllerWeb");
    }
  }

  @override
  bool get isWindowed => true;

  @override
  Future<void> enterFullscreen() async {
    await document.documentElement?.requestFullscreen();
  }

  @override
  Future<void> exitFullscreen() async {
    document.exitFullscreen();
  }

  @override
  Future<void> toggleFullscreen() async {
    if (document.fullscreenElement == null) {
      await enterFullscreen();
    } else {
      await exitFullscreen();
    }
  }
}

class VideoControllerWeb extends VideoController {
  final int id;
  final VideoElement _element;
  final List<void Function()> _listeners = [];

  VideoState _state = VideoState.idle;
  TextTrack? _activeTextTrack;

  VideoControllerWeb(this.id, this._element) {
    _element.addEventListener("durationchange", (event) => _notifyListeners());
    _element.addEventListener("pause", (event) => _notifyListeners());
    _element.addEventListener("play", (event) => _notifyListeners());

    _element.addEventListener("playing", (event) {
      _loading = false;
      _notifyListeners();
    });

    _element.addEventListener("waiting", (event) {
      _loading = true;
      _notifyListeners();
    });

    _element.addEventListener("ended", (event) {
      _state = VideoState.ended;
      _notifyListeners();
    });
  }

  @override
  bool get supportsAudioTrackSelection => false;

  @override
  VideoState get state => _state;

  @override
  double get position => _element.currentTime.toDouble();

  @override
  set position(double value) {
    _element.currentTime = value;
  }

  @override
  double get duration {
    final value = _element.duration.toDouble();
    return value.isNaN ? 0 : value;
  }

  @override
  bool get paused => _element.paused;

  @override
  bool get loading => _loading && !paused && state != VideoState.ended;
  bool _loading = true;

  @override
  void load(VideoItem item) {
    _element.src = item.url;
    _element.crossOrigin = "anonymous";
    _element.currentTime = item.startPosition;
    _element.children.clear();

    for (final subtitle in item.subtitles) {
      _element.children.add(
        TrackElement()
          ..id = "subtitle-track-${subtitle.id}"
          ..kind = "subtitles"
          ..src = subtitle.src
          ..srclang = subtitle.language
          ..label = subtitle.title,
      );
    }

    _state = VideoState.active;
    _activeTextTrack = null;
  }

  @override
  void play() {
    _element.play();
  }

  @override
  void pause() {
    _element.pause();
  }

  @override
  void setAudioTrack(int index) {
    throw UnsupportedError(
        'Changing audio tracks is not supported by the web player');
  }

  @override
  void setTextTrack(SubtitleTrack? track) {
    if (track == null) {
      _activeTextTrack?.mode = 'hidden';
      _activeTextTrack = null;
    } else {
      final trackElement = _element
          .querySelector("#subtitle-track-${track.id}")! as TrackElement;
      final textTrack = trackElement.track!;
      textTrack.mode = 'showing';
      _activeTextTrack = textTrack;
    }
  }

  @override
  void setFit(BoxFit fit) {
    _element.style.objectFit = () {
      switch (fit) {
        case BoxFit.cover:
          return "cover";
        case BoxFit.contain:
          return "contain";
        default:
          return "contain";
      }
    }();
  }

  @override
  void addListener(void Function() listener) {
    _listeners.add(listener);
  }

  @override
  void removeListener(void Function() listener) {
    _listeners.remove(listener);
  }

  @override
  void dispose() {
    _listeners.clear();
  }

  void _notifyListeners() {
    for (final listener in _listeners) {
      listener();
    }
  }
}
