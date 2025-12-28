import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/screens/video_player/local_player.dart';
import 'package:zenith/screens/video_player/remote_player.dart';
import 'package:zenith/screens/video_player/video_player_view_controller.dart';

@RoutePage()
class VideoPlayerScreen extends ConsumerWidget {
  final int id;
  final double startPosition;

  const VideoPlayerScreen({
    super.key,
    @pathParam required this.id,
    @queryParam this.startPosition = 0,
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
