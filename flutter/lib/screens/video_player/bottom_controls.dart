import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:video_player/video_player.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/window.dart';

class BottomControls extends ConsumerWidget {
  final List<SubtitleTrack> subtitles;
  final void Function(SubtitleTrack? track) onSubtitleTrackSelected;
  final void Function() onInteractionStart;
  final void Function() onInteractionEnd;

  const BottomControls({
    super.key,
    required this.subtitles,
    required this.onSubtitleTrackSelected,
    required this.onShowOptionsMenu,
    required this.onInteractionStart,
    required this.onInteractionEnd,
  });

  final VoidCallback onShowOptionsMenu;

  Future<void> _showSubtitlesMenuSheet(BuildContext context) {
    return showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      builder: (context) {
        return DraggableScrollableSheet(
          expand: false,
          builder: (context, scrollController) {
            return ListView(
              controller: scrollController,
              children: _buildSubtitlesMenuItems(context),
            );
          },
        );
      },
    );
  }

  List<Widget> _buildSubtitlesMenuItems(BuildContext context) {
    final items = [
      ListTile(
        title: const Text('None'),
        onTap: () {
          onSubtitleTrackSelected(null);
          Navigator.pop(context);
        },
      )
    ];

    for (final track in subtitles) {
      items.add(ListTile(
        title: Text(track.displayLanguage ?? 'Unknown'),
        subtitle: track.title != null ? Text(track.title!) : null,
        onTap: () {
          onSubtitleTrackSelected(track);
          Navigator.pop(context);
        },
      ));
    }

    return items;
  }

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final isDesktop = context.isDesktop;
    final window = ref.read(windowProvider);
    return Row(
      mainAxisAlignment: MainAxisAlignment.end,
      children: [
        isDesktop
            ? _SubtitleMenuButton(
                subtitles: subtitles,
                onSubtitleTrackSelected: onSubtitleTrackSelected,
                onInteractionStart: onInteractionStart,
                onInteractionEnd: onInteractionEnd,
              )
            : IconButton(
                icon: const Icon(Icons.closed_caption),
                splashRadius: 20,
                onPressed: () => _showSubtitlesMenuSheet(context),
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

class _SubtitleMenuButton extends StatelessWidget {
  final List<SubtitleTrack> subtitles;
  final void Function(SubtitleTrack? track) onSubtitleTrackSelected;
  final void Function() onInteractionStart;
  final void Function() onInteractionEnd;

  const _SubtitleMenuButton({
    required this.subtitles,
    required this.onSubtitleTrackSelected,
    required this.onInteractionStart,
    required this.onInteractionEnd,
  });

  @override
  Widget build(BuildContext context) {
    return MenuAnchor(
      menuChildren: [
        MenuItemButton(
          child: const Text('None'),
          onPressed: () => onSubtitleTrackSelected(null),
        ),
        ...subtitles.map((track) => MenuItemButton(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(track.displayLanguage ?? 'Unknown'),
                  if (track.title != null)
                    Text(
                      track.title!,
                      style: Theme.of(context).textTheme.bodySmall,
                    ),
                ],
              ),
              onPressed: () => onSubtitleTrackSelected(track),
            ))
      ],
      builder: (context, controller, child) {
        return IconButton(
          icon: const Icon(Icons.closed_caption),
          splashRadius: 20,
          onPressed: () {
            if (controller.isOpen) {
              controller.close();
            } else {
              controller.open();
            }
          },
        );
      },
      onOpen: onInteractionStart,
      onClose: onInteractionEnd,
    );
  }
}
