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
  static const _platform = const MethodChannel('zenith.hasali.uk/video-player');

  StreamInfo _info;

  int _texture;
  double _aspectRatio;
  bool _playing = true;
  int _position;

  @override
  void initState() {
    super.initState();
    Wakelock.enable();
    SystemChrome.setEnabledSystemUIOverlays([]);

    _platform.setMethodCallHandler((call) async {
      if (call.method == 'onAspectRatioChanged') {
        setState(() {
          _aspectRatio = call.arguments as double;
        });
      } else if (call.method == 'onProgressUpdate') {
        setState(() {
          _position = call.arguments as int;
        });
      } else if (call.method == 'onPlaybackStateChanged') {
        setState(() {
          _playing = call.arguments as bool;
        });
      }
    });

    final client = context.read<ApiClient>();

    client.getStreamInfo(widget.id).then((info) {
      setState(() {
        _info = info;
      });

      final url = 'https://zenith.hasali.uk/api/stream/${widget.id}/transcode';

      _platform.invokeMethod('init', url).then((texture) {
        setState(() {
          _texture = texture;
        });
      });
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

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        color: Colors.black,
        child: Stack(
          children: [
            Center(
              child: _info == null || _texture == null || _aspectRatio == null
                  ? Container()
                  : AspectRatio(
                      aspectRatio: _aspectRatio,
                      child: Texture(textureId: _texture),
                    ),
            ),
            Align(
              alignment: Alignment.center,
              child: FloatingActionButton(
                child: Icon(_playing ? Icons.pause : Icons.play_arrow),
                onPressed: () {
                  _platform.invokeMethod(_playing ? 'pause' : 'play', _texture);
                },
              ),
            ),
            if (_position != null)
              Positioned(
                bottom: 0,
                left: 0,
                right: 0,
                child: Container(
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
                        Text(
                          "${formatTime(_position / 1000, _info.duration >= 3600)}",
                          style: TextStyle(color: Colors.white),
                        ),
                        Expanded(
                          child: Slider(
                            min: 0,
                            max: _info.duration,
                            value: _position / 1000,
                            onChanged: (value) {},
                          ),
                        ),
                        Text(
                          "${formatTime(_info.duration - (_position / 1000), _info.duration >= 3600)}",
                          style: TextStyle(color: Colors.white),
                        ),
                      ],
                    ),
                  ),
                ),
              ),
          ],
        ),
      ),
    );
  }
}

String formatTimeSegment(int value) {
  return value.toString().padLeft(2, '0');
}

String formatTime(double value, bool showHours) {
  final hours = formatTimeSegment((value / 3600).floor());
  final mins = formatTimeSegment(((value % 3600) / 60).floor());
  final secs = formatTimeSegment(((value % 3600) % 60).floor());

  if (showHours) {
    return '$hours:$mins:$secs';
  } else {
    return '$mins:$secs';
  }
}
