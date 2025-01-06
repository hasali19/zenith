// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'github.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

GitReference _$GitReferenceFromJson(Map<String, dynamic> json) => GitReference(
      object:
          GitReferenceObject.fromJson(json['object'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$GitReferenceToJson(GitReference instance) =>
    <String, dynamic>{
      'object': instance.object,
    };

GitReferenceObject _$GitReferenceObjectFromJson(Map<String, dynamic> json) =>
    GitReferenceObject(
      sha: json['sha'] as String,
      type: json['type'] as String,
    );

Map<String, dynamic> _$GitReferenceObjectToJson(GitReferenceObject instance) =>
    <String, dynamic>{
      'sha': instance.sha,
      'type': instance.type,
    };

Release _$ReleaseFromJson(Map<String, dynamic> json) => Release(
      assets: (json['assets'] as List<dynamic>)
          .map((e) => ReleaseAsset.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$ReleaseToJson(Release instance) => <String, dynamic>{
      'assets': instance.assets,
    };

ReleaseAsset _$ReleaseAssetFromJson(Map<String, dynamic> json) => ReleaseAsset(
      name: json['name'] as String,
      browserDownloadUrl: json['browser_download_url'] as String,
    );

Map<String, dynamic> _$ReleaseAssetToJson(ReleaseAsset instance) =>
    <String, dynamic>{
      'name': instance.name,
      'browser_download_url': instance.browserDownloadUrl,
    };
