import 'dart:convert';

import 'package:http/http.dart' as http;

abstract class MediaItem {
  String? getPoster();
  String getTitle();
  int? getYear();
}

class Movie implements MediaItem {
  final int id;
  final String title;
  final String poster;
  final int? releaseYear;

  const Movie({
    required this.id,
    required this.title,
    required this.poster,
    required this.releaseYear,
  });

  factory Movie.fromJson(Map<String, dynamic> json) {
    return Movie(
      id: json['id'],
      title: json['title'],
      poster: json['poster'],
      releaseYear: json['release_date'] != null
          ? DateTime.fromMillisecondsSinceEpoch(json['release_date'] * 1000)
              .year
          : null,
    );
  }

  @override
  String? getPoster() {
    return poster;
  }

  @override
  String getTitle() {
    return title;
  }

  @override
  int? getYear() {
    return releaseYear;
  }
}

class Show implements MediaItem {
  final int id;
  final String name;
  final String poster;
  final String backdrop;
  final String overview;
  final int? startYear;

  const Show({
    required this.id,
    required this.name,
    required this.poster,
    required this.backdrop,
    required this.overview,
    required this.startYear,
  });

  factory Show.fromJson(Map<String, dynamic> json) {
    return Show(
      id: json['id'],
      name: json['name'],
      poster: json['poster'],
      backdrop: json['backdrop'],
      overview: json['overview'],
      startYear: json['start_date'] != null
          ? DateTime.fromMillisecondsSinceEpoch(json['start_date'] * 1000).year
          : null,
    );
  }

  @override
  String? getPoster() {
    return poster;
  }

  @override
  String getTitle() {
    return name;
  }

  @override
  int? getYear() {
    return startYear;
  }
}

class Season {
  final int id;
  final String name;
  final String poster;
  final String backdrop;
  final String overview;
  final String showName;

  Season({
    required this.id,
    required this.name,
    required this.poster,
    required this.backdrop,
    required this.overview,
    required this.showName,
  });

  factory Season.fromJson(Map<String, dynamic> json) {
    return Season(
      id: json['id'],
      name: json['name'],
      poster: json['poster'],
      backdrop: json['backdrop'],
      overview: json['overview'],
      showName: json['show_name'],
    );
  }
}

class Episode {
  final int id;
  final String name;
  final String poster;
  final String backdrop;
  final String thumbnail;
  final String overview;
  final int episodeNumber;

  Episode({
    required this.id,
    required this.name,
    required this.poster,
    required this.backdrop,
    required this.thumbnail,
    required this.overview,
    required this.episodeNumber,
  });

  factory Episode.fromJson(Map<String, dynamic> json) {
    return Episode(
      id: json['id'],
      name: json['name'],
      poster: json['poster'],
      backdrop: json['backdrop'],
      thumbnail: json['thumbnail'],
      overview: json['overview'],
      episodeNumber: json['episode_number'],
    );
  }
}

Future<List<Movie>> fetchMovies() async {
  final res = await http.get(Uri.parse('https://zenith.hasali.uk/api/movies'));
  if (res.statusCode == 200) {
    final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
    return json.map((e) => Movie.fromJson(e)).toList();
  } else {
    throw Exception('Failed to fetch movies');
  }
}

Future<List<Show>> fetchShows() async {
  final res =
      await http.get(Uri.parse('https://zenith.hasali.uk/api/tv/shows'));
  if (res.statusCode == 200) {
    final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
    return json.map((e) => Show.fromJson(e)).toList();
  } else {
    throw Exception('Failed to fetch shows');
  }
}

Future<List<Season>> fetchSeasons(int showId) async {
  final res = await http
      .get(Uri.parse('https://zenith.hasali.uk/api/tv/shows/$showId/seasons'));
  if (res.statusCode == 200) {
    final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
    return json.map((e) => Season.fromJson(e)).toList();
  } else {
    throw Exception('Failed to fetch seasons');
  }
}

Future<List<Episode>> fetchEpisodes(int seasonId) async {
  final res = await http.get(
      Uri.parse('https://zenith.hasali.uk/api/tv/seasons/$seasonId/episodes'));
  if (res.statusCode == 200) {
    final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
    return json.map((e) => Episode.fromJson(e)).toList();
  } else {
    throw Exception('Failed to fetch episodes');
  }
}
