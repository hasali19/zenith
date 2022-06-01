import 'dart:async';
import 'dart:html';
import 'dart:math';

import 'package:flutter/material.dart';

import '../api.dart' as api;
import "../shims/dart_ui.dart" as ui;

class VideoPlayerScreen extends StatefulWidget {
  final int id;

  const VideoPlayerScreen({
    Key? key,
    required this.id,
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
          return _VideoPlayer(item: snapshot.data! as api.VideoItem);
        } else {
          return const Center(child: CircularProgressIndicator());
        }
      },
    );
  }
}

enum VideoState { idle, active, ended }

class VideoController {
  VideoState _state = VideoState.idle;

  final VideoElement _element;
  final List<void Function()> _listeners = [];

  VideoController(this._element) {
    _element.addEventListener("durationchange", (event) => _notifyListeners());
    _element.addEventListener("pause", (event) => _notifyListeners());
    _element.addEventListener("play", (event) => _notifyListeners());
    _element.addEventListener("ended", (event) {
      _state = VideoState.ended;
      _notifyListeners();
    });
  }

  VideoState get state => _state;

  double get position => _element.currentTime.toDouble();
  set position(double value) {
    _element.currentTime = value;
  }

  double get duration {
    final value = _element.duration.toDouble();
    return value.isNaN ? 0 : value;
  }

  bool get paused => _element.paused;

  void load(String url) {
    _element.src = url;
    _state = VideoState.active;
  }

  void play() {
    _element.play();
  }

  void pause() {
    _element.pause();
  }

  void addListener(void Function() listener) {
    _listeners.add(listener);
  }

  void removeListener(void Function() listener) {
    _listeners.remove(listener);
  }

  void dispose() {
    _listeners.clear();
  }

  void _notifyListeners() {
    for (final listener in _listeners) {
      listener();
    }
  }
}

class VideoView extends StatefulWidget {
  final void Function(VideoController controller) onReady;

  const VideoView({Key? key, required this.onReady}) : super(key: key);

  @override
  State<VideoView> createState() => _VideoViewState();
}

class _VideoViewState extends State<VideoView> {
  static final Map<int, VideoElement> _views = {};

  late int _id;
  late VideoController _controller;

  @override
  void initState() {
    super.initState();
    ui.platformViewRegistry.registerViewFactory("video-player", (viewId) {
      final view = VideoElement()
        ..autoplay = true
        ..disableRemotePlayback = true
        ..style.background = "black";
      _views[viewId] = view;
      return view;
    });
  }

  @override
  Widget build(BuildContext context) {
    return HtmlElementView(
      viewType: "video-player",
      onPlatformViewCreated: (id) => setState(() {
        _id = id;
        _controller = VideoController(_views[id]!);
        widget.onReady(_controller);
      }),
    );
  }

  @override
  void dispose() {
    super.dispose();
    _controller.dispose();
    _views.remove(_id);
  }
}

class _VideoPlayer extends StatefulWidget {
  final api.VideoItem item;

  const _VideoPlayer({Key? key, required this.item}) : super(key: key);

  @override
  State<StatefulWidget> createState() {
    return _VideoPlayerState();
  }
}

class _VideoPlayerState extends State<_VideoPlayer> {
  VideoController? _controller;
  bool _shouldShowControls = true;

  late Timer _progressTimer;
  Timer? _controlsTimer;

  List<api.SubtitleTrack> get subtitles =>
      widget.item.videoInfo?.subtitles ?? [];

  @override
  void initState() {
    super.initState();

    _progressTimer = Timer.periodic(const Duration(seconds: 5), (timer) {
      final position = (_controller?.position ?? 0).toInt();
      if (_controller?.state == VideoState.active &&
          _controller?.paused == false &&
          position > 0) {
        api.updateProgress(widget.item.id, position);
      }
    });

    _showControls();
  }

  @override
  void dispose() {
    super.dispose();
    _progressTimer.cancel();
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
    return Scaffold(
      backgroundColor: Colors.black,
      body: Listener(
        behavior: HitTestBehavior.opaque,
        onPointerHover: (e) => _showControls(),
        child: Stack(
          children: [
            Center(
              child: VideoView(
                onReady: (controller) => setState(() {
                  _controller = controller
                    ..load(api.getVideoUrl(widget.item.id));
                }),
              ),
            ),
            if (_controller != null)
              GestureDetector(
                behavior: HitTestBehavior.opaque,
                onTap: _toggleControls,
                child: AnimatedSwitcher(
                  duration: const Duration(milliseconds: 200),
                  transitionBuilder: (child, animation) =>
                      FadeTransition(opacity: animation, child: child),
                  child: ControlsContainer(
                    key: ValueKey<bool>(_shouldShowControls),
                    controller: _controller!,
                    item: widget.item,
                    visible: _shouldShowControls,
                  ),
                ),
              )
          ],
        ),
      ),
    );
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
    _timer = Timer.periodic(const Duration(milliseconds: 500), (timer) {
      setState(() {});
    });
  }

  void _listener() {
    setState(() {});
  }

  @override
  Widget build(BuildContext context) {
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
              padding: const EdgeInsets.all(32),
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
          child: Container(
            constraints: const BoxConstraints(maxWidth: 600),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                IconButton(
                  onPressed: () => _controller.position -= 10,
                  icon: const Icon(Icons.replay_10),
                  iconSize: 64,
                ),
                _PlayPauseButton(
                  isPlaying: !_controller.paused,
                  onPause: _controller.pause,
                  onPlay: _controller.play,
                ),
                IconButton(
                  onPressed: () => _controller.position += 30,
                  icon: const Icon(Icons.forward_30),
                  iconSize: 64,
                ),
              ],
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
            onPause: _controller.pause,
            onPlay: _controller.play,
            onSeek: (position) =>
                _controller.position = position.inSeconds.toDouble(),
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

  final void Function() onPause;
  final void Function() onPlay;

  const _PlayPauseButton({
    required this.isPlaying,
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
          iconSize: 128,
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

  final void Function() onPause;
  final void Function() onPlay;
  final void Function(Duration position) onSeek;

  const _BottomControls({
    required this.duration,
    required this.position,
    required this.subtitles,
    required this.onPause,
    required this.onPlay,
    required this.onSeek,
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
        padding: const EdgeInsets.all(48),
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
              icon: const Icon(Icons.fullscreen),
              splashRadius: 20,
              onPressed: () {
                if (document.fullscreenElement == null) {
                  document.documentElement?.requestFullscreen();
                } else {
                  document.exitFullscreen();
                }
              },
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
