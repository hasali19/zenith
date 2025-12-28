// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'media_route_controller_controller.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$MediaRouteControllerState {
  cast.MediaRoute? get route;
  CastMediaStatus? get mediaStatus;
  cast.MediaInfo? get mediaInfo;

  /// Create a copy of MediaRouteControllerState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $MediaRouteControllerStateCopyWith<MediaRouteControllerState> get copyWith =>
      _$MediaRouteControllerStateCopyWithImpl<MediaRouteControllerState>(
          this as MediaRouteControllerState, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is MediaRouteControllerState &&
            (identical(other.route, route) || other.route == route) &&
            (identical(other.mediaStatus, mediaStatus) ||
                other.mediaStatus == mediaStatus) &&
            (identical(other.mediaInfo, mediaInfo) ||
                other.mediaInfo == mediaInfo));
  }

  @override
  int get hashCode => Object.hash(runtimeType, route, mediaStatus, mediaInfo);

  @override
  String toString() {
    return 'MediaRouteControllerState(route: $route, mediaStatus: $mediaStatus, mediaInfo: $mediaInfo)';
  }
}

/// @nodoc
abstract mixin class $MediaRouteControllerStateCopyWith<$Res> {
  factory $MediaRouteControllerStateCopyWith(MediaRouteControllerState value,
          $Res Function(MediaRouteControllerState) _then) =
      _$MediaRouteControllerStateCopyWithImpl;
  @useResult
  $Res call(
      {cast.MediaRoute? route,
      CastMediaStatus? mediaStatus,
      cast.MediaInfo? mediaInfo});
}

/// @nodoc
class _$MediaRouteControllerStateCopyWithImpl<$Res>
    implements $MediaRouteControllerStateCopyWith<$Res> {
  _$MediaRouteControllerStateCopyWithImpl(this._self, this._then);

  final MediaRouteControllerState _self;
  final $Res Function(MediaRouteControllerState) _then;

  /// Create a copy of MediaRouteControllerState
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? route = freezed,
    Object? mediaStatus = freezed,
    Object? mediaInfo = freezed,
  }) {
    return _then(_self.copyWith(
      route: freezed == route
          ? _self.route
          : route // ignore: cast_nullable_to_non_nullable
              as cast.MediaRoute?,
      mediaStatus: freezed == mediaStatus
          ? _self.mediaStatus
          : mediaStatus // ignore: cast_nullable_to_non_nullable
              as CastMediaStatus?,
      mediaInfo: freezed == mediaInfo
          ? _self.mediaInfo
          : mediaInfo // ignore: cast_nullable_to_non_nullable
              as cast.MediaInfo?,
    ));
  }
}

/// Adds pattern-matching-related methods to [MediaRouteControllerState].
extension MediaRouteControllerStatePatterns on MediaRouteControllerState {
  /// A variant of `map` that fallback to returning `orElse`.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case _:
  ///     return orElse();
  /// }
  /// ```

  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>(
    TResult Function(_MediaRouteControllerState value)? $default, {
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case _MediaRouteControllerState() when $default != null:
        return $default(_that);
      case _:
        return orElse();
    }
  }

  /// A `switch`-like method, using callbacks.
  ///
  /// Callbacks receives the raw object, upcasted.
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case final Subclass2 value:
  ///     return ...;
  /// }
  /// ```

  @optionalTypeArgs
  TResult map<TResult extends Object?>(
    TResult Function(_MediaRouteControllerState value) $default,
  ) {
    final _that = this;
    switch (_that) {
      case _MediaRouteControllerState():
        return $default(_that);
      case _:
        throw StateError('Unexpected subclass');
    }
  }

  /// A variant of `map` that fallback to returning `null`.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case final Subclass value:
  ///     return ...;
  ///   case _:
  ///     return null;
  /// }
  /// ```

  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>(
    TResult? Function(_MediaRouteControllerState value)? $default,
  ) {
    final _that = this;
    switch (_that) {
      case _MediaRouteControllerState() when $default != null:
        return $default(_that);
      case _:
        return null;
    }
  }

  /// A variant of `when` that fallback to an `orElse` callback.
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case _:
  ///     return orElse();
  /// }
  /// ```

  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>(
    TResult Function(cast.MediaRoute? route, CastMediaStatus? mediaStatus,
            cast.MediaInfo? mediaInfo)?
        $default, {
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case _MediaRouteControllerState() when $default != null:
        return $default(_that.route, _that.mediaStatus, _that.mediaInfo);
      case _:
        return orElse();
    }
  }

