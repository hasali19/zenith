import 'package:flutter/material.dart';

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
  Future<List> _episodes;

  @override
  void initState() {
    super.initState();
    _episodes = fetchEpisodes(widget.show.id);
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
      body: FutureBuilder(
        future: _episodes,
        builder: (context, snapshot) {
          if (snapshot.hasError) {
            return Center(child: Text("${snapshot.error}"));
          }

          if (!snapshot.hasData) {
            return Center(child: CircularProgressIndicator());
          }

          return CustomScrollView(
            slivers: [
              SliverList(
                delegate: SliverChildListDelegate([
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
                ]),
              ),
              SliverPadding(
                padding: EdgeInsets.symmetric(horizontal: 16),
                sliver: EpisodeGrid(snapshot.data),
              )
            ],
          );
        },
      ),
    );
  }
}

class EpisodeGrid extends StatelessWidget {
  final _episodes;

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
                  image: NetworkImage(episode['thumbnail']),
                  child: InkWell(
                    child: AspectRatio(aspectRatio: 16 / 9),
                    onTap: () {
                      Navigator.push(
                        context,
                        MaterialPageRoute(
                          builder: (context) => PlayerScreen(episode['id']),
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
                      episode['name'],
                      style: Theme.of(context).textTheme.subtitle2,
                      // overflow: TextOverflow.fade,
                    ),
                    SizedBox(height: 2),
                    Text(
                      episode['overview'],
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
