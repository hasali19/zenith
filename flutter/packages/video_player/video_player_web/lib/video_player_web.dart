import 'dart:convert';
import 'dart:js_interop';
import 'dart:ui_web';

import 'package:flutter/widgets.dart';
import 'package:flutter_web_plugins/flutter_web_plugins.dart';
import 'package:http/http.dart' as http;
import 'package:video_player_platform_interface/video_player_platform_interface.dart';
import 'package:video_player_web/text_track_parser.dart';
import 'package:web/web.dart';

class VideoPlayerWeb extends VideoPlayerPlatform {
  static void registerWith(Registrar registrar) {
    VideoPlayerPlatform.instance = VideoPlayerWeb();
  }

  int nextId = 1;

  @override
  Future<VideoController> createController(
      {Map<String, String>? headers}) async {
    final id = nextId++;
    final element = document.createElement('video') as HTMLVideoElement
      ..autoplay = true
      ..style.width = '100%'
      ..style.height = '100%'
      ..style.background = 'black';

    element.setAttribute('disableRemotePlayback', 'true');

    platformViewRegistry.registerViewFactory(
        'videoplayer-$id', (viewId) => element);

    return VideoControllerWeb(id, element);
  }

  @override
  Widget buildView(VideoController controller) {
    if (controller is VideoControllerWeb) {
      return HtmlElementView(viewType: 'videoplayer-${controller.id}');
    } else {
      throw ArgumentError(
          'controller must be an instance of VideoControllerWeb');
    }
  }
}

class VideoControllerWeb extends VideoController with ChangeNotifier {
  final int id;
  final HTMLVideoElement _element;
  final Map<int, TextTrack> _textTracks = {};

  VideoState _state = VideoState.idle;
  TextTrack? _activeTextTrack;

  VideoControllerWeb(this.id, this._element) {
    _element.addEventListener(
        'durationchange', ((event) => notifyListeners()).toJS);
    _element.onPause.listen((event) => notifyListeners());
    _element.onPlay.listen((event) => notifyListeners());

    _element.onPlaying.listen((event) {
      _loading = false;
      notifyListeners();
    });

    _element.onWaiting.listen((event) {
      _loading = true;
      notifyListeners();
    });

    _element.onEnded.listen((event) {
      _state = VideoState.ended;
      notifyListeners();
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
  int currentItemIndex = 0;

  @override
  BoxFit get fit => _fit;
  BoxFit _fit = BoxFit.contain;

  @override
  double get playbackSpeed => _playbackSpeed;
  double _playbackSpeed = 1.0;

  @override
  void load(List<VideoItem> items, int startIndex, double startPosition) {
    // TODO: Implement playlist support for web
    final item = items[startIndex];
    _element.src = item.url;
    _element.crossOrigin = 'anonymous';
    _element.currentTime = startPosition;

    while (_element.firstChild != null) {
      _element.removeChild(_element.firstChild!);
    }

    _state = VideoState.active;
    _activeTextTrack = null;
    currentItemIndex = startIndex;
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
  void seekToNextItem() {
    // TODO: implement seekToNextItem
  }

  @override
  void seekToPreviousItem() {
    // TODO: implement seekToPreviousItem
  }

  @override
  void setAudioTrack(int index) {
    throw UnsupportedError(
        'Changing audio tracks is not supported by the web player');
  }

  @override
  Future<void> setTextTrack(SubtitleTrack? track) async {
    _activeTextTrack?.mode = 'hidden';
    _activeTextTrack = null;

    if (track != null) {
      var tt = _textTracks[track.id];

      if (tt == null) {
        final res = await http.get(Uri.parse(track.src));
        final contentType = res.headers['content-type'];

        final TextTrackParser parser;
        if (contentType == 'text/vtt') {
          parser = VttParser();
        } else if (contentType == 'text/srt' ||
            contentType == 'application/x-subrip') {
          parser = SrtParser();
        } else {
          console.error('unsupported text track format: $contentType'.toJS);
          return;
        }

        final data = utf8.decode(res.bodyBytes);
        final tracks = parser.parse(data);

        tt = _element.addTextTrack('subtitles');
        for (final cue in tracks) {
          tt.addCue(cue);
        }
      }

      tt.mode = 'showing';
      _activeTextTrack = tt;
    }
  }

  @override
  void setFit(BoxFit fit) {
    _element.style.objectFit = switch (fit) {
      BoxFit.cover => 'cover',
      BoxFit.contain || _ => 'contain',
    };
    _fit = fit;
    notifyListeners();
  }

  @override
  void setPlaybackSpeed(double speed) {
    _element.playbackRate = speed;
    _playbackSpeed = speed;
    notifyListeners();
  }
}
