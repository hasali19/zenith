import 'dart:convert';

import 'package:meta/meta.dart';
import 'package:http/http.dart' as http;

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

Future<List<Movie>> fetchMovies() async {
  // TODO: Remove hardcoded url
  final res = await http.get(Uri.https('zenith.hasali.uk', 'api/movies'));
  if (res.statusCode == 200) {
    return List<Movie>.from(jsonDecode(utf8.decode(res.bodyBytes))
        .map((json) => Movie.fromJson(json)));
  } else {
    throw Exception('Failed to load movies');
  }
}

Future<List<Show>> fetchShows() async {
  // TODO: Remove hardcoded url
  final res = await http.get(Uri.https('zenith.hasali.uk', 'api/tv/shows'));
  if (res.statusCode == 200) {
    return List<Show>.from(jsonDecode(utf8.decode(res.bodyBytes))
        .map((json) => Show.fromJson(json)));
  } else {
    throw Exception('Failed to load movies');
  }
}

Future<List> fetchEpisodes(int id) async {
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
