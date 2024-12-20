import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:material_symbols_icons/material_symbols_icons.dart';
import 'package:video_player/video_player.dart';
import 'package:zenith/preferences.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/screens/video_player/play_pause_button.dart';
import 'package:zenith/themes.dart';

import 'bottom_controls.dart';
import 'video_progress_bar.dart';

class VideoPlayerUi extends HookConsumerWidget {
  final Widget title;
  final VideoController controller;

  final void Function() onInteractionStart;
  final void Function() onInteractionEnd;
  final void Function()? onSeekToNext;

  const VideoPlayerUi({
    super.key,
    required this.title,
    required this.controller,
    required this.onInteractionStart,
    required this.onInteractionEnd,
    this.onSeekToNext,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final desktop = MediaQuery.of(context).isDesktop;
    final appBarPadding = desktop ? 32.0 : 0.0;
    final bottomControlsPadding = desktop
        ? const EdgeInsets.symmetric(horizontal: 96, vertical: 48)
        : const EdgeInsets.symmetric(horizontal: 16, vertical: 8);

    return Theme(
      data: ref.watch(themesProvider).dark,
      child: DecoratedBox(
        decoration: BoxDecoration(
          gradient: LinearGradient(
            begin: const FractionalOffset(0, 0),
            end: const FractionalOffset(0, 1),
            colors: [
              Colors.black.withValues(alpha: 0.7),
              Colors.transparent,
              Colors.black.withValues(alpha: 0.7),
            ],
          ),
        ),
        child: Stack(
          children: [
            Positioned(
              top: 0,
              left: 0,
              right: 0,
              child: Padding(
                padding: EdgeInsets.all(appBarPadding),
                child: AppBar(
                  title: title,
                  backgroundColor: Colors.transparent,
                  elevation: 0,
                ),
              ),
            ),
            Align(
              alignment: Alignment.center,
              child: _CenterControls(
                isLoading: controller.loading,
                isPaused: controller.paused,
                onSetPaused: (paused) {
                  if (paused) {
                    controller.pause();
                  } else {
                    controller.play();
                  }
                  onInteractionEnd();
                },
                onSeekDelta: (delta) {
                  controller.position += delta;
                  onInteractionEnd();
                },
                onSkipPrevious: () {
                  controller.seekToPreviousItem();
                  onInteractionEnd();
                },
                onSkipNext: () {
                  onSeekToNext?.call();
                  controller.seekToNextItem();
                  onInteractionEnd();
                },
              ),
            ),
            Positioned(
              bottom: 0,
              left: 0,
              right: 0,
              child: SafeArea(
                child: Padding(
                  padding: bottomControlsPadding,
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: [
                      VideoProgressBar(
                        progress: () => VideoProgressData(
                          total: Duration(seconds: controller.duration.toInt()),
                          progress:
                              Duration(seconds: controller.position.toInt()),
                        ),
                        onSeek: (position) =>
                            controller.position = position.inSeconds.toDouble(),
                        onSeekStart: onInteractionStart,
                        onSeekEnd: onInteractionEnd,
                      ),
                      const SizedBox(height: 8),
                      BottomControls(
                        controller: controller,
                        onShowOptionsMenu: () {
                          onInteractionEnd();
                          _showOptionsMenu(context);
                        },
                        onInteractionStart: onInteractionStart,
                        onInteractionEnd: onInteractionEnd,
                      ),
                    ],
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Future<void> _showModalBottomSheet(
      BuildContext context, Widget Function(BuildContext context) builder,
      {bool safeArea = true}) {
    final width = MediaQuery.of(context).size.width;
    return showModalBottomSheet<void>(
      context: context,
      constraints: width > 600
          ? const BoxConstraints.expand(width: 600).copyWith(minHeight: 0)
          : null,
      builder: (context) =>
          safeArea ? SafeArea(child: builder(context)) : builder(context),
    );
  }

  Future<void> _showOptionsMenu(BuildContext context) {
    return _showModalBottomSheet(
      context,
      (context) => Wrap(
        children: [
          ListTile(
            leading: const Icon(Icons.aspect_ratio),
            title: const Text('Fit'),
            onTap: () {
              Navigator.pop(context);
              _showVideoFitMenu(context);
            },
          ),
          if (controller.supportsAudioTrackSelection &&
              controller.availableAudioTracks.length > 1)
            ListTile(
              leading: const Icon(Icons.audiotrack),
              title: const Text('Audio'),
              onTap: () {
                Navigator.pop(context);
                _showAudioMenu(context);
              },
            ),
          ListTile(
            leading: const Icon(Icons.speed),
            title: const Text('Playback speed'),
            onTap: () {
              Navigator.pop(context);
              _showPlaybackSpeedMenu(context);
            },
          ),
        ],
      ),
    );
  }

  Future<void> _showAudioMenu(BuildContext context) {
    final audioTracks = () {
      final audioTracks = [...controller.availableAudioTracks];
      audioTracks.sort((a, b) => a.language.compareTo(b.language));
      return audioTracks;
    }();

    return showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      builder: (context) {
        return DraggableScrollableSheet(
          expand: false,
          builder: (context, scrollController) {
            return ListView(
              controller: scrollController,
              children: _buildAudioMenuItems(context, audioTracks),
            );
          },
        );
      },
    );
  }

  List<Widget> _buildAudioMenuItems(
      BuildContext context, List<AudioTrack> audioTracks) {
    final items = <Widget>[];

    for (final track in audioTracks) {
      items.add(ListTile(
        title: Text(track.language),
        subtitle: Text(track.codec),
        onTap: () {
          controller.setAudioTrack(track.index);
          Navigator.pop(context);
        },
      ));
    }

    return items;
  }

  Future<void> _showVideoFitMenu(BuildContext context) {
    return showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      clipBehavior: Clip.antiAlias,
      builder: (context) => DraggableScrollableSheet(
        expand: false,
        builder: (context, scrollController) => HookBuilder(
          builder: (context) => VideoFitMenu(
            controller: controller,
            scrollController: scrollController,
          ),
        ),
      ),
    );
  }

  Future<void> _showPlaybackSpeedMenu(BuildContext context) {
    const speeds = [1.0, 1.25, 1.5, 1.75, 2.0];
    return showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      builder: (context) {
        return DraggableScrollableSheet(
          expand: false,
          builder: (context, scrollController) {
            Widget buildListTile(speed) {
              void onSetSpeed() {
                controller.setPlaybackSpeed(speed);
                Navigator.pop(context);
              }

              return ListTile(
                title: Text('${speed}x'),
                onTap: controller.playbackSpeed == speed ? null : onSetSpeed,
                trailing: controller.playbackSpeed != speed
                    ? null
                    : const Icon(Icons.check),
              );
            }

            return ListView(
              controller: scrollController,
              children: speeds.map(buildListTile).toList(),
            );
          },
        );
      },
    );
  }
}

class _CenterControls extends ConsumerWidget {
  final bool isLoading;
  final bool isPaused;
  final void Function(bool paused) onSetPaused;
  final void Function(double delta) onSeekDelta;
  final void Function() onSkipPrevious;
  final void Function() onSkipNext;

  const _CenterControls({
    required this.isLoading,
    required this.isPaused,
    required this.onSetPaused,
    required this.onSeekDelta,
    required this.onSkipPrevious,
    required this.onSkipNext,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final desktop = MediaQuery.of(context).isDesktop;
    final primaryIconSize = desktop ? 128.0 : 64.0;
    final secondaryIconSize = desktop ? 64.0 : 32.0;

    final fastForwardDuration = ref.watch(fastForwardDurationProvider);
    final rewindDuration = ref.watch(rewindDurationProvider);

    return Row(
      mainAxisAlignment: MainAxisAlignment.center,
      mainAxisSize: MainAxisSize.min,
      children: [
        IconButton(
          icon: const Icon(Icons.skip_previous),
          iconSize: secondaryIconSize,
          onPressed: onSkipPrevious,
        ),
        const SizedBox(width: 24),
        IconButton(
          icon: Icon(switch (rewindDuration) {
            PlayerSeekPresetDuration.d5 => Icons.replay_5,
            PlayerSeekPresetDuration.d10 => Icons.replay_10,
            PlayerSeekPresetDuration.d30 => Icons.replay_30,
          }),
          iconSize: secondaryIconSize,
          onPressed: () => onSeekDelta(-rewindDuration.value.toDouble()),
        ),
        const SizedBox(width: 24),
        Stack(
          children: [
            Container(
              decoration: BoxDecoration(
                shape: BoxShape.circle,
                color: Colors.grey.withAlpha(50),
              ),
              child: PlayPauseButton(
                isPlaying: !isPaused,
                size: primaryIconSize,
                onSetPlaying: (playing) => onSetPaused(!playing),
              ),
            ),
            if (isLoading)
              SizedBox(
                width: primaryIconSize + 16,
                height: primaryIconSize + 16,
                child: const CircularProgressIndicator(color: Colors.white),
              )
          ],
        ),
        const SizedBox(width: 24),
        IconButton(
          icon: Icon(switch (fastForwardDuration) {
            PlayerSeekPresetDuration.d5 => Icons.forward_5,
            PlayerSeekPresetDuration.d10 => Icons.forward_10,
            PlayerSeekPresetDuration.d30 => Icons.forward_30,
          }),
          iconSize: secondaryIconSize,
          onPressed: () => onSeekDelta(fastForwardDuration.value.toDouble()),
        ),
        const SizedBox(width: 24),
        IconButton(
          icon: const Icon(Icons.skip_next),
          iconSize: secondaryIconSize,
          onPressed: onSkipNext,
        ),
      ],
    );
  }
}

class VideoFitMenu extends HookWidget {
  final VideoController controller;
  final ScrollController? scrollController;

  const VideoFitMenu({
    super.key,
    required this.controller,
    this.scrollController,
  });

  @override
  Widget build(BuildContext context) {
    const fits = [
      (BoxFit.cover, 'Cover', Icons.crop_free),
      (BoxFit.contain, 'Contain', Icons.fit_screen),
      (BoxFit.fitWidth, 'Fit Width', Symbols.fit_page_width),
      (BoxFit.fitHeight, 'Fit Height', Symbols.fit_page_height),
    ];

    final (isUsingCropRects, currentCropRect, currentFit) =
        useListenableSelector(
            controller,
            () => (
                  controller.isUsingCropRects,
                  controller.currentCropRect,
                  controller.fit,
                ));

    return ListView(
      controller: scrollController,
      children: [
        if (controller.supportsCropRects)
          CheckboxListTile(
            secondary: Icon(Icons.crop),
            title: Text('Fix black bars'),
            subtitle: Text(switch (currentCropRect) {
              null => 'No crop rect available',
              final rect =>
                'Using rect ${rect.left.toInt()}:${rect.top.toInt()}:${rect.width.toInt()}:${rect.height.toInt()}',
            }),
            value: isUsingCropRects,
            onChanged: (value) {
              if (value != null) {
                controller.isUsingCropRects = value;
              }
            },
          ),
        if (controller.supportsCropRects) Divider(),
        for (final (value, label, icon) in fits)
          ListTile(
            leading: Icon(icon),
            title: Text(label),
            trailing: currentFit != value ? null : const Icon(Icons.check),
            onTap: switch (currentFit == value) {
              true => null,
              false => () => controller.setFit(value),
            },
          ),
      ],
    );
  }
}
