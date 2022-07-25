import 'dart:async';
import 'dart:math';

import 'package:flutter/foundation.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:video_player/video_player.dart';
import 'package:wakelock/wakelock.dart';
import 'package:zenith_flutter/responsive.dart';

import '../api.dart' as api;

class VideoPlayerScreen extends StatefulWidget {
  final int id;
  final double startPosition;

  const VideoPlayerScreen({
    Key? key,
    required this.id,
    required this.startPosition,
  }) : super(key: key);

  @override
  State<VideoPlayerScreen> createState() => _VideoPlayerScreenState();
}

class _VideoPlayerScreenState extends State<VideoPlayerScreen> {
  late Future<api.MediaItem> _item;

  @override
  void initState() {
    super.initState();
    _item = api.fetchMediaItem(widget.id);
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<api.MediaItem>(
      future: _item,
      builder: (context, snapshot) {
        if (snapshot.hasData) {
          return _VideoPlayer(
            item: snapshot.data! as api.VideoItem,
            startPosition: widget.startPosition,
          );
        } else {
          return const Center(child: CircularProgressIndicator());
        }
      },
    );
  }
}

class _VideoPlayer extends StatefulWidget {
  final api.VideoItem item;
  final double startPosition;

  const _VideoPlayer({
    Key? key,
    required this.item,
    required this.startPosition,
  }) : super(key: key);

  @override
  State<StatefulWidget> createState() {
    return _VideoPlayerState();
  }
}

SubtitleTrack subtitleFromApi(api.SubtitleTrack subtitle) {
  return SubtitleTrack(
    id: subtitle.id.toString(),
    src: api.getSubtitleUrl(subtitle.id),
    title: subtitle.title,
    language: subtitle.language,
  );
}

class _VideoPlayerState extends State<_VideoPlayer> {
  VideoController? _controller;
  bool _shouldShowControls = true;
  bool _isTap = false;

  late Timer _progressTimer;
  Timer? _controlsTimer;

  List<api.SubtitleTrack> get subtitles =>
      widget.item.videoInfo?.subtitles ?? [];

  @override
  void initState() {
    super.initState();
    Wakelock.enable();

    VideoPlayerPlatform.instance.createController().then((controller) {
      setState(() {
        _controller = controller
          ..load(
            api.getVideoUrl(widget.item.id),
            subtitles.map(subtitleFromApi).toList(),
            widget.startPosition,
          );
      });
    });

    _progressTimer = Timer.periodic(const Duration(seconds: 5), (timer) {
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
        api.updateProgress(widget.item.id, position);
      }
    });

    if (!VideoPlayerPlatform.instance.isWindowed) {
      VideoPlayerPlatform.instance.enterFullscreen();
    }

    _showControls();
  }

  @override
  void dispose() {
    super.dispose();
    Wakelock.disable();
    _controlsTimer?.cancel();
    _progressTimer.cancel();
    _controller?.dispose();
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
    setState(() {
      _shouldShowControls = false;
    });
  }

  void _showControls() {
    setState(() {
      _shouldShowControls = true;
    });

    _controlsTimer?.cancel();
    _controlsTimer = Timer(const Duration(seconds: 5), _hideControls);
  }

  @override
  Widget build(BuildContext context) {
    final content = Scaffold(
      backgroundColor: Colors.black,
      body: _controller == null
          ? const Center(child: CircularProgressIndicator())
          : Listener(
              behavior: HitTestBehavior.opaque,
              onPointerDown: (event) => _isTap = true,
              onPointerMove: (event) {
                _showControls();
                _isTap = false;
              },
              onPointerUp: (event) {
                if (_isTap) {
                  _toggleControls();
                } else {
                  _showControls();
                }
              },
              onPointerHover: (e) {
                if (e.kind == PointerDeviceKind.mouse) {
                  _showControls();
                }
              },
              child: Stack(
                children: [
                  Center(
                    child: VideoPlayerPlatform.instance.buildView(_controller!),
                  ),
                  AnimatedSwitcher(
                    duration: const Duration(milliseconds: 200),
                    transitionBuilder: (child, animation) => FadeTransition(
                      opacity: animation,
                      child: child,
                    ),
                    child: ControlsContainer(
                      key: ValueKey<bool>(_shouldShowControls),
                      controller: _controller!,
                      item: widget.item,
                      visible: _shouldShowControls,
                    ),
                  ),
                ],
              ),
            ),
    );

    return WillPopScope(
      onWillPop: _onWillPop,
      child: content,
    );
  }

  Future<bool> _onWillPop() async {
    if (!VideoPlayerPlatform.instance.isWindowed) {
      await VideoPlayerPlatform.instance.exitFullscreen();
    }
    return true;
  }
}

