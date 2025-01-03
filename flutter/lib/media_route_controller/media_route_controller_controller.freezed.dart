// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'media_route_controller_controller.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$MediaRouteControllerState {
  cast.MediaRoute? get route => throw _privateConstructorUsedError;
  cast.MediaStatus? get mediaStatus => throw _privateConstructorUsedError;
  cast.MediaInfo? get mediaInfo => throw _privateConstructorUsedError;

  /// Create a copy of MediaRouteControllerState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $MediaRouteControllerStateCopyWith<MediaRouteControllerState> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $MediaRouteControllerStateCopyWith<$Res> {
  factory $MediaRouteControllerStateCopyWith(MediaRouteControllerState value,
          $Res Function(MediaRouteControllerState) then) =
      _$MediaRouteControllerStateCopyWithImpl<$Res, MediaRouteControllerState>;
  @useResult
  $Res call(
      {cast.MediaRoute? route,
      cast.MediaStatus? mediaStatus,
      cast.MediaInfo? mediaInfo});
}

/// @nodoc
class _$MediaRouteControllerStateCopyWithImpl<$Res,
        $Val extends MediaRouteControllerState>
    implements $MediaRouteControllerStateCopyWith<$Res> {
  _$MediaRouteControllerStateCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of MediaRouteControllerState
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? route = freezed,
    Object? mediaStatus = freezed,
    Object? mediaInfo = freezed,
  }) {
    return _then(_value.copyWith(
      route: freezed == route
          ? _value.route
          : route // ignore: cast_nullable_to_non_nullable
              as cast.MediaRoute?,
      mediaStatus: freezed == mediaStatus
          ? _value.mediaStatus
          : mediaStatus // ignore: cast_nullable_to_non_nullable
              as cast.MediaStatus?,
      mediaInfo: freezed == mediaInfo
          ? _value.mediaInfo
          : mediaInfo // ignore: cast_nullable_to_non_nullable
              as cast.MediaInfo?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$MediaRouteControllerStateImplCopyWith<$Res>
    implements $MediaRouteControllerStateCopyWith<$Res> {
  factory _$$MediaRouteControllerStateImplCopyWith(
          _$MediaRouteControllerStateImpl value,
          $Res Function(_$MediaRouteControllerStateImpl) then) =
      __$$MediaRouteControllerStateImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {cast.MediaRoute? route,
      cast.MediaStatus? mediaStatus,
      cast.MediaInfo? mediaInfo});
}

/// @nodoc
class __$$MediaRouteControllerStateImplCopyWithImpl<$Res>
    extends _$MediaRouteControllerStateCopyWithImpl<$Res,
        _$MediaRouteControllerStateImpl>
    implements _$$MediaRouteControllerStateImplCopyWith<$Res> {
  __$$MediaRouteControllerStateImplCopyWithImpl(
      _$MediaRouteControllerStateImpl _value,
      $Res Function(_$MediaRouteControllerStateImpl) _then)
      : super(_value, _then);

  /// Create a copy of MediaRouteControllerState
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? route = freezed,
    Object? mediaStatus = freezed,
    Object? mediaInfo = freezed,
  }) {
    return _then(_$MediaRouteControllerStateImpl(
      route: freezed == route
          ? _value.route
          : route // ignore: cast_nullable_to_non_nullable
              as cast.MediaRoute?,
      mediaStatus: freezed == mediaStatus
          ? _value.mediaStatus
          : mediaStatus // ignore: cast_nullable_to_non_nullable
              as cast.MediaStatus?,
      mediaInfo: freezed == mediaInfo
          ? _value.mediaInfo
          : mediaInfo // ignore: cast_nullable_to_non_nullable
              as cast.MediaInfo?,
    ));
  }
}

/// @nodoc

class _$MediaRouteControllerStateImpl implements _MediaRouteControllerState {
  _$MediaRouteControllerStateImpl(
      {required this.route,
      required this.mediaStatus,
      required this.mediaInfo});

  @override
  final cast.MediaRoute? route;
  @override
  final cast.MediaStatus? mediaStatus;
  @override
  final cast.MediaInfo? mediaInfo;

  @override
  String toString() {
    return 'MediaRouteControllerState(route: $route, mediaStatus: $mediaStatus, mediaInfo: $mediaInfo)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MediaRouteControllerStateImpl &&
            (identical(other.route, route) || other.route == route) &&
            (identical(other.mediaStatus, mediaStatus) ||
                other.mediaStatus == mediaStatus) &&
            (identical(other.mediaInfo, mediaInfo) ||
                other.mediaInfo == mediaInfo));
  }

  @override
  int get hashCode => Object.hash(runtimeType, route, mediaStatus, mediaInfo);

  /// Create a copy of MediaRouteControllerState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$MediaRouteControllerStateImplCopyWith<_$MediaRouteControllerStateImpl>
      get copyWith => __$$MediaRouteControllerStateImplCopyWithImpl<
          _$MediaRouteControllerStateImpl>(this, _$identity);
}

abstract class _MediaRouteControllerState implements MediaRouteControllerState {
  factory _MediaRouteControllerState(
          {required final cast.MediaRoute? route,
          required final cast.MediaStatus? mediaStatus,
          required final cast.MediaInfo? mediaInfo}) =
      _$MediaRouteControllerStateImpl;

  @override
  cast.MediaRoute? get route;
  @override
  cast.MediaStatus? get mediaStatus;
  @override
  cast.MediaInfo? get mediaInfo;

  /// Create a copy of MediaRouteControllerState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$MediaRouteControllerStateImplCopyWith<_$MediaRouteControllerStateImpl>
      get copyWith => throw _privateConstructorUsedError;
}
