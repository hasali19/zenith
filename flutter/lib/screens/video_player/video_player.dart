import 'dart:async';
import 'dart:math';

import 'package:auto_route/auto_route.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:sized_context/sized_context.dart';
import 'package:video_player/video_player.dart';
import 'package:wakelock/wakelock.dart';
import 'package:zenith/theme.dart';

import '../../api.dart' as api;
import '../../platform.dart' as platform;
import 'ui.dart';
import 'utils.dart';
import 'video_progress_bar.dart';

class VideoPlayerScreen extends ConsumerStatefulWidget {
  final int id;
  final double startPosition;

  const VideoPlayerScreen({
    Key? key,
    @pathParam required this.id,
    @queryParam this.startPosition = 0,
  }) : super(key: key);

  @override
  ConsumerState<VideoPlayerScreen> createState() => _VideoPlayerScreenState();
}

class _VideoPlayerScreenState extends ConsumerState<VideoPlayerScreen> {
  late Future<api.MediaItem> _item;
  api.ZenithApiClient get _api => ref.watch(api.apiProvider);

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    _item = _api.fetchMediaItem(widget.id);
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<api.MediaItem>(
      future: _item,
      builder: (context, snapshot) {
        if (snapshot.hasData) {
          return _VideoPlayer(
            item: snapshot.data!,
            startPosition: widget.startPosition,
          );
        } else {
          return const Center(child: CircularProgressIndicator());
        }
      },
    );
  }
}

class _VideoPlayer extends ConsumerStatefulWidget {
  final api.MediaItem item;
  final double startPosition;

  const _VideoPlayer({
    Key? key,
    required this.item,
    required this.startPosition,
  }) : super(key: key);

  @override
  ConsumerState<_VideoPlayer> createState() {
    return _VideoPlayerState();
  }
}

class _VideoPlayerState extends ConsumerState<_VideoPlayer> {
  api.ZenithApiClient get _api => ref.watch(api.apiProvider);

  late FocusNode _focusNode;

  VideoController? _controller;
  bool _shouldShowControls = true;

  bool _isPaused = false;
  VideoState _videoState = VideoState.idle;

  final _progressController = ProgressController();

  late Timer _progressReportTimer;
  Timer? _controlsTimer;

  List<api.SubtitleTrack> get subtitles =>
      widget.item.videoFile?.subtitles ?? [];

  @override
  void initState() {
    super.initState();
    Wakelock.enable();

    _focusNode = FocusNode();

    platform.setPipEnabled(true);

    VideoPlayerPlatform.instance.createController().then((controller) {
      setState(() {
        _controller = controller
          ..load(
            _api.getVideoUrl(widget.item.id),
            subtitles.map((s) => subtitleFromApi(_api, s)).toList(),
            widget.startPosition,
          )
          ..addListener(() {
            setState(() {
              _videoState = controller.state;
            });

            if (_isPaused != controller.paused) {
              // video was paused or unpaused
              _isPaused = controller.paused;
              _showControls();
            }
          });
        _progressController.init(controller);
      });
    });

    _progressReportTimer = Timer.periodic(const Duration(seconds: 5), (timer) {
      final controller = _controller;
      if (controller == null) return;

      final position = controller.position.toInt();
      if (kReleaseMode &&
          controller.state == VideoState.active &&
          controller.paused == false &&
          position > 0) {
        // TODO: Be smarter about progress reporting
        // - report when playback state changes, after seeking, etc
        // - maybe disable timer altogether when video is paused?
        _api.updateProgress(widget.item.id, position);
      }
    });

    platform.setExtendIntoCutout(true);

    _showControls();
  }

  @override
  void dispose() {
    super.dispose();
    Wakelock.disable();
    _controlsTimer?.cancel();
    _progressReportTimer.cancel();
    _progressController.dispose();
    _controller?.dispose();
    platform.setPipEnabled(false);
  }

  void _toggleControls() {
    if (!_shouldShowControls) {
      _showControls();
    } else {
      _hideControls();
    }
  }

  void _hideControls() {
    _controlsTimer?.cancel();
    if (_shouldShowControls) {
      platform.setSystemBarsVisible(false);
      setState(() => _shouldShowControls = false);
    }
  }

  void _showControls() {
    if (!_shouldShowControls) {
      platform.setSystemBarsVisible(true);
      setState(() => _shouldShowControls = true);
    }
    _resetControlsTimer();
  }

  void _disableAutoHideControls() {
    _controlsTimer?.cancel();
    if (!_shouldShowControls) {
      platform.setSystemBarsVisible(true);
      setState(() => _shouldShowControls = true);
    }
  }

  void _resetControlsTimer() {
    _controlsTimer?.cancel();
    if (!_isPaused) {
      _controlsTimer = Timer(const Duration(seconds: 5), _hideControls);
    }
  }

