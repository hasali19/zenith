import 'dart:convert';

import 'package:http/http.dart' as http;

abstract class MediaItem {
  int get id;
  String? get poster;
  String? get thumbnail;
  String get title;
  String get fullTitle;
  String get subtitle;
  int? get year;
}

class SubtitleTrack {
  final int id;
  final String? title;
  final String? language;

  const SubtitleTrack({
    required this.id,
    required this.title,
    required this.language,
  });

  factory SubtitleTrack.fromJson(Map<String, dynamic> json) => SubtitleTrack(
        id: json['id'],
        title: json['title'],
        language: json['language'],
      );
}

class VideoInfo {
  final double duration;
  final List<SubtitleTrack> subtitles;

  const VideoInfo({
    required this.duration,
    required this.subtitles,
  });

  factory VideoInfo.fromJson(Map<String, dynamic> json) {
    final List<dynamic>? subtitlesJson = json['subtitles'];
    final List<SubtitleTrack> subtitles;
    if (subtitlesJson != null) {
      subtitles =
          subtitlesJson.map((json) => SubtitleTrack.fromJson(json)).toList();
    } else {
      subtitles = [];
    }
    return VideoInfo(
      duration: json['duration'],
      subtitles: subtitles,
    );
  }
}

class VideoUserData {
  final double position;

  VideoUserData({required this.position});

  factory VideoUserData.fromJson(Map<String, dynamic> json) {
    return VideoUserData(position: json['position'] ?? 0);
  }
}

abstract class VideoItem extends MediaItem {
  VideoInfo? get videoInfo;
  VideoUserData? get userData;
}

class Movie extends VideoItem {
  @override
  final int id;
  @override
  final String title;
  @override
  final String poster;
  final String backdrop;
  final int? releaseYear;
  @override
  final VideoInfo? videoInfo;
  @override
  final VideoUserData? userData;

  Movie({
    required this.id,
    required this.title,
    required this.poster,
    required this.backdrop,
    required this.releaseYear,
    required this.videoInfo,
    required this.userData,
  });

  factory Movie.fromJson(Map<String, dynamic> json) {
    return Movie(
      id: json['id'],
      title: json['title'],
      poster: json['poster'],
      backdrop: json['backdrop'],
      releaseYear: json['release_date'] != null
          ? DateTime.fromMillisecondsSinceEpoch(json['release_date'] * 1000)
              .year
          : null,
      videoInfo: json['video_info'] != null
          ? VideoInfo.fromJson(json['video_info'])
          : null,
      userData: json['user_data'] != null
          ? VideoUserData.fromJson(json['user_data'])
          : null,
    );
  }

  @override
  String get fullTitle => title;

  @override
  String get subtitle => year.toString();

  @override
  int? get year {
    return releaseYear;
  }

  @override
  String? get thumbnail => backdrop;
}

class Show implements MediaItem {
  @override
  final int id;
  final String name;
  @override
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
  String get title {
    return name;
  }

  @override
  String get fullTitle => title;

  @override
  String get subtitle => year.toString();

  @override
  int? get year {
    return startYear;
  }

  @override
  String? get thumbnail => backdrop;
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

class Episode extends VideoItem {
  @override
  final int id;
  final String name;
  final String showName;
  @override
  final String poster;
  final String backdrop;
  @override
  final String? thumbnail;
  final String overview;
  final int seasonNumber;
  final int episodeNumber;
  @override
  final VideoInfo? videoInfo;
  @override
  final VideoUserData? userData;

  Episode({
    required this.id,
    required this.name,
    required this.showName,
    required this.poster,
    required this.backdrop,
    required this.thumbnail,
    required this.overview,
    required this.seasonNumber,
    required this.episodeNumber,
    required this.videoInfo,
    required this.userData,
  });

  factory Episode.fromJson(Map<String, dynamic> json) {
    return Episode(
      id: json['id'],
      name: json['name'],
      showName: json['show_name'],
      poster: json['poster'],
      backdrop: json['backdrop'],
      thumbnail: json['thumbnail'],
      overview: json['overview'],
      seasonNumber: json['season_number'],
      episodeNumber: json['episode_number'],
      videoInfo: json['video_info'] != null
          ? VideoInfo.fromJson(json['video_info'])
          : null,
      userData: json['user_data'] != null
          ? VideoUserData.fromJson(json['user_data'])
          : null,
    );
  }

  @override
  String get title => name;

  @override
  String get fullTitle => "$episodeNumber - $name";

  @override
  String get subtitle => showName;

  @override
  // TODO: implement year
  int? get year => throw UnimplementedError();

  String formatSeasonEpisode() {
    final season = this.seasonNumber.toString().padLeft(2, '0');
    final episode = this.episodeNumber.toString().padLeft(2, '0');
    return "S${season}E$episode";
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

Future<List<Movie>> fetchRecentMovies() async {
  final res =
      await http.get(Uri.parse('https://zenith.hasali.uk/api/movies/recent'));
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

Future<List<Show>> fetchRecentShows() async {
  final res =
      await http.get(Uri.parse('https://zenith.hasali.uk/api/tv/shows/recent'));
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

Future<MediaItem> fetchMediaItem(int id) async {
  final res =
      await http.get(Uri.parse('https://zenith.hasali.uk/api/items/$id'));
  if (res.statusCode == 200) {
    final dynamic json = jsonDecode(utf8.decode(res.bodyBytes));
    if (json["type"] == "movie") {
      return Movie.fromJson(json);
    } else if (json['type'] == "episode") {
      return Episode.fromJson(json);
    } else {
      throw Exception("Unsupported media item type");
    }
  } else {
    throw Exception('Failed to fetch shows');
  }
}

Future<List<VideoItem>> fetchContinueWatching() async {
  final res = await http
      .get(Uri.parse('https://zenith.hasali.uk/api/items/continue_watching'));
  if (res.statusCode == 200) {
    final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
    return json.map((json) {
      if (json["type"] == "movie") {
        return Movie.fromJson(json);
      } else if (json['type'] == "episode") {
        return Episode.fromJson(json);
      } else {
        throw Exception("Unsupported media item type");
      }
    }).toList();
  } else {
    throw Exception('Failed to fetch shows');
  }
}

String getVideoUrl(int id) {
  return "https://zenith.hasali.uk/api/videos/$id";
}

String getSubtitleUrl(int id) {
  return "https://zenith.hasali.uk/api/subtitles/$id";
}

Future updateProgress(int id, int position) async {
  await http.post(Uri.parse(
      "https://zenith.hasali.uk/api/progress/$id?position=$position"));
}
