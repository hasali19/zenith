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
  Future _movies;

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
      home: Scaffold(
        appBar: AppBar(
          title: Text("Zenith"),
        ),
        body: Center(
          child: FutureBuilder<List<Movie>>(
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
                children: snapshot.data
                    .map(
                      (movie) => Column(
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
                                onTap: () {},
                                child: AspectRatio(aspectRatio: 2 / 3),
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
                      ),
                    )
                    .toList(),
              );
            },
          ),
        ),
      ),
    );
  }
}
