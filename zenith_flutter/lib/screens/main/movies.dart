import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import '../../api.dart';
import '../../widgets.dart';
import '../movie_details.dart';

class MoviesScreen extends StatefulWidget {
  @override
  State<StatefulWidget> createState() => MoviesScreenState();
}

class MoviesScreenState extends State<MoviesScreen> {
  Future<List<Movie>>? _movies;

  @override
  void initState() {
    super.initState();
    _movies = context.read<ApiClient>().getMovies();
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

          final movies = snapshot.data;

          if (movies == null) {
            return CircularProgressIndicator();
          }

          return PosterGrid(
            count: movies.length,
            poster: (i) => movies[i].poster,
            primary: (i) => movies[i].title,
            secondary: (i) => movies[i].releaseYear()?.toString() ?? "",
            onItemTap: (i) => _handleItemTap(movies[i]),
          );
        },
      ),
    );
  }
}
