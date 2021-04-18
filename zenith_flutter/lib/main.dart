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

class Show {
  final int id;
  final String name;
  final int startDate;
  final int endDate;
  final String overview;
  final String poster;
  final String backdrop;
  final int unwatchedEpisodes;

  Show(
      {@required this.id,
      @required this.name,
      @required this.startDate,
      @required this.endDate,
      @required this.overview,
      @required this.poster,
      @required this.backdrop,
      @required this.unwatchedEpisodes});

  factory Show.fromJson(Map<String, dynamic> json) {
    return Show(
        id: json['id'],
        name: json['name'],
        startDate: json['start_date'],
        endDate: json['end_date'],
        overview: json['overview'],
        poster: json['poster'],
        backdrop: json['backdrop'],
        unwatchedEpisodes: json['unwatched_episodes']);
  }

  int startYear() {
    return DateTime.fromMillisecondsSinceEpoch(this.startDate * 1000).year;
  }
}

Future<List<Movie>> _fetchMovies() async {
  // TODO: Remove hardcoded url
  final res = await http.get(Uri.https('zenith.hasali.uk', 'api/movies'));
  if (res.statusCode == 200) {
    return List<Movie>.from(jsonDecode(utf8.decode(res.bodyBytes))
        .map((json) => Movie.fromJson(json)));
  } else {
    throw Exception('Failed to load movies');
  }
}

Future<List<Show>> _fetchShows() async {
  // TODO: Remove hardcoded url
  final res = await http.get(Uri.https('zenith.hasali.uk', 'api/tv/shows'));
  if (res.statusCode == 200) {
    return List<Show>.from(jsonDecode(utf8.decode(res.bodyBytes))
        .map((json) => Show.fromJson(json)));
  } else {
    throw Exception('Failed to load movies');
  }
}

class AppState extends State<App> {
  int _current = 0;

  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    Widget screen;

    switch (_current) {
      case 0:
        screen = HomeScreen();
        break;

      case 1:
        screen = MovieListScreen();
        break;

      case 2:
        screen = ShowListScreen();
        break;
    }

    return MaterialApp(
      title: 'Zenith',
      themeMode: ThemeMode.system,
      theme: ThemeData.light(),
      darkTheme: ThemeData.dark(),
      home: Scaffold(
        appBar: AppBar(
          title: Text("Zenith"),
        ),
        body: AnimatedSwitcher(
          duration: const Duration(milliseconds: 200),
          child: screen,
        ),
        bottomNavigationBar: BottomNavigationBar(
          items: [
            BottomNavigationBarItem(icon: Icon(Icons.home), label: "Home"),
            BottomNavigationBarItem(icon: Icon(Icons.movie), label: "Movies"),
            BottomNavigationBarItem(icon: Icon(Icons.tv), label: "Shows"),
          ],
          currentIndex: _current,
          onTap: (item) {
            setState(() {
              _current = item;
            });
          },
        ),
      ),
    );
  }
}

class HomeScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Container();
  }
}

class MovieListScreen extends StatefulWidget {
  @override
  State<StatefulWidget> createState() {
    return MovieListScreenState();
  }
}

class MovieListScreenState extends State<MovieListScreen> {
  Future<List<Movie>> _movies;

  @override
  void initState() {
    super.initState();
    _movies = _fetchMovies();
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<List<Movie>>(
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
          childAspectRatio: 2 / 3.85,
          padding: EdgeInsets.all(8),
          crossAxisSpacing: 8,
          children: [
            for (final movie in snapshot.data)
              PosterCard(
                poster: movie.poster,
                primary: movie.title,
                secondary: movie.releaseYear().toString(),
                onTap: () {
                  Navigator.push(
                    context,
                    MaterialPageRoute(
                      builder: (context) => MovieDetailsScreen(movie),
                    ),
                  );
                },
              )
          ],
        );
      },
    );
  }
}

class ShowListScreen extends StatefulWidget {
  @override
  State<StatefulWidget> createState() {
    return ShowListScreenState();
  }
}

class ShowListScreenState extends State<ShowListScreen> {
  Future<List<Show>> _shows;

  @override
  void initState() {
    super.initState();
    _shows = _fetchShows();
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<List<Show>>(
      future: _shows,
      builder: (context, snapshot) {
        if (snapshot.hasError) {
          return Text('${snapshot.error}');
        }

        if (!snapshot.hasData) {
          return CircularProgressIndicator();
        }

        return GridView.extent(
          maxCrossAxisExtent: 150,
          childAspectRatio: 2 / 3.85,
          padding: EdgeInsets.all(8),
          crossAxisSpacing: 8,
          children: [
            for (final show in snapshot.data)
              PosterCard(
                poster: show.poster,
                primary: show.name,
                secondary: show.startYear().toString(),
                onTap: () {
                  Navigator.push(
                    context,
                    MaterialPageRoute(
                      builder: (context) => ShowDetailsScreen(show),
                    ),
                  );
                },
              )
          ],
        );
      },
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

class ShowDetailsScreen extends StatefulWidget {
  final Show show;

  ShowDetailsScreen(this.show);

  @override
  State<StatefulWidget> createState() {
    return ShowDetailsScreenState();
  }
}

Future<List> _fetchEpisodes(int id) async {
  // TODO: Remove hardcoded urls
  var res =
      await http.get(Uri.https('zenith.hasali.uk', 'api/tv/shows/$id/seasons'));

  if (res.statusCode != 200) {
    throw Exception("Failed to load seasons for show $id");
  }

  final seasons = jsonDecode(utf8.decode(res.bodyBytes));
  final episodes = [];

  for (final season in seasons) {
    final res = await http.get(Uri.https(
        'zenith.hasali.uk', 'api/tv/seasons/${season['id']}/episodes'));

    if (res.statusCode != 200) {
      throw Exception("Failed to load episodes for season ${season['id']}");
    }

    episodes.addAll(jsonDecode(utf8.decode(res.bodyBytes)));
  }

  return episodes;
}

class ShowDetailsScreenState extends State<ShowDetailsScreen> {
  Future<List> _episodes;

  @override
  void initState() {
    super.initState();
    _episodes = _fetchEpisodes(widget.show.id);
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Scaffold(
      // appBar: AppBar(),
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
              const SliverAppBar(
                pinned: true,
              ),
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
                      // TODO:
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

class PosterCard extends StatelessWidget {
  final String poster;
  final String primary;
  final String secondary;

  final void Function() onTap;

  PosterCard(
      {@required this.poster,
      @required this.primary,
      @required this.secondary,
      this.onTap});

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
            image: NetworkImage(poster),
            child: InkWell(
              child: AspectRatio(aspectRatio: 2 / 3),
              onTap: onTap,
            ),
          ),
        ),
        Container(
          padding: EdgeInsets.symmetric(vertical: 8),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              TextOneLine(
                primary,
                style: Theme.of(context).textTheme.subtitle2,
                overflow: TextOverflow.fade,
              ),
              SizedBox(height: 2),
              Text(
                secondary,
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
