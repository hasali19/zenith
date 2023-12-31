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
abstract class _$$_AccessTokenCopyWith<$Res>
    implements $AccessTokenCopyWith<$Res> {
  factory _$$_AccessTokenCopyWith(
          _$_AccessToken value, $Res Function(_$_AccessToken) then) =
      __$$_AccessTokenCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({AccessTokenOwner owner, String name, String token});
}

/// @nodoc
class __$$_AccessTokenCopyWithImpl<$Res>
    extends _$AccessTokenCopyWithImpl<$Res, _$_AccessToken>
    implements _$$_AccessTokenCopyWith<$Res> {
  __$$_AccessTokenCopyWithImpl(
      _$_AccessToken _value, $Res Function(_$_AccessToken) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? owner = null,
    Object? name = null,
    Object? token = null,
  }) {
    return _then(_$_AccessToken(
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
class _$_AccessToken implements _AccessToken {
  _$_AccessToken(
      {required this.owner, required this.name, required this.token});

  factory _$_AccessToken.fromJson(Map<String, dynamic> json) =>
      _$$_AccessTokenFromJson(json);

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
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_AccessToken &&
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
  _$$_AccessTokenCopyWith<_$_AccessToken> get copyWith =>
      __$$_AccessTokenCopyWithImpl<_$_AccessToken>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$_AccessTokenToJson(
      this,
    );
  }
}

abstract class _AccessToken implements AccessToken {
  factory _AccessToken(
      {required final AccessTokenOwner owner,
      required final String name,
      required final String token}) = _$_AccessToken;

  factory _AccessToken.fromJson(Map<String, dynamic> json) =
      _$_AccessToken.fromJson;

  @override
  AccessTokenOwner get owner;
  @override
  String get name;
  @override
  String get token;
  @override
  @JsonKey(ignore: true)
  _$$_AccessTokenCopyWith<_$_AccessToken> get copyWith =>
      throw _privateConstructorUsedError;
}
