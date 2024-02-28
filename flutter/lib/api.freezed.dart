// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'api.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

AccessToken _$AccessTokenFromJson(Map<String, dynamic> json) {
  return _AccessToken.fromJson(json);
}

/// @nodoc
mixin _$AccessToken {
  AccessTokenOwner get owner => throw _privateConstructorUsedError;
  String get name => throw _privateConstructorUsedError;
  String get token => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $AccessTokenCopyWith<AccessToken> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $AccessTokenCopyWith<$Res> {
  factory $AccessTokenCopyWith(
          AccessToken value, $Res Function(AccessToken) then) =
      _$AccessTokenCopyWithImpl<$Res, AccessToken>;
  @useResult
  $Res call({AccessTokenOwner owner, String name, String token});
}

/// @nodoc
class _$AccessTokenCopyWithImpl<$Res, $Val extends AccessToken>
    implements $AccessTokenCopyWith<$Res> {
  _$AccessTokenCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? owner = null,
    Object? name = null,
    Object? token = null,
  }) {
    return _then(_value.copyWith(
      owner: null == owner
          ? _value.owner
          : owner // ignore: cast_nullable_to_non_nullable
              as AccessTokenOwner,
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      token: null == token
          ? _value.token
          : token // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$AccessTokenImplCopyWith<$Res>
    implements $AccessTokenCopyWith<$Res> {
  factory _$$AccessTokenImplCopyWith(
          _$AccessTokenImpl value, $Res Function(_$AccessTokenImpl) then) =
      __$$AccessTokenImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({AccessTokenOwner owner, String name, String token});
}

/// @nodoc
class __$$AccessTokenImplCopyWithImpl<$Res>
    extends _$AccessTokenCopyWithImpl<$Res, _$AccessTokenImpl>
    implements _$$AccessTokenImplCopyWith<$Res> {
  __$$AccessTokenImplCopyWithImpl(
      _$AccessTokenImpl _value, $Res Function(_$AccessTokenImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? owner = null,
    Object? name = null,
    Object? token = null,
  }) {
    return _then(_$AccessTokenImpl(
      owner: null == owner
          ? _value.owner
          : owner // ignore: cast_nullable_to_non_nullable
              as AccessTokenOwner,
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      token: null == token
          ? _value.token
          : token // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$AccessTokenImpl implements _AccessToken {
  _$AccessTokenImpl(
      {required this.owner, required this.name, required this.token});

  factory _$AccessTokenImpl.fromJson(Map<String, dynamic> json) =>
      _$$AccessTokenImplFromJson(json);

  @override
  final AccessTokenOwner owner;
  @override
  final String name;
  @override
  final String token;

  @override
  String toString() {
    return 'AccessToken(owner: $owner, name: $name, token: $token)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AccessTokenImpl &&
            (identical(other.owner, owner) || other.owner == owner) &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.token, token) || other.token == token));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, owner, name, token);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AccessTokenImplCopyWith<_$AccessTokenImpl> get copyWith =>
      __$$AccessTokenImplCopyWithImpl<_$AccessTokenImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$AccessTokenImplToJson(
      this,
    );
  }
}

abstract class _AccessToken implements AccessToken {
  factory _AccessToken(
      {required final AccessTokenOwner owner,
      required final String name,
      required final String token}) = _$AccessTokenImpl;

  factory _AccessToken.fromJson(Map<String, dynamic> json) =
      _$AccessTokenImpl.fromJson;

  @override
  AccessTokenOwner get owner;
  @override
  String get name;
  @override
  String get token;
  @override
  @JsonKey(ignore: true)
  _$$AccessTokenImplCopyWith<_$AccessTokenImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

CastConfig _$CastConfigFromJson(Map<String, dynamic> json) {
  return _CastConfig.fromJson(json);
}

/// @nodoc
mixin _$CastConfig {
// ignore: invalid_annotation_target
  @JsonKey(name: 'app_id')
  String? get appId => throw _privateConstructorUsedError;

  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  $CastConfigCopyWith<CastConfig> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CastConfigCopyWith<$Res> {
  factory $CastConfigCopyWith(
          CastConfig value, $Res Function(CastConfig) then) =
      _$CastConfigCopyWithImpl<$Res, CastConfig>;
  @useResult
  $Res call({@JsonKey(name: 'app_id') String? appId});
}

/// @nodoc
class _$CastConfigCopyWithImpl<$Res, $Val extends CastConfig>
    implements $CastConfigCopyWith<$Res> {
  _$CastConfigCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? appId = freezed,
  }) {
    return _then(_value.copyWith(
      appId: freezed == appId
          ? _value.appId
          : appId // ignore: cast_nullable_to_non_nullable
              as String?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$CastConfigImplCopyWith<$Res>
    implements $CastConfigCopyWith<$Res> {
  factory _$$CastConfigImplCopyWith(
          _$CastConfigImpl value, $Res Function(_$CastConfigImpl) then) =
      __$$CastConfigImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({@JsonKey(name: 'app_id') String? appId});
}

/// @nodoc
class __$$CastConfigImplCopyWithImpl<$Res>
    extends _$CastConfigCopyWithImpl<$Res, _$CastConfigImpl>
    implements _$$CastConfigImplCopyWith<$Res> {
  __$$CastConfigImplCopyWithImpl(
      _$CastConfigImpl _value, $Res Function(_$CastConfigImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? appId = freezed,
  }) {
    return _then(_$CastConfigImpl(
      appId: freezed == appId
          ? _value.appId
          : appId // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$CastConfigImpl implements _CastConfig {
  _$CastConfigImpl({@JsonKey(name: 'app_id') required this.appId});

  factory _$CastConfigImpl.fromJson(Map<String, dynamic> json) =>
      _$$CastConfigImplFromJson(json);

// ignore: invalid_annotation_target
  @override
  @JsonKey(name: 'app_id')
  final String? appId;

  @override
  String toString() {
    return 'CastConfig(appId: $appId)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CastConfigImpl &&
            (identical(other.appId, appId) || other.appId == appId));
  }

  @JsonKey(ignore: true)
  @override
  int get hashCode => Object.hash(runtimeType, appId);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$CastConfigImplCopyWith<_$CastConfigImpl> get copyWith =>
      __$$CastConfigImplCopyWithImpl<_$CastConfigImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$CastConfigImplToJson(
      this,
    );
  }
}

abstract class _CastConfig implements CastConfig {
  factory _CastConfig({@JsonKey(name: 'app_id') required final String? appId}) =
      _$CastConfigImpl;

  factory _CastConfig.fromJson(Map<String, dynamic> json) =
      _$CastConfigImpl.fromJson;

  @override // ignore: invalid_annotation_target
  @JsonKey(name: 'app_id')
  String? get appId;
  @override
  @JsonKey(ignore: true)
  _$$CastConfigImplCopyWith<_$CastConfigImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
