import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:sized_context/sized_context.dart';
import 'package:zenith_flutter/api.dart';
import 'package:zenith_flutter/poster_item.dart';
import 'package:zenith_flutter/responsive.dart';
import 'package:zenith_flutter/router.dart';
import 'package:zenith_flutter/text_one_line.dart';
import 'package:zenith_flutter/theme.dart';

class HomeScreenData {
  List<MediaItem> continueWatching;
  List<MediaItem> recentMovies;
  List<MediaItem> recentShows;

  HomeScreenData({
    required this.continueWatching,
    required this.recentMovies,
    required this.recentShows,
  });
}

class HomeScreen extends ConsumerStatefulWidget {
  const HomeScreen({Key? key}) : super(key: key);

  @override
  ConsumerState<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends ConsumerState<HomeScreen> {
  final _scrollController = ScrollController();

  late Future<HomeScreenData> _data;

  ZenithApiClient get api => ref.watch(apiProvider);

  @override
  void initState() {
    super.initState();
    _refresh();
  }

  void _refresh() {
    setState(() {
      _data = Future(() async {
        final results = await Future.wait([
          api.fetchContinueWatching(),
          api.fetchRecentMovies(),
          api.fetchRecentShows(),
        ]);

        return HomeScreenData(
          continueWatching: results[0],
          recentMovies: results[1],
          recentShows: results[2],
        );
      });
    });
  }

  void _navigateToItem(MediaItem item) async {
    await context.router.push(ItemDetailsScreenRoute(id: item.id));
    _refresh();
  }

  @override
  Widget build(BuildContext context) {
    final desktop = MediaQuery.of(context).isDesktop;

    final sectionTitlePadding = desktop
        ? const EdgeInsets.fromLTRB(64, 16, 64, 16)
        : const EdgeInsets.fromLTRB(16, 8, 16, 8);
    final sectionListSpacing = desktop ? 32.0 : 8.0;
    final sectionEndPadding = desktop ? 32.0 : 16.0;

    final thumbnailItemWidth = desktop ? 440.0 : 268.0;
    final thumbnailItemHeight = thumbnailItemWidth / (16 / 9);
    final thumbnailItemPadding = desktop ? 16.0 : 12.0;

    final posterItemWidth = desktop ? 200.0 : 120.0;
    final posterItemHeight = posterItemWidth / (2 / 3) + 52;
    final posterItemInfoSeparator = desktop ? 16.0 : 4.0;

    buildPosterItemSection(
      List<MediaItem> items,
      String title,
    ) =>
        Section<MediaItem>(
          title: title,
          titlePadding: sectionTitlePadding,
          listSpacing: sectionListSpacing,
          listItemWidth: posterItemWidth,
          listItemHeight: posterItemHeight,
          endPadding: sectionEndPadding,
          items: items,
          itemBuilder: (context, item) => PosterItem(
            poster: api.getMediaImageUrl(item.id, ImageType.poster),
            title: item.name,
            subtitle: item.startDate?.year.toString() ?? "",
            infoSeparator: posterItemInfoSeparator,
            onTap: () => _navigateToItem(item),
          ),
        );

    return FutureBuilder<HomeScreenData>(
      future: _data,
      builder: (context, snapshot) {
        if (!snapshot.hasData) {
          return const Center(
            child: CircularProgressIndicator(),
          );
        }

        final data = snapshot.data!;
        return RefreshIndicator(
          onRefresh: () async {
            _refresh();
            await _data;
          },
          child: ListView(
            controller: _scrollController,
            padding:
                const EdgeInsets.symmetric(vertical: 16) + context.mq.padding,
            children: [
              Section<MediaItem>(
                title: "Continue Watching",
                titlePadding: sectionTitlePadding,
                listSpacing: sectionListSpacing,
                listItemWidth: thumbnailItemWidth,
                listItemHeight: thumbnailItemHeight,
                endPadding: sectionEndPadding,
                items: data.continueWatching,
                itemBuilder: (context, item) => ContinueWatchingCard(
                  thumbnail: api.getMediaImageUrl(item.id, ImageType.thumbnail),
                  title: item.name,
                  subtitle: item.type == MediaType.episode
                      ? item.getSeasonEpisode()! + ": " + item.grandparent!.name
                      : item.startDate?.year.toString() ?? "",
                  progress: (item.videoUserData?.position ?? 0) /
                      (item.videoInfo?.duration ?? 1),
                  padding: thumbnailItemPadding,
                  onTap: () => _navigateToItem(item),
                ),
              ),
              buildPosterItemSection(data.recentMovies, "Recent Movies"),
              buildPosterItemSection(data.recentShows, "Recent Shows"),
            ],
          ),
        );
      },
    );
  }
}

class Section<T> extends StatefulWidget {
  final String title;
  final List<T> items;
  final EdgeInsets titlePadding;
  final double listSpacing;
  final double listItemWidth;
  final double listItemHeight;
  final double endPadding;
  final Widget Function(BuildContext context, T item) itemBuilder;

