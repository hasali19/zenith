import 'package:dio_image_provider/dio_image_provider.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:gap/gap.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:sized_context/sized_context.dart';
import 'package:zenith/api.dart';
import 'package:zenith/main_router.dart';
import 'package:zenith/poster_item.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/router/stack_observer.dart';
import 'package:zenith/router/stack_router.dart';
import 'package:zenith/text_one_line.dart';
import 'package:zenith/theme.dart';

part 'home.freezed.dart';
part 'home.g.dart';

@freezed
class HomeScreenData with _$HomeScreenData {
  factory HomeScreenData({
    required List<MediaItem> continueWatching,
    required List<MediaItem> recentMovies,
    required List<MediaItem> recentShows,
  }) = _HomeScreenData;
}

@riverpod
Future<HomeScreenData> _state(_StateRef ref) async {
  final api = ref.watch(apiProvider);

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
}

class HomeScreen extends ConsumerStatefulWidget {
  const HomeScreen({super.key});

  @override
  ConsumerState<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends ConsumerState<HomeScreen> {
  final _scrollController = ScrollController();

  ZenithApiClient get api => ref.watch(apiProvider);

  void _navigateToItem(MediaItem item) async {
    StackRouter.of<PrimaryRoute>(context).push(ItemDetailsRoute(id: item.id));
  }

  @override
  Widget build(BuildContext context) {
    final desktop = MediaQuery.of(context).isDesktop;

    final sectionTitlePadding =
        EdgeInsets.symmetric(horizontal: desktop ? 32 : 16, vertical: 12);
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
            posterFallback: posterFallback,
            title: item.name,
            subtitle: item.startDate?.year.toString() ?? '',
            isWatched: true, // hide new icon since they're all new
            infoSeparator: posterItemInfoSeparator,
            onTap: () => _navigateToItem(item),
          ),
        );

    final state = ref.watch(_stateProvider);
    return StackObserver(
      onPopNext: () {
        ref.invalidate(_stateProvider);
      },
      child: state.when(
        loading: () => const Center(
          child: CircularProgressIndicator(),
        ),
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
            controller: _scrollController,
            padding:
                const EdgeInsets.symmetric(vertical: 16) + context.mq.padding,
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
                    thumbnail:
                        api.getMediaImageUrl(item.id, ImageType.thumbnail),
                    title: item.name,
                    subtitle: item.type == MediaType.episode
                        ? '${item.getSeasonEpisode()!}: ${item.grandparent!.name}'
                        : item.startDate?.year.toString() ?? '',
                    progress: (item.videoUserData?.position ?? 0) /
                        (item.videoFile?.duration ?? 1),
                    padding: thumbnailItemPadding,
                    onTap: () => _navigateToItem(item),
                  ),
                ),
              if (data.recentMovies.isNotEmpty)
                buildPosterItemSection(
                    data.recentMovies, 'Recent Movies', Icons.movie),
              if (data.recentShows.isNotEmpty)
                buildPosterItemSection(
                    data.recentShows, 'Recent Shows', Icons.tv),
            ],
          ),
        ),
      ),
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
    const scrollbarHeight = 16.0;
    return SizedBox(
      height: widget.listItemHeight + scrollbarHeight,
      child: ScrollbarTheme(
        data: ScrollbarTheme.of(context).copyWith(
          mainAxisMargin: widget.listSpacing * 2,
        ),
        child: Scrollbar(
          controller: _scrollController,
          child: Padding(
            padding: const EdgeInsets.only(bottom: scrollbarHeight),
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
          ),
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
    super.key,
    required this.thumbnail,
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

    final titleStyle =
        context.zenithTheme.bodyMedium.copyWith(color: Colors.white);
    final subtitleStyle =
        context.zenithTheme.bodySmall.copyWith(color: Colors.white);

    return AspectRatio(
      aspectRatio: 16 / 9,
      child: Card(
        margin: EdgeInsets.zero,
        elevation: cardTheme.elevation ?? 1,
        clipBehavior: Clip.hardEdge,
        shape: cardTheme.shape,
        child: Ink(
          decoration: BoxDecoration(
            image: DecorationImage(
              image: DioImage.string(thumbnail),
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
                    const Gap(8),
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
