import 'dart:async';
import 'dart:math' as math;

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:signals/signals_flutter.dart';

import '../video_player_platform_interface.dart';

const _methodChannel = MethodChannel('video_player');
const _eventChannel = EventChannel('video_player/events');

class VideoPlayerAndroid extends VideoPlayerPlatform {
  static registerWith() {
    VideoPlayerPlatform.instance = VideoPlayerAndroid();
  }

  @override
  Future<VideoController> createController(
      {Map<String, String>? headers}) async {
    final int id =
        await _methodChannel.invokeMethod('create', {'headers': headers});
    return VideoControllerAndroid(id);
  }

  @override
  Widget buildView(VideoController controller) {
    if (controller is VideoControllerAndroid) {
      return _VideoView(controller: controller);
    } else {
      throw ArgumentError.value(controller, 'controller');
    }
  }
}

class _VideoView extends StatelessWidget {
  final VideoControllerAndroid controller;

  const _VideoView({required this.controller});

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        final pixelRatio = MediaQuery.devicePixelRatioOf(context);

        final videoSize = controller.size.watch(context);
        final fit = controller._fit.watch(context);
        final cropRect = controller._cropRect.watch(context);

        final size = Size(
          videoSize.width / pixelRatio,
          videoSize.height / pixelRatio,
        );

        Widget video = Container(
          width: size.width,
          height: size.height,
          color: Colors.blue,
          child: Texture(
            textureId: controller.id,
            filterQuality: FilterQuality.medium,
          ),
        );

        if (cropRect case Rect cropRect) {
          final cropOffset = cropRect.topLeft / pixelRatio;
          final cropSize = cropRect.size / pixelRatio;

          video = SizedOverflowBox(
            size: Size(cropSize.width, cropSize.height),
            alignment: Alignment(
              size.width == cropSize.width
                  ? 0
                  : cropOffset.dx / (size.width - cropSize.width) * 2 - 1,
              size.height == cropSize.height
                  ? 0
                  : cropOffset.dy / (size.height - cropSize.height) * 2 - 1,
            ),
            child: ClipRect(
              clipper: _VideoClipper(cropOffset & cropSize),
              child: video,
            ),
          );
        }

        return Stack(
          children: [
            Positioned.fill(
              child: FittedBox(
                fit: fit,
                clipBehavior: Clip.hardEdge,
                alignment: Alignment.center,
                child: video,
              ),
            ),
            _SubtitleView(
              events: controller._subsController.stream,
              textScale: constraints.maxWidth / 900,
            ),
          ],
        );
      },
    );
  }
}

class _VideoClipper extends CustomClipper<Rect> {
  final Rect rect;

  const _VideoClipper(this.rect);

  @override
  Rect getClip(Size size) {
    return rect;
  }

  @override
  bool shouldReclip(covariant _VideoClipper oldClipper) {
    return rect != oldClipper.rect;
  }
}

class _SubtitleView extends StatelessWidget {
  const _SubtitleView({
    required this.events,
    required this.textScale,
  });

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
              padding: const EdgeInsets.only(bottom: 24),
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

class VideoControllerAndroid extends VideoController with ChangeNotifier {
  final int id;
  late StreamSubscription<dynamic> _subscription;
  final StreamController<String?> _subsController =
      StreamController.broadcast();

  final _positionHandler = MediaPositionHandler();

  double _playbackSpeed = 1.0;
  List<AudioTrack> _audioTracks = [];
  List<SubtitleTrack> _subtitleTracks = [];
  String? _activeTextTrack;
  List<Rect?>? _cropRects;

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

  @override
  int currentItemIndex = 0;

  final size = signal(Size.zero);
  final _fit = signal(BoxFit.contain);
  final _cropRect = signal<Rect?>(null);

  @override
  BoxFit get fit => _fit.value;

  @override
  double get playbackSpeed => _playbackSpeed;

  bool _playing = false;

  @override
  double get position => _positionHandler.positionMs / 1000;

  @override
  List<AudioTrack> get availableAudioTracks => _audioTracks;

  @override
  List<SubtitleTrack> get currentSubtitleTracks => _subtitleTracks;

  @override
  String? get activeSubtitleTrackId => _activeTextTrack;

  @override
  bool get supportsEmbeddedSubtitles => true;

