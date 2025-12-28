import 'package:dio/dio.dart';
import 'package:json_annotation/json_annotation.dart';

part 'github.g.dart';

final _client = Dio(BaseOptions(baseUrl: 'https://api.github.com'));

@JsonSerializable(fieldRename: FieldRename.snake)
class GitReference {
  final GitReferenceObject object;

  GitReference({required this.object});

  factory GitReference.fromJson(Map<String, dynamic> json) =>
      _$GitReferenceFromJson(json);
}

@JsonSerializable(fieldRename: FieldRename.snake)
class GitReferenceObject {
  final String sha;
  final String type;

  GitReferenceObject({required this.sha, required this.type});

  factory GitReferenceObject.fromJson(Map<String, dynamic> json) =>
      _$GitReferenceObjectFromJson(json);
}

@JsonSerializable(fieldRename: FieldRename.snake)
class Release {
  final List<ReleaseAsset> assets;

  Release({required this.assets});

  factory Release.fromJson(Map<String, dynamic> json) =>
      _$ReleaseFromJson(json);
}

@JsonSerializable(fieldRename: FieldRename.snake)
class ReleaseAsset {
  final String name;
  final String browserDownloadUrl;

  ReleaseAsset({required this.name, required this.browserDownloadUrl});

  factory ReleaseAsset.fromJson(Map<String, dynamic> json) =>
      _$ReleaseAssetFromJson(json);
}

class GitHub {
  Future<GitReference> getGitRef(String ref) async {
    return GitReference.fromJson(
      await _get(Uri.parse('/repos/hasali19/zenith/git/ref/$ref')),
    );
  }

  Future<Release> getRelease(String tag) async {
    return Release.fromJson(
      await _get(Uri.parse('/repos/hasali19/zenith/releases/tags/$tag')),
    );
  }

  Future<dynamic> _get(Uri uri) async {
    final res = await _client.getUri(uri);
    if (res.statusCode! >= 200 && res.statusCode! < 300) {
      return res.data;
    } else {
      throw Exception('Request returned status ${res.statusCode}');
    }
  }
}
