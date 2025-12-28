import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:material_symbols_icons/material_symbols_icons.dart';
import 'package:video_player/video_player.dart';
import 'package:wolt_modal_sheet/wolt_modal_sheet.dart';
import 'package:zenith/preferences.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/screens/video_player/play_pause_button.dart';
import 'package:zenith/themes.dart';

import 'bottom_controls.dart';
import 'video_progress_bar.dart';

class VideoPlayerUi extends HookConsumerWidget {
  final Widget title;
  final VideoController controller;
  final bool isOffline;

  final void Function() onInteractionStart;
  final void Function() onInteractionEnd;
  final void Function()? onSeekToNext;

  const VideoPlayerUi({
    super.key,
    required this.title,
    required this.controller,
    required this.isOffline,
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
                  actions: [
                    if (isOffline)
                      Padding(
                        padding: const EdgeInsets.symmetric(horizontal: 16),
                        child: Tooltip(
                          message: 'Offline',
                          child: Icon(Icons.cloud_off),
                        ),
                      ),
                  ],
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
                          progress: Duration(
                            seconds: controller.position.toInt(),
                          ),
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

  Future<void> _showOptionsMenu(BuildContext context) {
    return WoltModalSheet.show(
      context: context,
      showDragHandle: false,
      pageListBuilder: (context) {
        return [
          _OptionsMenuSheetPage(
            controller: controller,
            onShowVideoFitMenu: () {
              WoltModalSheet.of(context)
                ..addOrReplacePage(
                  _VideoFitMenuSheetPage(controller: controller),
                )
                ..showNext();
            },
            onShowAudioTrackMenu: () {
              _showAudioMenu(context);
            },
            onShowPlaybackSpeedMenu: () {
              _showPlaybackSpeedMenu(context);
            },
          ),
        ];
      },
    );
  }

  void _showAudioMenu(BuildContext context) {
    final audioTracks = () {
      final audioTracks = [...controller.availableAudioTracks];
      audioTracks.sort((a, b) => a.language.compareTo(b.language));
      return audioTracks;
    }();

    WoltModalSheet.of(context)
      ..addOrReplacePage(
        SliverWoltModalSheetPage(
          leadingNavBarWidget: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 8),
            child: IconButton(
              icon: const Icon(Icons.arrow_back_rounded),
              onPressed: WoltModalSheet.of(context).showPrevious,
            ),
          ),
          mainContentSliversBuilder: (context) => [
            SliverList.list(
              children: _buildAudioMenuItems(context, audioTracks),
            ),
          ],
        ),
      )
      ..showNext();
  }

  List<Widget> _buildAudioMenuItems(
    BuildContext context,
    List<AudioTrack> audioTracks,
  ) {
    final items = <Widget>[];

    for (final track in audioTracks) {
      items.add(
        ListTile(
          title: Text(track.language),
          subtitle: Text(track.codec),
          onTap: () {
            controller.setAudioTrack(track.index);
            WoltModalSheet.of(context).popPage();
          },
        ),
      );
    }

    return items;
  }

  void _showPlaybackSpeedMenu(BuildContext context) {
    const speeds = [1.0, 1.25, 1.5, 1.75, 2.0];

    final modalSheet = WoltModalSheet.of(context);

    Widget buildListTile(speed) {
      void onSetSpeed() {
        controller.setPlaybackSpeed(speed);
        modalSheet.popPage();
      }

      return ListTile(
        title: Text('${speed}x'),
        onTap: controller.playbackSpeed == speed ? null : onSetSpeed,
        trailing: controller.playbackSpeed != speed
            ? null
            : const Icon(Icons.check),
      );
    }

    WoltModalSheet.of(context)
      ..addOrReplacePage(
        SliverWoltModalSheetPage(
          leadingNavBarWidget: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 8),
            child: IconButton(
              icon: const Icon(Icons.arrow_back_rounded),
              onPressed: WoltModalSheet.of(context).popPage,
            ),
          ),
          mainContentSliversBuilder: (context) => [
            SliverList.list(children: speeds.map(buildListTile).toList()),
          ],
        ),
      )
      ..showNext();
  }
}

