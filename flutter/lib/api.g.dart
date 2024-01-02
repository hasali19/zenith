// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'api.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_$AccessTokenImpl _$$AccessTokenImplFromJson(Map<String, dynamic> json) =>
    _$AccessTokenImpl(
      owner: $enumDecode(_$AccessTokenOwnerEnumMap, json['owner']),
      name: json['name'] as String,
      token: json['token'] as String,
    );

Map<String, dynamic> _$$AccessTokenImplToJson(_$AccessTokenImpl instance) =>
    <String, dynamic>{
      'owner': _$AccessTokenOwnerEnumMap[instance.owner]!,
      'name': instance.name,
      'token': instance.token,
    };

const _$AccessTokenOwnerEnumMap = {
  AccessTokenOwner.system: 'system',
  AccessTokenOwner.user: 'user',
};