class ControlsContainer extends StatelessWidget {
  final VideoController controller;
  final api.VideoItem item;
  final bool visible;

  const ControlsContainer({
    Key? key,
    required this.controller,
    required this.item,
    required this.visible,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    if (visible) {
      return _Controls(controller: controller, item: item);
    } else {
      return Container();
    }
  }
}

class _Controls extends StatefulWidget {
  final VideoController controller;
  final api.VideoItem item;

  const _Controls({
    required this.controller,
    required this.item,
  });

  @override
  State<_Controls> createState() => _ControlsState();
}

class _ControlsState extends State<_Controls> {
  VideoController get _controller => widget.controller;

  late Timer _timer;

  @override
  void initState() {
    super.initState();

    // Update on controller state change.
    _controller.addListener(_listener);

    // Controller doesn't notify position changes, so update periodically too.
    _timer = Timer.periodic(
        const Duration(milliseconds: 500), (timer) => setState(() {}));
  }

  void _listener() {
    setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    final desktop = MediaQuery.of(context).isDesktop;
    final appBarPadding = desktop ? 32.0 : 0.0;
    final playPauseIconSize = desktop ? 128.0 : 96.0;
    final seekIconSize = desktop ? 64.0 : 56.0;
    final bottomControlsPadding = desktop
        ? const EdgeInsets.all(48)
        : const EdgeInsets.symmetric(horizontal: 16, vertical: 8);
    return Stack(
      children: [
        Positioned(
          top: 0,
          left: 0,
          right: 0,
          child: Container(
            decoration: const BoxDecoration(
              gradient: LinearGradient(
                colors: [Colors.black, Colors.transparent],
                begin: FractionalOffset(0, 0),
                end: FractionalOffset(0, 1),
              ),
            ),
            child: Padding(
              padding: EdgeInsets.all(appBarPadding),
              child: AppBar(
                title: Text(widget.item.title),
                backgroundColor: Colors.transparent,
                elevation: 0,
              ),
            ),
          ),
        ),
        Align(
          alignment: Alignment.center,
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 32),
            child: Container(
              constraints: const BoxConstraints(maxWidth: 400),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  IconButton(
                    onPressed: () => _controller.position -= 10,
                    icon: const Icon(Icons.replay_10),
                    iconSize: seekIconSize,
                  ),
                  _PlayPauseButton(
                    isPlaying: !_controller.paused,
                    size: playPauseIconSize,
                    onPause: _controller.pause,
                    onPlay: _controller.play,
                  ),
                  IconButton(
                    onPressed: () => _controller.position += 30,
                    icon: const Icon(Icons.forward_30),
                    iconSize: seekIconSize,
                  ),
                ],
              ),
            ),
          ),
        ),
        Positioned(
          bottom: 0,
          left: 0,
          right: 0,
          child: _BottomControls(
            duration: Duration(seconds: _controller.duration.toInt()),
            position: Duration(seconds: _controller.position.toInt()),
            subtitles: widget.item.videoInfo?.subtitles ?? [],
            padding: bottomControlsPadding,
            onPause: _controller.pause,
            onPlay: _controller.play,
            onSeek: (position) =>
                _controller.position = position.inSeconds.toDouble(),
            onSelectSubtitle: (track) => _controller
                .setTextTrack(track != null ? subtitleFromApi(track) : null),
          ),
        ),
      ],
    );
  }

  @override
  void dispose() {
    super.dispose();
    _controller.removeListener(_listener);
    _timer.cancel();
  }
}

class _PlayPauseButton extends StatefulWidget {
  final bool isPlaying;
  final double size;

  final void Function() onPause;
  final void Function() onPlay;

  const _PlayPauseButton({
    required this.isPlaying,
    required this.size,
    required this.onPause,
    required this.onPlay,
  });

