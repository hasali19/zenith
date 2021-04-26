import 'dart:math';
import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:provider/provider.dart';
import 'package:wakelock/wakelock.dart';
import 'package:zenith/api.dart';

class PlayerScreen extends StatefulWidget {
  final int id;

  PlayerScreen(this.id);

  @override
  State<StatefulWidget> createState() {
    return PlayerScreenState();
  }
}

class PlayerScreenState extends State<PlayerScreen> {
  static const _platform = MethodChannel('zenith.hasali.uk/video-player');

  StreamInfo? _info;

  int? _texture;
  double? _aspectRatio;

  bool _playing = true;
  bool _controls = true;

  double _offset = 0;
  double _position = 0;

  _ProgressReporter? _progressReporter;

  PlayerScreenState() {
    _progressReporter = _ProgressReporter(_reportProgress);
  }

  @override
  void initState() {
    super.initState();

    Wakelock.enable();
    SystemChrome.setEnabledSystemUIOverlays([]);

    _platform.setMethodCallHandler(_handlePlatformMethodCall);

    context.read<ApiClient>().getStreamInfo(widget.id).then((info) {
      setState(() {
        _info = info;
        _offset = info.position ?? 0;
      });

      _initPlayer();
    });
  }

  @override
  void dispose() {
    if (_texture != null) {
      _platform.invokeMethod('destroy', _texture);
    }

    Wakelock.disable();
    SystemChrome.setEnabledSystemUIOverlays(SystemUiOverlay.values);

    super.dispose();
  }

  Future _handlePlatformMethodCall(MethodCall call) async {
    switch (call.method) {
      case 'onAspectRatioChanged':
        setState(() => _aspectRatio = call.arguments);
        break;

      case 'onPlaybackStateChanged':
        setState(() => _playing = call.arguments as bool);
        break;

      case 'onProgressUpdate':
        final double position = call.arguments / 1000;
        _progressReporter!.onProgressUpdate((position + _offset).floor());
        setState(() => _position = position.toDouble());
        break;
    }
  }

  void _initPlayer() {
    final position = _info!.position?.floor() ?? 0;
    final url =
        'https://zenith.hasali.uk/api/stream/${widget.id}/transcode?start=$position';

    _platform.invokeMethod('init', url).then((texture) {
      setState(() {
        _texture = texture;
      });
    });
  }

  void _pause() {
    _platform.invokeMethod('pause', _texture);
  }

  void _play() {
    _platform.invokeMethod('play', _texture);
  }

  void _seek(double position) {
    setState(() {
      _offset = position;
      _position = 0;
    });

    _platform.invokeMethod('setUrl', {
      'textureId': _texture,
      'url':
          'https://zenith.hasali.uk/api/stream/${widget.id}/transcode?start=${_offset.floor()}',
    });
  }

  void _toggleControls() {
    setState(() {
      _controls = !_controls;
    });
  }

  void _reportProgress(int position) async {
    await context.read<ApiClient>().updateProgress(widget.id, position);
  }

  @override
  Widget build(BuildContext context) {
    final video = _info == null || _texture == null || _aspectRatio == null
        ? Container()
        : AspectRatio(
            aspectRatio: _aspectRatio!,
            child: Texture(textureId: _texture!),
          );

    return Scaffold(
      body: Container(
        color: Colors.black,
        child: Stack(
          children: [
            GestureDetector(
              behavior: HitTestBehavior.opaque,
              onTap: _toggleControls,
              child: Center(child: video),
            ),
            AnimatedOpacity(
              opacity: _controls ? 1 : 0,
              duration: Duration(milliseconds: 200),
              child: _Controls(
                duration: _info?.duration ?? 0,
                position: _position + _offset,
                isPlaying: _playing,
                onPause: _pause,
                onPlay: _play,
                onSeek: _seek,
              ),
            )
          ],
        ),
      ),
    );
  }
}

class _Controls extends StatelessWidget {
  final double duration;
  final double position;

  final bool isPlaying;

  final void Function() onPause;
  final void Function() onPlay;
  final void Function(double position) onSeek;

