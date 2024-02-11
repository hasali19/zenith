// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'home.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$HomeScreenData {
  List<MediaItem> get continueWatching => throw _privateConstructorUsedError;
  List<MediaItem> get recentMovies => throw _privateConstructorUsedError;
  List<MediaItem> get recentShows => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $HomeScreenDataCopyWith<HomeScreenData> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $HomeScreenDataCopyWith<$Res> {
  factory $HomeScreenDataCopyWith(
          HomeScreenData value, $Res Function(HomeScreenData) then) =
      _$HomeScreenDataCopyWithImpl<$Res, HomeScreenData>;
  @useResult
  $Res call(
      {List<MediaItem> continueWatching,
      List<MediaItem> recentMovies,
      List<MediaItem> recentShows});
}

/// @nodoc
class _$HomeScreenDataCopyWithImpl<$Res, $Val extends HomeScreenData>
    implements $HomeScreenDataCopyWith<$Res> {
  _$HomeScreenDataCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? continueWatching = null,
    Object? recentMovies = null,
    Object? recentShows = null,
  }) {
    return _then(_value.copyWith(
      continueWatching: null == continueWatching
          ? _value.continueWatching
          : continueWatching // ignore: cast_nullable_to_non_nullable
              as List<MediaItem>,
      recentMovies: null == recentMovies
          ? _value.recentMovies
          : recentMovies // ignore: cast_nullable_to_non_nullable
              as List<MediaItem>,
      recentShows: null == recentShows
          ? _value.recentShows
          : recentShows // ignore: cast_nullable_to_non_nullable
              as List<MediaItem>,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$HomeScreenDataImplCopyWith<$Res>
    implements $HomeScreenDataCopyWith<$Res> {
  factory _$$HomeScreenDataImplCopyWith(_$HomeScreenDataImpl value,
          $Res Function(_$HomeScreenDataImpl) then) =
      __$$HomeScreenDataImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {List<MediaItem> continueWatching,
      List<MediaItem> recentMovies,
      List<MediaItem> recentShows});
}

/// @nodoc
class __$$HomeScreenDataImplCopyWithImpl<$Res>
    extends _$HomeScreenDataCopyWithImpl<$Res, _$HomeScreenDataImpl>
    implements _$$HomeScreenDataImplCopyWith<$Res> {
  __$$HomeScreenDataImplCopyWithImpl(
      _$HomeScreenDataImpl _value, $Res Function(_$HomeScreenDataImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? continueWatching = null,
    Object? recentMovies = null,
    Object? recentShows = null,
  }) {
    return _then(_$HomeScreenDataImpl(
      continueWatching: null == continueWatching
          ? _value._continueWatching
          : continueWatching // ignore: cast_nullable_to_non_nullable
              as List<MediaItem>,
      recentMovies: null == recentMovies
          ? _value._recentMovies
          : recentMovies // ignore: cast_nullable_to_non_nullable
              as List<MediaItem>,
      recentShows: null == recentShows
          ? _value._recentShows
          : recentShows // ignore: cast_nullable_to_non_nullable
              as List<MediaItem>,
    ));
  }
}

/// @nodoc

class _$HomeScreenDataImpl
    with DiagnosticableTreeMixin
    implements _HomeScreenData {
  _$HomeScreenDataImpl(
      {required final List<MediaItem> continueWatching,
      required final List<MediaItem> recentMovies,
      required final List<MediaItem> recentShows})
      : _continueWatching = continueWatching,
        _recentMovies = recentMovies,
        _recentShows = recentShows;

  final List<MediaItem> _continueWatching;
  @override
  List<MediaItem> get continueWatching {
    if (_continueWatching is EqualUnmodifiableListView)
      return _continueWatching;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_continueWatching);
  }

  final List<MediaItem> _recentMovies;
  @override
  List<MediaItem> get recentMovies {
    if (_recentMovies is EqualUnmodifiableListView) return _recentMovies;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_recentMovies);
  }

  final List<MediaItem> _recentShows;
  @override
  List<MediaItem> get recentShows {
    if (_recentShows is EqualUnmodifiableListView) return _recentShows;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_recentShows);
  }

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'HomeScreenData(continueWatching: $continueWatching, recentMovies: $recentMovies, recentShows: $recentShows)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'HomeScreenData'))
      ..add(DiagnosticsProperty('continueWatching', continueWatching))
      ..add(DiagnosticsProperty('recentMovies', recentMovies))
      ..add(DiagnosticsProperty('recentShows', recentShows));
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$HomeScreenDataImpl &&
            const DeepCollectionEquality()
                .equals(other._continueWatching, _continueWatching) &&
            const DeepCollectionEquality()
                .equals(other._recentMovies, _recentMovies) &&
            const DeepCollectionEquality()
                .equals(other._recentShows, _recentShows));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType,
      const DeepCollectionEquality().hash(_continueWatching),
      const DeepCollectionEquality().hash(_recentMovies),
      const DeepCollectionEquality().hash(_recentShows));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$HomeScreenDataImplCopyWith<_$HomeScreenDataImpl> get copyWith =>
      __$$HomeScreenDataImplCopyWithImpl<_$HomeScreenDataImpl>(
          this, _$identity);
}

abstract class _HomeScreenData implements HomeScreenData {
  factory _HomeScreenData(
      {required final List<MediaItem> continueWatching,
      required final List<MediaItem> recentMovies,
      required final List<MediaItem> recentShows}) = _$HomeScreenDataImpl;

  @override
  List<MediaItem> get continueWatching;
  @override
  List<MediaItem> get recentMovies;
  @override
  List<MediaItem> get recentShows;
  @override
  @JsonKey(ignore: true)
  _$$HomeScreenDataImplCopyWith<_$HomeScreenDataImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
