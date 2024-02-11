import 'package:dio/dio.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:zenith/format_utils.dart';

part 'api.freezed.dart';
part 'api.g.dart';

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
    if (parent != null) {
      if (grandparent != null) {
        return formatSeasonEpisode(grandparent!.index, parent!.index);
      } else {
        return formatSeason(parent!.index);
      }
    }
    return null;
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

enum AccessTokenOwner {
  system,
  user,
}

@freezed
class AccessToken with _$AccessToken {
  factory AccessToken({
    required AccessTokenOwner owner,
    required String name,
    required String token,
  }) = _AccessToken;

  factory AccessToken.fromJson(Map<String, Object?> json) =>
      _$AccessTokenFromJson(json);
}

enum SubtitleFormat { webvtt }

class ZenithApiClient {
  final Dio _client;

  bool? _isLoggedIn;

  ZenithApiClient(this._client);

  String get baseUrl => _client.options.baseUrl;

  Future<bool> isLoggedIn() async {
    if (_isLoggedIn != null) return _isLoggedIn!;
    final res = await _client.get('/api/users/me');
    return _isLoggedIn = res.statusCode == 200;
  }

  Future<bool> login(String username, String password) async {
    final res = await _client.post(
      '/api/auth/login',
      data: {'username': username, 'password': password},
    );

    return _isLoggedIn = res.statusCode == 200;
  }

  Future<void> logout() async {
    await _client.post('/api/auth/logout');
    _isLoggedIn = false;
  }

  Future<List<User>> fetchUsers() async {
    final res = await _client.get('/api/users');
    if (res.statusCode == 200) {
      final List<dynamic> json = res.data;
      return json.map((e) => User.fromJson(e)).toList();
    } else {
      throw Exception('Failed to fetch users');
    }
  }

  Future<void> createUser(String username, String password,
      [String? code]) async {
    final res = await _client.post(
      '/api/users',
      data: {
        'username': username,
        'password': password,
        'registration_code': code
      },
    );

    if (res.statusCode != 200) {
      throw Exception('Failed to create user');
    }
  }

  Future<List<MediaItem>> fetchMovies() async {
    final res = await _client.get('/api/movies');
    if (res.statusCode == 200) {
      final List<dynamic> json = res.data;
      return json.map((e) => MediaItem.fromJson(MediaType.movie, e)).toList();
    } else {
      throw Exception('Failed to fetch movies');
    }
  }

  Future<List<MediaItem>> fetchRecentMovies() async {
    final res = await _client.get('/api/movies/recent');
    if (res.statusCode == 200) {
      final List<dynamic> json = res.data;
      return json.map((e) => MediaItem.fromJson(MediaType.movie, e)).toList();
    } else {
      throw Exception('Failed to fetch movies');
    }
  }

  Future<List<MediaItem>> fetchShows() async {
    final res = await _client.get('/api/shows');
    if (res.statusCode == 200) {
      final List<dynamic> json = res.data;
      return json.map((e) => MediaItem.fromJson(MediaType.show, e)).toList();
    } else {
      throw Exception('Failed to fetch shows');
    }
  }

  Future<List<MediaItem>> fetchRecentShows() async {
    final res = await _client.get('/api/shows/recent');
    if (res.statusCode == 200) {
      final List<dynamic> json = res.data;
      return json.map((e) => MediaItem.fromJson(MediaType.show, e)).toList();
    } else {
      throw Exception('Failed to fetch shows');
    }
  }

  Future<List<MediaItem>> fetchSeasons(int showId) async {
    final res = await _client.get('/api/shows/$showId/seasons');
    if (res.statusCode == 200) {
      final List<dynamic> json = res.data;
      return json.map((e) => MediaItem.fromJson(MediaType.season, e)).toList();
    } else {
      throw Exception('Failed to fetch seasons');
    }
  }

  Future<List<MediaItem>> fetchShowEpisodes(int showId) async {
    final res = await _client.get('/api/shows/$showId/episodes');
    if (res.statusCode == 200) {
      final List<dynamic> json = res.data;
      return json.map((e) => MediaItem.fromJson(MediaType.episode, e)).toList();
    } else {
      throw Exception('Failed to fetch episodes');
    }
  }

