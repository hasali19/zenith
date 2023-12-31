import 'dart:async';
import 'dart:math';

import 'package:auto_route/auto_route.dart';
import 'package:cookie_jar/cookie_jar.dart';
import 'package:dio_cookie_manager/dio_cookie_manager.dart';
import 'package:dio_image_provider/dio_image_provider.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:sized_context/sized_context.dart';
import 'package:video_player/video_player.dart';
import 'package:wakelock_plus/wakelock_plus.dart';
import 'package:zenith/api.dart' as api;
import 'package:zenith/remote_playback.dart';
import 'package:zenith/remote_playback_api.g.dart';
import 'package:zenith/screens/item_details/item_details.dart';
import 'package:zenith/screens/video_player/video_player_cubit.dart';
import 'package:zenith/theme.dart';
import 'package:zenith/window.dart';

import '../../platform.dart' as platform;
import 'ui.dart';
import 'utils.dart';
import 'video_progress_bar.dart';

@RoutePage()
class VideoPlayerScreen extends ConsumerWidget {
  final int id;
  final double startPosition;

  const VideoPlayerScreen({
    Key? key,
    @pathParam required this.id,
    @queryParam this.startPosition = 0,
  }) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return BlocProvider(
      create: (context) => VideoPlayerCubit(
        ref.read(api.apiProvider),
        context.read<MediaRouter>(),
      )..loadPlaylist(id),
      child: BlocBuilder<VideoPlayerCubit, VideoPlayerState>(
        builder: (context, state) {
          if (state.playlist == null) {
            return const Center(child: CircularProgressIndicator());
          }
          final Playlist(:items, start: startIndex) = state.playlist!;
          return switch (state.location) {
            PlaybackLocation.local => _LocalVideoPlayer(
                items: items,
                startIndex: startIndex,
                startPosition: startPosition,
              ),
            PlaybackLocation.remote => _RemoteVideoPlayer(
                items: items,
                startIndex: startIndex,
                startPosition: startPosition,
              ),
          };
        },
      ),
    );
  }
}

class _RemoteVideoPlayer extends ConsumerStatefulWidget {
  final List<api.MediaItem> items;
  final int startIndex;
  final double startPosition;

  const _RemoteVideoPlayer({
    required this.items,
    required this.startIndex,
    required this.startPosition,
  });

  @override
  ConsumerState<ConsumerStatefulWidget> createState() =>
      _RemoteVideoPlayerState();
}

class _RemoteVideoPlayerState extends ConsumerState<_RemoteVideoPlayer> {
  late final api.ZenithApiClient _api;
  late final RemotePlaybackApi _remote;
  late final MediaRouter _mediaRouter;

  late final Stream<VideoProgressData> _progress;

  final _positionHandler = MediaPositionHandler();

  api.MediaItem get item => widget.items[widget.startIndex];

  @override
  void initState() {
    super.initState();
    _api = ref.read(api.apiProvider);
    _remote = RemotePlaybackApi();
    _mediaRouter = context.read<MediaRouter>();

    _progress = Stream.periodic(
      const Duration(milliseconds: 500),
      (count) => VideoProgressData(
        total: Duration(
            milliseconds:
                _mediaRouter.mediaStatus.value?.mediaInfo.streamDuration ??
                    _positionHandler.positionMs.toInt()),
        progress: Duration(milliseconds: _positionHandler.positionMs.toInt()),
      ),
    );

    _mediaRouter.mediaStatus.addListener(_onMediaStatusUpdated);

    _loadMedia();
  }

  @override
  void dispose() {
    _mediaRouter.mediaStatus.removeListener(_onMediaStatusUpdated);
    super.dispose();
  }

  void _loadMedia() async {
    final token = await _api.getAccessToken(api.AccessTokenOwner.system, 'cast',
        create: true);

    String withToken(String url) {
      final uri = Uri.parse(url);
      var params = {...uri.queryParameters, 'token': token.token};
      return uri.replace(queryParameters: params).toString();
    }

    await _remote.load(MediaLoadRequestData(
      mediaInfo: MediaLoadInfo(
        url: withToken(_api.getVideoUrl(item.videoFile!.id)),
        metadata: MediaMetadata(
          mediaType: switch (item.type) {
            api.MediaType.movie => MediaType.movie,
            api.MediaType.episode => MediaType.tvShow,
            _ => throw Error(),
          },
          title: item.name,
          seriesTitle: item.grandparent?.name,
          seasonNumber: item.grandparent?.index,
          episodeNumber: item.parent?.index,
          poster: MediaMetadataImage(
            url:
                withToken(_api.getMediaImageUrl(item.id, api.ImageType.poster)),
            width: 0,
            height: 0,
          ),
          backdrop: MediaMetadataImage(
            url: withToken(
                _api.getMediaImageUrl(item.id, api.ImageType.backdrop)),
            width: 0,
            height: 0,
          ),
        ),
      ),
    ));
  }

