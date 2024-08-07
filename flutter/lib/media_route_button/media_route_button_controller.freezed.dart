// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'media_route_button_controller.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$MediaRouteButtonState {
  bool get isConnected => throw _privateConstructorUsedError;

  /// Create a copy of MediaRouteButtonState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
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

  /// Create a copy of MediaRouteButtonState
  /// with the given fields replaced by the non-null parameter values.
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
abstract class _$$MediaRouteButtonStateImplCopyWith<$Res>
    implements $MediaRouteButtonStateCopyWith<$Res> {
  factory _$$MediaRouteButtonStateImplCopyWith(
          _$MediaRouteButtonStateImpl value,
          $Res Function(_$MediaRouteButtonStateImpl) then) =
      __$$MediaRouteButtonStateImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({bool isConnected});
}

/// @nodoc
class __$$MediaRouteButtonStateImplCopyWithImpl<$Res>
    extends _$MediaRouteButtonStateCopyWithImpl<$Res,
        _$MediaRouteButtonStateImpl>
    implements _$$MediaRouteButtonStateImplCopyWith<$Res> {
  __$$MediaRouteButtonStateImplCopyWithImpl(_$MediaRouteButtonStateImpl _value,
      $Res Function(_$MediaRouteButtonStateImpl) _then)
      : super(_value, _then);

  /// Create a copy of MediaRouteButtonState
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? isConnected = null,
  }) {
    return _then(_$MediaRouteButtonStateImpl(
      isConnected: null == isConnected
          ? _value.isConnected
          : isConnected // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc

class _$MediaRouteButtonStateImpl implements _MediaRouteButtonState {
  _$MediaRouteButtonStateImpl({required this.isConnected});

  @override
  final bool isConnected;

  @override
  String toString() {
    return 'MediaRouteButtonState(isConnected: $isConnected)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MediaRouteButtonStateImpl &&
            (identical(other.isConnected, isConnected) ||
                other.isConnected == isConnected));
  }

  @override
  int get hashCode => Object.hash(runtimeType, isConnected);

  /// Create a copy of MediaRouteButtonState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$MediaRouteButtonStateImplCopyWith<_$MediaRouteButtonStateImpl>
      get copyWith => __$$MediaRouteButtonStateImplCopyWithImpl<
          _$MediaRouteButtonStateImpl>(this, _$identity);
}

abstract class _MediaRouteButtonState implements MediaRouteButtonState {
  factory _MediaRouteButtonState({required final bool isConnected}) =
      _$MediaRouteButtonStateImpl;

  @override
  bool get isConnected;

  /// Create a copy of MediaRouteButtonState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$MediaRouteButtonStateImplCopyWith<_$MediaRouteButtonStateImpl>
      get copyWith => throw _privateConstructorUsedError;
}
