import 'dart:math' as math;

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:sized_context/sized_context.dart';
import 'package:zenith/api.dart';
import 'package:zenith/main.dart';
import 'package:zenith/poster_item.dart';
import 'package:zenith/responsive.dart';

final _moviesProvider = FutureProvider((ref) async {
  final api = ref.watch(apiProvider);
  final movies = await api.fetchMovies();
  return movies.map((e) => _MediaLibraryItem.fromMediaItem(e, api)).toList();
});

class MoviesScreen extends ConsumerWidget {
  const MoviesScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return _MediaLibraryScreen(
      provider: _moviesProvider,
      posterFallback: Icons.movie,
      onRefresh: () => ref.refresh(_moviesProvider.future),
      onItemTap: (item) =>
          ref.read(routerProvider).push(ItemDetailsRoute(id: item.id)),
    );
  }
}

final _showsProvider = FutureProvider((ref) async {
  final api = ref.watch(apiProvider);
  final shows = await api.fetchShows();
  return shows.map((e) => _MediaLibraryItem.fromMediaItem(e, api)).toList();
});

class ShowsScreen extends ConsumerWidget {
  const ShowsScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return _MediaLibraryScreen(
      provider: _showsProvider,
      posterFallback: Icons.tv,
      onRefresh: () => ref.refresh(_showsProvider.future),
      onItemTap: (item) =>
          ref.read(routerProvider).push(ItemDetailsRoute(id: item.id)),
    );
  }
}

class _MediaLibraryItem {
  final int id;
  final String title;
  final String? subtitle;
  final String? poster;
  final bool isWatched;

  _MediaLibraryItem({
    required this.id,
    required this.title,
    required this.subtitle,
    required this.poster,
    required this.isWatched,
  });

  factory _MediaLibraryItem.fromMediaItem(
          MediaItem item, ZenithApiClient client) =>
      _MediaLibraryItem(
        id: item.id,
        title: item.name,
        subtitle: item.startDate?.year.toString(),
        poster: client.getMediaImageUrl(item.id, ImageType.poster),
        isWatched: () {
          if (item.type == MediaType.show) {
            return item.collectionUserData?.unwatched == 0;
          } else {
            return item.videoUserData?.isWatched ?? false;
          }
        }(),
      );
}

class _MediaLibraryScreen extends ConsumerWidget {
  final ProviderBase<AsyncValue<List<_MediaLibraryItem>>> provider;
  final IconData posterFallback;
  final Future<void> Function() onRefresh;
  final void Function(_MediaLibraryItem item) onItemTap;

  const _MediaLibraryScreen({
    required this.provider,
    required this.posterFallback,
    required this.onRefresh,
    required this.onItemTap,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final items = ref.watch(provider);
    return items.when(
      data: (data) => _MediaItemGrid(
        items: data,
        posterFallback: posterFallback,
        onRefresh: onRefresh,
        onItemTap: onItemTap,
      ),
      error: (error, stackTrace) => Center(child: Text('$error')),
      loading: () => const Center(child: CircularProgressIndicator()),
    );
  }
}

class _MediaItemGrid extends StatelessWidget {
  final List<_MediaLibraryItem> items;
  final IconData posterFallback;
  final Future<void> Function() onRefresh;
  final void Function(_MediaLibraryItem item) onItemTap;

  final ScrollController _scrollController = ScrollController();

  _MediaItemGrid({
    required this.items,
    required this.posterFallback,
    required this.onRefresh,
    required this.onItemTap,
  });

  @override
  Widget build(BuildContext context) {
    final desktop = MediaQuery.of(context).isDesktop;
    return LayoutBuilder(builder: ((context, constraints) {
      final maxColWidth = desktop ? 240.0 : 150.0;
      final gridPadding = desktop ? 24.0 : 12.0;
      final itemSpacing = desktop ? 16.0 : 8.0;

      final gridWidth = constraints.maxWidth - gridPadding * 2;
      final cols = (gridWidth / (maxColWidth + itemSpacing * 2)).ceil();
      final colWidth = gridWidth / cols;
      final infoTopPadding = desktop ? 16.0 : 8.0;

      return RefreshIndicator(
        triggerMode: RefreshIndicatorTriggerMode.anywhere,
        onRefresh: onRefresh,
        child: ListView.builder(
          physics: const AlwaysScrollableScrollPhysics(),
          controller: _scrollController,
          padding: EdgeInsets.all(gridPadding) + context.mq.padding,
          itemCount: (items.length / cols).ceil(),
          itemBuilder: (context, rowIndex) {
            final columns = <Widget>[];
            final maxItemIndex = math.min((rowIndex + 1) * cols, items.length);

            for (var i = rowIndex * cols; i < maxItemIndex; i++) {
              final item = items[i];
              columns.add(Container(
                width: colWidth,
                padding: EdgeInsets.all(itemSpacing / 2),
                child: PosterItem(
                  infoSeparator: infoTopPadding,
                  poster: item.poster,
                  posterFallback: posterFallback,
                  title: item.title,
                  subtitle: item.subtitle,
                  isWatched: item.isWatched,
                  onTap: () => onItemTap(item),
                ),
              ));
            }

            return Padding(
              padding: EdgeInsets.symmetric(vertical: itemSpacing / 2),
              child: Row(children: columns),
            );
          },
        ),
      );
    }));
  }
}
