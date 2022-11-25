import 'dart:convert';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:http/http.dart' as http;

enum MediaType {
  movie,
  show,
  season,
  episode,
}

class MediaItem {
  final int id;
  final MediaType type;
  final String name;
  final String? overview;
  final DateTime? startDate;
  final DateTime? endDate;
  final MediaItemParent? parent;
  final MediaItemParent? grandparent;
  final VideoInfo? videoInfo;
  final VideoUserData? videoUserData;

  MediaItem({
    required this.id,
    required this.type,
    required this.name,
    required this.overview,
    required this.startDate,
    required this.endDate,
    required this.parent,
    required this.grandparent,
    required this.videoInfo,
    required this.videoUserData,
  });

  factory MediaItem.fromJson(MediaType type, Map<String, dynamic> json) {
    final String name;
    final int? startDate;
    MediaItemParent? parent;
    MediaItemParent? grandparent;
    switch (type) {
      case MediaType.movie:
        name = json['title'];
        startDate = json['release_date'];
        break;
      case MediaType.show:
        name = json['name'];
        startDate = json['start_date'];
        break;
      case MediaType.season:
        name = json['name'];
        startDate = json['start_date'];
        parent = MediaItemParent(
            json['show_id'], json['season_number'], json['show_name']);
        break;
      case MediaType.episode:
        name = json['name'];
        startDate = json['air_date'];
        parent = MediaItemParent(json['season_id'], json['episode_number'], '');
        grandparent = MediaItemParent(
            json['show_id'], json['season_number'], json['show_name']);
        break;
    }
    return MediaItem(
      id: json['id'],
      type: type,
      name: name,
      overview: json['overview'],
      startDate: startDate != null
          ? DateTime.fromMillisecondsSinceEpoch(startDate * 1000)
          : null,
      endDate: json['end_date'] != null
          ? DateTime.fromMillisecondsSinceEpoch(json['end_date'] * 1000)
          : null,
      parent: parent,
      grandparent: grandparent,
      videoInfo: json['video_info'] != null
          ? VideoInfo.fromJson(json['video_info'])
          : null,
      videoUserData: (type == MediaType.movie || type == MediaType.episode) &&
              json['user_data'] != null
          ? VideoUserData.fromJson(json['user_data'])
          : null,
    );
  }

  String? getSeasonEpisode() {
    String? seasonEpisode;
    if (parent != null) {
      final parent = this.parent!.index.toString().padLeft(2, '0');
      if (grandparent != null) {
        final grandparent = this.grandparent!.index.toString().padLeft(2, '0');
        seasonEpisode = "S${grandparent}E$parent";
      } else {
        seasonEpisode = "S$parent";
      }
    }
    return seasonEpisode;
  }

  bool get shouldResume {
    final position = videoUserData?.position ?? 0;
    final duration = videoInfo!.duration;
    return position > 0.05 * duration && position < 0.9 * duration;
  }
}

class MediaItemParent {
  final int id;
  final int index;
  final String name;

  MediaItemParent(this.id, this.index, this.name);
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

class VideoStreamInfo {
  final int id;
  final int index;
  final String codec;
  final int width;
  final int height;

  const VideoStreamInfo({
    required this.id,
    required this.index,
    required this.codec,
    required this.width,
    required this.height,
  });

  factory VideoStreamInfo.fromJson(Map<String, dynamic> json) {
    return VideoStreamInfo(
      id: json['id'],
      index: json['index'],
      codec: json['codec'],
      width: json['width'],
      height: json['height'],
    );
  }
}

class AudioStreamInfo {
  final int id;
  final int index;
  final String codec;
  final String language;

  const AudioStreamInfo({
    required this.id,
    required this.index,
    required this.codec,
    required this.language,
  });

  factory AudioStreamInfo.fromJson(Map<String, dynamic> json) {
    return AudioStreamInfo(
      id: json['id'],
      index: json['index'],
      codec: json['codec'],
      language: json['language'],
    );
  }
}

class VideoInfo {
  final String path;
  final double duration;
  final String format;
  final VideoStreamInfo? video;
  final List<AudioStreamInfo>? audio;
  final List<SubtitleTrack> subtitles;

  const VideoInfo({
    required this.path,
    required this.duration,
    required this.format,
    required this.video,
    required this.audio,
    required this.subtitles,
  });

  factory VideoInfo.fromJson(Map<String, dynamic> json) {
    final List<dynamic>? audio = json['audio'];
    final List<dynamic>? subtitlesJson = json['subtitles'];
    final List<SubtitleTrack> subtitles;
    if (subtitlesJson != null) {
      subtitles =
          subtitlesJson.map((json) => SubtitleTrack.fromJson(json)).toList();
    } else {
      subtitles = [];
    }
    return VideoInfo(
      path: json['path'],
      duration: json['duration'],
      format: json['format'],
      video: json['video'] != null
          ? VideoStreamInfo.fromJson(json['video'])
          : null,
      audio: audio?.map((e) => AudioStreamInfo.fromJson(e)).toList(),
      subtitles: subtitles,
    );
  }
}

class VideoUserData {
  final double position;
  final bool isWatched;

