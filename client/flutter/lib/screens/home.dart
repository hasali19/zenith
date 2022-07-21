import 'package:flutter/material.dart';
import 'package:zenith_flutter/screens/show_details.dart';
import 'package:zenith_flutter/screens/video_player.dart';
import 'package:zenith_flutter/text_one_line.dart';

import '../api.dart' as api;

class HomeScreen extends StatefulWidget {
  const HomeScreen({Key? key}) : super(key: key);

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class ContinueWatchingItem {
  final int id;
  final String? thumbnail;
  final String title;
  final String subtitle;
  final double progress;
  final double startPosition;

  ContinueWatchingItem({
    required this.id,
    required this.thumbnail,
    required this.title,
    required this.subtitle,
    required this.progress,
    required this.startPosition,
  });

  factory ContinueWatchingItem.fromMediaItem(api.VideoItem item) {
    final String title;
    final String subtitle;
    if (item is api.Movie) {
      title = item.title;
      subtitle = item.year.toString();
    } else if (item is api.Episode) {
      title = item.showName;
      subtitle = item.formatSeasonEpisode();
    } else {
      throw Exception("invalid media item: $item");
    }

    return ContinueWatchingItem(
      id: item.id,
      thumbnail: item.thumbnail,
      title: title,
      subtitle: subtitle,
      progress:
          (item.userData?.position ?? 0) / (item.videoInfo?.duration ?? 1),
      startPosition: item.userData?.position ?? 0,
    );
  }
}

class _HomeScreenState extends State<HomeScreen> {
  final _scrollController = ScrollController();

  late Future<List<ContinueWatchingItem>> _continueWatching;
  late Future<List<api.Movie>> _recentMovies;
  late Future<List<api.Show>> _recentShows;

  @override
  void initState() {
    super.initState();
    _refresh();
  }

  void _refresh() {
    setState(() {
      _continueWatching = api.fetchContinueWatching().then((value) =>
          value.map((e) => ContinueWatchingItem.fromMediaItem(e)).toList());
      _recentMovies = api.fetchRecentMovies();
      _recentShows = api.fetchRecentShows();
    });
  }

  @override
  Widget build(BuildContext context) {
    const sectionTitlePadding = EdgeInsets.fromLTRB(32, 32, 32, 16);
    const sectionListPadding = EdgeInsets.symmetric(horizontal: 32);
    const sectionListSpacing = 16.0;

    const thumbnailItemWidth = 350.0;
    const thumbnailItemHeight = thumbnailItemWidth / (16 / 9);

    const posterItemWidth = 180.0;
    const posterItemHeight = posterItemWidth / (2 / 3) + 64;

    return ListView(
      controller: _scrollController,
      padding: const EdgeInsets.symmetric(vertical: 16),
      children: [
        Section<ContinueWatchingItem>(
          title: "Continue Watching",
          titlePadding: sectionTitlePadding,
          listPadding: sectionListPadding,
          listSpacing: sectionListSpacing,
          listItemWidth: thumbnailItemWidth,
          listItemHeight: thumbnailItemHeight,
          future: _continueWatching,
          itemBuilder: (context, item) => ThumbnailItem(
            thumbnail: item.thumbnail!,
            title: item.title,
            subtitle: item.subtitle,
            progress: item.progress,
            onTap: () async {
              await Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (context) => VideoPlayerScreen(
                    id: item.id,
                    startPosition: item.startPosition,
                  ),
                ),
              );
              _refresh();
            },
          ),
        ),
        Section<api.Movie>(
          title: "Recent Movies",
          titlePadding: sectionTitlePadding,
          listPadding: sectionListPadding,
          listSpacing: sectionListSpacing,
          listItemWidth: posterItemWidth,
          listItemHeight: posterItemHeight,
          future: _recentMovies,
          itemBuilder: (context, item) => PosterItem(
              poster: item.poster,
              title: item.title,
              subtitle: item.subtitle,
              onTap: () async {
                await Navigator.push(
                  context,
                  MaterialPageRoute(
                    builder: (context) => VideoPlayerScreen(
                      id: item.id,
                      startPosition: item.userData?.position ?? 0,
                    ),
                  ),
                );
                _refresh();
              }),
        ),
        Section<api.Show>(
          title: "Recent Shows",
          titlePadding: sectionTitlePadding,
          listPadding: sectionListPadding,
          listSpacing: sectionListSpacing,
          listItemWidth: posterItemWidth,
          listItemHeight: posterItemHeight,
          future: _recentShows,
          itemBuilder: (context, item) => PosterItem(
              poster: item.poster,
              title: item.title,
              subtitle: item.subtitle,
              onTap: () async {
                await Navigator.push(
                  context,
                  MaterialPageRoute(
                    builder: (context) => ShowDetailsScreen(show: item),
                  ),
                );
                _refresh();
              }),
        ),
      ],
    );
  }
}