  Future<List<MediaItem>> fetchEpisodes(int seasonId) async {
    final res = await _client.get('/api/seasons/$seasonId/episodes');
    if (res.statusCode == 200) {
      final List<dynamic> json = res.data;
      return json.map((e) => MediaItem.fromJson(MediaType.episode, e)).toList();
    } else {
      throw Exception('Failed to fetch episodes');
    }
  }

  Future<MediaItem> fetchMediaItem(int id) async {
    final res = await _client.get('/api/items/$id');
    if (res.statusCode == 200) {
      final dynamic json = res.data;
      final type =
          MediaType.values.firstWhere((type) => type.name == json['type']);
      return MediaItem.fromJson(type, json);
    } else {
      throw Exception('Failed to fetch shows');
    }
  }

  Future<void> deleteMediaItem(int id, {bool removeFiles = false}) async {
    await _client.delete('/api/items/$id',
        queryParameters: {if (removeFiles) 'remove_files': 'true'});
  }

  Future<List<MediaItem>> fetchContinueWatching() async {
    final res = await _client.get('/api/items/continue_watching');
    if (res.statusCode == 200) {
      final List<dynamic> json = res.data;
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
    await _client.post('/api/metadata/$id/find_match');
  }

  Future<void> fixMetadataMatch(int id, FixMetadataMatch data) async {
    await _client.post(
      '/api/metadata/$id/set_match',
      data: {
        'tmdb_id': data.tmdbId,
        'season_number': data.season,
        'episode_number': data.episode,
      },
    );
  }

  Future<void> refreshMetadata(int id) async {
    await _client.post('/api/metadata/$id/refresh');
  }

  Future<Collection> createCollection(String name) async {
    final res = await _client.post(
      '/api/collections',
      data: {'name': name},
    );
    if (res.statusCode == 200) {
      final dynamic json = res.data;
      return Collection.fromJson(json);
    } else {
      throw Exception('Failed to fetch collection');
    }
  }

  Future<List<Collection>> fetchCollections() async {
    final res = await _client.get('/api/collections');
    if (res.statusCode == 200) {
      final List<dynamic> json = res.data;
      return json.map((e) => Collection.fromJson(e)).toList();
    } else {
      throw Exception('Failed to fetch collections');
    }
  }

  Future<List<MediaItem>> fetchCollectionItems(int id) async {
    final res = await _client
        .get('/api/items?collection_id=$id&sort_by[]=collection_index');
    if (res.statusCode == 200) {
      final List<dynamic> json = res.data;
      return json.map((e) => MediaItem.fromJson(MediaType.episode, e)).toList();
    } else {
      throw Exception('Failed to fetch items');
    }
  }

  Future<void> updateCollection(int id, List<int> itemIds) async {
    await _client.put(
      '/api/collections/$id',
      data: {'items': itemIds},
    );
  }

  Future<void> deleteCollection(int id) async {
    await _client.delete('/api/collections/$id');
  }

  String getVideoUrl(int id, {bool attachment = false}) {
    var url = '${_client.options.baseUrl}/api/videos/$id';
    if (attachment) {
      url += '?attachment=true';
    }
    return url;
  }

  String getSubtitleUrl(int id, {SubtitleFormat? format}) {
    var url = '${_client.options.baseUrl}/api/subtitles/$id';
    if (format != null) {
      url += '?format=${format.name}';
    }
    return url;
  }

  String getMediaImageUrl(int id, ImageType type, {int? width}) {
    return Uri.parse(_client.options.baseUrl).replace(
      path: 'api/items/$id/images/${type.name}',
      queryParameters: {if (width != null) 'width': width.toString()},
    ).toString();
  }

  Future updateProgress(int id, int position) async {
    await _client.post('/api/progress/$id?position=$position');
  }

  Future updateUserData(int id, VideoUserDataPatch data) async {
    await _client.patch(
      '/api/items/$id/user_data',
      data: {
        'is_watched': data.isWatched,
        'position': data.position,
      },
    );
  }

  Future<AccessToken> getAccessToken(AccessTokenOwner owner, String name,
      {bool create = false}) async {
    final res = await _client.post('/api/auth/token', queryParameters: {
      'owner': owner.name,
      'name': name,
      'create': create,
    });
    if (res.statusCode == 200) {
      return AccessToken.fromJson(res.data);
    } else {
      throw Exception('Failed to get token');
    }
  }
}

@Riverpod(keepAlive: true)
ZenithApiClient api(ApiRef ref) {
  throw UnimplementedError();
}
