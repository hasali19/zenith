import 'package:flutter/material.dart';
import 'package:zenith/responsive.dart';

class SubtitleTrackData {
  final String id;
  final String language;
  final String? label;

  const SubtitleTrackData({
    required this.id,
    required this.language,
    required this.label,
  });
}

class SubtitlesMenuButton extends StatelessWidget {
  final List<SubtitleTrackData> tracks;
  final String? activeTrackId;
  final void Function(SubtitleTrackData? track) onTrackSelected;
  final void Function() onInteractionStart;
  final void Function() onInteractionEnd;

  const SubtitlesMenuButton({
    super.key,
    required this.tracks,
    required this.activeTrackId,
    required this.onTrackSelected,
    required this.onInteractionStart,
    required this.onInteractionEnd,
  });

  @override
  Widget build(BuildContext context) {
    if (context.isDesktop) {
      return _SubtitlesDropdownMenuButton(
        subtitles: tracks,
        activeTrackId: activeTrackId,
        onSubtitleTrackSelected: onTrackSelected,
        onInteractionStart: onInteractionStart,
        onInteractionEnd: onInteractionEnd,
      );
    } else {
      return IconButton(
        icon: const Icon(Icons.closed_caption),
        splashRadius: 20,
        onPressed: () => _showSubtitlesMenuSheet(context),
      );
    }
  }

  Future<void> _showSubtitlesMenuSheet(BuildContext context) {
    final initialActiveTrackId = activeTrackId;
    return showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      builder: (context) => DraggableScrollableSheet(
        expand: false,
        builder: (context, scrollController) => _SubtitlesMenuSheet(
          scrollController: scrollController,
          subtitles: tracks,
          activeTrackId: initialActiveTrackId,
          onItemSelected: onTrackSelected,
        ),
      ),
    );
  }
}

class _SubtitlesMenuSheet extends StatelessWidget {
  final ScrollController scrollController;
  final List<SubtitleTrackData> subtitles;
  final String? activeTrackId;
  final void Function(SubtitleTrackData? track) onItemSelected;

  const _SubtitlesMenuSheet({
    required this.scrollController,
    required this.subtitles,
    required this.activeTrackId,
    required this.onItemSelected,
  });

  @override
  Widget build(BuildContext context) {
    return ListView(
      controller: scrollController,
      children: _buildMenuItems(context),
    );
  }

  List<Widget> _buildMenuItems(BuildContext context) {
    SubtitleTrackData? activeTrack;
    if (activeTrackId != null) {
      activeTrack =
          subtitles.where((track) => track.id == activeTrackId).firstOrNull;
    }

    final items = <Widget>[
      _buildMenuItem(
        context,
        'None',
        null,
        activeTrackId == null,
        () => onItemSelected(null),
      ),
      if (activeTrack != null)
        _buildMenuItem(
          context,
          activeTrack.language,
          activeTrack.label,
          activeTrackId == activeTrack.id,
          () => onItemSelected(activeTrack),
        ),
      const Divider(),
    ];

    for (final track in subtitles) {
      if (track.id == activeTrackId) continue;
      items.add(_buildMenuItem(context, track.language, track.label,
          activeTrackId == track.id, () => onItemSelected(track)));
    }

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

class _SubtitlesDropdownMenuButton extends StatelessWidget {
  final List<SubtitleTrackData> subtitles;
  final String? activeTrackId;
  final void Function(SubtitleTrackData? track) onSubtitleTrackSelected;
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
    SubtitleTrackData? activeTrack;
    if (activeTrackId != null) {
      activeTrack =
          subtitles.where((track) => track.id == activeTrackId).firstOrNull;
    }

    return MenuAnchor(
      menuChildren: [
        _buildNoneMenuItem(),
        if (activeTrack != null) _buildMenuItem(context, activeTrack),
        const Divider(),
        ...subtitles
            .where((track) => track.id != activeTrackId)
            .map((track) => _buildMenuItem(context, track)),
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
      onPressed:
          activeTrackId == null ? null : () => onSubtitleTrackSelected(null),
      child: const Text('None'),
    );
  }

  Widget _buildMenuItem(BuildContext context, SubtitleTrackData track) {
    return MenuItemButton(
      trailingIcon: track.id == activeTrackId ? const Icon(Icons.check) : null,
      onPressed: track.id == activeTrackId
          ? null
          : () => onSubtitleTrackSelected(track),
      child: Builder(builder: (context) {
        final secondaryTextStyle = DefaultTextStyle.of(context)
            .style
            .merge(Theme.of(context).textTheme.bodySmall);

        return ConstrainedBox(
          constraints: const BoxConstraints(minWidth: 150),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(track.language),
              if (track.label case String label)
                Text(
                  label,
                  style: secondaryTextStyle,
                ),
            ],
          ),
        );
      }),
    );
  }
}
