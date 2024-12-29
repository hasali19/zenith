import 'dart:async';

import 'package:audio_video_progress_bar/audio_video_progress_bar.dart';
import 'package:cast_framework/cast_framework.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:video_player/video_player.dart';
import 'package:zenith/image.dart';
import 'package:zenith/format_utils.dart';
import 'package:zenith/media_route_controller/media_route_controller_controller.dart';
import 'package:zenith/screens/video_player/play_pause_button.dart';

class MediaRouteControllerDialog extends ConsumerStatefulWidget {
  const MediaRouteControllerDialog({super.key});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() =>
      _MediaRouteControllerDialogState();
}

typedef _VideoProgress = ({Duration total, Duration progress});

class _MediaRouteControllerDialogState
    extends ConsumerState<MediaRouteControllerDialog> {
  final RemoteMediaClient _client =
      CastFrameworkPlatform.instance.remoteMediaClient;

  late final Stream<({Duration total, Duration progress})> _progress;

  final _positionHandler = MediaPositionHandler();

  @override
  void initState() {
    super.initState();
    _client.mediaStatus.addListener(_onMediaStatusUpdated);

    _onMediaStatusUpdated();

    _progress = _createProgressStream();
  }

  @override
  void dispose() {
    _client.mediaStatus.removeListener(_onMediaStatusUpdated);
    super.dispose();
  }

  Stream<_VideoProgress> _createProgressStream() {
    _VideoProgress getProgress() {
      final positionMs = _positionHandler.positionMs.toInt();
      final durationMs = _client.mediaInfo.value?.streamDuration ?? positionMs;
      return (
        total: Duration(milliseconds: durationMs),
        progress: Duration(milliseconds: positionMs),
      );
    }

    final controller = StreamController<_VideoProgress>();

    controller.add(getProgress());
    controller.addStream(Stream.periodic(
        const Duration(milliseconds: 500), (count) => getProgress()));

    return controller.stream;
  }

  void _onMediaStatusUpdated() {
    final mediaStatus = _client.mediaStatus.value;
    if (mediaStatus == null) return;

    _positionHandler.update(
      positionMs: mediaStatus.streamPosition,
      isPlaying: mediaStatus.playerState == PlayerState.playing,
      speed: mediaStatus.playbackRate,
    );
  }

  @override
  Widget build(BuildContext context) {
    ref.listen(mediaRouteControllerControllerProvider, (previous, next) {
      if ((previous == null || previous.route != null) && next.route == null) {
        Navigator.pop(context);
      }
    });

    final state = ref.watch(mediaRouteControllerControllerProvider);
    return AlertDialog(
      title: Text(state.route?.name ?? ''),
      contentPadding: const EdgeInsets.only(top: 16),
      content: switch ((state.mediaStatus, state.mediaInfo)) {
        (null, _) ||
        (MediaStatus(playerState: PlayerState.idle), _) =>
          Container(
            color: Colors.black.withAlpha(30),
            padding:
                const EdgeInsets.only(left: 20, right: 20, top: 16, bottom: 16),
            child: const Text('No media selected'),
          ),
        (final mediaStatus, final mediaInfo) => Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              if (mediaInfo?.metadata?.backdrop != null)
                Flexible(
                  child: ZenithImage(
                    image: NetworkImage(mediaInfo!.metadata!.backdrop!.url),
                  ),
                ),
              const SizedBox(height: 8),
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 16),
                child: Text(
                  mediaInfo?.metadata?.title ?? 'Unknown',
                  style: Theme.of(context).textTheme.titleLarge,
                  textAlign: TextAlign.center,
                ),
              ),
              if (mediaInfo!.metadata?.seriesTitle != null)
                Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 16),
                  child: Text(
                    '${formatSeasonEpisode(mediaInfo.metadata!.seasonNumber!, mediaInfo.metadata!.episodeNumber!)} - ${mediaInfo.metadata!.seriesTitle!}',
                    style: Theme.of(context).textTheme.titleMedium,
                    textAlign: TextAlign.center,
                  ),
                ),
              const SizedBox(height: 16),
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 20),
                child: StreamBuilder(
                  stream: _progress,
                  builder: (context, snapshot) => ProgressBar(
                    progress: snapshot.data?.progress ?? Duration.zero,
                    total: snapshot.data?.total ?? Duration.zero,
                    barHeight: 4,
                    thumbRadius: 7,
                    thumbGlowRadius: 25,
                    timeLabelLocation: TimeLabelLocation.none,
                    onSeek: (value) => _client.seek(MediaSeekOptions(
                      position: value.inMilliseconds,
                      resumeState: ResumeState.unchanged,
                    )),
                  ),
                ),
              ),
              const SizedBox(height: 8),
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 16),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    PlayPauseButton(
                      isPlaying: mediaStatus!.playerState != PlayerState.paused,
                      onSetPlaying: (playing) {
                        if (mediaStatus.playerState != PlayerState.paused) {
                          _client.pause();
                        } else {
                          _client.play();
                        }
                      },
                    ),
                    IconButton(
                      icon: const Icon(Icons.stop),
                      onPressed: () {
                        _client.stop();
                      },
                    ),
                  ],
                ),
              ),
            ],
          )
      },
      actions: [
        TextButton(
          onPressed: () => ref
              .read(mediaRouteControllerControllerProvider.notifier)
              .deselectRoute(),
          child: const Text('Stop Casting'),
        ),
      ],
    );
  }
}
