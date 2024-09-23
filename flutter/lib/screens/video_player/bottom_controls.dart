import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/screens/video_player/subtitles.dart';
import 'package:zenith/window.dart';

class BottomControls extends ConsumerWidget {
  final List<SubtitleTrackData> subtitles;
  final String? activeSubtitleId;
  final void Function(SubtitleTrackData? track) onSubtitleTrackSelected;
  final void Function() onShowOptionsMenu;
  final void Function() onInteractionStart;
  final void Function() onInteractionEnd;

  const BottomControls({
    super.key,
    required this.subtitles,
    required this.activeSubtitleId,
    required this.onSubtitleTrackSelected,
    required this.onShowOptionsMenu,
    required this.onInteractionStart,
    required this.onInteractionEnd,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final window = ref.read(windowProvider);
    return Row(
      mainAxisAlignment: MainAxisAlignment.end,
      children: [
        SubtitlesMenuButton(
          tracks: subtitles,
          activeTrackId: activeSubtitleId,
          onTrackSelected: onSubtitleTrackSelected,
          onInteractionStart: onInteractionStart,
          onInteractionEnd: onInteractionEnd,
        ),
        if (window.isWindowed)
          IconButton(
            icon: const Icon(Icons.fullscreen),
            splashRadius: 20,
            onPressed: window.toggleFullscreen,
          ),
        IconButton(
          icon: const Icon(Icons.more_vert),
          splashRadius: 20,
          onPressed: onShowOptionsMenu,
        ),
      ],
    );
  }
}
