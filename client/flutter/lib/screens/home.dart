import 'package:assorted_layout_widgets/assorted_layout_widgets.dart';
import 'package:flutter/material.dart';
import 'package:zenith_flutter/screens/show_details.dart';
import 'package:zenith_flutter/screens/video_player.dart';

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
    return ListView(
      controller: _scrollController,
      padding: const EdgeInsets.symmetric(vertical: 16),
      children: [
        Section<ContinueWatchingItem>(
          title: "Continue Watching",
          height: 300,
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
          height: 340,
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
          height: 340,
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
  final double height;
  final Widget Function(BuildContext context, T item) itemBuilder;

  const Section({
    Key? key,
    required this.title,
    required this.future,
    required this.height,
    required this.itemBuilder,
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
          padding: const EdgeInsets.fromLTRB(32, 32, 32, 16),
          child: Text(widget.title, style: textTheme.headline5),
        ),
        FutureBuilder<List<T>>(
          future: widget.future,
          builder: ((context, snapshot) {
            if (snapshot.hasData) {
              final data = snapshot.data!;
              return SizedBox(
                height: widget.height,
                child: ListView.separated(
                  controller: _scrollController,
                  padding: const EdgeInsets.symmetric(horizontal: 32),
                  separatorBuilder: (context, index) =>
                      const SizedBox(width: 16),
                  scrollDirection: Axis.horizontal,
                  itemCount: data.length,
                  itemBuilder: (context, index) =>
                      widget.itemBuilder(context, data[index]),
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

  const ThumbnailItem({
    Key? key,
    required this.thumbnail,
    required this.title,
    required this.subtitle,
    required this.progress,
    required this.onTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    const width = 350.0;
    const height = width * (9.0 / 16.0);
    final textTheme = Theme.of(context).textTheme;
    return SizedBox(
      width: width,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Stack(
            children: [
              SizedBox(
                height: height,
                child: Material(
                  elevation: 2.0,
                  type: MaterialType.card,
                  clipBehavior: Clip.hardEdge,
                  borderRadius: const BorderRadius.all(Radius.circular(16)),
                  child: Ink.image(
                    fit: BoxFit.cover,
                    image: NetworkImage(thumbnail),
                    child: InkWell(onTap: onTap),
                  ),
                ),
              ),
              Positioned(
                bottom: 0,
                left: 0,
                right: 0,
                child: Padding(
                  padding: const EdgeInsets.all(16.0),
                  child: ClipRRect(
                    borderRadius: BorderRadius.circular(4),
                    child: LinearProgressIndicator(
                      value: progress,
                      backgroundColor: Colors.white,
                    ),
                  ),
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),
          TextOneLine(
            title,
            style: textTheme.titleMedium,
          ),
          const SizedBox(height: 4),
          TextOneLine(
            subtitle,
            style: textTheme.titleSmall!
                .copyWith(color: textTheme.titleSmall!.color!.withAlpha(150)),
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
    const height = 250.0;
    const width = height * 2.0 / 3.0;
    final textTheme = Theme.of(context).textTheme;
    return SizedBox(
      width: width,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(
            height: height,
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
          const SizedBox(height: 4),
          TextOneLine(
            subtitle,
            style: textTheme.titleSmall!
                .copyWith(color: textTheme.titleSmall!.color!.withAlpha(150)),
          ),
        ],
      ),
    );
  }
}
