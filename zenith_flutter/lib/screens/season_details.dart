import 'package:assorted_layout_widgets/assorted_layout_widgets.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../api.dart';
import 'player.dart';

class SeasonDetailsScreen extends StatefulWidget {
  final Show show;
  final Season season;

  SeasonDetailsScreen(this.show, this.season);

  @override
  State<StatefulWidget> createState() {
    return SeasonDetailsScreenState();
  }
}

class SeasonDetailsScreenState extends State<SeasonDetailsScreen> {
  Future<List<Episode>>? _episodes;

  @override
  void initState() {
    super.initState();
    _episodes = context.read<ApiClient>().getEpisodes(widget.season.id);
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Scaffold(
      extendBodyBehindAppBar: true,
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      body: FutureBuilder<List<Episode>>(
        future: _episodes,
        builder: (context, snapshot) {
          if (snapshot.hasError) {
            return Center(child: Text("${snapshot.error}"));
          }

          if (snapshot.data != null) {
            return Center(child: CircularProgressIndicator());
          }

          return CustomScrollView(
            slivers: [
              SliverList(
                delegate: SliverChildListDelegate([
                  AspectRatio(
                    aspectRatio: 16 / 9,
                    child: widget.season.backdrop == null
                        ? Container()
                        : Image.network(widget.season.backdrop!),
                  ),
                  Container(
                    padding: EdgeInsets.all(16),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.stretch,
                      children: [
                        Text(
                          widget.season.name ??
                              "Season ${widget.season.seasonNumber}",
                          style: theme.textTheme.headline4,
                        ),
                        Text(
                          widget.show.name ?? "",
                          style: theme.textTheme.caption,
                        ),
                        if (widget.season.overview?.isNotEmpty ?? false)
                          Column(children: [
                            SizedBox(height: 16),
                            Text(
                              widget.season.overview ?? "",
                              style: theme.textTheme.bodyText2,
                            ),
                          ])
                      ],
                    ),
                  ),
                  Container(
                    margin: EdgeInsets.symmetric(horizontal: 16),
                    child: Text(
                      "Episodes",
                      style: theme.textTheme.headline5,
                    ),
                  ),
                  SizedBox(height: 16),
                ]),
              ),
              SliverPadding(
                padding: EdgeInsets.symmetric(horizontal: 16),
                sliver: EpisodeGrid(snapshot.data!),
              )
            ],
          );
        },
      ),
    );
  }
}

class EpisodeGrid extends StatelessWidget {
  final List<Episode> _episodes;

  EpisodeGrid(this._episodes);

  @override
  Widget build(BuildContext context) {
    final size = MediaQuery.of(context).size;

    final items = (size.width / 200).floor();
    final itemWidth = size.width / items;
    final itemHeight = (9 / 16) * itemWidth + 100;

    return SliverLayoutBuilder(builder: (context, constraints) {
      return SliverGrid(
        delegate: SliverChildBuilderDelegate(
          (context, i) {
            final episode = _episodes[i];
            return Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                Material(
                  elevation: 2.0,
                  type: MaterialType.card,
                  clipBehavior: Clip.hardEdge,
                  child: episode.thumbnail == null
                      ? Container()
                      : Ink.image(
                          fit: BoxFit.cover,
                          image: NetworkImage(episode.thumbnail!),
                          child: InkWell(
                            child: AspectRatio(
                              aspectRatio: 16 / 9,
                              child: episode.isWatched
                                  ? Container(
                                      color: Color.fromARGB(100, 0, 0, 0),
                                      child: Center(
                                        child: Icon(Icons.check),
                                      ),
                                    )
                                  : null,
                            ),
                            onTap: () {
                              Navigator.push(
                                context,
                                MaterialPageRoute(
                                  builder: (context) =>
                                      PlayerScreen(episode.id),
                                ),
                              );
                            },
                          ),
                        ),
                ),
                Container(
                  padding: EdgeInsets.symmetric(vertical: 8),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: [
                      TextOneLine(
                        "${episode.episodeNumber} - ${episode.name}",
                        style: Theme.of(context).textTheme.subtitle2,
                        overflow: TextOverflow.fade,
                      ),
                      SizedBox(height: 2),
                      Text(
                        episode.overview ?? "",
                        style: Theme.of(context).textTheme.caption,
                        maxLines: 3,
                        overflow: TextOverflow.ellipsis,
                      )
                    ],
                  ),
                )
              ],
            );
          },
          childCount: _episodes.length,
        ),
        gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
          crossAxisCount: items,
          crossAxisSpacing: 8,
          childAspectRatio: itemWidth / itemHeight,
        ),
      );
    });
  }
}