  Widget _buildPlayer(VideoController controller) {
    return KeyboardListener(
      focusNode: _focusNode,
      autofocus: true,
      onKeyEvent: _onKeyEvent,
      child: Listener(
        behavior: HitTestBehavior.opaque,
        onPointerHover: (e) {
          if (e.kind == PointerDeviceKind.mouse) {
            _showControls();
          }
        },
        child: Stack(
          children: [
            Positioned.fill(
              child: VideoPlayerPlatform.instance.buildView(_controller!),
            ),
            ValueListenableBuilder<bool>(
              valueListenable: platform.isInPipMode,
              builder: (context, isInPipMode, child) {
                if (isInPipMode) return const SizedBox.expand();
                return GestureDetector(
                  behavior: HitTestBehavior.opaque,
                  onTap: _toggleControls,
                  child: AnimatedSwitcher(
                    duration: const Duration(milliseconds: 200),
                    transitionBuilder: (child, animation) => FadeTransition(
                      opacity: animation,
                      child: child,
                    ),
                    child: _buildUi(),
                  ),
                );
              },
            )
          ],
        ),
      ),
    );
  }

  void _onKeyEvent(KeyEvent event) {
    if (kIsWeb) {
      // Browser handles keyboard shortcuts for toggling fullscreen mode itself
      return;
    }

    if (event is KeyUpEvent) {
      if (event.logicalKey == LogicalKeyboardKey.escape) {
        VideoPlayerPlatform.instance.exitFullscreen();
      } else if (event.logicalKey == LogicalKeyboardKey.f11) {
        VideoPlayerPlatform.instance.toggleFullscreen();
      }
    }
  }

  Widget _buildUi() {
    if (!_shouldShowControls && _videoState != VideoState.ended) {
      return const SizedBox.expand();
    }

    final Widget title;
    if (widget.item.type == api.MediaType.episode) {
      title = Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            widget.item.getSeasonEpisode()! + ": " + widget.item.name,
            style: context.zenithTheme.titleMedium,
          ),
          Text(
            widget.item.grandparent!.name,
            style: context.zenithTheme.bodyMedium,
          ),
        ],
      );
    } else {
      title = Text(widget.item.name);
    }

    return VideoPlayerUi(
      controller: _controller!,
      title: title,
      audioTracks: widget.item.videoFile!.streams
          .whereType<api.AudioStreamInfo>()
          .map(audioTrackFromApi)
          .toList(),
      subtitles: widget.item.videoFile!.subtitles
          .map((s) => subtitleFromApi(_api, s))
          .toList(),
      progress: _progressController.stream,
      onInteractionStart: _disableAutoHideControls,
      onInteractionEnd: _resetControlsTimer,
    );
  }

  @override
  Widget build(BuildContext context) {
    final content = ValueListenableBuilder<EdgeInsets>(
      valueListenable: platform.stableSystemBarInsets,
      builder: (context, physicalInsets, child) {
        final stableSystemBarInsets =
            physicalInsets / context.mq.devicePixelRatio;
        final insets = EdgeInsets.fromLTRB(
            max(stableSystemBarInsets.left, context.mq.padding.left),
            max(stableSystemBarInsets.top, context.mq.padding.top),
            max(stableSystemBarInsets.right, context.mq.padding.right),
            max(stableSystemBarInsets.bottom, context.mq.padding.bottom));
        return MediaQuery(
          data: MediaQuery.of(context).copyWith(padding: insets),
          child: child!,
        );
      },
      child: _controller == null
          ? const Center(child: CircularProgressIndicator())
          : _buildPlayer(_controller!),
    );

    return WillPopScope(
      onWillPop: _onWillPop,
      child: Scaffold(
        backgroundColor: Colors.black,
        body: content,
      ),
    );
  }

  Future<bool> _onWillPop() async {
    if (!VideoPlayerPlatform.instance.isWindowed) {
      await platform.setExtendIntoCutout(false);
      await platform.setSystemBarsVisible(true);
    }
    return true;
  }
}

class ProgressController {
  late final StreamController<VideoProgressData> _controller =
      StreamController.broadcast(onListen: _onListen);

  Timer? _timer;
  VideoController? _videoController;

  Stream<VideoProgressData> get stream => _controller.stream;

  void init(VideoController controller) {
    _videoController = controller;
    _timer = Timer.periodic(
        const Duration(milliseconds: 500), (timer) => _emitProgress());
  }

  void dispose() {
    _timer?.cancel();
  }

  void _emitProgress() {
    final controller = _videoController;
    if (controller == null) {
      return;
    }

    _controller.add(VideoProgressData(
      total: Duration(seconds: controller.duration.toInt()),
      progress: Duration(seconds: controller.position.toInt()),
    ));
  }

  void _onListen() {
    _emitProgress();
  }
}
