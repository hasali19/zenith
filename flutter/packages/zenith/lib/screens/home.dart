import 'package:auto_route/auto_route.dart';
import 'package:drift/drift.dart' as drift;
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:gap/gap.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:sized_context/sized_context.dart';
import 'package:zenith/api.dart';
import 'package:zenith/constants.dart';
import 'package:zenith/database/database.dart' as db;
import 'package:zenith/image.dart';
import 'package:zenith/poster_item.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/router.dart';
import 'package:zenith/text_one_line.dart';
import 'package:zenith/theme.dart';

part 'home.freezed.dart';
part 'home.g.dart';

@freezed
abstract class HomeScreenData with _$HomeScreenData {
  factory HomeScreenData({
    required List<MediaItem> continueWatching,
    required List<MediaItem> recentMovies,
    required List<MediaItem> recentShows,
    required List<MediaItem> offlineItems,
  }) = _HomeScreenData;
}

@riverpod
Future<HomeScreenData> _state(Ref ref) async {
  final api = ref.watch(apiProvider);
  final database = ref.watch(db.databaseProvider);

  List<MediaItem> continueWatching = [];
  List<MediaItem> movies = [];
  List<MediaItem> shows = [];

  try {
    [continueWatching, movies, shows] = await Future.wait([
      api.fetchContinueWatching(),
      api.fetchRecentMovies(),
      api.fetchRecentShows(),
    ]);
  } catch (e) {
    // TODO: Show an error
  }

  final parentTable = database.mediaItems.createAlias('parent');
  final grandparentTable = database.mediaItems.createAlias('grandparent');
  final offlineItemsQuery =
      database.select(database.mediaItems).addColumns([]).join([
        drift.leftOuterJoin(
          parentTable,
          database.mediaItems.parentId.equalsExp(parentTable.id),
        ),
        drift.leftOuterJoin(
          grandparentTable,
          database.mediaItems.grandparentId.equalsExp(grandparentTable.id),
        ),
      ])..where(
        database.mediaItems.id.isInQuery(
          database.selectOnly(database.downloadedFiles)
            ..addColumns([database.downloadedFiles.itemId]),
        ),
      );

  final offlineItems = await offlineItemsQuery.get();

  return HomeScreenData(
    continueWatching: continueWatching,
    recentMovies: movies,
    recentShows: shows,
    offlineItems: offlineItems.map((row) {
      final item = row.readTable(database.mediaItems);
      final parentItem = row.readTableOrNull(parentTable);
      final grandparentItem = row.readTableOrNull(grandparentTable);

      MediaItemParent? parent;
      MediaItemParent? grandparent;

      if ((parentItem, item.parentIndex) case (
        final parentItem?,
        final index?,
      )) {
        parent = MediaItemParent(parentItem.id, index, parentItem.name);
      }

      if ((grandparentItem, item.grandparentIndex) case (
        final grandparentItem?,
        final index?,
      )) {
        grandparent = MediaItemParent(
          grandparentItem.id,
          index,
          grandparentItem.name,
        );
      }

      return MediaItem(
        id: item.id,
        type: switch (item.type) {
          .movie => .movie,
          .show => .show,
          .season => .season,
          .episode => .episode,
        },
        name: item.name,
        overview: item.overview,
        startDate: DateTime.tryParse(item.startDate ?? ''),
        endDate: DateTime.tryParse(item.endDate ?? ''),
        poster: item.poster as ImageId?,
        backdrop: item.backdrop as ImageId?,
        thumbnail: item.thumbnail as ImageId?,
        parent: parent,
        grandparent: grandparent,
        videoFile: null,
        videoUserData: null,
        collectionUserData: null,
        genres: [],
        ageRating: null,
        trailer: null,
        director: null,
        cast: [],
      );
    }).toList(),
  );
}

@RoutePage()
class HomeScreen extends ConsumerWidget {
  const HomeScreen({super.key});

