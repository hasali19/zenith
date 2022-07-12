import 'dart:convert';
import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:zenith_flutter/screens/video_player.dart';
import 'package:zenith_flutter/text_one_line.dart';

import '../api.dart' as api;

final transparentImage = base64Decode(
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=");

class Season {
  final api.Season data;
  final List<api.Episode> episodes;

  Season({
    required this.data,
    required this.episodes,
  });
}

Future<List<Season>> fetchSeasons(int showId) async {
  final seasons = <Season>[];
  for (final api.Season season in await api.fetchSeasons(showId)) {
    final episodes = await api.fetchEpisodes(season.id);
    seasons.add(Season(data: season, episodes: episodes));
  }
  return seasons;
}

class ShowDetailsScreen extends StatefulWidget {
  final api.Show show;

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
    return Scaffold(
      extendBodyBehindAppBar: true,
      appBar: AppBar(
        // title: Text(widget.show.name),
        backgroundColor: Colors.transparent,
        elevation: 0,
        scrolledUnderElevation: 0,
      ),
      body: Stack(
        fit: StackFit.expand,
        children: [
          Backdrop(url: widget.show.backdrop),
          BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 10, sigmaY: 10),
            child: Container(
              decoration: BoxDecoration(color: Colors.black.withOpacity(0.5)),
              child: FutureBuilder<List<Season>>(
                future: _seasons,
                builder: ((context, snapshot) => ListView(
                      padding: const EdgeInsets.all(128),
                      children: <Widget>[
                        ItemDetails(show: widget.show),
                        if (snapshot.hasData)
                          for (final Season season in snapshot.data!)
                            EpisodeList(season: season),
                      ],
                    )),
              ),
            ),
          )
        ],
      ),
    );
  }
}

class ItemDetails extends StatelessWidget {
  const ItemDetails({
    Key? key,
    required this.show,
  }) : super(key: key);

  final api.Show show;

  @override
  Widget build(BuildContext context) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        SizedBox(width: 300, child: Poster(url: show.poster)),
        const SizedBox(width: 48),
        Flexible(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Title(text: show.name),
              if (show.startYear != null)
                Padding(
                  padding: const EdgeInsets.only(top: 16.0),
                  child: Subtitle(text: "${show.startYear}"),
                ),
              const SizedBox(height: 32),
              Container(
                constraints: const BoxConstraints(maxWidth: 600),
                child: Overview(text: show.overview),
              ),
            ],
          ),
        ),
      ],
    );
  }
}

class Backdrop extends StatelessWidget {
  final String url;

  const Backdrop({
    Key? key,
    required this.url,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return FadeInImage.memoryNetwork(
      placeholder: transparentImage,
      image: url,
      fit: BoxFit.cover,
    );
  }
}

class Poster extends StatelessWidget {
  final String url;

  const Poster({
    Key? key,
    required this.url,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: AspectRatio(
        aspectRatio: 2.0 / 3.0,
        child: Image.network(url),
      ),
    );
  }
}

class Title extends StatelessWidget {
  final String text;

  const Title({
    Key? key,
    required this.text,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final style = theme.textTheme.headline3!.copyWith(color: Colors.white);
    return Text(text, style: style);
  }
}

class Subtitle extends StatelessWidget {
  final String text;

  const Subtitle({
    Key? key,
    required this.text,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final style = theme.textTheme.headline5;
    return Text(text, style: style);
  }
}

class Overview extends StatelessWidget {
  final String text;

  const Overview({
    Key? key,
    required this.text,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final style = theme.textTheme.bodyLarge!.copyWith(fontSize: 16);
    return Text(text, style: style);
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

  api.Season get season => widget.season.data;
  List<api.Episode> get episodes => widget.season.episodes;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const SizedBox(height: 32),
        Text(season.name, style: theme.textTheme.headline4),
        const SizedBox(height: 16),
        SizedBox(
          height: 320,
          child: Scrollbar(
            controller: controller,
            child: ListView.separated(
              controller: controller,
              scrollDirection: Axis.horizontal,
              separatorBuilder: (context, index) => const SizedBox(width: 32),
              itemCount: episodes.length,
              itemBuilder: (context, index) =>
                  EpisodeListItem(episode: episodes[index]),
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
  }) : super(key: key);

  final api.Episode episode;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    const width = 352.0;
    const height = width * (9.0 / 16.0);
    return SizedBox(
      width: width,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(
            height: height,
            child: EpisodeThumbnail(
              url: episode.thumbnail,
              onTap: () {
                Navigator.push(
                  context,
                  MaterialPageRoute(
                    builder: (context) => VideoPlayerScreen(
                      id: episode.id,
                      startPosition: episode.userData?.position ?? 0,
                    ),
                  ),
                );
              },
            ),
          ),
          const SizedBox(height: 16),
          TextOneLine("${episode.episodeNumber} - ${episode.name}",
              style: theme.textTheme.titleLarge),
          // const SizedBox(height: 8),
          // TextOneLine("Episode ${episode.episodeNumber}",
          //     style: theme.textTheme.titleMedium),
          const SizedBox(height: 8),
          Flexible(
            child: Text(
              episode.overview,
              style: theme.textTheme.bodyLarge,
              maxLines: 3,
              overflow: TextOverflow.ellipsis,
            ),
          ),
        ],
      ),
    );
  }
}

class EpisodeThumbnail extends StatelessWidget {
  final String url;
  final void Function() onTap;

  const EpisodeThumbnail({Key? key, required this.url, required this.onTap})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Material(
      elevation: 2.0,
      type: MaterialType.card,
      clipBehavior: Clip.hardEdge,
      borderRadius: const BorderRadius.all(Radius.circular(16)),
      child: Ink.image(
        fit: BoxFit.cover,
        image: NetworkImage(url),
        child: InkWell(onTap: onTap),
      ),
    );
  }
}
