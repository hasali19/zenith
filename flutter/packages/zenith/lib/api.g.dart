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

_$CastConfigImpl _$$CastConfigImplFromJson(Map<String, dynamic> json) =>
    _$CastConfigImpl(
      appId: json['app_id'] as String?,
    );

Map<String, dynamic> _$$CastConfigImplToJson(_$CastConfigImpl instance) =>
    <String, dynamic>{
      'app_id': instance.appId,
    };

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

String _$apiHash() => r'c927c76e3b1ebf34ec44afd55979b6d7cceac951';

/// See also [api].
@ProviderFor(api)
final apiProvider = Provider<ZenithApiClient>.internal(
  api,
  name: r'apiProvider',
  debugGetCreateSourceHash:
      const bool.fromEnvironment('dart.vm.product') ? null : _$apiHash,
  dependencies: null,
  allTransitiveDependencies: null,
);

@Deprecated('Will be removed in 3.0. Use Ref instead')
// ignore: unused_element
typedef ApiRef = ProviderRef<ZenithApiClient>;
// ignore_for_file: type=lint
// ignore_for_file: subtype_of_sealed_class, invalid_use_of_internal_member, invalid_use_of_visible_for_testing_member, deprecated_member_use_from_same_package
