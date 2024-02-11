// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'media_route_chooser_controller.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$MediaRouteChooserState {
  List<MediaRoute> get routes => throw _privateConstructorUsedError;
  String? get requestedId => throw _privateConstructorUsedError;
  bool get isConnected => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $MediaRouteChooserStateCopyWith<MediaRouteChooserState> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $MediaRouteChooserStateCopyWith<$Res> {
  factory $MediaRouteChooserStateCopyWith(MediaRouteChooserState value,
          $Res Function(MediaRouteChooserState) then) =
      _$MediaRouteChooserStateCopyWithImpl<$Res, MediaRouteChooserState>;
  @useResult
  $Res call({List<MediaRoute> routes, String? requestedId, bool isConnected});
}

/// @nodoc
class _$MediaRouteChooserStateCopyWithImpl<$Res,
        $Val extends MediaRouteChooserState>
    implements $MediaRouteChooserStateCopyWith<$Res> {
  _$MediaRouteChooserStateCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? routes = null,
    Object? requestedId = freezed,
    Object? isConnected = null,
  }) {
    return _then(_value.copyWith(
      routes: null == routes
          ? _value.routes
          : routes // ignore: cast_nullable_to_non_nullable
              as List<MediaRoute>,
      requestedId: freezed == requestedId
          ? _value.requestedId
          : requestedId // ignore: cast_nullable_to_non_nullable
              as String?,
      isConnected: null == isConnected
          ? _value.isConnected
          : isConnected // ignore: cast_nullable_to_non_nullable
              as bool,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$MediaRouteChooserStateImplCopyWith<$Res>
    implements $MediaRouteChooserStateCopyWith<$Res> {
  factory _$$MediaRouteChooserStateImplCopyWith(
          _$MediaRouteChooserStateImpl value,
          $Res Function(_$MediaRouteChooserStateImpl) then) =
      __$$MediaRouteChooserStateImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({List<MediaRoute> routes, String? requestedId, bool isConnected});
}

/// @nodoc
class __$$MediaRouteChooserStateImplCopyWithImpl<$Res>
    extends _$MediaRouteChooserStateCopyWithImpl<$Res,
        _$MediaRouteChooserStateImpl>
    implements _$$MediaRouteChooserStateImplCopyWith<$Res> {
  __$$MediaRouteChooserStateImplCopyWithImpl(
      _$MediaRouteChooserStateImpl _value,
      $Res Function(_$MediaRouteChooserStateImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? routes = null,
    Object? requestedId = freezed,
    Object? isConnected = null,
  }) {
    return _then(_$MediaRouteChooserStateImpl(
      routes: null == routes
          ? _value._routes
          : routes // ignore: cast_nullable_to_non_nullable
              as List<MediaRoute>,
      requestedId: freezed == requestedId
          ? _value.requestedId
          : requestedId // ignore: cast_nullable_to_non_nullable
              as String?,
      isConnected: null == isConnected
          ? _value.isConnected
          : isConnected // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc

class _$MediaRouteChooserStateImpl implements _MediaRouteChooserState {
  _$MediaRouteChooserStateImpl(
      {required final List<MediaRoute> routes,
      required this.requestedId,
      required this.isConnected})
      : _routes = routes;

  final List<MediaRoute> _routes;
  @override
  List<MediaRoute> get routes {
    if (_routes is EqualUnmodifiableListView) return _routes;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_routes);
  }

  @override
  final String? requestedId;
  @override
  final bool isConnected;

  @override
  String toString() {
    return 'MediaRouteChooserState(routes: $routes, requestedId: $requestedId, isConnected: $isConnected)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MediaRouteChooserStateImpl &&
            const DeepCollectionEquality().equals(other._routes, _routes) &&
            (identical(other.requestedId, requestedId) ||
                other.requestedId == requestedId) &&
            (identical(other.isConnected, isConnected) ||
                other.isConnected == isConnected));
  }

  @override
  int get hashCode => Object.hash(runtimeType,
      const DeepCollectionEquality().hash(_routes), requestedId, isConnected);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$MediaRouteChooserStateImplCopyWith<_$MediaRouteChooserStateImpl>
      get copyWith => __$$MediaRouteChooserStateImplCopyWithImpl<
          _$MediaRouteChooserStateImpl>(this, _$identity);
}

abstract class _MediaRouteChooserState implements MediaRouteChooserState {
  factory _MediaRouteChooserState(
      {required final List<MediaRoute> routes,
      required final String? requestedId,
      required final bool isConnected}) = _$MediaRouteChooserStateImpl;

  @override
  List<MediaRoute> get routes;
  @override
  String? get requestedId;
  @override
  bool get isConnected;
  @override
  @JsonKey(ignore: true)
  _$$MediaRouteChooserStateImplCopyWith<_$MediaRouteChooserStateImpl>
      get copyWith => throw _privateConstructorUsedError;
}
