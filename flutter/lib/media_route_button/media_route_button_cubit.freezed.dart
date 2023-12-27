// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'media_route_button_cubit.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$MediaRouteButtonState {
  bool get isConnected => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $MediaRouteButtonStateCopyWith<MediaRouteButtonState> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $MediaRouteButtonStateCopyWith<$Res> {
  factory $MediaRouteButtonStateCopyWith(MediaRouteButtonState value,
          $Res Function(MediaRouteButtonState) then) =
      _$MediaRouteButtonStateCopyWithImpl<$Res, MediaRouteButtonState>;
  @useResult
  $Res call({bool isConnected});
}

/// @nodoc
class _$MediaRouteButtonStateCopyWithImpl<$Res,
        $Val extends MediaRouteButtonState>
    implements $MediaRouteButtonStateCopyWith<$Res> {
  _$MediaRouteButtonStateCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? isConnected = null,
  }) {
    return _then(_value.copyWith(
      isConnected: null == isConnected
          ? _value.isConnected
          : isConnected // ignore: cast_nullable_to_non_nullable
              as bool,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_MediaRouteButtonStateCopyWith<$Res>
    implements $MediaRouteButtonStateCopyWith<$Res> {
  factory _$$_MediaRouteButtonStateCopyWith(_$_MediaRouteButtonState value,
          $Res Function(_$_MediaRouteButtonState) then) =
      __$$_MediaRouteButtonStateCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({bool isConnected});
}

/// @nodoc
class __$$_MediaRouteButtonStateCopyWithImpl<$Res>
    extends _$MediaRouteButtonStateCopyWithImpl<$Res, _$_MediaRouteButtonState>
    implements _$$_MediaRouteButtonStateCopyWith<$Res> {
  __$$_MediaRouteButtonStateCopyWithImpl(_$_MediaRouteButtonState _value,
      $Res Function(_$_MediaRouteButtonState) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? isConnected = null,
  }) {
    return _then(_$_MediaRouteButtonState(
      isConnected: null == isConnected
          ? _value.isConnected
          : isConnected // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc

class _$_MediaRouteButtonState implements _MediaRouteButtonState {
  _$_MediaRouteButtonState({required this.isConnected});

  @override
  final bool isConnected;

  @override
  String toString() {
    return 'MediaRouteButtonState(isConnected: $isConnected)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_MediaRouteButtonState &&
            (identical(other.isConnected, isConnected) ||
                other.isConnected == isConnected));
  }

  @override
  int get hashCode => Object.hash(runtimeType, isConnected);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_MediaRouteButtonStateCopyWith<_$_MediaRouteButtonState> get copyWith =>
      __$$_MediaRouteButtonStateCopyWithImpl<_$_MediaRouteButtonState>(
          this, _$identity);
}

abstract class _MediaRouteButtonState implements MediaRouteButtonState {
  factory _MediaRouteButtonState({required final bool isConnected}) =
      _$_MediaRouteButtonState;

  @override
  bool get isConnected;
  @override
  @JsonKey(ignore: true)
  _$$_MediaRouteButtonStateCopyWith<_$_MediaRouteButtonState> get copyWith =>
      throw _privateConstructorUsedError;
}