  /// A `switch`-like method, using callbacks.
  ///
  /// As opposed to `map`, this offers destructuring.
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case Subclass2(:final field2):
  ///     return ...;
  /// }
  /// ```

  @optionalTypeArgs
  TResult when<TResult extends Object?>(
    TResult Function(cast.MediaRoute? route, CastMediaStatus? mediaStatus,
            cast.MediaInfo? mediaInfo)
        $default,
  ) {
    final _that = this;
    switch (_that) {
      case _MediaRouteControllerState():
        return $default(_that.route, _that.mediaStatus, _that.mediaInfo);
      case _:
        throw StateError('Unexpected subclass');
    }
  }

  /// A variant of `when` that fallback to returning `null`
  ///
  /// It is equivalent to doing:
  /// ```dart
  /// switch (sealedClass) {
  ///   case Subclass(:final field):
  ///     return ...;
  ///   case _:
  ///     return null;
  /// }
  /// ```

  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>(
    TResult? Function(cast.MediaRoute? route, CastMediaStatus? mediaStatus,
            cast.MediaInfo? mediaInfo)?
        $default,
  ) {
    final _that = this;
    switch (_that) {
      case _MediaRouteControllerState() when $default != null:
        return $default(_that.route, _that.mediaStatus, _that.mediaInfo);
      case _:
        return null;
    }
  }
}

/// @nodoc

class _MediaRouteControllerState implements MediaRouteControllerState {
  _MediaRouteControllerState(
      {required this.route,
      required this.mediaStatus,
      required this.mediaInfo});

  @override
  final cast.MediaRoute? route;
  @override
  final CastMediaStatus? mediaStatus;
  @override
  final cast.MediaInfo? mediaInfo;

  /// Create a copy of MediaRouteControllerState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$MediaRouteControllerStateCopyWith<_MediaRouteControllerState>
      get copyWith =>
          __$MediaRouteControllerStateCopyWithImpl<_MediaRouteControllerState>(
              this, _$identity);

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _MediaRouteControllerState &&
            (identical(other.route, route) || other.route == route) &&
            (identical(other.mediaStatus, mediaStatus) ||
                other.mediaStatus == mediaStatus) &&
            (identical(other.mediaInfo, mediaInfo) ||
                other.mediaInfo == mediaInfo));
  }

  @override
  int get hashCode => Object.hash(runtimeType, route, mediaStatus, mediaInfo);

  @override
  String toString() {
    return 'MediaRouteControllerState(route: $route, mediaStatus: $mediaStatus, mediaInfo: $mediaInfo)';
  }
}

/// @nodoc
abstract mixin class _$MediaRouteControllerStateCopyWith<$Res>
    implements $MediaRouteControllerStateCopyWith<$Res> {
  factory _$MediaRouteControllerStateCopyWith(_MediaRouteControllerState value,
          $Res Function(_MediaRouteControllerState) _then) =
      __$MediaRouteControllerStateCopyWithImpl;
  @override
  @useResult
  $Res call(
      {cast.MediaRoute? route,
      CastMediaStatus? mediaStatus,
      cast.MediaInfo? mediaInfo});
}

/// @nodoc
class __$MediaRouteControllerStateCopyWithImpl<$Res>
    implements _$MediaRouteControllerStateCopyWith<$Res> {
  __$MediaRouteControllerStateCopyWithImpl(this._self, this._then);

  final _MediaRouteControllerState _self;
  final $Res Function(_MediaRouteControllerState) _then;

  /// Create a copy of MediaRouteControllerState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? route = freezed,
    Object? mediaStatus = freezed,
    Object? mediaInfo = freezed,
  }) {
    return _then(_MediaRouteControllerState(
      route: freezed == route
          ? _self.route
          : route // ignore: cast_nullable_to_non_nullable
              as cast.MediaRoute?,
      mediaStatus: freezed == mediaStatus
          ? _self.mediaStatus
          : mediaStatus // ignore: cast_nullable_to_non_nullable
              as CastMediaStatus?,
      mediaInfo: freezed == mediaInfo
          ? _self.mediaInfo
          : mediaInfo // ignore: cast_nullable_to_non_nullable
              as cast.MediaInfo?,
    ));
  }
}

// dart format on
