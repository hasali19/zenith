// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'api.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_$_AccessToken _$$_AccessTokenFromJson(Map<String, dynamic> json) =>
    _$_AccessToken(
      owner: $enumDecode(_$AccessTokenOwnerEnumMap, json['owner']),
      name: json['name'] as String,
      token: json['token'] as String,
    );

Map<String, dynamic> _$$_AccessTokenToJson(_$_AccessToken instance) =>
    <String, dynamic>{
      'owner': _$AccessTokenOwnerEnumMap[instance.owner]!,
      'name': instance.name,
      'token': instance.token,
    };

const _$AccessTokenOwnerEnumMap = {
  AccessTokenOwner.system: 'system',
  AccessTokenOwner.user: 'user',
};
