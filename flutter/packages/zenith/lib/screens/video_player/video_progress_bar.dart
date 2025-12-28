import 'dart:async';

import 'package:audio_video_progress_bar/audio_video_progress_bar.dart';
import 'package:flutter/material.dart';

class VideoProgressData {
  final Duration total;
  final Duration progress;

  VideoProgressData({required this.total, required this.progress});
}

class VideoProgressBar extends StatefulWidget {
  final VideoProgressData Function() progress;

  final void Function(Duration) onSeek;
  final void Function() onSeekStart;
  final void Function() onSeekEnd;

  const VideoProgressBar({
    super.key,
    required this.progress,
    required this.onSeek,
    required this.onSeekStart,
    required this.onSeekEnd,
  });

  @override
  State<VideoProgressBar> createState() => _VideoProgressBarState();
}

class _VideoProgressBarState extends State<VideoProgressBar> {
  late final Timer _timer;

  VideoProgressData _progress = VideoProgressData(
    total: Duration.zero,
    progress: Duration.zero,
  );

  @override
  void initState() {
    super.initState();
    _progress = widget.progress();
    _timer = Timer.periodic(const Duration(milliseconds: 500), (timer) {
      setState(() {
        _progress = widget.progress();
      });
    });
  }

  @override
  void didUpdateWidget(covariant VideoProgressBar oldWidget) {
    super.didUpdateWidget(oldWidget);
    _progress = widget.progress();
  }

  @override
  void dispose() {
    _timer.cancel();
    super.dispose();
  }

  String _formatSegment(int value) {
    return value.toString().padLeft(2, '0');
  }

  String _formatTime(Duration value) {
    final hours = _formatSegment((value.inSeconds / 3600).floor());
    final mins = _formatSegment(((value.inSeconds % 3600) / 60).floor());
    final secs = _formatSegment(((value.inSeconds % 3600) % 60).floor());

    String string;

    if (value.inHours > 0) {
      string = '$hours:$mins:$secs';
    } else {
      string = '$mins:$secs';
    }

    return string;
  }

  @override
  Widget build(BuildContext context) {
    final total = _progress.total;
    final progress = total > Duration.zero ? _progress.progress : Duration.zero;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text.rich(
          TextSpan(
            children: [
              TextSpan(
                text: _formatTime(progress),
                style: const TextStyle(color: Colors.white),
              ),
              TextSpan(
                text: ' / ${_formatTime(total)}',
                style: const TextStyle(color: Colors.grey),
              ),
            ],
          ),
        ),
        const SizedBox(height: 8),
        ProgressBar(
          progress: progress,
          total: total,
          buffered: progress,
          barHeight: 4,
          progressBarColor: Colors.white,
          baseBarColor: Colors.white.withAlpha(61),
          thumbColor: Colors.white,
          thumbRadius: 7,
          thumbGlowRadius: 25,
          timeLabelLocation: TimeLabelLocation.none,
          onDragStart: (details) => widget.onSeekStart(),
          onDragEnd: () => widget.onSeekEnd(),
          onSeek: (value) {
            setState(() {
              _progress = VideoProgressData(total: total, progress: value);
            });
            widget.onSeek(value);
          },
        ),
      ],
    );
  }
}
