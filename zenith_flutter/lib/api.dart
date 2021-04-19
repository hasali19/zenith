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

  Movie({
    @required this.id,
    @required this.title,
    @required this.releaseDate,
    @required this.overview,
    @required this.poster,
    @required this.backdrop,
    @required this.duration,
  });

  factory Movie.fromJson(Map<String, dynamic> json) {
    return Movie(
      id: json['id'],
      title: json['title'],
      releaseDate: json['release_date'],
      overview: json['overview'],
      poster: json['poster'],
      backdrop: json['backdrop'],
      duration: json['duration'],
    );
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

  Show({
    @required this.id,
    @required this.name,
    @required this.startDate,
    @required this.endDate,
    @required this.overview,
    @required this.poster,
    @required this.backdrop,
    @required this.unwatchedEpisodes,
  });

  factory Show.fromJson(Map<String, dynamic> json) {
    return Show(
      id: json['id'],
      name: json['name'],
      startDate: json['start_date'],
      endDate: json['end_date'],
      overview: json['overview'],
      poster: json['poster'],
      backdrop: json['backdrop'],
      unwatchedEpisodes: json['unwatched_episodes'],
    );
  }

  int startYear() {
    return DateTime.fromMillisecondsSinceEpoch(this.startDate * 1000).year;
  }
}

class Season {
  final int id;
  final int showId;
  final int seasonNumber;
  final String name;
  final String overview;
  final String poster;
  final String backdrop;

  Season({
    @required this.id,
    @required this.showId,
    @required this.seasonNumber,
    @required this.name,
    @required this.overview,
    @required this.poster,
    @required this.backdrop,
  });

  factory Season.fromJson(Map<String, dynamic> json) {
    return Season(
      id: json['id'],
      showId: json['show_id'],
      seasonNumber: json['season_number'],
      name: json['name'],
      overview: json['overview'],
      poster: json['poster'],
      backdrop: json['backdrop'],
    );
  }
}

class Episode {
  final int id;
  final int showId;
  final int seasonId;
  final int episodeNumber;
  final String name;
  final int airDate;
  final String overview;
  final String thumbnail;
  final double duration;
  final bool isWatched;

  Episode({
    @required this.id,
    @required this.showId,
    @required this.seasonId,
    @required this.episodeNumber,
    @required this.name,
    @required this.airDate,
    @required this.overview,
    @required this.thumbnail,
    @required this.duration,
    @required this.isWatched,
  });

  factory Episode.fromJson(Map<String, dynamic> json) {
    return Episode(
      id: json['id'],
      showId: json['show_id'],
      seasonId: json['season_id'],
      episodeNumber: json['episode_number'],
      name: json['name'],
      airDate: json['air_date'],
      overview: json['overview'],
      thumbnail: json['thumbnail'],
      duration: json['duration'],
      isWatched: json['is_watched'],
    );
  }
}

class ApiClient {
  final String scheme;
  final String host;
  final int port;

  ApiClient(this.scheme, this.host, this.port);

  Future<List<Movie>> getMovies() async {
    final json = await _get('/api/movies');
    return List<Movie>.from(json.map((json) => Movie.fromJson(json)));
  }

  Future<List<Show>> getShows() async {
    final json = await _get('/api/tv/shows');
    return List<Show>.from(json.map((json) => Show.fromJson(json)));
  }

  Future<List<Season>> getSeasons(int showId) async {
    final json = await _get('/api/tv/shows/$showId/seasons');
    return List<Season>.from(json.map((json) => Season.fromJson(json)));
  }

  Future<List<Episode>> getEpisodes(int seasonId) async {
    final json = await _get('/api/tv/seasons/$seasonId/episodes');
    return List<Episode>.from(json.map((json) => Episode.fromJson(json)));
  }

  Future<dynamic> _get(String path) async {
    final uri = Uri(scheme: scheme, host: host, port: port, path: path);
    final res = await http.get(uri);

    if (res.statusCode != 200) {
      throw Exception("Failed to fetch $uri");
    }

    return jsonDecode(utf8.decode(res.bodyBytes));
  }
}
