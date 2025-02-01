import 'dart:math' as math;

import 'package:flutter/material.dart';
import 'package:sliver_expandable/sliver_expandable.dart';
import 'package:sliver_tools/sliver_tools.dart';
import 'package:zenith/api.dart';
import 'package:zenith/constants.dart';
import 'package:zenith/image.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/routes/item_details/item_details_state.dart';
import 'package:zenith/text_one_line.dart';
import 'package:zenith/theme.dart';

class EpisodesList extends StatelessWidget {
  final int? initialExpanded;
  final List<EpisodeGroupState> groups;
  final void Function(EpisodeState) onEpisodePressed;

  const EpisodesList({
    super.key,
    required this.initialExpanded,
    required this.groups,
    required this.onEpisodePressed,
  });

  @override
  Widget build(BuildContext context) {
    final isDesktop = MediaQuery.of(context).isDesktop;
    return MultiSliver(
      children: [
        for (final (index, group) in groups.indexed)
          _EpisodesListInner(
            expanded: initialExpanded == index,
            group: group,
            onEpisodePressed: onEpisodePressed,
          ),
        SliverToBoxAdapter(
          child: SizedBox(height: isDesktop ? 128 : 16),
        ),
      ],
    );
  }
}

class _EpisodesListInner extends StatefulWidget {
  final bool expanded;
  final EpisodeGroupState group;
  final void Function(EpisodeState) onEpisodePressed;

  const _EpisodesListInner({
    required this.expanded,
    required this.group,
    required this.onEpisodePressed,
  });

  @override
  State<_EpisodesListInner> createState() => _EpisodesListInnerState();
}

class _EpisodesListInnerState extends State<_EpisodesListInner> {
  final controller = ScrollController();

  late bool _expanded;

  EpisodeGroupState get _group => widget.group;
  List<EpisodeState> get _episodes => widget.group.episodes;

  @override
  void initState() {
    super.initState();
    _expanded = widget.expanded;
  }

  @override
  void didUpdateWidget(covariant _EpisodesListInner oldWidget) {
    super.didUpdateWidget(oldWidget);
    _expanded = widget.expanded;
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final mediaQuery = MediaQuery.of(context);
    final isDesktop = mediaQuery.isDesktop;
    final width = mediaQuery.size.width - (isDesktop ? 224 : 0);
    final thumbnailWidth = isDesktop ? 280.0 : 160.0;
    return AnimatedSliverExpandable(
      expanded: _expanded,
      headerBuilder: (context, animation) {
        return Padding(
          padding: isDesktop
              ? const EdgeInsets.symmetric(horizontal: 112)
              : EdgeInsets.zero,
          child: ListTile(
            contentPadding: EdgeInsets.only(top: 8, bottom: 8) +
                EdgeInsets.symmetric(horizontal: 16),
            title: Text(_group.name, style: theme.textTheme.headlineMedium),
            trailing: AnimatedBuilder(
              animation: animation,
              builder: (context, child) => Transform.rotate(
                angle: (animation.value - 0.5) * math.pi,
                child: child,
              ),
              child: const Icon(Icons.chevron_left),
            ),
            onTap: () {
              setState(() {
                _expanded = !_expanded;
              });
            },
          ),
        );
      },
      sliver: SliverPadding(
        padding: isDesktop
            ? const EdgeInsets.symmetric(horizontal: 112)
            : EdgeInsets.zero,
        sliver: SliverGrid(
          gridDelegate: SliverGridDelegateWithMaxCrossAxisExtent(
            maxCrossAxisExtent: 700.0,
            childAspectRatio: width /
                (width / 700.0).ceil() /
                (thumbnailWidth * (9.0 / 16.0) + 16),
          ),
          delegate: SliverChildBuilderDelegate(
            (context, index) => _EpisodeListItem(
              episode: _episodes[index],
              width: thumbnailWidth,
              onPressed: () => widget.onEpisodePressed(_episodes[index]),
            ),
            childCount: _episodes.length,
          ),
        ),
      ),
    );
  }
}

class _EpisodeListItem extends StatelessWidget {
  const _EpisodeListItem({
    required this.episode,
    required this.width,
    required this.onPressed,
  });

  final EpisodeState episode;
  final double width;
  final void Function() onPressed;

  @override
  Widget build(BuildContext context) {
    final height = width * (9.0 / 16.0);
    return SizedBox(
      height: height + 16,
      child: Stack(
        children: [
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16),
            child: Center(
              child: Row(
                children: [
                  SizedBox(
                    width: width,
                    height: height,
                    child: _EpisodeThumbnail(
                      imageId: episode.thumbnail,
                      imageWidth: mediaThumbnailImageWidth,
                      isWatched: episode.isWatched,
                    ),
                  ),
                  const SizedBox(width: 12),
                  Flexible(
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        TextOneLine(episode.title,
                            style: context.zenithTheme.titleMedium),
                        const SizedBox(height: 8),
                        Flexible(
                          child: Text(
                            episode.overview ?? '',
                            style: context.zenithTheme.bodySmall,
                            maxLines: 3,
                            overflow: TextOverflow.ellipsis,
                          ),
                        ),
                      ],
                    ),
                  ),
                ],
              ),
            ),
          ),
          Positioned.fill(
            child: Material(
              color: Colors.transparent,
              child: InkWell(onTap: onPressed),
            ),
          )
        ],
      ),
    );
  }
}

class _EpisodeThumbnail extends StatelessWidget {
  final ImageId? imageId;
  final int imageWidth;
  final bool isWatched;

  const _EpisodeThumbnail({
    required this.imageId,
    required this.imageWidth,
    required this.isWatched,
  });

  @override
  Widget build(BuildContext context) {
    return Material(
      elevation: 2.0,
      type: MaterialType.card,
      clipBehavior: Clip.hardEdge,
      borderRadius: const BorderRadius.all(Radius.circular(8)),
      child: Stack(children: [
        switch (imageId) {
          null => const Icon(Icons.video_file, size: 48),
          final imageId => Positioned.fill(
              child: ZenithApiImage(id: imageId!, requestWidth: imageWidth),
            )
        },
        if (isWatched)
          Container(
            color: Colors.black.withAlpha(127),
            child: const Center(
              child: Icon(Icons.check, size: 36, color: Colors.white),
            ),
          ),
      ]),
    );
  }
}
