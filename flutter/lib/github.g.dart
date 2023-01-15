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
