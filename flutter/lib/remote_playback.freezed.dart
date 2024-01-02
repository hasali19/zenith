// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'remote_playback.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$MediaRoute {
  String get id => throw _privateConstructorUsedError;
  String get name => throw _privateConstructorUsedError;
  String? get description => throw _privateConstructorUsedError;
  bool get isSelected => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $MediaRouteCopyWith<MediaRoute> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $MediaRouteCopyWith<$Res> {
  factory $MediaRouteCopyWith(
          MediaRoute value, $Res Function(MediaRoute) then) =
      _$MediaRouteCopyWithImpl<$Res, MediaRoute>;
  @useResult
  $Res call({String id, String name, String? description, bool isSelected});
}

/// @nodoc
class _$MediaRouteCopyWithImpl<$Res, $Val extends MediaRoute>
    implements $MediaRouteCopyWith<$Res> {
  _$MediaRouteCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? name = null,
    Object? description = freezed,
    Object? isSelected = null,
  }) {
    return _then(_value.copyWith(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      description: freezed == description
          ? _value.description
          : description // ignore: cast_nullable_to_non_nullable
              as String?,
      isSelected: null == isSelected
          ? _value.isSelected
          : isSelected // ignore: cast_nullable_to_non_nullable
              as bool,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$MediaRouteImplCopyWith<$Res>
    implements $MediaRouteCopyWith<$Res> {
  factory _$$MediaRouteImplCopyWith(
          _$MediaRouteImpl value, $Res Function(_$MediaRouteImpl) then) =
      __$$MediaRouteImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String id, String name, String? description, bool isSelected});
}

/// @nodoc
class __$$MediaRouteImplCopyWithImpl<$Res>
    extends _$MediaRouteCopyWithImpl<$Res, _$MediaRouteImpl>
    implements _$$MediaRouteImplCopyWith<$Res> {
  __$$MediaRouteImplCopyWithImpl(
      _$MediaRouteImpl _value, $Res Function(_$MediaRouteImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? name = null,
    Object? description = freezed,
    Object? isSelected = null,
  }) {
    return _then(_$MediaRouteImpl(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as String,
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      description: freezed == description
          ? _value.description
          : description // ignore: cast_nullable_to_non_nullable
              as String?,
      isSelected: null == isSelected
          ? _value.isSelected
          : isSelected // ignore: cast_nullable_to_non_nullable
              as bool,
    ));
  }
}

/// @nodoc

class _$MediaRouteImpl with DiagnosticableTreeMixin implements _MediaRoute {
  const _$MediaRouteImpl(
      {required this.id,
      required this.name,
      required this.description,
      required this.isSelected});

  @override
  final String id;
  @override
  final String name;
  @override
  final String? description;
  @override
  final bool isSelected;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'MediaRoute(id: $id, name: $name, description: $description, isSelected: $isSelected)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'MediaRoute'))
      ..add(DiagnosticsProperty('id', id))
      ..add(DiagnosticsProperty('name', name))
      ..add(DiagnosticsProperty('description', description))
      ..add(DiagnosticsProperty('isSelected', isSelected));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MediaRouteImpl &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.description, description) ||
                other.description == description) &&
            (identical(other.isSelected, isSelected) ||
                other.isSelected == isSelected));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, id, name, description, isSelected);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$MediaRouteImplCopyWith<_$MediaRouteImpl> get copyWith =>
      __$$MediaRouteImplCopyWithImpl<_$MediaRouteImpl>(this, _$identity);
}

abstract class _MediaRoute implements MediaRoute {
  const factory _MediaRoute(
      {required final String id,
      required final String name,
      required final String? description,
      required final bool isSelected}) = _$MediaRouteImpl;

  @override
  String get id;
  @override
  String get name;
  @override
  String? get description;
  @override
  bool get isSelected;
  @override
  @JsonKey(ignore: true)
  _$$MediaRouteImplCopyWith<_$MediaRouteImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