class Section<T> extends StatefulWidget {
  final String title;
  final Future<List<T>> future;
  final EdgeInsets titlePadding;
  final EdgeInsets listPadding;
  final double listSpacing;
  final double listItemWidth;
  final double listItemHeight;
  final Widget Function(BuildContext context, T item) itemBuilder;

  const Section({
    Key? key,
    required this.title,
    required this.future,
    required this.itemBuilder,
    required this.titlePadding,
    required this.listPadding,
    required this.listSpacing,
    required this.listItemWidth,
    required this.listItemHeight,
  }) : super(key: key);

  @override
  State<Section<T>> createState() => _SectionState<T>();
}

class _SectionState<T> extends State<Section<T>> {
  final ScrollController _scrollController = ScrollController();

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final textTheme = theme.textTheme;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Padding(
          padding: widget.titlePadding,
          child: Text(widget.title, style: textTheme.headline5),
        ),
        FutureBuilder<List<T>>(
          future: widget.future,
          builder: ((context, snapshot) {
            if (snapshot.hasData) {
              final data = snapshot.data!;
              return SizedBox(
                height: widget.listItemHeight,
                child: ListView.separated(
                  controller: _scrollController,
                  padding: widget.listPadding,
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
            } else {
              return const Center(
                child: CircularProgressIndicator(),
              );
            }
          }),
        ),
      ],
    );
  }
}

class ThumbnailItem extends StatelessWidget {
  final String thumbnail;
  final String title;
  final String subtitle;
  final double progress;
  final void Function() onTap;
  final double borderRadius;

  const ThumbnailItem({
    Key? key,
    required this.thumbnail,
    required this.title,
    required this.subtitle,
    required this.progress,
    required this.onTap,
    this.borderRadius = 16,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final textTheme = Theme.of(context).textTheme;
    return AspectRatio(
      aspectRatio: 16 / 9,
      child: Stack(
        children: [
          Material(
            elevation: 2.0,
            type: MaterialType.card,
            clipBehavior: Clip.hardEdge,
            borderRadius: BorderRadius.all(Radius.circular(borderRadius)),
            child: Ink.image(
              fit: BoxFit.cover,
              image: NetworkImage(thumbnail),
              child: InkWell(onTap: onTap),
            ),
          ),
          Positioned(
            bottom: 0,
            left: 0,
            right: 0,
            child: Padding(
              padding: const EdgeInsets.all(16.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  TextOneLine(
                    title,
                    style: textTheme.titleMedium,
                  ),
                  TextOneLine(
                    subtitle,
                    style: textTheme.titleSmall!.copyWith(
                        color: textTheme.titleSmall!.color!.withAlpha(150)),
                  ),
                  const SizedBox(height: 8),
                  ClipRRect(
                    borderRadius: BorderRadius.circular(4),
                    child: LinearProgressIndicator(
                      value: progress,
                      backgroundColor: Colors.white,
                    ),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class PosterItem extends StatelessWidget {
  final String poster;
  final String title;
  final String subtitle;
  final void Function() onTap;

  const PosterItem({
    Key? key,
    required this.poster,
    required this.title,
    required this.subtitle,
    required this.onTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final textTheme = Theme.of(context).textTheme;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        AspectRatio(
          aspectRatio: 2 / 3,
          child: Material(
            elevation: 2.0,
            type: MaterialType.card,
            clipBehavior: Clip.hardEdge,
            borderRadius: const BorderRadius.all(Radius.circular(16)),
            child: Ink.image(
              fit: BoxFit.cover,
              image: NetworkImage(poster),
              child: InkWell(onTap: onTap),
            ),
          ),
        ),
        const SizedBox(height: 16),
        TextOneLine(title, style: textTheme.titleMedium),
        TextOneLine(
          subtitle,
          style: textTheme.titleSmall!
              .copyWith(color: textTheme.titleSmall!.color!.withAlpha(150)),
        ),
      ],
    );
  }
}
