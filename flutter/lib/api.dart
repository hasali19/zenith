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
  final VideoFile? videoFile;
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
    required this.videoFile,
    required this.videoUserData,
  });

  factory MediaItem.fromJson(MediaType type, Map<String, dynamic> json) {
    return MediaItem(
      id: json['id'],
      type: type,
      name: json['name'],
      overview: json['overview'],
      startDate: json['start_date'] != null
          ? DateTime.fromMillisecondsSinceEpoch(json['start_date'] * 1000)
          : null,
      endDate: json['end_date'] != null
          ? DateTime.fromMillisecondsSinceEpoch(json['end_date'] * 1000)
          : null,
      parent: json['parent'] != null
          ? MediaItemParent.fromJson(json['parent'])
          : null,
      grandparent: json['grandparent'] != null
          ? MediaItemParent.fromJson(json['grandparent'])
          : null,
      videoFile: json['video_file'] != null
          ? VideoFile.fromJson(json['video_file'])
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
    final duration = videoFile!.duration;
    return position > 0.05 * duration && position < 0.9 * duration;
  }
}

class MediaItemParent {
  final int id;
  final int index;
  final String name;

  MediaItemParent(this.id, this.index, this.name);

  factory MediaItemParent.fromJson(Map<String, dynamic> json) =>
      MediaItemParent(json['id'], json['index'], json['name']);
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

abstract class StreamInfo {
  final int id;
  final int index;
  final String codec;

  const StreamInfo({
    required this.id,
    required this.index,
    required this.codec,
  });

  factory StreamInfo.fromJson(Map<String, dynamic> json) {
    switch (json['type']) {
      case "audio":
        return AudioStreamInfo.fromJson(json);
      case "video":
        return VideoStreamInfo.fromJson(json);
      default:
        throw Exception("Invalid stream type: ${json['type']}");
    }
  }
}

class VideoStreamInfo extends StreamInfo {
  final int width;
  final int height;

  const VideoStreamInfo({
    required int id,
    required int index,
    required String codec,
    required this.width,
    required this.height,
  }) : super(id: id, index: index, codec: codec);

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

class AudioStreamInfo extends StreamInfo {
  final String language;

  const AudioStreamInfo({
    required int id,
    required int index,
    required String codec,
    required this.language,
  }) : super(id: id, index: index, codec: codec);

  factory AudioStreamInfo.fromJson(Map<String, dynamic> json) {
    return AudioStreamInfo(
      id: json['id'],
      index: json['index'],
      codec: json['codec'],
      language: json['language'],
    );
  }
}

class VideoFile {
  final String path;
  final double duration;
  final String format;
  final List<StreamInfo> streams;
  final List<SubtitleTrack> subtitles;

  const VideoFile({
    required this.path,
    required this.duration,
    required this.format,
    required this.streams,
    required this.subtitles,
  });

  factory VideoFile.fromJson(Map<String, dynamic> json) {
    final List<dynamic> streams = json['streams'];
    final List<dynamic> subtitles = json['subtitles'];
    return VideoFile(
      path: json['path'],
      duration: json['duration'],
      format: json['format'],
      streams: streams.map((s) => StreamInfo.fromJson(s)).toList(),
      subtitles: subtitles.map((json) => SubtitleTrack.fromJson(json)).toList(),
    );
  }
}

class VideoUserData {
  final double position;
  final bool isWatched;
  final DateTime? lastWatchedAt;

  VideoUserData({
    required this.position,
    required this.isWatched,
    required this.lastWatchedAt,
  });

  factory VideoUserData.fromJson(Map<String, dynamic> json) {
    return VideoUserData(
      position: json['position'] ?? 0,
      isWatched: json['is_watched'],
      lastWatchedAt: json['last_watched_at'] != null
          ? DateTime.fromMillisecondsSinceEpoch(json['last_watched_at'] * 1000)
          : null,
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
    final res = await http.get(Uri.parse('$_baseUrl/api/shows'));
    if (res.statusCode == 200) {
      final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
      return json.map((e) => MediaItem.fromJson(MediaType.show, e)).toList();
    } else {
      throw Exception('Failed to fetch shows');
    }
  }

  Future<List<MediaItem>> fetchRecentShows() async {
    final res = await http.get(Uri.parse('$_baseUrl/api/shows/recent'));
    if (res.statusCode == 200) {
      final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
      return json.map((e) => MediaItem.fromJson(MediaType.show, e)).toList();
    } else {
      throw Exception('Failed to fetch shows');
    }
  }

  Future<List<MediaItem>> fetchSeasons(int showId) async {
    final res =
        await http.get(Uri.parse('$_baseUrl/api/shows/$showId/seasons'));
    if (res.statusCode == 200) {
      final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
      return json.map((e) => MediaItem.fromJson(MediaType.season, e)).toList();
    } else {
      throw Exception('Failed to fetch seasons');
    }
  }

  Future<List<MediaItem>> fetchEpisodes(int seasonId) async {
    final res =
        await http.get(Uri.parse('$_baseUrl/api/seasons/$seasonId/episodes'));
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
