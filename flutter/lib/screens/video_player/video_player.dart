import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/screens/video_player/local_player.dart';
import 'package:zenith/screens/video_player/remote_player.dart';
import 'package:zenith/screens/video_player/video_player_view_controller.dart';

class VideoPlayerScreen extends ConsumerWidget {
  final int id;
  final double startPosition;

  const VideoPlayerScreen({
    super.key,
    required this.id,
    required this.startPosition,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(videoPlayerViewControllerProvider(id));
    return state.maybeWhen(
      data: (data) {
        if (data.playlist == null) {
          return const Center(child: CircularProgressIndicator());
        }
        final Playlist(:items, start: startIndex) = data.playlist!;
        return switch (data.location) {
          PlaybackLocation.local => LocalVideoPlayer(
              items: items,
              startIndex: startIndex,
              startPosition: startPosition,
            ),
          PlaybackLocation.remote => RemoteVideoPlayer(
              items: items,
              startIndex: startIndex,
              startPosition: startPosition,
            ),
        };
      },
      orElse: () => const Center(child: CircularProgressIndicator()),
    );
  }
}
