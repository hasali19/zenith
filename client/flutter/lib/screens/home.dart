import 'package:flutter/material.dart';
import 'package:zenith_flutter/responsive.dart';
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
      thumbnail:
          "https://zenith.hasali.uk/api/items/${item.id}/images/thumbnail",
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
    final desktop = MediaQuery.of(context).isDesktop;
    final theme = Theme.of(context);

    final sectionTitlePadding = desktop
        ? const EdgeInsets.fromLTRB(32, 16, 32, 16)
        : const EdgeInsets.fromLTRB(16, 8, 16, 8);
    final sectionTitleStyle = desktop
        ? theme.textTheme.headline5
        : theme.textTheme.titleMedium?.copyWith(fontWeight: FontWeight.bold);
    final sectionListPadding = desktop
        ? const EdgeInsets.symmetric(horizontal: 32)
        : const EdgeInsets.symmetric(horizontal: 16);
    final sectionListSpacing = desktop ? 16.0 : 8.0;

    final cardBorderRadius = desktop ? 16.0 : 8.0;

    final thumbnailItemWidth = desktop ? 350.0 : 268.0;
    final thumbnailItemHeight = thumbnailItemWidth / (16 / 9);
    final thumbnailItemPadding = desktop ? 16.0 : 12.0;

    final posterItemWidth = desktop ? 180.0 : 120.0;
    final posterItemHeight = posterItemWidth / (2 / 3) + (desktop ? 64 : 40);
    final posterItemInfoSeparator = desktop ? 16.0 : 4.0;
    final primaryTextStyle =
        desktop ? theme.textTheme.titleMedium : theme.textTheme.bodyText2;
    final secondaryTextStyle = desktop
        ? theme.textTheme.titleSmall!
            .copyWith(color: theme.textTheme.titleSmall!.color!.withAlpha(150))
        : theme.textTheme.caption;

    return ListView(
      controller: _scrollController,
      padding: const EdgeInsets.symmetric(vertical: 16),
      children: [
        Section<ContinueWatchingItem>(
          title: "Continue Watching",
          titlePadding: sectionTitlePadding,
          titleStyle: sectionTitleStyle,
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
            borderRadius: cardBorderRadius,
            padding: thumbnailItemPadding,
            primaryTextStyle: primaryTextStyle,
            secondaryTextStyle: secondaryTextStyle,
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
          titleStyle: sectionTitleStyle,
          listPadding: sectionListPadding,
          listSpacing: sectionListSpacing,
          listItemWidth: posterItemWidth,
          listItemHeight: posterItemHeight,
          future: _recentMovies,
          itemBuilder: (context, item) => PosterItem(
              poster: item.poster,
              title: item.title,
              subtitle: item.subtitle,
              borderRadius: cardBorderRadius,
              infoSeparator: posterItemInfoSeparator,
              primaryTextStyle: primaryTextStyle,
              secondaryTextStyle: secondaryTextStyle,
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
          titleStyle: sectionTitleStyle,
          listPadding: sectionListPadding,
          listSpacing: sectionListSpacing,
          listItemWidth: posterItemWidth,
          listItemHeight: posterItemHeight,
          future: _recentShows,
          itemBuilder: (context, item) => PosterItem(
              poster: item.poster,
              title: item.title,
              subtitle: item.subtitle,
              borderRadius: cardBorderRadius,
              infoSeparator: posterItemInfoSeparator,
              primaryTextStyle: primaryTextStyle,
              secondaryTextStyle: secondaryTextStyle,
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
  final TextStyle? titleStyle;
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
    required this.titleStyle,
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
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Padding(
          padding: widget.titlePadding,
          child: Text(widget.title, style: widget.titleStyle),
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
        const SizedBox(height: 16),
      ],
    );
  }
}

class ThumbnailItem extends StatelessWidget {
  final String thumbnail;
  final String title;
  final String subtitle;
  final double progress;
  final double borderRadius;
  final double padding;
  final TextStyle? primaryTextStyle;
  final TextStyle? secondaryTextStyle;
  final void Function() onTap;

  const ThumbnailItem({
    Key? key,
    required this.thumbnail,
    required this.title,
    required this.subtitle,
    required this.progress,
    required this.borderRadius,
    required this.padding,
    required this.primaryTextStyle,
    required this.secondaryTextStyle,
    required this.onTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return AspectRatio(
      aspectRatio: 16 / 9,
      child: Material(
        elevation: 2.0,
        type: MaterialType.card,
        clipBehavior: Clip.hardEdge,
        borderRadius: BorderRadius.all(Radius.circular(borderRadius)),
        child: Ink(
          decoration: BoxDecoration(
            image: DecorationImage(
              image: NetworkImage(thumbnail),
              fit: BoxFit.cover,
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
                  TextOneLine(title,
                      style: primaryTextStyle?.copyWith(color: Colors.white)),
                  TextOneLine(subtitle,
                      style: secondaryTextStyle?.copyWith(color: Colors.white)),
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

class PosterItem extends StatelessWidget {
  final String poster;
  final String title;
  final String subtitle;
  final double borderRadius;
  final double infoSeparator;
  final TextStyle? primaryTextStyle;
  final TextStyle? secondaryTextStyle;
  final void Function() onTap;

  const PosterItem({
    Key? key,
    required this.poster,
    required this.title,
    required this.subtitle,
    required this.infoSeparator,
    required this.borderRadius,
    required this.primaryTextStyle,
    required this.secondaryTextStyle,
    required this.onTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        AspectRatio(
          aspectRatio: 2 / 3,
          child: Material(
            elevation: 2.0,
            type: MaterialType.card,
            clipBehavior: Clip.hardEdge,
            borderRadius: BorderRadius.all(Radius.circular(borderRadius)),
            child: Ink.image(
              fit: BoxFit.cover,
              image: NetworkImage(poster),
              child: InkWell(onTap: onTap),
            ),
          ),
        ),
        SizedBox(height: infoSeparator),
        TextOneLine(title, style: primaryTextStyle),
        TextOneLine(subtitle, style: secondaryTextStyle),
      ],
    );
  }
}