  VideoUserData({
    required this.position,
    required this.isWatched,
  });

  factory VideoUserData.fromJson(Map<String, dynamic> json) {
    return VideoUserData(
      position: json['position'] ?? 0,
      isWatched: json['is_watched'],
    );
  }
}

enum ImageType { poster, backdrop, thumbnail }

class VideoUserDataPatch {
  bool? isWatched;
  double? position;
  VideoUserDataPatch({this.isWatched, this.position});
}

class ZenithApiClient {
  final String _baseUrl;

  ZenithApiClient(this._baseUrl);

  Future<List<MediaItem>> fetchMovies() async {
    final res = await http.get(Uri.parse('$_baseUrl/api/movies'));
    if (res.statusCode == 200) {
      final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
      return json.map((e) => MediaItem.fromJson(MediaType.movie, e)).toList();
    } else {
      throw Exception('Failed to fetch movies');
    }
  }

  Future<List<MediaItem>> fetchRecentMovies() async {
    final res = await http.get(Uri.parse('$_baseUrl/api/movies/recent'));
    if (res.statusCode == 200) {
      final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
      return json.map((e) => MediaItem.fromJson(MediaType.movie, e)).toList();
    } else {
      throw Exception('Failed to fetch movies');
    }
  }

  Future<List<MediaItem>> fetchShows() async {
    final res = await http.get(Uri.parse('$_baseUrl/api/tv/shows'));
    if (res.statusCode == 200) {
      final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
      return json.map((e) => MediaItem.fromJson(MediaType.show, e)).toList();
    } else {
      throw Exception('Failed to fetch shows');
    }
  }

  Future<List<MediaItem>> fetchRecentShows() async {
    final res = await http.get(Uri.parse('$_baseUrl/api/tv/shows/recent'));
    if (res.statusCode == 200) {
      final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
      return json.map((e) => MediaItem.fromJson(MediaType.show, e)).toList();
    } else {
      throw Exception('Failed to fetch shows');
    }
  }

  Future<List<MediaItem>> fetchSeasons(int showId) async {
    final res =
        await http.get(Uri.parse('$_baseUrl/api/tv/shows/$showId/seasons'));
    if (res.statusCode == 200) {
      final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
      return json.map((e) => MediaItem.fromJson(MediaType.season, e)).toList();
    } else {
      throw Exception('Failed to fetch seasons');
    }
  }

  Future<List<MediaItem>> fetchEpisodes(int seasonId) async {
    final res = await http
        .get(Uri.parse('$_baseUrl/api/tv/seasons/$seasonId/episodes'));
    if (res.statusCode == 200) {
      final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
      return json.map((e) => MediaItem.fromJson(MediaType.episode, e)).toList();
    } else {
      throw Exception('Failed to fetch episodes');
    }
  }

  Future<MediaItem> fetchMediaItem(int id) async {
    final res = await http.get(Uri.parse('$_baseUrl/api/items/$id'));
    if (res.statusCode == 200) {
      final dynamic json = jsonDecode(utf8.decode(res.bodyBytes));
      final type =
          MediaType.values.firstWhere((type) => type.name == json['type']);
      return MediaItem.fromJson(type, json);
    } else {
      throw Exception('Failed to fetch shows');
    }
  }

  Future<List<MediaItem>> fetchContinueWatching() async {
    final res =
        await http.get(Uri.parse('$_baseUrl/api/items/continue_watching'));
    if (res.statusCode == 200) {
      final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
      return json.map((json) {
        if (json["type"] == "movie") {
          return MediaItem.fromJson(MediaType.movie, json);
        } else if (json['type'] == "episode") {
          return MediaItem.fromJson(MediaType.episode, json);
        } else {
          throw Exception("Unsupported media item type");
        }
      }).toList();
    } else {
      throw Exception('Failed to fetch shows');
    }
  }

  String getVideoUrl(int id, {bool attachment = false}) {
    var url = "$_baseUrl/api/videos/$id";
    if (attachment) {
      url += "?attachment=true";
    }
    return url;
  }

  String getSubtitleUrl(int id) {
    return "$_baseUrl/api/subtitles/$id";
  }

  String getMediaImageUrl(int id, ImageType type, {int? width}) {
    return Uri.parse(_baseUrl).replace(
      path: "api/items/$id/images/${type.name}",
      queryParameters: {if (width != null) 'width': width.toString()},
    ).toString();
  }

  Future updateProgress(int id, int position) async {
    await http.post(Uri.parse("$_baseUrl/api/progress/$id?position=$position"));
  }

  Future updateUserData(int id, VideoUserDataPatch data) async {
    await http.patch(
      Uri.parse("$_baseUrl/api/items/$id/user_data"),
      headers: {
        'Content-Type': 'application/json',
      },
      body: jsonEncode({
        'is_watched': data.isWatched,
        'position': data.position,
      }),
    );
  }
}

final apiProvider =
    Provider<ZenithApiClient>((ref) => throw UnimplementedError());
