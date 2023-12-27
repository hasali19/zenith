// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'media_route_controller_cubit.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$MediaRouteControllerState {
  MediaRoute? get route => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $MediaRouteControllerStateCopyWith<MediaRouteControllerState> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $MediaRouteControllerStateCopyWith<$Res> {
  factory $MediaRouteControllerStateCopyWith(MediaRouteControllerState value,
          $Res Function(MediaRouteControllerState) then) =
      _$MediaRouteControllerStateCopyWithImpl<$Res, MediaRouteControllerState>;
  @useResult
  $Res call({MediaRoute? route});

  $MediaRouteCopyWith<$Res>? get route;
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

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? route = freezed,
  }) {
    return _then(_value.copyWith(
      route: freezed == route
          ? _value.route
          : route // ignore: cast_nullable_to_non_nullable
              as MediaRoute?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $MediaRouteCopyWith<$Res>? get route {
    if (_value.route == null) {
      return null;
    }

    return $MediaRouteCopyWith<$Res>(_value.route!, (value) {
      return _then(_value.copyWith(route: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$_MediaRouteControllerStateCopyWith<$Res>
    implements $MediaRouteControllerStateCopyWith<$Res> {
  factory _$$_MediaRouteControllerStateCopyWith(
          _$_MediaRouteControllerState value,
          $Res Function(_$_MediaRouteControllerState) then) =
      __$$_MediaRouteControllerStateCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({MediaRoute? route});

  @override
  $MediaRouteCopyWith<$Res>? get route;
}

/// @nodoc
class __$$_MediaRouteControllerStateCopyWithImpl<$Res>
    extends _$MediaRouteControllerStateCopyWithImpl<$Res,
        _$_MediaRouteControllerState>
    implements _$$_MediaRouteControllerStateCopyWith<$Res> {
  __$$_MediaRouteControllerStateCopyWithImpl(
      _$_MediaRouteControllerState _value,
      $Res Function(_$_MediaRouteControllerState) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? route = freezed,
  }) {
    return _then(_$_MediaRouteControllerState(
      route: freezed == route
          ? _value.route
          : route // ignore: cast_nullable_to_non_nullable
              as MediaRoute?,
    ));
  }
}

/// @nodoc

class _$_MediaRouteControllerState implements _MediaRouteControllerState {
  _$_MediaRouteControllerState({required this.route});

  @override
  final MediaRoute? route;

  @override
  String toString() {
    return 'MediaRouteControllerState(route: $route)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_MediaRouteControllerState &&
            (identical(other.route, route) || other.route == route));
  }

  @override
  int get hashCode => Object.hash(runtimeType, route);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_MediaRouteControllerStateCopyWith<_$_MediaRouteControllerState>
      get copyWith => __$$_MediaRouteControllerStateCopyWithImpl<
          _$_MediaRouteControllerState>(this, _$identity);
}

abstract class _MediaRouteControllerState implements MediaRouteControllerState {
  factory _MediaRouteControllerState({required final MediaRoute? route}) =
      _$_MediaRouteControllerState;

  @override
  MediaRoute? get route;
  @override
  @JsonKey(ignore: true)
  _$$_MediaRouteControllerStateCopyWith<_$_MediaRouteControllerState>
      get copyWith => throw _privateConstructorUsedError;
}