  const Section({
    Key? key,
    required this.title,
    required this.items,
    required this.itemBuilder,
    required this.titlePadding,
    required this.listSpacing,
    required this.listItemWidth,
    required this.listItemHeight,
    required this.endPadding,
  }) : super(key: key);

  @override
  State<Section<T>> createState() => _SectionState<T>();
}

class _SectionState<T> extends State<Section<T>> {
  final ScrollController _scrollController = ScrollController();

  @override
  Widget build(BuildContext context) {
    final titleStyle =
        context.zenithTheme.titleMedium.copyWith(fontWeight: FontWeight.bold);
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Padding(
          padding: widget.titlePadding,
          child: Text(widget.title, style: titleStyle),
        ),
        _buildContent(widget.items),
        SizedBox(height: widget.endPadding),
      ],
    );
  }

  Widget _buildContent(List<T> data) {
    return SizedBox(
      height: widget.listItemHeight,
      child: ListView.separated(
        controller: _scrollController,
        padding: EdgeInsets.symmetric(horizontal: widget.listSpacing * 2),
        separatorBuilder: (context, index) =>
            SizedBox(width: widget.listSpacing),
        scrollDirection: Axis.horizontal,
        itemCount: data.length,
        itemBuilder: (context, index) => SizedBox(
          width: widget.listItemWidth,
          child: widget.itemBuilder(context, data[index]),
        ),
      ),
    );
  }
}

class ContinueWatchingCard extends StatelessWidget {
  final String thumbnail;
  final String title;
  final String subtitle;
  final double progress;
  final double padding;
  final void Function() onTap;

  const ContinueWatchingCard({
    Key? key,
    required this.thumbnail,
    required this.title,
    required this.subtitle,
    required this.progress,
    required this.padding,
    required this.onTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final cardTheme = theme.cardTheme;

    final titleStyle =
        context.zenithTheme.bodyMedium.copyWith(color: Colors.white);
    final subtitleStyle =
        context.zenithTheme.bodySmall.copyWith(color: Colors.white);

    return AspectRatio(
      aspectRatio: 16 / 9,
      child: Material(
        elevation: cardTheme.elevation ?? 1,
        type: MaterialType.card,
        clipBehavior: Clip.hardEdge,
        shape: cardTheme.shape,
        child: Ink(
          decoration: BoxDecoration(
            image: DecorationImage(
              image: NetworkImage(thumbnail),
              fit: BoxFit.cover,
              filterQuality: FilterQuality.medium,
            ),
          ),
          child: InkWell(
            onTap: onTap,
            child: Container(
              alignment: Alignment.bottomLeft,
              padding: EdgeInsets.all(padding),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisSize: MainAxisSize.min,
                children: [
                  TextOneLine(title, style: titleStyle),
                  TextOneLine(subtitle, style: subtitleStyle),
                  if (progress > 0.05 && progress < 0.9) ...[
                    const SizedBox(height: 8),
                    ClipRRect(
                      borderRadius: BorderRadius.circular(4),
                      child: LinearProgressIndicator(
                        value: progress,
                        backgroundColor: Colors.white,
                      ),
                    ),
                  ]
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
