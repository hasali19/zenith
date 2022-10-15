import 'package:flutter/material.dart';
import 'package:sliver_tools/sliver_tools.dart';
import 'package:zenith_flutter/responsive.dart';
import 'package:zenith_flutter/screens/item_details.dart';
import 'package:zenith_flutter/screens/video_details_screen.dart';
import 'package:zenith_flutter/text_one_line.dart';

import '../api.dart' as api;

class Season {
  final api.MediaItem data;
  final List<api.MediaItem> episodes;

  Season({
    required this.data,
    required this.episodes,
  });
}

Future<List<Season>> fetchSeasons(int showId) async {
  final seasons = <Season>[];
  for (final season in await api.fetchSeasons(showId)) {
    final episodes = await api.fetchEpisodes(season.id);
    seasons.add(Season(data: season, episodes: episodes));
  }
  return seasons;
}

class ShowDetailsScreen extends StatefulWidget {
  final api.MediaItem show;

  const ShowDetailsScreen({Key? key, required this.show}) : super(key: key);

  @override
  State<ShowDetailsScreen> createState() => _ShowDetailsScreenState();
}

class _ShowDetailsScreenState extends State<ShowDetailsScreen> {
  late Future<List<Season>> _seasons;

  @override
  void initState() {
    super.initState();
    _seasons = fetchSeasons(widget.show.id);
  }

  @override
  Widget build(BuildContext context) {
    final isDesktop = MediaQuery.of(context).isDesktop;
    return ItemDetailsScreen(
      item: widget.show,
      body: FutureBuilder<List<Season>>(
        future: _seasons,
        builder: (context, snapshot) {
          return MultiSliver(
            children: [
              if (snapshot.hasData) ...[
                for (final Season season in snapshot.data!)
                  EpisodeList(season: season),
                SliverToBoxAdapter(
                  child: SizedBox(height: isDesktop ? 128 : 16),
                ),
              ]
            ],
          );
        },
      ),
    );
  }
}

class EpisodeList extends StatefulWidget {
  final Season season;

  const EpisodeList({
    Key? key,
    required this.season,
  }) : super(key: key);

  @override
  State<EpisodeList> createState() => _EpisodeListState();
}

class _EpisodeListState extends State<EpisodeList> {
  final controller = ScrollController();

  api.MediaItem get season => widget.season.data;
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
                Text(season.name, style: theme.textTheme.headline4),
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
              ),
              childCount: episodes.length,
            ),
          ),
        ),
      ],
    );
  }
}

class EpisodeListItem extends StatelessWidget {
  const EpisodeListItem({
    Key? key,
    required this.episode,
    required this.width,
  }) : super(key: key);

  final api.MediaItem episode;
  final double width;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
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
                      url: api.getMediaImageUrl(
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
                            "${episode.parent!.index} - ${episode.name}",
                            style: theme.textTheme.titleMedium),
                        const SizedBox(height: 8),
                        Flexible(
                          child: Text(
                            episode.overview ?? "",
                            style: theme.textTheme.bodySmall,
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
              child: InkWell(
                onTap: () {
                  Navigator.push(
                    context,
                    MaterialPageRoute(
                      builder: (context) => VideoDetailsScreen(item: episode),
                    ),
                  );
                },
              ),
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
                child: FadeInImage.memoryNetwork(
                  placeholder: transparentImage,
                  image: url!,
                  fit: BoxFit.cover,
                ),
              ),
        if (isWatched)
          Container(
            color: Colors.black.withAlpha(127),
            child: const Center(child: Icon(Icons.check, size: 36)),
          ),
      ]),
    );
  }
}
