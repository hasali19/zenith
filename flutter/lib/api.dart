import 'dart:convert';
import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:http/http.dart';
import 'package:zenith/http_client/http_client.dart';

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
  final CollectionUserData? collectionUserData;
  final List<String> genres;
  final String? ageRating;
  final String? trailer;
  final String? director;
  final List<CastMember> cast;

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
    required this.collectionUserData,
    required this.genres,
    required this.ageRating,
    required this.trailer,
    required this.director,
    required this.cast,
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
      collectionUserData:
          (type == MediaType.show || type == MediaType.season) &&
                  json['user_data'] != null
              ? CollectionUserData.fromJson(json['user_data'])
              : null,
      genres: List<String>.from(json['genres']),
      ageRating: json['age_rating'],
      trailer: json['trailer'],
      director: json['director'],
      cast: List<dynamic>.from(json['cast'])
          .map((json) => CastMember.fromJson(json))
          .toList(),
    );
  }

  String? getSeasonEpisode() {
    String? seasonEpisode;
    if (parent != null) {
      final parent = this.parent!.index.toString().padLeft(2, '0');
      if (grandparent != null) {
        final grandparent = this.grandparent!.index.toString().padLeft(2, '0');
        seasonEpisode = 'S${grandparent}E$parent';
      } else {
        seasonEpisode = 'S$parent';
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

class CastMember {
  final String name;
  final String? character;
  final String? profile;

  CastMember({
    required this.name,
    required this.character,
    required this.profile,
  });

  factory CastMember.fromJson(Map<String, dynamic> json) => CastMember(
      name: json['name'],
      character: json['character'],
      profile: json['profile']);
}

class SubtitleTrack {
  final int id;
  final int? streamIndex;
  final String? format;
  final String? title;
  final String? language;

  const SubtitleTrack({
    required this.id,
    required this.streamIndex,
    required this.format,
    required this.title,
    required this.language,
  });

  factory SubtitleTrack.fromJson(Map<String, dynamic> json) => SubtitleTrack(
        id: json['id'],
        streamIndex: json['stream_index'],
        format: json['format'],
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
    return switch (json['type']) {
      'audio' => AudioStreamInfo.fromJson(json),
      'video' => VideoStreamInfo.fromJson(json),
      _ => throw Exception("Invalid stream type: ${json['type']}"),
    };
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
  final String? language;

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
  final int id;
  final String path;
  final double duration;
  final String format;
  final List<StreamInfo> streams;
  final List<SubtitleTrack> subtitles;

  const VideoFile({
    required this.id,
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
      id: json['id'],
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

class CollectionUserData {
  final int unwatched;

  CollectionUserData({
    required this.unwatched,
  });

  factory CollectionUserData.fromJson(Map<String, dynamic> json) =>
      CollectionUserData(unwatched: json['unwatched']);
}

enum ImageType { poster, backdrop, thumbnail }

class VideoUserDataPatch {
  bool? isWatched;
  double? position;
  VideoUserDataPatch({this.isWatched, this.position});
}

class Collection {
  final int id;
  final String name;
  final String? overview;
  final String? poster;

  Collection({
    required this.id,
    required this.name,
    required this.overview,
    required this.poster,
  });

  factory Collection.fromJson(Map<String, dynamic> json) => Collection(
        id: json['id'],
        name: json['name'],
        overview: json['overview'],
        poster: json['poster'],
      );
}

class FixMetadataMatch {
  final int? tmdbId;
  final int? season;
  final int? episode;

  FixMetadataMatch({
    this.tmdbId,
    this.season,
    this.episode,
  });
}

class User {
  final int id;
  final String username;

  User({
    required this.id,
    required this.username,
  });

  factory User.fromJson(Map<String, dynamic> json) => User(
        id: json['id'],
        username: json['username'],
      );
}

const _store = FlutterSecureStorage(
  aOptions: AndroidOptions(encryptedSharedPreferences: true),
);

class ZenithApiClient {
  final Client _client = createHttpClient();
  final String _baseUrl;

  bool? _isLoggedIn;
  String? _authToken;

  ZenithApiClient(this._baseUrl);

  Future<bool> isLoggedIn() async {
    if (_isLoggedIn != null) return _isLoggedIn!;
    if (!kIsWeb) {
      _authToken = await _store.read(key: 'auth_token');
    }
    try {
      final res = await _get(Uri.parse('$_baseUrl/api/users/me'));
      return _isLoggedIn = res.statusCode == 200;
    } catch (e) {
      return false;
    }
  }

  Future<bool> login(String username, String password) async {
    final res = await _post(
      Uri.parse('$_baseUrl/api/auth/login'),
      {'username': username, 'password': password},
    );

    if (!kIsWeb) {
      final setCookie = res.headers['set-cookie'];
      if (setCookie == null) return false;
      final cookie = Cookie.fromSetCookieValue(setCookie);
      if (cookie.name == 'auth') {
        _authToken = cookie.value;
        await _store.write(key: 'auth_token', value: _authToken);
      }
    }

    return _isLoggedIn = res.statusCode == 200;
  }

  Future<void> logout() async {
    await _post(Uri.parse('$_baseUrl/api/auth/logout'));
    _isLoggedIn = false;
    if (!kIsWeb) {
      await _store.delete(key: 'auth_token');
      _authToken = null;
    }
  }

  Future<List<User>> fetchUsers() async {
    final res = await _get(Uri.parse('$_baseUrl/api/users'));
    if (res.statusCode == 200) {
      final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
      return json.map((e) => User.fromJson(e)).toList();
    } else {
      throw Exception('Failed to fetch users');
    }
  }

  Future<void> createUser(String username, String password,
      [String? code]) async {
    final res = await _post(
      Uri.parse('$_baseUrl/api/users'),
      {'username': username, 'password': password, 'registration_code': code},
    );

    if (res.statusCode != 200) {
      throw Exception('Failed to create user');
    }
  }

  Future<List<MediaItem>> fetchMovies() async {
    final res = await _get(Uri.parse('$_baseUrl/api/movies'));
    if (res.statusCode == 200) {
      final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
      return json.map((e) => MediaItem.fromJson(MediaType.movie, e)).toList();
    } else {
      throw Exception('Failed to fetch movies');
    }
  }

  Future<List<MediaItem>> fetchRecentMovies() async {
    final res = await _get(Uri.parse('$_baseUrl/api/movies/recent'));
    if (res.statusCode == 200) {
      final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
      return json.map((e) => MediaItem.fromJson(MediaType.movie, e)).toList();
    } else {
      throw Exception('Failed to fetch movies');
    }
  }

  Future<List<MediaItem>> fetchShows() async {
    final res = await _get(Uri.parse('$_baseUrl/api/shows'));
    if (res.statusCode == 200) {
      final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
      return json.map((e) => MediaItem.fromJson(MediaType.show, e)).toList();
    } else {
      throw Exception('Failed to fetch shows');
    }
  }

  Future<List<MediaItem>> fetchRecentShows() async {
    final res = await _get(Uri.parse('$_baseUrl/api/shows/recent'));
    if (res.statusCode == 200) {
      final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
      return json.map((e) => MediaItem.fromJson(MediaType.show, e)).toList();
    } else {
      throw Exception('Failed to fetch shows');
    }
  }

  Future<List<MediaItem>> fetchSeasons(int showId) async {
    final res = await _get(Uri.parse('$_baseUrl/api/shows/$showId/seasons'));
    if (res.statusCode == 200) {
      final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
      return json.map((e) => MediaItem.fromJson(MediaType.season, e)).toList();
    } else {
      throw Exception('Failed to fetch seasons');
    }
  }

  Future<List<MediaItem>> fetchShowEpisodes(int showId) async {
    final res = await _get(Uri.parse('$_baseUrl/api/shows/$showId/episodes'));
    if (res.statusCode == 200) {
      final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
      return json.map((e) => MediaItem.fromJson(MediaType.episode, e)).toList();
    } else {
      throw Exception('Failed to fetch episodes');
    }
  }

  Future<List<MediaItem>> fetchEpisodes(int seasonId) async {
    final res =
        await _get(Uri.parse('$_baseUrl/api/seasons/$seasonId/episodes'));
    if (res.statusCode == 200) {
      final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
      return json.map((e) => MediaItem.fromJson(MediaType.episode, e)).toList();
    } else {
      throw Exception('Failed to fetch episodes');
    }
  }

  Future<MediaItem> fetchMediaItem(int id) async {
    final res = await _get(Uri.parse('$_baseUrl/api/items/$id'));
    if (res.statusCode == 200) {
      final dynamic json = jsonDecode(utf8.decode(res.bodyBytes));
      final type =
          MediaType.values.firstWhere((type) => type.name == json['type']);
      return MediaItem.fromJson(type, json);
    } else {
      throw Exception('Failed to fetch shows');
    }
  }

  Future<void> deleteMediaItem(int id, {bool removeFiles = false}) async {
    final uri = Uri.parse('$_baseUrl/api/items/$id');
    if (removeFiles) {
      uri.queryParameters['remove_files'] = 'true';
    }
    await _delete(uri);
  }

  Future<List<MediaItem>> fetchContinueWatching() async {
    final res = await _get(Uri.parse('$_baseUrl/api/items/continue_watching'));
    if (res.statusCode == 200) {
      final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
      return json.map((json) {
        if (json['type'] == 'movie') {
          return MediaItem.fromJson(MediaType.movie, json);
        } else if (json['type'] == 'episode') {
          return MediaItem.fromJson(MediaType.episode, json);
        } else {
          throw Exception('Unsupported media item type');
        }
      }).toList();
    } else {
      throw Exception('Failed to fetch items');
    }
  }

  Future<void> findMetadataMatch(int id) async {
    await _post(Uri.parse('$_baseUrl/api/metadata/$id/find_match'));
  }

  Future<void> fixMetadataMatch(int id, FixMetadataMatch data) async {
    await _post(
      Uri.parse('$_baseUrl/api/metadata/$id/set_match'),
      {
        'tmdb_id': data.tmdbId,
        'season_number': data.season,
        'episode_number': data.episode,
      },
    );
  }

  Future<void> refreshMetadata(int id) async {
    await _post(Uri.parse('$_baseUrl/api/metadata/$id/refresh'));
  }

  Future<Collection> createCollection(String name) async {
    final res = await _post(
      Uri.parse('$_baseUrl/api/collections'),
      {'name': name},
    );
    if (res.statusCode == 200) {
      final dynamic json = jsonDecode(utf8.decode(res.bodyBytes));
      return Collection.fromJson(json);
    } else {
      throw Exception('Failed to fetch collection');
    }
  }

  Future<List<Collection>> fetchCollections() async {
    final res = await _get(Uri.parse('$_baseUrl/api/collections'));
    if (res.statusCode == 200) {
      final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
      return json.map((e) => Collection.fromJson(e)).toList();
    } else {
      throw Exception('Failed to fetch collections');
    }
  }

  Future<List<MediaItem>> fetchCollectionItems(int id) async {
    final res = await _get(Uri.parse(
        '$_baseUrl/api/items?collection_id=$id&sort_by[]=collection_index'));
    if (res.statusCode == 200) {
      final List<dynamic> json = jsonDecode(utf8.decode(res.bodyBytes));
      return json.map((e) => MediaItem.fromJson(MediaType.episode, e)).toList();
    } else {
      throw Exception('Failed to fetch items');
    }
  }

  Future<void> updateCollection(int id, List<int> itemIds) async {
    await _put(
      Uri.parse('$_baseUrl/api/collections/$id'),
      {'items': itemIds},
    );
  }

  Future<void> deleteCollection(int id) async {
    await _delete(Uri.parse('$_baseUrl/api/collections/$id'));
  }

  String getVideoUrl(int id, {bool attachment = false}) {
    var url = '$_baseUrl/api/videos/$id';
    if (attachment) {
      url += '?attachment=true';
    }
    return url;
  }

  String getSubtitleUrl(int id) {
    return '$_baseUrl/api/subtitles/$id';
  }

  String getMediaImageUrl(int id, ImageType type, {int? width}) {
    return Uri.parse(_baseUrl).replace(
      path: 'api/items/$id/images/${type.name}',
      queryParameters: {if (width != null) 'width': width.toString()},
    ).toString();
  }

  Future updateProgress(int id, int position) async {
    await _post(Uri.parse('$_baseUrl/api/progress/$id?position=$position'));
  }

  Future updateUserData(int id, VideoUserDataPatch data) async {
    await _patch(
      Uri.parse('$_baseUrl/api/items/$id/user_data'),
      {
        'is_watched': data.isWatched,
        'position': data.position,
      },
    );
  }

  Future<Response> _get(Uri uri) {
    return _send(uri, 'GET', null);
  }

  Future<Response> _post(Uri uri, [Object? body]) {
    return _send(uri, 'POST', body);
  }

  Future<Response> _put(Uri uri, [Object? body]) {
    return _send(uri, 'PUT', body);
  }

  Future<Response> _patch(Uri uri, [Object? body]) {
    return _send(uri, 'PATCH', body);
  }

  Future<Response> _delete(Uri uri) {
    return _send(uri, 'DELETE', null);
  }

  Future<Response> _send(Uri uri, String method, Object? body) async {
    final req = Request(method, uri);
    if (_authToken != null) {
      req.headers['cookie'] = 'auth=$_authToken';
    }
    if (body != null) {
      req.headers['content-type'] = 'application/json';
      req.bodyBytes = utf8.encode(jsonEncode(body));
    }
    final res = await Response.fromStream(await _client.send(req));
    if (res.statusCode == 401) {
      _isLoggedIn = false;
      _authToken = null;
    }
    return res;
  }
}

final apiProvider =
    Provider<ZenithApiClient>((ref) => throw UnimplementedError());