  @override
  State<_PlayPauseButton> createState() => _PlayPauseButtonState();
}

class _PlayPauseButtonState extends State<_PlayPauseButton>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 200),
      value: widget.isPlaying ? 1.0 : 0.0,
    );
  }

  @override
  void didUpdateWidget(covariant _PlayPauseButton oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.isPlaying != widget.isPlaying) {
      widget.isPlaying ? _controller.forward() : _controller.reverse();
    }
  }

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: Material(
        type: MaterialType.transparency,
        child: IconButton(
          icon: AnimatedIcon(
              icon: AnimatedIcons.play_pause, progress: _controller),
          iconSize: widget.size,
          hoverColor: Colors.transparent,
          highlightColor: Colors.transparent,
          onPressed: () =>
              widget.isPlaying ? widget.onPause() : widget.onPlay(),
        ),
      ),
    );
  }
}

class _BottomControls extends StatelessWidget {
  final Duration duration;
  final Duration position;
  final List<api.SubtitleTrack> subtitles;
  final EdgeInsets padding;

  final void Function() onPause;
  final void Function() onPlay;
  final void Function(Duration position) onSeek;
  final void Function(api.SubtitleTrack?) onSelectSubtitle;

  const _BottomControls({
    required this.duration,
    required this.position,
    required this.subtitles,
    required this.padding,
    required this.onPause,
    required this.onPlay,
    required this.onSeek,
    required this.onSelectSubtitle,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: const BoxDecoration(
        gradient: LinearGradient(
          colors: [Colors.transparent, Colors.black],
          begin: FractionalOffset(0, 0),
          end: FractionalOffset(0, 1),
        ),
      ),
      child: Padding(
        padding: padding,
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            _TimeText(
              value: position,
            ),
            Expanded(
              child: _SeekBar(
                max: duration,
                value: position,
                onSeek: onSeek,
              ),
            ),
            _TimeText(
              value: duration - position,
            ),
            const SizedBox(width: 8),
            IconButton(
              icon: const Icon(Icons.closed_caption),
              splashRadius: 20,
              onPressed: () {
                final width = MediaQuery.of(context).size.width;
                showModalBottomSheet<void>(
                  context: context,
                  constraints: width > 600
                      ? const BoxConstraints.expand(width: 600)
                      : null,
                  builder: (context) {
                    return ListView(
                      children: subtitles
                          .map(
                            (track) => ListTile(
                              title: Text(track.language ?? "Unknown"),
                              subtitle: Text(track.title ?? ""),
                              onTap: () {
                                onSelectSubtitle(track);
                                Navigator.pop(context);
                              },
                            ),
                          )
                          .toList(),
                    );
                  },
                );
              },
            ),
            IconButton(
              icon: const Icon(Icons.fullscreen),
              splashRadius: 20,
              onPressed: VideoPlayerPlatform.instance.toggleFullscreen,
            )
          ],
        ),
      ),
    );
  }
}

class _TimeText extends StatelessWidget {
  final Duration value;

  const _TimeText({required this.value});

  String _formatSegment(int value) {
    return value.toString().padLeft(2, '0');
  }

  @override
  Widget build(BuildContext context) {
    final hours = _formatSegment((value.inSeconds / 3600).floor());
    final mins = _formatSegment(((value.inSeconds % 3600) / 60).floor());
    final secs = _formatSegment(((value.inSeconds % 3600) % 60).floor());

    String string;

    if (value.inHours > 0) {
      string = '$hours:$mins:$secs';
    } else {
      string = '$mins:$secs';
    }

    return Text(
      string,
      style: const TextStyle(color: Colors.white),
    );
  }
}

class _SeekBar extends StatelessWidget {
  final Duration max;
  final Duration value;

  final void Function(Duration) onSeek;

  const _SeekBar({
    required this.max,
    required this.value,
    required this.onSeek,
  });

  @override
  Widget build(BuildContext context) {
    final max = this.max.inSeconds.toDouble();
    final value = this.value.inSeconds.toDouble();
    return Slider(
      min: 0,
      max: max,
      value: min(value, max),
      onChanged: (value) => onSeek(Duration(seconds: value.toInt())),
      onChangeEnd: (value) => onSeek(Duration(seconds: value.toInt())),
    );
  }
}
