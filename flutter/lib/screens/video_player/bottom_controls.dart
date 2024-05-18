import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/window.dart';

class BottomControls extends ConsumerWidget {
  const BottomControls({
    super.key,
    required this.onShowCaptionsMenu,
    required this.onShowOptionsMenu,
  });

  final VoidCallback onShowCaptionsMenu;
  final VoidCallback onShowOptionsMenu;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final window = ref.read(windowProvider);
    return Row(
      mainAxisAlignment: MainAxisAlignment.end,
      children: [
        IconButton(
          icon: const Icon(Icons.closed_caption),
          splashRadius: 20,
          onPressed: onShowCaptionsMenu,
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
