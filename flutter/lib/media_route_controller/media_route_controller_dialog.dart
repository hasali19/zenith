import 'dart:async';

import 'package:audio_video_progress_bar/audio_video_progress_bar.dart';
import 'package:cast_framework/cast_framework.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:video_player/video_player.dart';
import 'package:zenith/fade_in_image.dart';
import 'package:zenith/format_utils.dart';
import 'package:zenith/media_route_controller/media_route_controller_cubit.dart';
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
  late final MediaRouter _mediaRouter;
  late final Stream<({Duration total, Duration progress})> _progress;

  final _positionHandler = MediaPositionHandler();

  @override
  void initState() {
    super.initState();
    _mediaRouter = context.read<MediaRouter>();
    _mediaRouter.mediaStatus.addListener(_onMediaStatusUpdated);

    _onMediaStatusUpdated();

    _progress = _createProgressStream();
  }

  @override
  void dispose() {
    _mediaRouter.mediaStatus.removeListener(_onMediaStatusUpdated);
    super.dispose();
  }

  Stream<_VideoProgress> _createProgressStream() {
    _VideoProgress getProgress() {
      final positionMs = _positionHandler.positionMs.toInt();
      final durationMs =
          _mediaRouter.mediaStatus.value?.mediaInfo?.streamDuration ??
              positionMs;
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
    final mediaStatus = _mediaRouter.mediaStatus.value;
    if (mediaStatus == null) return;

    _positionHandler.update(
      positionMs: mediaStatus.streamPosition,
      isPlaying: mediaStatus.playerState == PlayerState.playing,
      speed: mediaStatus.playbackRate,
    );
  }

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create: (context) =>
          MediaRouteControllerCubit(context.read<MediaRouter>()),
      child: BlocConsumer<MediaRouteControllerCubit, MediaRouteControllerState>(
        listenWhen: (previous, current) => previous.route != null,
        listener: (context, state) {
          if (state.route == null) {
            Navigator.pop(context);
          }
        },
        builder: (context, state) => AlertDialog(
          title: Text(state.route?.name ?? ''),
          contentPadding: const EdgeInsets.only(top: 16),
          content: switch (state.mediaStatus) {
            null || MediaStatus(playerState: PlayerState.idle) => Container(
                color: Colors.black.withAlpha(30),
                padding: const EdgeInsets.only(
                    left: 20, right: 20, top: 16, bottom: 16),
                child: const Text('No media selected'),
              ),
            final mediaStatus => Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  if (mediaStatus.mediaInfo?.metadata?.backdrop != null)
                    Flexible(
                      child: ZenithFadeInImage.dio(
                        url: mediaStatus.mediaInfo!.metadata!.backdrop!.url,
                      ),
                    ),
                  const SizedBox(height: 8),
                  Padding(
                    padding: const EdgeInsets.symmetric(horizontal: 16),
                    child: Text(
                      mediaStatus.mediaInfo?.metadata?.title ?? 'Unknown',
                      style: Theme.of(context).textTheme.titleLarge,
                      textAlign: TextAlign.center,
                    ),
                  ),
                  if (mediaStatus.mediaInfo!.metadata?.seriesTitle != null)
                    Padding(
                      padding: const EdgeInsets.symmetric(horizontal: 16),
                      child: Text(
                        '${formatSeasonEpisode(mediaStatus.mediaInfo!.metadata!.seasonNumber!, mediaStatus.mediaInfo!.metadata!.episodeNumber!)} - ${mediaStatus.mediaInfo!.metadata!.seriesTitle!}',
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
                        onSeek: (value) => CastApi().seek(MediaSeekOptions(
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
                          isPlaying:
                              mediaStatus.playerState != PlayerState.paused,
                          onSetPlaying: (playing) {
                            if (mediaStatus.playerState != PlayerState.paused) {
                              CastApi().pause();
                            } else {
                              CastApi().play();
                            }
                          },
                        ),
                        IconButton(
                          icon: const Icon(Icons.stop),
                          onPressed: () {
                            CastApi().stop();
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
              onPressed: () =>
                  context.read<MediaRouteControllerCubit>().deselectRoute(),
              child: const Text('Stop Casting'),
            ),
          ],
        ),
      ),
    );
  }
}
