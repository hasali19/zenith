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
            var textStyle = Theme.of(context).textTheme.titleLarge!;
            textStyle = textStyle.copyWith(
                fontSize: math.max(textStyle.fontSize! * (width / 730), 14));
            return Center(
              child: SizedBox(
                width: width,
                height: height,
                child: Stack(children: [
                  Texture(textureId: controller.id),
                  Align(
                    alignment: Alignment.bottomCenter,
                    child: StreamBuilder<String?>(
                      stream: controller._subsController.stream,
                      builder: (context, snapshot) {
                        final event = snapshot.data;
                        if (event != null) {
                          return Padding(
                            padding: const EdgeInsets.only(bottom: 16),
                            child: Stack(
                              children: [
                                Text(
                                  event,
                                  textAlign: TextAlign.center,
                                  style: textStyle.copyWith(
                                      foreground: Paint()
                                        ..style = PaintingStyle.stroke
                                        ..strokeWidth = 2
                                        ..color = Colors.black),
                                ),
                                Text(
                                  event,
                                  textAlign: TextAlign.center,
                                  style: textStyle,
                                ),
                              ],
                            ),
                          );
                        } else {
                          return const SizedBox();
                        }
                      },
                    ),
                  ),
                ]),
              ),
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

class _VideoController extends VideoController {
  final int id;
  late StreamSubscription<dynamic> _subscription;
  final StreamController<String?> _subsController =
      StreamController.broadcast();

  @override
  VideoState state = VideoState.idle;

  @override
  bool paused = false;

  @override
  double duration = 0.0;

  final aspectRatio = ValueNotifier(1.0);
  final fit = ValueNotifier(BoxFit.contain);

  bool playing = false;
  int _lastKnownPosition = 0;
  int _lastKnownPositionTs = DateTime.now().millisecondsSinceEpoch;

  @override
  double get position {
    var position = _lastKnownPosition;
    if (playing) {
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
        playing = event["value"];
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
  void load(
      String url, List<SubtitleTrack> subtitles, double startPosition) async {
    await _methodChannel.invokeMethod("load", {
      "id": id,
      "url": url,
      "subtitles": subtitles
          .map((track) => {
                "id": track.id,
                "src": track.src,
                "title": track.title,
                "language": track.language
              })
          .toList(),
      "startPosition": (startPosition * 1000).toInt(),
    });
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