  void _navigateToItem(BuildContext context, WidgetRef ref, int itemId) async {
    await context.router.push(ItemDetailsRoute(id: itemId));
    ref.invalidate(_stateProvider);
  }

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final desktop = MediaQuery.of(context).isDesktop;

    final sectionTitlePadding = EdgeInsets.symmetric(
      horizontal: desktop ? 32 : 16,
      vertical: 12,
    );
    final sectionListSpacing = desktop ? 16.0 : 8.0;
    const sectionEndPadding = 0.0;

    final thumbnailItemWidth = desktop ? 440.0 : 268.0;
    final thumbnailItemHeight = thumbnailItemWidth / (16 / 9);
    final thumbnailItemPadding = desktop ? 16.0 : 12.0;

    final posterItemWidth = desktop ? 200.0 : 120.0;
    final posterItemHeight = posterItemWidth / (2 / 3) + 52;
    final posterItemInfoSeparator = desktop ? 16.0 : 4.0;

    buildPosterItemSection(
      List<MediaItem> items,
      String title,
      IconData posterFallback,
    ) => Section<MediaItem>(
      title: title,
      titlePadding: sectionTitlePadding,
      listSpacing: sectionListSpacing,
      listItemWidth: posterItemWidth,
      listItemHeight: posterItemHeight,
      endPadding: sectionEndPadding,
      items: items,
      itemBuilder: (context, item) => PosterItem(
        imageId: item.poster,
        requestWidth: mediaPosterImageWidth,
        fallbackIcon: posterFallback,
        title: () {
          if (item.grandparent?.name case final showName?) {
            return showName;
          }
          return item.name;
        }(),
        subtitle: () {
          if (item.startDate case final startDate?) {
            return startDate.year.toString();
          }

          if (item.getSeasonEpisode() case final name?) {
            return name;
          }

          return '';
        }(),
        isWatched: true, // hide new icon since they're all new
        infoSeparator: posterItemInfoSeparator,
        onTap: () => _navigateToItem(context, ref, item.id),
      ),
    );

    final state = ref.watch(_stateProvider);
    return state.when(
      loading: () => const Center(child: CircularProgressIndicator()),
      error: (error, stackTrace) => Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            const Text('Failed to load data from server'),
            TextButton(
              onPressed: () => ref.invalidate(_stateProvider),
              child: const Text('Retry'),
            ),
          ],
        ),
      ),
      data: (data) => RefreshIndicator(
        triggerMode: RefreshIndicatorTriggerMode.anywhere,
        onRefresh: () => ref.refresh(_stateProvider.future),
        child: ListView(
          padding: const EdgeInsets.symmetric(vertical: 16),
          children: [
            if (data.continueWatching.isNotEmpty)
              Section<MediaItem>(
                title: 'Continue Watching',
                titlePadding: sectionTitlePadding,
                listSpacing: sectionListSpacing,
                listItemWidth: thumbnailItemWidth,
                listItemHeight: thumbnailItemHeight,
                endPadding: sectionEndPadding,
                items: data.continueWatching,
                itemBuilder: (context, item) => ContinueWatchingCard(
                  imageId: item.thumbnail,
                  requestWidth: mediaThumbnailImageWidth,
                  title: item.name,
                  subtitle: item.type == MediaType.episode
                      ? '${item.getSeasonEpisode()!}: ${item.grandparent!.name}'
                      : item.startDate?.year.toString() ?? '',
                  progress:
                      (item.videoUserData?.position ?? 0) /
                      (item.videoFile?.duration ?? 1),
                  padding: thumbnailItemPadding,
                  onTap: () => _navigateToItem(context, ref, item.id),
                ),
              ),
            if (data.recentMovies.isNotEmpty)
              buildPosterItemSection(
                data.recentMovies,
                'Recent Movies',
                Icons.movie,
              ),
            if (data.recentShows.isNotEmpty)
              buildPosterItemSection(
                data.recentShows,
                'Recent Shows',
                Icons.tv,
              ),
            if (data.offlineItems.isNotEmpty)
              buildPosterItemSection(
                data.offlineItems,
                'Available Offline',
                Icons.movie,
              ),
          ],
        ),
      ),
    );
  }
}

