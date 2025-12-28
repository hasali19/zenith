// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'home.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$HomeScreenData implements DiagnosticableTreeMixin {

 List<MediaItem> get continueWatching; List<MediaItem> get recentMovies; List<MediaItem> get recentShows;
/// Create a copy of HomeScreenData
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$HomeScreenDataCopyWith<HomeScreenData> get copyWith => _$HomeScreenDataCopyWithImpl<HomeScreenData>(this as HomeScreenData, _$identity);


@override
void debugFillProperties(DiagnosticPropertiesBuilder properties) {
  properties
    ..add(DiagnosticsProperty('type', 'HomeScreenData'))
    ..add(DiagnosticsProperty('continueWatching', continueWatching))..add(DiagnosticsProperty('recentMovies', recentMovies))..add(DiagnosticsProperty('recentShows', recentShows));
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is HomeScreenData&&const DeepCollectionEquality().equals(other.continueWatching, continueWatching)&&const DeepCollectionEquality().equals(other.recentMovies, recentMovies)&&const DeepCollectionEquality().equals(other.recentShows, recentShows));
}


@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(continueWatching),const DeepCollectionEquality().hash(recentMovies),const DeepCollectionEquality().hash(recentShows));

@override
String toString({ DiagnosticLevel minLevel = DiagnosticLevel.info }) {
  return 'HomeScreenData(continueWatching: $continueWatching, recentMovies: $recentMovies, recentShows: $recentShows)';
}


}

/// @nodoc
abstract mixin class $HomeScreenDataCopyWith<$Res>  {
  factory $HomeScreenDataCopyWith(HomeScreenData value, $Res Function(HomeScreenData) _then) = _$HomeScreenDataCopyWithImpl;
@useResult
$Res call({
 List<MediaItem> continueWatching, List<MediaItem> recentMovies, List<MediaItem> recentShows
});




}
/// @nodoc
class _$HomeScreenDataCopyWithImpl<$Res>
    implements $HomeScreenDataCopyWith<$Res> {
  _$HomeScreenDataCopyWithImpl(this._self, this._then);

  final HomeScreenData _self;
  final $Res Function(HomeScreenData) _then;

/// Create a copy of HomeScreenData
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? continueWatching = null,Object? recentMovies = null,Object? recentShows = null,}) {
  return _then(_self.copyWith(
continueWatching: null == continueWatching ? _self.continueWatching : continueWatching // ignore: cast_nullable_to_non_nullable
as List<MediaItem>,recentMovies: null == recentMovies ? _self.recentMovies : recentMovies // ignore: cast_nullable_to_non_nullable
as List<MediaItem>,recentShows: null == recentShows ? _self.recentShows : recentShows // ignore: cast_nullable_to_non_nullable
as List<MediaItem>,
  ));
}

}


/// Adds pattern-matching-related methods to [HomeScreenData].
extension HomeScreenDataPatterns on HomeScreenData {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _HomeScreenData value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _HomeScreenData() when $default != null:
return $default(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _HomeScreenData value)  $default,){
final _that = this;
switch (_that) {
case _HomeScreenData():
return $default(_that);case _:
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _HomeScreenData value)?  $default,){
final _that = this;
switch (_that) {
case _HomeScreenData() when $default != null:
return $default(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( List<MediaItem> continueWatching,  List<MediaItem> recentMovies,  List<MediaItem> recentShows)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _HomeScreenData() when $default != null:
return $default(_that.continueWatching,_that.recentMovies,_that.recentShows);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( List<MediaItem> continueWatching,  List<MediaItem> recentMovies,  List<MediaItem> recentShows)  $default,) {final _that = this;
switch (_that) {
case _HomeScreenData():
return $default(_that.continueWatching,_that.recentMovies,_that.recentShows);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( List<MediaItem> continueWatching,  List<MediaItem> recentMovies,  List<MediaItem> recentShows)?  $default,) {final _that = this;
switch (_that) {
case _HomeScreenData() when $default != null:
return $default(_that.continueWatching,_that.recentMovies,_that.recentShows);case _:
  return null;

}
}

}

/// @nodoc


class _HomeScreenData with DiagnosticableTreeMixin implements HomeScreenData {
   _HomeScreenData({required final  List<MediaItem> continueWatching, required final  List<MediaItem> recentMovies, required final  List<MediaItem> recentShows}): _continueWatching = continueWatching,_recentMovies = recentMovies,_recentShows = recentShows;
  

 final  List<MediaItem> _continueWatching;
@override List<MediaItem> get continueWatching {
  if (_continueWatching is EqualUnmodifiableListView) return _continueWatching;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_continueWatching);
}

 final  List<MediaItem> _recentMovies;
@override List<MediaItem> get recentMovies {
  if (_recentMovies is EqualUnmodifiableListView) return _recentMovies;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_recentMovies);
}

 final  List<MediaItem> _recentShows;
@override List<MediaItem> get recentShows {
  if (_recentShows is EqualUnmodifiableListView) return _recentShows;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_recentShows);
}


/// Create a copy of HomeScreenData
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$HomeScreenDataCopyWith<_HomeScreenData> get copyWith => __$HomeScreenDataCopyWithImpl<_HomeScreenData>(this, _$identity);


@override
void debugFillProperties(DiagnosticPropertiesBuilder properties) {
  properties
    ..add(DiagnosticsProperty('type', 'HomeScreenData'))
    ..add(DiagnosticsProperty('continueWatching', continueWatching))..add(DiagnosticsProperty('recentMovies', recentMovies))..add(DiagnosticsProperty('recentShows', recentShows));
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _HomeScreenData&&const DeepCollectionEquality().equals(other._continueWatching, _continueWatching)&&const DeepCollectionEquality().equals(other._recentMovies, _recentMovies)&&const DeepCollectionEquality().equals(other._recentShows, _recentShows));
}


@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(_continueWatching),const DeepCollectionEquality().hash(_recentMovies),const DeepCollectionEquality().hash(_recentShows));

@override
String toString({ DiagnosticLevel minLevel = DiagnosticLevel.info }) {
  return 'HomeScreenData(continueWatching: $continueWatching, recentMovies: $recentMovies, recentShows: $recentShows)';
}


}

/// @nodoc
abstract mixin class _$HomeScreenDataCopyWith<$Res> implements $HomeScreenDataCopyWith<$Res> {
  factory _$HomeScreenDataCopyWith(_HomeScreenData value, $Res Function(_HomeScreenData) _then) = __$HomeScreenDataCopyWithImpl;
@override @useResult
$Res call({
 List<MediaItem> continueWatching, List<MediaItem> recentMovies, List<MediaItem> recentShows
});




}
/// @nodoc
class __$HomeScreenDataCopyWithImpl<$Res>
    implements _$HomeScreenDataCopyWith<$Res> {
  __$HomeScreenDataCopyWithImpl(this._self, this._then);

  final _HomeScreenData _self;
  final $Res Function(_HomeScreenData) _then;

/// Create a copy of HomeScreenData
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? continueWatching = null,Object? recentMovies = null,Object? recentShows = null,}) {
  return _then(_HomeScreenData(
continueWatching: null == continueWatching ? _self._continueWatching : continueWatching // ignore: cast_nullable_to_non_nullable
as List<MediaItem>,recentMovies: null == recentMovies ? _self._recentMovies : recentMovies // ignore: cast_nullable_to_non_nullable
as List<MediaItem>,recentShows: null == recentShows ? _self._recentShows : recentShows // ignore: cast_nullable_to_non_nullable
as List<MediaItem>,
  ));
}


}

// dart format on
