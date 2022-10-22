import 'package:flutter/material.dart';
import 'package:video_player/video_player.dart';

import 'play_pause_button.dart';

class BottomControls extends StatelessWidget {
  const BottomControls({
    Key? key,
    required this.seekIconSize,
    required VideoController controller,
    required this.playPauseIconSize,
    required this.onButtonTap,
  })  : _controller = controller,
        super(key: key);

  final double seekIconSize;
  final VideoController _controller;
  final double playPauseIconSize;

  final VoidCallback onButtonTap;

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        Align(
          alignment: Alignment.center,
          child: Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              IconButton(
                icon: const Icon(Icons.replay_10),
                iconSize: seekIconSize,
                onPressed: () {
                  _controller.position -= 10;
                  onButtonTap();
                },
              ),
              const SizedBox(width: 32),
              PlayPauseButton(
                isPlaying: !_controller.paused,
                size: playPauseIconSize,
                onSetPlaying: (playing) {
                  playing ? _controller.play() : _controller.pause();
                  onButtonTap();
                },
              ),
              const SizedBox(width: 32),
              IconButton(
                icon: const Icon(Icons.forward_30),
                iconSize: seekIconSize,
                onPressed: () {
                  _controller.position += 30;
                  onButtonTap();
                },
              ),
            ],
          ),
        ),
        if (VideoPlayerPlatform.instance.isWindowed)
          Positioned(
            right: 0,
            bottom: 0,
            top: 0,
            child: IconButton(
              icon: const Icon(Icons.fullscreen),
              splashRadius: 20,
              onPressed: VideoPlayerPlatform.instance.toggleFullscreen,
            ),
          ),
      ],
    );
  }
}
