import 'package:flutter/material.dart';

import '../../api.dart';
import '../../widgets.dart';
import '../movie_details.dart';

class MoviesScreen extends StatefulWidget {
  @override
  State<StatefulWidget> createState() {
    return MoviesScreenState();
  }
}

class MoviesScreenState extends State<MoviesScreen> {
  Future<List<Movie>> _movies;

  @override
  void initState() {
    super.initState();
    _movies = fetchMovies();
  }

  void _handleItemTap(Movie movie) {
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => MovieDetailsScreen(movie),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Center(
      child: FutureBuilder<List<Movie>>(
        future: _movies,
        builder: (context, snapshot) {
          if (snapshot.hasError) {
            return Text('${snapshot.error}');
          }

          if (!snapshot.hasData) {
            return CircularProgressIndicator();
          }

          return PosterGrid(
            count: snapshot.data.length,
            poster: (i) => snapshot.data[i].poster,
            primary: (i) => snapshot.data[i].title,
            secondary: (i) => snapshot.data[i].releaseYear().toString(),
            onItemTap: (i) => _handleItemTap(snapshot.data[i]),
          );
        },
      ),
    );
  }
}
