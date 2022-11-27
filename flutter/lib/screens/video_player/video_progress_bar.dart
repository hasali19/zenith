import 'package:audio_video_progress_bar/audio_video_progress_bar.dart';
import 'package:flutter/material.dart';
import 'package:zenith/theme.dart';

class VideoProgressData {
  final Duration total;
  final Duration progress;

  VideoProgressData({required this.total, required this.progress});
}

class VideoProgressBar extends StatelessWidget {
  final Stream<VideoProgressData> stream;

  final void Function(Duration) onSeek;
  final void Function() onSeekStart;
  final void Function() onSeekEnd;

  const VideoProgressBar({
    Key? key,
    required this.stream,
    required this.onSeek,
    required this.onSeekStart,
    required this.onSeekEnd,
  }) : super(key: key);

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
    final textStyle =
        context.zenithTheme.bodyMedium.copyWith(color: Colors.white);
    return StreamBuilder<VideoProgressData>(
      stream: stream,
      builder: (context, snapshot) {
        final data = snapshot.data;
        final total = data?.total ?? Duration.zero;
        final progress = total > Duration.zero
            ? (data?.progress ?? Duration.zero)
            : Duration.zero;
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            ProgressBar(
              progress: progress,
              total: total,
              buffered: progress,
              barHeight: 4,
              progressBarColor: Colors.white,
              baseBarColor: Colors.white.withOpacity(0.24),
              thumbColor: Colors.white,
              thumbRadius: 7,
              thumbGlowRadius: 25,
              timeLabelTextStyle: textStyle,
              timeLabelLocation: TimeLabelLocation.none,
              onDragStart: (details) => onSeekStart(),
              onDragEnd: () => onSeekEnd(),
              onSeek: (value) => onSeek(value),
            ),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text(_formatTime(progress)),
                Text(
                  _formatTime(total),
                  style: textStyle.copyWith(color: Colors.white60),
                ),
              ],
            ),
          ],
        );
      },
    );
  }
}