  const _Controls({
    required this.duration,
    required this.position,
    required this.isPlaying,
    required this.onPause,
    required this.onPlay,
    required this.onSeek,
  });

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        Align(
          alignment: Alignment.center,
          child: _PlayPauseButton(
            isPlaying: isPlaying,
            onPause: onPause,
            onPlay: onPlay,
          ),
        ),
        Positioned(
          bottom: 0,
          left: 0,
          right: 0,
          child: _BottomControls(
            duration: duration,
            position: position,
            onPause: onPause,
            onPlay: onPlay,
            onSeek: onSeek,
          ),
        ),
      ],
    );
  }
}

class _PlayPauseButton extends StatelessWidget {
  final bool isPlaying;

  final void Function() onPause;
  final void Function() onPlay;

  const _PlayPauseButton({
    required this.isPlaying,
    required this.onPause,
    required this.onPlay,
  });

  @override
  Widget build(BuildContext context) {
    return FloatingActionButton(
      onPressed: () => isPlaying ? onPause() : onPlay(),
      child: Icon(isPlaying ? Icons.pause : Icons.play_arrow),
    );
  }
}

class _BottomControls extends StatefulWidget {
  final double duration;
  final double position;

  final void Function() onPause;
  final void Function() onPlay;
  final void Function(double position) onSeek;

  const _BottomControls({
    required this.duration,
    required this.position,
    required this.onPause,
    required this.onPlay,
    required this.onSeek,
  });

  @override
  _BottomControlsState createState() => _BottomControlsState();
}

class _BottomControlsState extends State<_BottomControls> {
  bool _isSeeking = false;
  double _position = 0;

  void _onSeekStart() {
    widget.onPause();

    setState(() {
      _isSeeking = true;
      _position = widget.position;
    });
  }

  void _onSeek(double position) {
    setState(() {
      _position = position;
    });
  }

  void _onSeekEnd(double position) {
    setState(() {
      _isSeeking = false;
      _position = 0;
    });

    widget.onSeek(position);
  }

  @override
  Widget build(BuildContext context) {
    final position = _isSeeking ? _position : widget.position;
    return Container(
      decoration: BoxDecoration(
        gradient: LinearGradient(
          colors: [Colors.transparent, Colors.black],
          begin: const FractionalOffset(0, 0),
          end: const FractionalOffset(0, 1),
        ),
      ),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 16),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            _TimeText(
              value: position,
            ),
            Expanded(
              child: _SeekBar(
                max: widget.duration,
                value: position,
                onSeekStart: _onSeekStart,
                onSeek: _onSeek,
                onSeekEnd: _onSeekEnd,
              ),
            ),
            _TimeText(
              value: widget.duration - position,
            )
          ],
        ),
      ),
    );
  }
}

class _TimeText extends StatelessWidget {
  final double value;

  const _TimeText({required this.value});

  String _formatSegment(int value) {
    return value.toString().padLeft(2, '0');
  }

  @override
  Widget build(BuildContext context) {
    final hours = _formatSegment((value / 3600).floor());
    final mins = _formatSegment(((value % 3600) / 60).floor());
    final secs = _formatSegment(((value % 3600) % 60).floor());

    var string;

    if (value > 3600) {
      string = '$hours:$mins:$secs';
    } else {
      string = '$mins:$secs';
    }

    return Text(
      string,
      style: TextStyle(color: Colors.white),
    );
  }
}

class _SeekBar extends StatelessWidget {
  final double max;
  final double value;

  final void Function() onSeekStart;
  final void Function(double) onSeek;
  final void Function(double) onSeekEnd;

  const _SeekBar({
    required this.max,
    required this.value,
    required this.onSeekStart,
    required this.onSeek,
    required this.onSeekEnd,
  });

  @override
  Widget build(BuildContext context) {
    return Slider(
      min: 0,
      max: max,
      value: min(value, max),
      onChangeStart: (value) => onSeekStart(),
      onChanged: onSeek,
      onChangeEnd: onSeekEnd,
    );
  }
}

class _ProgressReporter {
  final void Function(int) _callback;

  int _counter = 0;

  _ProgressReporter(this._callback);

  void onProgressUpdate(int position) {
    _counter += 1;

    if (_counter == 4) {
      _counter = 0;
    }

    if (_counter == 0) {
      _callback(position);
    }
  }
}
