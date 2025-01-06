import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:video_player/video_player.dart' show VideoController;
import 'package:zenith/screens/video_player/subtitles.dart';
import 'package:zenith/window.dart';

class BottomControls extends ConsumerWidget {
  final VideoController controller;
  final void Function() onShowOptionsMenu;
  final void Function() onInteractionStart;
  final void Function() onInteractionEnd;

  const BottomControls({
    super.key,
    required this.controller,
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
          controller: controller,
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
