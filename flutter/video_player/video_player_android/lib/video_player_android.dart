import 'dart:async';
import 'dart:math' as math;

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:video_player_platform_interface/video_player_platform_interface.dart';

const _methodChannel = MethodChannel("video_player_android");
const _eventChannel = EventChannel("video_player_android/events");

class VideoPlayerAndroid extends VideoPlayerPlatform {
  static registerWith() {
    VideoPlayerPlatform.instance = VideoPlayerAndroid();
  }

  @override
  Future<VideoController> createController() async {
    final int id = await _methodChannel.invokeMethod("create");
    return _VideoController(id);
  }

  @override
  Widget buildView(VideoController controller) {
    if (controller is _VideoController) {
      return ValueListenableBuilder<double>(
        valueListenable: controller.aspectRatio,
        builder: (context, aspectRatio, child) =>
            ValueListenableBuilder<BoxFit>(
          valueListenable: controller.fit,
          builder: (context, fit, child) =>
              LayoutBuilder(builder: (context, constraints) {
            var width = constraints.maxWidth;
            var height = width / aspectRatio;
            if (height > constraints.maxHeight) {
              height = constraints.maxHeight;
              width = height * aspectRatio;
            }
            return Stack(
              children: [
                Positioned.fill(
                  child: FittedBox(
                    fit: fit,
                    alignment: Alignment.center,
                    child: SizedBox(
                      width: width,
                      height: height,
                      child: Texture(textureId: controller.id),
                    ),
                  ),
                ),
                SubtitleView(
                  events: controller._subsController.stream,
                  textScale: width / 730,
                ),
              ],
            );
          }),
        ),
      );
    } else {
      throw ArgumentError.value(controller, "controller");
    }
  }

  @override
  bool get isWindowed => false;

  @override
  Future<void> enterFullscreen() async {}

  @override
  Future<void> exitFullscreen() async {}

  @override
  Future<void> toggleFullscreen() async {}
}

class SubtitleView extends StatelessWidget {
  const SubtitleView({
    Key? key,
    required this.events,
    required this.textScale,
  }) : super(key: key);

  final double textScale;
  final Stream<String?> events;

  @override
  Widget build(BuildContext context) {
    var textStyle = Theme.of(context).textTheme.titleLarge!;
    textStyle = textStyle.copyWith(
        fontSize: math.max(textStyle.fontSize! * textScale, 14));

    final outlineStyle = textStyle.copyWith(
        foreground: Paint()
          ..style = PaintingStyle.stroke
          ..strokeWidth = 2
          ..color = Colors.black);

    return Align(
      alignment: Alignment.bottomCenter,
      child: StreamBuilder<String?>(
        stream: events,
        builder: (context, snapshot) {
          final event = snapshot.data;
          if (event != null) {
            return Padding(
              padding: const EdgeInsets.only(bottom: 16),
              child: Stack(
                children: [
                  Text(event, textAlign: TextAlign.center, style: outlineStyle),
                  Text(event, textAlign: TextAlign.center, style: textStyle),
                ],
              ),
            );
          } else {
            return const SizedBox();
          }
        },
      ),
    );
  }
}

class _VideoController extends VideoController {
  final int id;
  late StreamSubscription<dynamic> _subscription;
  final StreamController<String?> _subsController =
      StreamController.broadcast();

  @override
  bool get supportsAudioTrackSelection => true;

  @override
  VideoState state = VideoState.idle;

  @override
  bool paused = false;

  @override
  double duration = 0.0;

  @override
  bool get loading => !paused && !_playing && state != VideoState.ended;

  final aspectRatio = ValueNotifier(1.0);
  final fit = ValueNotifier(BoxFit.contain);

  bool _playing = false;
  int _lastKnownPosition = 0;
  int _lastKnownPositionTs = DateTime.now().millisecondsSinceEpoch;

  @override
  double get position {
    var position = _lastKnownPosition;
    if (_playing) {
      position += DateTime.now().millisecondsSinceEpoch - _lastKnownPositionTs;
    }
    return position.toDouble() / 1000.0;
  }

  @override
  set position(value) {
    _methodChannel.invokeMethod(
        "seekTo", {"id": id, "position": (value * 1000.0).toInt()});
  }

  _VideoController(this.id) {
    _subscription = _eventChannel.receiveBroadcastStream(id).listen((event) {
      final String type = event["type"];
      if (type == "durationChanged") {
        final int duration = event["value"];
        this.duration = duration.toDouble() / 1000.0;
      } else if (type == "playWhenReadyChanged") {
        paused = !event["value"];
      } else if (type == "playbackStateChanged") {
        final int state = event["value"];
        if (state == 0) {
          this.state = VideoState.idle;
        } else if (state == 1) {
          this.state = VideoState.active;
        } else if (state == 2) {
          this.state = VideoState.ended;
        } else {
          throw ArgumentError.value(state, "value");
        }
      } else if (type == "isPlayingChanged") {
        _playing = event["value"];
      } else if (type == "aspectRatioChanged") {
        aspectRatio.value = event["value"];
      } else if (type == "cues") {
        _subsController.add(event["text"]);
      }
      if (event.containsKey("position")) {
        _lastKnownPosition = event["position"];
        _lastKnownPositionTs = DateTime.now().millisecondsSinceEpoch;
      }
      _notifyListeners();
    });
  }

  @override
  void dispose() async {
    _subscription.cancel();
    await _methodChannel.invokeMethod("dispose", {"id": id});
  }

  @override
  void load(VideoItem item) async {
    await _methodChannel.invokeMethod("load", {
      "id": id,
      "url": item.url,
      "subtitles": item.subtitles
          .map((track) => {
                "id": track.id,
                "src": track.src,
                "mimeType": track.mimeType,
                "title": track.title,
                "language": track.language
              })
          .toList(),
      "startPosition": (item.startPosition * 1000).toInt(),
    });
  }

  @override
  void setAudioTrack(int index) {
    _methodChannel.invokeMethod("setAudioTrack", {"id": id, "index": index});
  }

  @override
  void setTextTrack(SubtitleTrack? track) {
    _methodChannel
        .invokeMethod("setTextTrack", {"id": id, "trackId": track?.id});
  }

  @override
  void pause() {
    _methodChannel.invokeListMethod("pause", {"id": id});
  }

  @override
  void play() {
    _methodChannel.invokeListMethod("play", {"id": id});
  }

  @override
  void setFit(BoxFit fit) {
    this.fit.value = fit;
  }

  @override
  void setPlaybackSpeed(double speed) {
    // TODO: implement setPlaybackSped
  }

  final List<void Function()> _listeners = [];

  @override
  void addListener(void Function() listener) {
    _listeners.add(listener);
  }

  @override
  void removeListener(void Function() listener) {
    _listeners.remove(listener);
  }

  void _notifyListeners() {
    for (final listener in _listeners) {
      listener();
    }
  }
}
