// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'api.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

_AccessToken _$AccessTokenFromJson(Map<String, dynamic> json) => _AccessToken(
  owner: $enumDecode(_$AccessTokenOwnerEnumMap, json['owner']),
  name: json['name'] as String,
  token: json['token'] as String,
);

Map<String, dynamic> _$AccessTokenToJson(_AccessToken instance) =>
    <String, dynamic>{
      'owner': _$AccessTokenOwnerEnumMap[instance.owner]!,
      'name': instance.name,
      'token': instance.token,
    };

const _$AccessTokenOwnerEnumMap = {
  AccessTokenOwner.system: 'system',
  AccessTokenOwner.user: 'user',
};

_CastConfig _$CastConfigFromJson(Map<String, dynamic> json) =>
    _CastConfig(appId: json['app_id'] as String?);

Map<String, dynamic> _$CastConfigToJson(_CastConfig instance) =>
    <String, dynamic>{'app_id': instance.appId};

_TranscoderState _$TranscoderStateFromJson(Map<String, dynamic> json) =>
    _TranscoderState(
      queue: (json['queue'] as List<dynamic>)
          .map((e) => TranscoderJob.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$TranscoderStateToJson(_TranscoderState instance) =>
    <String, dynamic>{'queue': instance.queue};

Queued _$QueuedFromJson(Map<String, dynamic> json) => Queued(
  (json['video_id'] as num).toInt(),
  (json['item_id'] as num).toInt(),
  $type: json['state'] as String?,
);

Map<String, dynamic> _$QueuedToJson(Queued instance) => <String, dynamic>{
  'video_id': instance.videoId,
  'item_id': instance.itemId,
  'state': instance.$type,
};

Processing _$ProcessingFromJson(Map<String, dynamic> json) => Processing(
  (json['video_id'] as num).toInt(),
  (json['item_id'] as num).toInt(),
  (json['progress'] as num).toDouble(),
  $type: json['state'] as String?,
);

Map<String, dynamic> _$ProcessingToJson(Processing instance) =>
    <String, dynamic>{
      'video_id': instance.videoId,
      'item_id': instance.itemId,
      'progress': instance.progress,
      'state': instance.$type,
    };

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(api)
final apiProvider = ApiProvider._();

final class ApiProvider
    extends
        $FunctionalProvider<ZenithApiClient, ZenithApiClient, ZenithApiClient>
    with $Provider<ZenithApiClient> {
  ApiProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'apiProvider',
        isAutoDispose: false,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$apiHash();

  @$internal
  @override
  $ProviderElement<ZenithApiClient> $createElement($ProviderPointer pointer) =>
      $ProviderElement(pointer);

  @override
  ZenithApiClient create(Ref ref) {
    return api(ref);
  }

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(ZenithApiClient value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<ZenithApiClient>(value),
    );
  }
}

String _$apiHash() => r'ca408fb87c23d1acd19fc531c3cd6cb3eb3617e6';
