import 'dart:html';

import 'package:flutter/widgets.dart';
import 'package:flutter_web_plugins/flutter_web_plugins.dart';
import 'package:video_player_platform_interface/video_player_platform_interface.dart';

import "shims/dart_ui.dart" as ui;

class VideoPlayerPlugin extends VideoPlayerPlatform {
  static void registerWith(Registrar registrar) {
    VideoPlayerPlatform.instance = VideoPlayerPlugin();
  }

  int nextId = 1;

  @override
  VideoController createController() {
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
  Widget createView(VideoController controller) {
    if (controller is VideoControllerWeb) {
      return HtmlElementView(viewType: "videoplayer-${controller.id}");
    } else {
      throw ArgumentError(
          "controller must be an instance of VideoControllerWeb");
    }
  }

  @override
  void toggleFullscreen() {
    if (document.fullscreenElement == null) {
      document.documentElement?.requestFullscreen();
    } else {
      document.exitFullscreen();
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
    _element.addEventListener("ended", (event) {
      _state = VideoState.ended;
      _notifyListeners();
    });
  }

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
  void load(
    String url,
    List<SubtitleTrack> subtitles,
    double startPosition,
  ) {
    _element.src = url;
    _element.crossOrigin = "anonymous";
    _element.currentTime = startPosition;
    _element.children.clear();

    for (final subtitle in subtitles) {
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

// class VideoView extends StatefulWidget {
//   final void Function(VideoController controller) onReady;

//   const VideoView({Key? key, required this.onReady}) : super(key: key);

//   @override
//   State<VideoView> createState() => _VideoViewState();
// }

// class _VideoViewState extends State<VideoView> {
//   static final Map<int, VideoElement> _views = {};

//   late int _id;
//   late VideoControllerWeb _controller;

//   @override
//   void initState() {
//     super.initState();
//     ui.platformViewRegistry.registerViewFactory("video-player", (viewId) {
//       final view = VideoElement()
//         ..autoplay = true
//         ..disableRemotePlayback = true
//         ..style.background = "black";
//       _views[viewId] = view;
//       return view;
//     });
//   }

//   @override
//   Widget build(BuildContext context) {
//     return HtmlElementView(
//       viewType: "video-player",
//       onPlatformViewCreated: (id) => setState(() {
//         _id = id;
//         _controller = VideoControllerWeb(_views[id]!);
//         widget.onReady(_controller);
//       }),
//     );
//   }

//   @override
//   void dispose() {
//     super.dispose();
//     _controller.dispose();
//     _views.remove(_id);
//   }
// }