class Section<T> extends HookWidget {
  final String title;
  final List<T> items;
  final EdgeInsets titlePadding;
  final double listSpacing;
  final double listItemWidth;
  final double listItemHeight;
  final double endPadding;
  final Widget Function(BuildContext context, T item) itemBuilder;

  const Section({
    super.key,
    required this.title,
    required this.items,
    required this.itemBuilder,
    required this.titlePadding,
    required this.listSpacing,
    required this.listItemWidth,
    required this.listItemHeight,
    required this.endPadding,
  });

  @override
  Widget build(BuildContext context) {
    final scrollController = useScrollController();

    final titleStyle = context.zenithTheme.titleMedium.copyWith(
      fontWeight: FontWeight.bold,
    );

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Padding(
          padding:
              titlePadding + context.mq.padding.copyWith(top: 0, bottom: 0),
          child: Text(title, style: titleStyle),
        ),
        _buildContent(context, scrollController, items),
        SizedBox(height: endPadding),
      ],
    );
  }

  Widget _buildContent(
    BuildContext context,
    ScrollController scrollController,
    List<T> data,
  ) {
    const scrollbarHeight = 16.0;
    return SizedBox(
      height: listItemHeight + scrollbarHeight,
      child: ScrollbarTheme(
        data: ScrollbarTheme.of(
          context,
        ).copyWith(mainAxisMargin: listSpacing * 2),
        child: Scrollbar(
          controller: scrollController,
          child: Padding(
            padding: const EdgeInsets.only(bottom: scrollbarHeight),
            child: ListView.separated(
              controller: scrollController,
              padding:
                  EdgeInsets.symmetric(horizontal: listSpacing * 2) +
                  context.mq.padding.copyWith(top: 0, bottom: 0),
              separatorBuilder: (context, index) =>
                  SizedBox(width: listSpacing),
              scrollDirection: Axis.horizontal,
              itemCount: data.length,
              itemBuilder: (context, index) => SizedBox(
                width: listItemWidth,
                child: itemBuilder(context, data[index]),
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class ContinueWatchingCard extends StatelessWidget {
  final ImageId? imageId;
  final int requestWidth;
  final String title;
  final String subtitle;
  final double progress;
  final double padding;
  final void Function() onTap;

  const ContinueWatchingCard({
    super.key,
    required this.imageId,
    required this.requestWidth,
    required this.title,
    required this.subtitle,
    required this.progress,
    required this.padding,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final cardTheme = theme.cardTheme;

    final titleStyle = context.zenithTheme.bodyMedium.copyWith(
      color: Colors.white,
    );
    final subtitleStyle = context.zenithTheme.bodySmall.copyWith(
      color: Colors.white,
    );

    return AspectRatio(
      aspectRatio: 16 / 9,
      child: Stack(
        children: [
          Positioned.fill(
            child: Card(
              elevation: cardTheme.elevation ?? 1,
              margin: EdgeInsets.zero,
              clipBehavior: Clip.hardEdge,
              shape: cardTheme.shape,
              child: switch (imageId) {
                null => Center(child: Icon(Icons.video_file, size: 40)),
                final imageId => ZenithApiImage(
                  id: imageId,
                  requestWidth: requestWidth,
                ),
              },
            ),
          ),
          Positioned.fill(
            child: Material(
              color: Colors.transparent,
              clipBehavior: Clip.hardEdge,
              shape: cardTheme.shape,
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
                        const Gap(8),
                        ClipRRect(
                          borderRadius: BorderRadius.circular(4),
                          child: LinearProgressIndicator(
                            value: progress,
                            backgroundColor: Colors.white,
                          ),
                        ),
                      ],
                    ],
                  ),
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
