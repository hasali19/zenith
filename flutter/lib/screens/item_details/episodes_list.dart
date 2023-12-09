import 'package:dio_image_provider/dio_image_provider.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:sliver_tools/sliver_tools.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/screens/item_details/item_details.dart';
import 'package:zenith/screens/item_details/model.dart';
import 'package:zenith/text_one_line.dart';
import 'package:zenith/theme.dart';

import '../../api.dart' as api;

class Season {
  final api.MediaItem data;
  final List<api.MediaItem> episodes;

  Season({
    required this.data,
    required this.episodes,
  });
}

Future<List<Season>> fetchSeasons(
    api.ZenithApiClient zenith, int showId) async {
  final seasons = <Season>[];
  for (final season in await zenith.fetchSeasons(showId)) {
    final episodes = await zenith.fetchEpisodes(season.id);
    seasons.add(Season(data: season, episodes: episodes));
  }
  return seasons;
}

class EpisodesList extends ConsumerWidget {
  final ItemDetailsModel model;
  final void Function(api.MediaItem) onEpisodePressed;

  const EpisodesList({
    Key? key,
    required this.model,
    required this.onEpisodePressed,
  }) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final isDesktop = MediaQuery.of(context).isDesktop;
    return MultiSliver(
      children: [
        for (final season in model.seasons)
          _EpisodesListInner(
            season: season,
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
  final SeasonModel season;
  final void Function(api.MediaItem) onEpisodePressed;

  const _EpisodesListInner({
    Key? key,
    required this.season,
    required this.onEpisodePressed,
  }) : super(key: key);

  @override
  State<_EpisodesListInner> createState() => _EpisodesListInnerState();
}

class _EpisodesListInnerState extends State<_EpisodesListInner> {
  final controller = ScrollController();

  api.MediaItem get season => widget.season.item;
  List<api.MediaItem> get episodes => widget.season.episodes;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final mediaQuery = MediaQuery.of(context);
    final isDesktop = mediaQuery.isDesktop;
    final width = mediaQuery.size.width - (isDesktop ? 224 : 0);
    final thumbnailWidth = isDesktop ? 280.0 : 160.0;
    return MultiSliver(
      children: [
        SliverToBoxAdapter(
          child: Padding(
            padding: isDesktop
                ? const EdgeInsets.symmetric(horizontal: 128)
                : const EdgeInsets.symmetric(horizontal: 16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const SizedBox(height: 32),
                Text(season.name, style: theme.textTheme.headlineMedium),
                const SizedBox(height: 16),
              ],
            ),
          ),
        ),
        SliverPadding(
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
              (context, index) => EpisodeListItem(
                episode: episodes[index],
                width: thumbnailWidth,
                onPressed: () => widget.onEpisodePressed(episodes[index]),
              ),
              childCount: episodes.length,
            ),
          ),
        ),
      ],
    );
  }
}

class EpisodeListItem extends ConsumerWidget {
  const EpisodeListItem({
    Key? key,
    required this.episode,
    required this.width,
    required this.onPressed,
  }) : super(key: key);

  final api.MediaItem episode;
  final double width;
  final void Function() onPressed;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final zenith = ref.watch(api.apiProvider);
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
                    child: EpisodeThumbnail(
                      url: zenith.getMediaImageUrl(
                          episode.id, api.ImageType.thumbnail),
                      isWatched: episode.videoUserData?.isWatched ?? false,
                    ),
                  ),
                  const SizedBox(width: 12),
                  Flexible(
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        TextOneLine(
                            '${episode.parent!.index} - ${episode.name}',
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

class EpisodeThumbnail extends StatelessWidget {
  final String? url;
  final bool isWatched;

  const EpisodeThumbnail({
    Key? key,
    required this.url,
    required this.isWatched,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Material(
      elevation: 2.0,
      type: MaterialType.card,
      clipBehavior: Clip.hardEdge,
      borderRadius: const BorderRadius.all(Radius.circular(8)),
      child: Stack(children: [
        url == null
            ? const Icon(Icons.video_file, size: 48)
            : Positioned.fill(
                child: FadeInImage(
                  placeholder: MemoryImage(transparentImage),
                  image: DioImage.string(url!),
                  fit: BoxFit.cover,
                ),
              ),
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
