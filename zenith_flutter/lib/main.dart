import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:assorted_layout_widgets/assorted_layout_widgets.dart';

void main() {
  runApp(App());
}

class App extends StatefulWidget {
  @override
  State<StatefulWidget> createState() {
    return AppState();
  }
}

class Movie {
  final int id;
  final String title;
  final int releaseDate;
  final String overview;
  final String poster;
  final String backdrop;
  final double duration;

  Movie(
      {@required this.id,
      @required this.title,
      @required this.releaseDate,
      @required this.overview,
      @required this.poster,
      @required this.backdrop,
      @required this.duration});

  factory Movie.fromJson(Map<String, dynamic> json) {
    return Movie(
        id: json['id'],
        title: json['title'],
        releaseDate: json['release_date'],
        overview: json['overview'],
        poster: json['poster'],
        backdrop: json['backdrop'],
        duration: json['duration']);
  }

  int releaseYear() {
    return DateTime.fromMillisecondsSinceEpoch(this.releaseDate * 1000).year;
  }
}

Future<List<Movie>> _fetchMovies() async {
  // TODO: Remove hardcoded url
  final res = await http.get(Uri.https('zenith.hasali.uk', 'api/movies'));
  if (res.statusCode == 200) {
    return List<Movie>.from(
        jsonDecode(res.body).map((json) => Movie.fromJson(json)));
  } else {
    throw Exception('Failed to load movies');
  }
}

class AppState extends State<App> {
  Future<List<Movie>> _movies;

  @override
  void initState() {
    super.initState();
    _movies = _fetchMovies();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Zenith',
      themeMode: ThemeMode.system,
      theme: ThemeData.light(),
      darkTheme: ThemeData.dark(),
      home: MovieListScreen(_movies),
    );
  }
}

class MovieListScreen extends StatelessWidget {
  final Future<List<Movie>> _movies;

  MovieListScreen(this._movies);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("Movies"),
      ),
      body: FutureBuilder<List<Movie>>(
        future: _movies,
        builder: (context, snapshot) {
          if (snapshot.hasError) {
            return Text('${snapshot.error}');
          }

          if (!snapshot.hasData) {
            return CircularProgressIndicator();
          }

          return GridView.extent(
            maxCrossAxisExtent: 150,
            childAspectRatio: 2 / 3.8,
            padding: EdgeInsets.all(8),
            crossAxisSpacing: 8,
            children: [for (final movie in snapshot.data) PosterCard(movie)],
          );
        },
      ),
    );
  }
}

String formatDuration(double duration) {
  if (duration <= 90 * 60) {
    return '${(duration / 60).floor()}m';
  } else {
    final hours = (duration / 3600).floor();
    final minutes = ((duration % 3600) / 60).floor();
    return '${hours}h ${minutes}m';
  }
}

class MovieDetailsScreen extends StatelessWidget {
  final Movie movie;

  MovieDetailsScreen(this.movie);

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Scaffold(
      appBar: AppBar(),
      body: ListView(
        // crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          AspectRatio(
            aspectRatio: 16 / 9,
            child: Image.network(movie.backdrop),
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
                    onPressed: () {},
                    icon: Icon(Icons.play_arrow, size: 18),
                    label: Text("Play"),
                  ),
                ),
                Text(
                  movie.overview,
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

class PosterCard extends StatelessWidget {
  final Movie movie;

  PosterCard(this.movie);

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Material(
          elevation: 2.0,
          type: MaterialType.card,
          clipBehavior: Clip.hardEdge,
          child: Ink.image(
            fit: BoxFit.cover,
            image: NetworkImage(movie.poster),
            child: InkWell(
              child: AspectRatio(aspectRatio: 2 / 3),
              onTap: () {
                Navigator.push(
                  context,
                  MaterialPageRoute(
                    builder: (context) => MovieDetailsScreen(movie),
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
                movie.title,
                style: Theme.of(context).textTheme.subtitle2,
                overflow: TextOverflow.fade,
              ),
              Text(
                movie.releaseYear().toString(),
                maxLines: 1,
                style: Theme.of(context).textTheme.caption,
              )
            ],
          ),
        )
      ],
    );
  }
}