class _OptionsMenuSheetPage extends SliverWoltModalSheetPage {
  _OptionsMenuSheetPage({
    required VideoController controller,
    required void Function() onShowVideoFitMenu,
    required void Function() onShowAudioTrackMenu,
    required void Function() onShowPlaybackSpeedMenu,
  }) : super(
         hasTopBarLayer: false,
         mainContentSliversBuilder: (context) {
           return [
             SliverList.list(
               children: [
                 if (controller.supportsVideoFitting)
                   ListTile(
                     leading: const Icon(Icons.aspect_ratio),
                     title: const Text('Fit'),
                     subtitle: Text(switch (controller.fit) {
                       BoxFit.cover => 'Cover',
                       BoxFit.contain => 'Contain',
                       BoxFit.fitWidth => 'Fit Width',
                       BoxFit.fitHeight => 'Fit Height',
                       _ => 'Unknown',
                     }),
                     onTap: onShowVideoFitMenu,
                   ),
                 if (controller.supportsAudioTrackSelection &&
                     controller.availableAudioTracks.length > 1)
                   ListTile(
                     leading: const Icon(Icons.audiotrack),
                     title: const Text('Audio'),
                     onTap: onShowAudioTrackMenu,
                   ),
                 ListTile(
                   leading: const Icon(Icons.speed),
                   title: const Text('Playback speed'),
                   subtitle: Text('${controller.playbackSpeed}'),
                   onTap: onShowPlaybackSpeedMenu,
                 ),
                 if (controller.subtitleStyle case SubtitleStyleOptions style)
                   _SubtitlesSizeListTile(style: style),
               ],
             ),
           ];
         },
       );
}

class _VideoFitMenuSheetPage extends SliverWoltModalSheetPage {
  _VideoFitMenuSheetPage({required VideoController controller})
    : super(
        leadingNavBarWidget: Builder(
          builder: (context) {
            return Padding(
              padding: const EdgeInsets.symmetric(horizontal: 8),
              child: IconButton(
                icon: const Icon(Icons.arrow_back_rounded),
                onPressed: WoltModalSheet.of(context).showPrevious,
              ),
            );
          },
        ),
        mainContentSliversBuilder: (context) {
          return [VideoFitMenu(controller: controller)];
        },
      );
}

class _SubtitlesSizeListTile extends ConsumerStatefulWidget {
  final SubtitleStyleOptions style;

  const _SubtitlesSizeListTile({required this.style});

  @override
  ConsumerState<_SubtitlesSizeListTile> createState() =>
      _SubtitlesSizeListTileState();
}

class _SubtitlesSizeListTileState
    extends ConsumerState<_SubtitlesSizeListTile> {
  late int _value;

  @override
  void initState() {
    super.initState();
    _value = widget.style.size;
  }

  @override
  Widget build(BuildContext context) {
    return ListTile(
      title: Text('Subtitles size'),
      subtitle: SliderTheme(
        data: SliderThemeData(
          trackHeight: 2,
          thumbShape: RoundSliderThumbShape(enabledThumbRadius: 8),
          overlayShape: RoundSliderOverlayShape(overlayRadius: 16),
        ),
        child: Slider(
          padding: EdgeInsets.symmetric(vertical: 12),
          min: 10,
          max: 50,
          divisions: (50 - 10) ~/ 2,
          label: _value.toString(),
          value: _value.toDouble(),
          onChanged: (value) {
            setState(() {
              _value = value.toInt();
              widget.style.size = _value;
              ref.read(subtitleSizeProvider.notifier).update(_value);
            });
          },
        ),
      ),
      leading: Icon(Icons.closed_caption),
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
              ),
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

class VideoFitMenu extends HookConsumerWidget {
  final VideoController controller;

  const VideoFitMenu({super.key, required this.controller});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    const fits = [
      (BoxFit.cover, 'Cover', Icons.crop_free),
      (BoxFit.contain, 'Contain', Icons.fit_screen),
      (BoxFit.fitWidth, 'Fit Width', Symbols.fit_page_width),
      (BoxFit.fitHeight, 'Fit Height', Symbols.fit_page_height),
    ];

    final (
      isUsingCropRects,
      currentCropRect,
      currentFit,
    ) = useListenableSelector(
      controller,
      () => (
        controller.isUsingCropRects,
        controller.currentCropRect,
        controller.fit,
      ),
    );

    return SliverList.list(
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
                ref.read(applyCropRectsProvider.notifier).update(value);
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
