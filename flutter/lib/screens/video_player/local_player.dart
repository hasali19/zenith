import 'dart:async';
import 'dart:math';
import 'dart:ui';

import 'package:dio_cookie_manager/dio_cookie_manager.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:sized_context/sized_context.dart';
import 'package:video_player/video_player.dart';
import 'package:wakelock_plus/wakelock_plus.dart';
import 'package:zenith/api.dart' as api;
import 'package:zenith/cookies.dart';
import 'package:zenith/platform.dart' as platform;
import 'package:zenith/preferences.dart';
import 'package:zenith/screens/video_player/media_title.dart';
import 'package:zenith/screens/video_player/ui.dart';
import 'package:zenith/screens/video_player/utils.dart';
import 'package:zenith/window.dart';

class LocalVideoPlayer extends ConsumerStatefulWidget {
  final List<api.MediaItem> items;
  final int startIndex;
  final double startPosition;

  const LocalVideoPlayer({
    super.key,
    required this.items,
    required this.startIndex,
    required this.startPosition,
  });

  @override
  ConsumerState<LocalVideoPlayer> createState() {
    return _VideoPlayerState();
  }
}

class _VideoPlayerState extends ConsumerState<LocalVideoPlayer> {
  api.ZenithApiClient get _api => ref.read(api.apiProvider);

  late FocusNode _focusNode;

  VideoController? _controller;
  bool _shouldShowControls = true;

  bool _isPaused = false;
  VideoState _videoState = VideoState.idle;

  late Timer _progressReportTimer;
  Timer? _controlsTimer;

  api.MediaItem get currentItem =>
      widget.items[_controller?.currentItemIndex ?? widget.startIndex];

  List<api.SubtitleTrack> get subtitles =>
      currentItem.videoFile?.subtitles ?? [];

  bool get shouldShowControls =>
      _shouldShowControls || _videoState == VideoState.ended;

  @override
  void initState() {
    super.initState();

    _focusNode = FocusNode();

    WakelockPlus.enable();
    platform.setPipEnabled(true);
    platform.setExtendIntoCutout(true);

    _progressReportTimer = Timer.periodic(
        const Duration(seconds: 5), (timer) => _onProgressReporterTick());

    _showControls();

    Future.microtask(_initController);
  }

  @override
  void dispose() {
    super.dispose();
    WakelockPlus.disable();
    _controlsTimer?.cancel();
    _progressReportTimer.cancel();
    _controller?.dispose();
    platform.setPipEnabled(false);
  }

  Future<void> _initController() async {
    final cookies = ref.read(cookieJarProvider);

    final controller = await VideoPlayerPlatform.instance.createController(
      headers: {
        'Cookie': CookieManager.getCookies(
            await cookies.loadForRequest(Uri.parse(_api.baseUrl)))
      },
    );

    final videos = widget.items.map((item) {
      final String? title;
      final String? subtitle;
      if (item.type == api.MediaType.movie) {
        title = item.name;
        subtitle = item.startDate?.year.toString();
      } else {
        title = '${item.getSeasonEpisode()!}: ${item.name}';
        subtitle = item.grandparent!.name;
      }

      final videoStream =
          item.videoFile?.streams.whereType<api.VideoStreamInfo>().firstOrNull;

      return VideoItem(
        url: _api.getVideoUrl(item.videoFile!.id),
        subtitles: item.videoFile!.subtitles
            .where((s) =>
                !controller.supportsEmbeddedSubtitles || s.streamIndex == null)
            .map((s) => subtitleFromApi(_api, s))
            .toList(),
        metadata: MediaMetadata(
          title: title,
          subtitle: subtitle,
        ),
        cropRect: switch ((videoStream?.crop1, videoStream?.crop2)) {
          ((int x1, int y1), (int x2, int y2)) => Rect.fromPoints(
              Offset(x1.toDouble(), y1.toDouble()),
              Offset(x2.toDouble(), y2.toDouble()),
            ),
          _ => null,
        },
      );
    }).toList();

    setState(() {
      _controller = controller
        ..load(videos, widget.startIndex, widget.startPosition)
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
    });
  }

  void _onProgressReporterTick() {
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
      _api.updateProgress(currentItem.id, position);
    }
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
    final content = Stack(
      children: [
        Positioned.fill(
          child: VideoPlayerPlatform.instance.buildView(_controller!),
        ),
        GestureDetector(
          behavior: HitTestBehavior.opaque,
          onTap: _toggleControls,
          child: _buildUi(),
        )
      ],
    );

    return KeyboardListener(
      focusNode: _focusNode,
      autofocus: true,
      onKeyEvent: _onKeyEvent,
      child: MouseRegion(
        cursor:
            shouldShowControls ? MouseCursor.defer : SystemMouseCursors.none,
        child: Listener(
          behavior: HitTestBehavior.opaque,
          onPointerHover: (e) {
            if (e.kind == PointerDeviceKind.mouse) {
              _showControls();
            }
          },
          child: content,
        ),
      ),
    );
  }

  void _onKeyEvent(KeyEvent event) {
    final window = ref.read(windowProvider);
    if (event is KeyUpEvent) {
      if (event.logicalKey == LogicalKeyboardKey.space) {
        _togglePaused();
      } else if (event.logicalKey == LogicalKeyboardKey.arrowLeft) {
        _controller?.position -= 10;
      } else if (event.logicalKey == LogicalKeyboardKey.arrowRight) {
        _controller?.position += 30;
      } else if (!kIsWeb) {
        // Browser handles keyboard shortcuts for toggling fullscreen mode itself
        if (event.logicalKey == LogicalKeyboardKey.escape) {
          window.setFullscreen(false);
        } else if (event.logicalKey == LogicalKeyboardKey.f11) {
          window.toggleFullscreen();
        }
      }
    }
  }

  void _togglePaused() {
    final controller = _controller;
    if (controller != null) {
      if (controller.paused) {
        controller.play();
      } else {
        controller.pause();
      }
    }
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

    return PopScope(
      onPopInvokedWithResult: _onPopInvoked,
      child: Scaffold(
        backgroundColor: Colors.black,
        body: content,
      ),
    );
  }

  Widget _buildUi() {
    final controller = _controller!;
    final content = ListenableBuilder(
      listenable: controller,
      builder: (context, child) => VideoPlayerUi(
        title: MediaTitle(item: currentItem),
        controller: controller,
        onInteractionStart: _disableAutoHideControls,
        onInteractionEnd: _resetControlsTimer,
        onSeekToNext: _onSeekToNext,
      ),
    );

    return ValueListenableBuilder(
      valueListenable: platform.isInPipMode,
      builder: (context, isInPipMode, child) {
        return AnimatedSwitcher(
          duration: const Duration(milliseconds: 200),
          transitionBuilder: (child, animation) => FadeTransition(
            opacity: animation,
            child: child,
          ),
          child: (shouldShowControls && !isInPipMode) ||
                  ModalRoute.of(context)?.isCurrent == false
              ? content
              : const SizedBox.expand(),
        );
      },
    );
  }

  void _onSeekToNext() {
    if (ref.read(setWatchedOnSkipProvider)) {
      _api.updateUserData(
          currentItem.id, api.VideoUserDataPatch(isWatched: true));
    }
  }

  Future<void> _onPopInvoked(bool didPop, dynamic result) async {
    if (didPop) {
      final window = ref.read(windowProvider);
      if (window.isWindowed) {
        await window.setFullscreen(false);
      } else {
        await Future.wait([
          platform.setExtendIntoCutout(false),
          platform.setSystemBarsVisible(true),
        ]);
      }
    }
  }
}