  void _onMediaStatusUpdated() {
    final mediaStatus = _mediaRouter.mediaStatus.value;
    if (mediaStatus == null) return;

    _positionHandler.update(
      positionMs: mediaStatus.streamPosition,
      isPlaying: mediaStatus.playerState == PlayerState.playing,
      speed: mediaStatus.playbackRate,
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Stack(
        children: [
          Positioned.fill(
            child: FadeInImage(
              placeholder: MemoryImage(transparentImage),
              image: DioImage.string(
                  _api.getMediaImageUrl(item.id, api.ImageType.backdrop)),
              fit: BoxFit.cover,
            ),
          ),
          ValueListenableBuilder(
            valueListenable: _mediaRouter.mediaStatus,
            builder: (context, mediaStatus, child) => VideoPlayerUi(
              title: _MediaTitle(item: item),
              audioTracks: const [],
              subtitles: const [],
              progress: _progress,
              isAudioTrackSelectionSupported: false,
              fit: BoxFit.cover,
              playbackSpeed: 1.0,
              isLoading: mediaStatus?.playerState == PlayerState.loading ||
                  mediaStatus?.playerState == PlayerState.buffering,
              isPaused: mediaStatus?.playerState == PlayerState.paused,
              onInteractionStart: () {},
              onInteractionEnd: () {},
              onAudioTrackSelected: (index) {},
              onTextTrackSelected: (track) {},
              onFitSelected: (fit) {},
              onPlaybackSpeedSelected: _remote.setPlaybackRate,
              onSeek: (position) {
                _remote.seek(MediaSeekOptions(
                  position: (position * 1000).toInt(),
                  resumeState: ResumeState.unchanged,
                ));
              },
              onSeekDelta: (delta) {
                _remote.seek(MediaSeekOptions(
                  position:
                      (_positionHandler.positionMs + delta * 1000).toInt(),
                  resumeState: ResumeState.unchanged,
                ));
              },
              onSeekToPrevious: () {},
              onSeekToNext: () {},
              onSetPaused: (isPaused) =>
                  isPaused ? _remote.pause() : _remote.play(),
            ),
          ),
        ],
      ),
    );
  }
}

class _LocalVideoPlayer extends ConsumerStatefulWidget {
  final List<api.MediaItem> items;
  final int startIndex;
  final double startPosition;

  const _LocalVideoPlayer({
    required this.items,
    required this.startIndex,
    required this.startPosition,
  });

  @override
  ConsumerState<_LocalVideoPlayer> createState() {
    return _VideoPlayerState();
  }
}

class _VideoPlayerState extends ConsumerState<_LocalVideoPlayer> {
  api.ZenithApiClient get _api => ref.read(api.apiProvider);

  late FocusNode _focusNode;

  VideoController? _controller;
  bool _shouldShowControls = true;

  bool _isPaused = false;
  VideoState _videoState = VideoState.idle;

  late Timer _progressReportTimer;
  Timer? _controlsTimer;

  api.MediaItem get currentItem =>
      widget.items[_controller?.currentItemIndex ?? 0];

  List<api.SubtitleTrack> get subtitles =>
      currentItem.videoFile?.subtitles ?? [];

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
    final cookies = context.read<CookieJar>();

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

      return VideoItem(
        url: _api.getVideoUrl(item.videoFile!.id),
        subtitles: item.videoFile!.subtitles
            .map((s) => subtitleFromApi(_api, s))
            .toList(),
        title: title,
        subtitle: subtitle,
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

  Widget _buildUi() {
    if (!_shouldShowControls && _videoState != VideoState.ended) {
      return const SizedBox.expand();
    }

    return ListenableBuilder(
      listenable: _controller!,
      builder: (context, child) => VideoPlayerUi(
        title: _MediaTitle(item: currentItem),
        audioTracks: currentItem.videoFile!.streams
            .whereType<api.AudioStreamInfo>()
            .map(audioTrackFromApi)
            .toList(),
        subtitles: currentItem.videoFile!.subtitles
            .map((s) => subtitleFromApi(_api, s))
            .toList(),
        progress: () => VideoProgressData(
          total: Duration(seconds: _controller!.duration.toInt()),
          progress: Duration(seconds: _controller!.position.toInt()),
        ),
        isAudioTrackSelectionSupported:
            _controller!.supportsAudioTrackSelection,
        fit: _controller!.fit,
        playbackSpeed: _controller!.playbackSpeed,
        isLoading: _controller!.loading,
        isPaused: _controller!.paused,
        onInteractionStart: _disableAutoHideControls,
        onInteractionEnd: _resetControlsTimer,
        onAudioTrackSelected: (index) => _controller!.setAudioTrack(index),
        onTextTrackSelected: (track) => _controller!.setTextTrack(track),
        onFitSelected: (fit) => _controller!.setFit(fit),
        onPlaybackSpeedSelected: (speed) =>
            _controller!.setPlaybackSpeed(speed),
        onSeek: (position) => _controller!.position = position,
        onSeekDelta: (delta) => _controller!.position += delta,
        onSeekToPrevious: () => _controller!.seekToPreviousItem(),
        onSeekToNext: () => _controller!.seekToNextItem(),
        onSetPaused: (isPaused) =>
            isPaused ? _controller!.pause() : _controller!.play(),
      ),
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
    final window = ref.read(windowProvider);
    if (window.isWindowed) {
      await window.setFullscreen(false);
    } else {
      await platform.setExtendIntoCutout(false);
      await platform.setSystemBarsVisible(true);
    }
    return true;
  }
}

class _MediaTitle extends StatelessWidget {
  final api.MediaItem item;

  const _MediaTitle({required this.item});

  @override
  Widget build(BuildContext context) {
    if (item.type == api.MediaType.episode) {
      return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            '${item.getSeasonEpisode()!}: ${item.name}',
            style: context.zenithTheme.titleMedium,
          ),
          Text(
            item.grandparent!.name,
            style: context.zenithTheme.bodyMedium,
          ),
        ],
      );
    } else {
      return Text(item.name);
    }
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
