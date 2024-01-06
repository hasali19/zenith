import 'package:flutter/material.dart';

class PlayPauseButton extends StatefulWidget {
  final bool isPlaying;
  final double? size;

  final void Function(bool playing) onSetPlaying;

  const PlayPauseButton({
    Key? key,
    required this.isPlaying,
    required this.onSetPlaying,
    this.size,
  }) : super(key: key);

  @override
  State<PlayPauseButton> createState() => _PlayPauseButtonState();
}

class _PlayPauseButtonState extends State<PlayPauseButton>
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
  void didUpdateWidget(covariant PlayPauseButton oldWidget) {
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
          onPressed: () => widget.isPlaying
              ? widget.onSetPlaying(false)
              : widget.onSetPlaying(true),
        ),
      ),
    );
  }
}
