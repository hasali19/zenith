import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/api.dart' as api;
import 'package:zenith/screens/video_player/local_player.dart';
import 'package:zenith/screens/video_player/remote_player.dart';
import 'package:zenith/screens/video_player/video_player_cubit.dart';

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
    return BlocProvider(
      create: (context) =>
          VideoPlayerCubit(ref.read(api.apiProvider))..loadPlaylist(id),
      child: BlocBuilder<VideoPlayerCubit, VideoPlayerState>(
        builder: (context, state) {
          if (state.playlist == null) {
            return const Center(child: CircularProgressIndicator());
          }
          final Playlist(:items, start: startIndex) = state.playlist!;
          return switch (state.location) {
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
      ),
    );
  }
}
