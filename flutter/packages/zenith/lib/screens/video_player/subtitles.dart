import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:video_player/video_player.dart'
    show VideoController, SubtitleTrack;
import 'package:zenith/language_codes.dart';
import 'package:zenith/responsive.dart';

class SubtitlesMenuButton extends StatelessWidget {
  final VideoController controller;
  final void Function() onInteractionStart;
  final void Function() onInteractionEnd;

  const SubtitlesMenuButton({
    super.key,
    required this.controller,
    required this.onInteractionStart,
    required this.onInteractionEnd,
  });

  @override
  Widget build(BuildContext context) {
    if (context.isDesktop) {
      return _SubtitlesDropdownMenuButton(
        subtitles: controller.currentSubtitleTracks,
        activeTrackId: controller.activeSubtitleTrackId,
        onSubtitleTrackSelected: (track) =>
            controller.setSubtitleTrack(track?.id),
        onInteractionStart: onInteractionStart,
        onInteractionEnd: onInteractionEnd,
      );
    } else {
      return IconButton(
        icon: const Icon(Icons.closed_caption),
        splashRadius: 20,
        onPressed: () => showModalBottomSheet(
          context: context,
          isScrollControlled: true,
          builder: (context) => DraggableScrollableSheet(
            expand: false,
            builder: (context, scrollController) => _SubtitlesMenuSheet(
              scrollController: scrollController,
              controller: controller,
            ),
          ),
        ),
      );
    }
  }
}

class _SubtitlesMenuSheet extends HookWidget {
  final ScrollController scrollController;
  final VideoController controller;

  const _SubtitlesMenuSheet({
    required this.scrollController,
    required this.controller,
  });

  @override
  Widget build(BuildContext context) {
    useListenable(controller);

    return ListView(
      controller: scrollController,
      children: _buildMenuItems(context),
    );
  }

  List<Widget> _buildMenuItems(BuildContext context) {
    final subtitles = useSortedSubtitleTracks(controller.currentSubtitleTracks);

    SubtitleTrack? activeTrack;
    if (controller.activeSubtitleTrackId != null) {
      activeTrack = subtitles
          .where((track) => track.id == controller.activeSubtitleTrackId)
          .firstOrNull;
    }

    final items = <Widget>[
      _buildMenuItem(
        context,
        'None',
        null,
        activeTrack == null,
        () => controller.setSubtitleTrack(null),
      ),
      if (activeTrack case SubtitleTrack track)
        _buildMenuItem(
          context,
          track.language!,
          track.label,
          true,
          () => controller.setSubtitleTrack(track.id),
        ),
      const Divider(),
      for (final track in subtitles.where((t) => t.id != activeTrack?.id))
        _buildMenuItem(
          context,
          track.language!,
          track.label,
          false,
          () => controller.setSubtitleTrack(track.id),
        ),
    ];

    return items;
  }

  Widget _buildMenuItem(
    BuildContext context,
    String title,
    String? subtitle,
    bool isSelected,
    void Function() onSelect,
  ) {
    final onTap = isSelected
        ? null
        : () {
            onSelect();
            Navigator.pop(context);
          };

    return ListTile(
      title: Text(title),
      subtitle: switch (subtitle) {
        null => null,
        final subtitle => Text(subtitle),
      },
      trailing: isSelected ? const Icon(Icons.check) : null,
      onTap: onTap,
    );
  }
}

class _SubtitlesDropdownMenuButton extends HookWidget {
  final List<SubtitleTrack> subtitles;
  final String? activeTrackId;
  final void Function(SubtitleTrack? track) onSubtitleTrackSelected;
  final void Function() onInteractionStart;
  final void Function() onInteractionEnd;

  const _SubtitlesDropdownMenuButton({
    required this.subtitles,
    required this.activeTrackId,
    required this.onSubtitleTrackSelected,
    required this.onInteractionStart,
    required this.onInteractionEnd,
  });

  @override
  Widget build(BuildContext context) {
    final subtitles = useSortedSubtitleTracks(this.subtitles);

    SubtitleTrack? activeTrack;
    if (activeTrackId != null) {
      activeTrack = subtitles
          .where((track) => track.id == activeTrackId)
          .firstOrNull;
    }

    return MenuAnchor(
      menuChildren: [
        _buildNoneMenuItem(),
        if (activeTrack != null) _buildMenuItem(activeTrack),
        const Divider(),
        ...subtitles
            .where((track) => track.id != activeTrackId)
            .map(_buildMenuItem),
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
      style: const MenuStyle(
        maximumSize: WidgetStatePropertyAll(Size.fromHeight(800)),
      ),
    );
  }

  Widget _buildNoneMenuItem() {
    return MenuItemButton(
      trailingIcon: activeTrackId == null ? const Icon(Icons.check) : null,
      onPressed: activeTrackId == null
          ? null
          : () => onSubtitleTrackSelected(null),
      child: const Text('None'),
    );
  }

  Widget _buildMenuItem(SubtitleTrack track) {
    return MenuItemButton(
      trailingIcon: track.id == activeTrackId ? const Icon(Icons.check) : null,
      onPressed: track.id == activeTrackId
          ? null
          : () => onSubtitleTrackSelected(track),
      child: Builder(
        builder: (context) {
          final defaultStyle = DefaultTextStyle.of(context).style;
          final secondaryTextStyle = Theme.of(
            context,
          ).textTheme.bodySmall!.copyWith(color: defaultStyle.color);

          return ConstrainedBox(
            constraints: const BoxConstraints(minWidth: 150),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(track.language!),
                if (track.label case String label)
                  Text(label, style: secondaryTextStyle),
              ],
            ),
          );
        },
      ),
    );
  }
}

List<SubtitleTrack> useSortedSubtitleTracks(List<SubtitleTrack> tracks) {
  return useMemoized(() {
    final mappedTracks = tracks.map(_resolveLanguage).toList();
    mappedTracks.sort((a, b) => a.language!.compareTo(b.language!));
    return mappedTracks;
  }, [tracks]);
}

SubtitleTrack _resolveLanguage(SubtitleTrack track) {
  return SubtitleTrack(
    id: track.id,
    language: switch (track.language) {
      null => 'Unknown',
      final lang => tryResolveLanguageCode(lang),
    },
    label: track.label,
  );
}
