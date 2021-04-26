import 'package:flutter/material.dart';

import '../api.dart';
import '../utils.dart';
import 'player.dart';

class MovieDetailsScreen extends StatelessWidget {
  final Movie movie;

  MovieDetailsScreen(this.movie);

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Scaffold(
      extendBodyBehindAppBar: true,
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        elevation: 0,
      ),
      body: ListView(
        padding: EdgeInsets.only(top: 0),
        children: [
          AspectRatio(
            aspectRatio: 16 / 9,
            child: movie.backdrop == null
                ? Container()
                : Image.network(movie.backdrop!),
          ),
          Container(
            padding: EdgeInsets.all(16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                Text(
                  movie.title,
                  style: theme.textTheme.headline4,
                ),
                Text(
                  '${movie.releaseYear()} • ${formatDuration(movie.duration)}',
                  style: theme.textTheme.caption,
                ),
                Container(
                  alignment: Alignment.centerLeft,
                  padding: EdgeInsets.symmetric(vertical: 4),
                  child: ElevatedButton.icon(
                    icon: Icon(Icons.play_arrow, size: 18),
                    label: Text("Play"),
                    onPressed: () {
                      Navigator.push(
                        context,
                        MaterialPageRoute(
                          builder: (context) => PlayerScreen(movie.id),
                        ),
                      );
                    },
                  ),
                ),
                Text(
                  movie.overview ?? "",
                  style: theme.textTheme.bodyText2,
                ),
              ],
            ),
          )
        ],
      ),
    );
  }
}
