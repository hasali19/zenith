import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:video_player/video_player.dart';
import 'package:zenith/window.dart';

import 'play_pause_button.dart';

class BottomControls extends ConsumerWidget {
  const BottomControls({
    Key? key,
    required VideoController controller,
    required this.primaryIconSize,
    required this.secondaryIconSize,
    required this.onButtonTap,
  })  : _controller = controller,
        super(key: key);

  final VideoController _controller;
  final double primaryIconSize;
  final double secondaryIconSize;

  final VoidCallback onButtonTap;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final window = ref.read(windowProvider);
    return Stack(
      children: [
        Align(
          alignment: Alignment.center,
          child: Row(
            mainAxisAlignment: MainAxisAlignment.center,
            mainAxisSize: MainAxisSize.min,
            children: [
              IconButton(
                icon: const Icon(Icons.skip_previous),
                iconSize: secondaryIconSize,
                onPressed: () {
                  _controller.seekToPreviousItem();
                  onButtonTap();
                },
              ),
              const SizedBox(width: 24),
              IconButton(
                icon: const Icon(Icons.replay_10),
                iconSize: secondaryIconSize,
                onPressed: () {
                  _controller.position -= 10;
                  onButtonTap();
                },
              ),
              const SizedBox(width: 24),
              PlayPauseButton(
                isPlaying: !_controller.paused,
                size: primaryIconSize,
                onSetPlaying: (playing) {
                  playing ? _controller.play() : _controller.pause();
                  onButtonTap();
                },
              ),
              const SizedBox(width: 24),
              IconButton(
                icon: const Icon(Icons.forward_30),
                iconSize: secondaryIconSize,
                onPressed: () {
                  _controller.position += 30;
                  onButtonTap();
                },
              ),
              const SizedBox(width: 24),
              IconButton(
                icon: const Icon(Icons.skip_next),
                iconSize: secondaryIconSize,
                onPressed: () {
                  _controller.seekToNextItem();
                  onButtonTap();
                },
              ),
            ],
          ),
        ),
        if (window.isWindowed)
          Positioned(
            right: 0,
            bottom: 0,
            top: 0,
            child: IconButton(
              icon: const Icon(Icons.fullscreen),
              splashRadius: 20,
              onPressed: window.toggleFullscreen,
            ),
          ),
      ],
    );
  }
}
