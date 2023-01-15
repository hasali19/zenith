import 'dart:convert';

import 'package:http/http.dart' as http;
import 'package:json_annotation/json_annotation.dart';

part 'github.g.dart';

@JsonSerializable(fieldRename: FieldRename.snake)
class GitReference {
  final GitReferenceObject object;

  GitReference({
    required this.object,
  });

  factory GitReference.fromJson(Map<String, dynamic> json) =>
      _$GitReferenceFromJson(json);
}

@JsonSerializable(fieldRename: FieldRename.snake)
class GitReferenceObject {
  final String sha;
  final String type;

  GitReferenceObject({
    required this.sha,
    required this.type,
  });

  factory GitReferenceObject.fromJson(Map<String, dynamic> json) =>
      _$GitReferenceObjectFromJson(json);
}

class GitHub {
  Future<GitReference> getGitRef(String ref) async {
    return GitReference.fromJson(await _get(Uri.parse(
        'https://api.github.com/repos/hasali19/zenith/git/ref/$ref')));
  }

  Future<dynamic> _get(Uri uri) async {
    final res = await http.get(uri);
    if (res.statusCode >= 200 && res.statusCode < 300) {
      return jsonDecode(utf8.decode(res.bodyBytes));
    } else {
      throw Exception('Request returned status ${res.statusCode}');
    }
  }
}