  @override
  set position(value) {
    _methodChannel.invokeMethod(
        'seekTo', {'id': id, 'position': (value * 1000.0).toInt()});
  }

  VideoControllerAndroid(this.id) {
    _subscription = _eventChannel.receiveBroadcastStream(id).listen((event) {
      final String type = event['type'];
      if (type == 'durationChanged') {
        final int duration = event['value'];
        this.duration = duration.toDouble() / 1000.0;
      } else if (type == 'playWhenReadyChanged') {
        paused = !event['value'];
      } else if (type == 'playbackStateChanged') {
        final int state = event['value'];
        if (state == 0) {
          this.state = VideoState.idle;
        } else if (state == 1) {
          this.state = VideoState.active;
        } else if (state == 2) {
          this.state = VideoState.ended;
        } else {
          throw ArgumentError.value(state, 'value');
        }
      } else if (type == 'isPlayingChanged') {
        _playing = event['value'];
      } else if (type == 'videoSizeChanged') {
        int width = event['width'];
        int height = event['height'];
        size.value = Size(width.toDouble(), height.toDouble());
      } else if (type == 'cues') {
        _subsController.add(event['text']);
      } else if (type == 'playbackSpeed') {
        _playbackSpeed = event['speed'];
      } else if (type == 'mediaItemTransition') {
        currentItemIndex = event['index'];
        _cropRect.value = _cropRects?[currentItemIndex];
      } else if (type == 'tracksChanged') {
        _audioTracks = [];
        _subtitleTracks = [];

        List<dynamic> tracks = event['tracks'];

        for (final (index, track) in tracks.indexed) {
          switch (track['type']) {
            case 2:
              _audioTracks.add(AudioTrack(
                index: index,
                language: track['lang'],
                codec: track['codec'],
              ));
              break;

            case 3:
              _subtitleTracks.add(SubtitleTrack(
                id: track['id'],
                label: track['label'],
                language: track['lang'],
              ));
              break;
          }
        }

        _activeTextTrack = event['activeTextTrack'];
      }
      if (event.containsKey('position')) {
        _positionHandler.update(
          positionMs: event['position'],
          isPlaying: _playing,
          speed: _playbackSpeed,
        );
      }
      notifyListeners();
    });
  }

  @override
  void dispose() async {
    super.dispose();
    _subscription.cancel();
    await _methodChannel.invokeMethod('dispose', {'id': id});
  }

  @override
  void load(List<VideoItem> items, int startIndex, double startPosition) async {
    toSubtitleDto(ExternalSubtitleTrack track) => {
          'id': track.id,
          'src': track.src,
          'mimeType': track.mimeType,
          'title': track.title,
          'language': track.language
        };

    toItemDto(VideoItem item) => {
          'url': item.url,
          'subtitles': item.subtitles.map(toSubtitleDto).toList(),
        };

    await _methodChannel.invokeMethod(
      'load',
      {
        'id': id,
        'items': items.map(toItemDto).toList(),
        'startIndex': startIndex,
        'startPosition': (startPosition * 1000).toInt(),
      },
    );

    _cropRects = items.map((item) => item.cropRect).toList();
    _cropRect.value = items[startIndex].cropRect;
  }

  @override
  void setAudioTrack(int index) {
    _methodChannel.invokeMethod('setAudioTrack', {'id': id, 'index': index});
  }

  @override
  void setSubtitleTrack(String? trackId) {
    _methodChannel.invokeMethod('setTextTrack', {'id': id, 'trackId': trackId});
  }

  @override
  void pause() {
    _methodChannel.invokeListMethod('pause', {'id': id});
  }

  @override
  void play() {
    _methodChannel.invokeListMethod('play', {'id': id});
  }

  @override
  void seekToNextItem() {
    _methodChannel.invokeListMethod('seekToNextItem', {'id': id});
  }

  @override
  void seekToPreviousItem() {
    _methodChannel.invokeListMethod('seekToPreviousItem', {'id': id});
  }

  @override
  void setFit(BoxFit fit) {
    _fit.value = fit;
    notifyListeners();
  }

  @override
  void setPlaybackSpeed(double speed) {
    _methodChannel.invokeMethod('setPlaybackSpeed', {'id': id, 'speed': speed});
  }
}