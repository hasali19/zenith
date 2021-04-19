import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:zenith/screens/season_details.dart';
import 'package:zenith/widgets.dart';

import '../api.dart';
import 'player.dart';

class ShowDetailsScreen extends StatefulWidget {
  final Show show;

  ShowDetailsScreen(this.show);

  @override
  State<StatefulWidget> createState() {
    return ShowDetailsScreenState();
  }
}

class ShowDetailsScreenState extends State<ShowDetailsScreen> {
  Future<List<Season>> _seasons;

  @override
  void initState() {
    super.initState();
    _seasons = context.read<ApiClient>().getSeasons(widget.show.id);
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
      body: FutureBuilder<List<Season>>(
        future: _seasons,
        builder: (context, snapshot) {
          if (snapshot.hasError) {
            return Center(child: Text("${snapshot.error}"));
          }

          if (!snapshot.hasData) {
            return Center(child: CircularProgressIndicator());
          }

          return SingleChildScrollView(
            padding: EdgeInsets.only(top: 0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                AspectRatio(
                  aspectRatio: 16 / 9,
                  child: Image.network(widget.show.backdrop),
                ),
                Container(
                  padding: EdgeInsets.all(16),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: [
                      Text(
                        widget.show.name,
                        style: theme.textTheme.headline4,
                      ),
                      SizedBox(height: 8),
                      Text(
                        widget.show.overview,
                        style: theme.textTheme.bodyText2,
                      ),
                    ],
                  ),
                ),
                Container(
                  margin: EdgeInsets.symmetric(horizontal: 16),
                  child: Text(
                    "Seasons",
                    style: theme.textTheme.headline5,
                  ),
                ),
                SizedBox(
                  height: 236,
                  child: ListView(
                    shrinkWrap: true,
                    scrollDirection: Axis.horizontal,
                    padding: EdgeInsets.symmetric(horizontal: 12, vertical: 8),
                    children: [
                      for (final season in snapshot.data)
                        Container(
                          width: 120,
                          margin: EdgeInsets.symmetric(horizontal: 4),
                          child: PosterItem(
                            poster: season.poster,
                            primary:
                                season.name ?? "Season ${season.seasonNumber}",
                            secondary: widget.show.name,
                            onTap: () {
                              Navigator.push(
                                  context,
                                  MaterialPageRoute(
                                      builder: (context) => SeasonDetailsScreen(
                                          widget.show, season)));
                            },
                          ),
                        )
                    ],
                  ),
                )
              ],
            ),
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

    final items = (size.width / 300).floor();
    final itemWidth = size.width / items;
    final itemHeight = (9 / 16) * itemWidth + 100;

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
                child: Ink.image(
                  fit: BoxFit.cover,
                  image: NetworkImage(episode.thumbnail),
                  child: InkWell(
                    child: AspectRatio(aspectRatio: 16 / 9),
                    onTap: () {
                      Navigator.push(
                        context,
                        MaterialPageRoute(
                          builder: (context) => PlayerScreen(episode.id),
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
                    Text(
                      episode.name,
                      style: Theme.of(context).textTheme.subtitle2,
                      // overflow: TextOverflow.fade,
                    ),
                    SizedBox(height: 2),
                    Text(
                      episode.overview,
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
  }
}
