// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'media_route_chooser_controller.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$MediaRouteChooserState {

 List<MediaRoute> get routes; String? get requestedId; bool get isConnected;
/// Create a copy of MediaRouteChooserState
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$MediaRouteChooserStateCopyWith<MediaRouteChooserState> get copyWith => _$MediaRouteChooserStateCopyWithImpl<MediaRouteChooserState>(this as MediaRouteChooserState, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is MediaRouteChooserState&&const DeepCollectionEquality().equals(other.routes, routes)&&(identical(other.requestedId, requestedId) || other.requestedId == requestedId)&&(identical(other.isConnected, isConnected) || other.isConnected == isConnected));
}


@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(routes),requestedId,isConnected);

@override
String toString() {
  return 'MediaRouteChooserState(routes: $routes, requestedId: $requestedId, isConnected: $isConnected)';
}


}

/// @nodoc
abstract mixin class $MediaRouteChooserStateCopyWith<$Res>  {
  factory $MediaRouteChooserStateCopyWith(MediaRouteChooserState value, $Res Function(MediaRouteChooserState) _then) = _$MediaRouteChooserStateCopyWithImpl;
@useResult
$Res call({
 List<MediaRoute> routes, String? requestedId, bool isConnected
});




}
/// @nodoc
class _$MediaRouteChooserStateCopyWithImpl<$Res>
    implements $MediaRouteChooserStateCopyWith<$Res> {
  _$MediaRouteChooserStateCopyWithImpl(this._self, this._then);

  final MediaRouteChooserState _self;
  final $Res Function(MediaRouteChooserState) _then;

/// Create a copy of MediaRouteChooserState
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? routes = null,Object? requestedId = freezed,Object? isConnected = null,}) {
  return _then(_self.copyWith(
routes: null == routes ? _self.routes : routes // ignore: cast_nullable_to_non_nullable
as List<MediaRoute>,requestedId: freezed == requestedId ? _self.requestedId : requestedId // ignore: cast_nullable_to_non_nullable
as String?,isConnected: null == isConnected ? _self.isConnected : isConnected // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}

}


/// Adds pattern-matching-related methods to [MediaRouteChooserState].
extension MediaRouteChooserStatePatterns on MediaRouteChooserState {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _MediaRouteChooserState value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _MediaRouteChooserState() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _MediaRouteChooserState value)  $default,){
final _that = this;
switch (_that) {
case _MediaRouteChooserState():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _MediaRouteChooserState value)?  $default,){
final _that = this;
switch (_that) {
case _MediaRouteChooserState() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( List<MediaRoute> routes,  String? requestedId,  bool isConnected)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _MediaRouteChooserState() when $default != null:
return $default(_that.routes,_that.requestedId,_that.isConnected);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( List<MediaRoute> routes,  String? requestedId,  bool isConnected)  $default,) {final _that = this;
switch (_that) {
case _MediaRouteChooserState():
return $default(_that.routes,_that.requestedId,_that.isConnected);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( List<MediaRoute> routes,  String? requestedId,  bool isConnected)?  $default,) {final _that = this;
switch (_that) {
case _MediaRouteChooserState() when $default != null:
return $default(_that.routes,_that.requestedId,_that.isConnected);case _:
  return null;

}
}

}

/// @nodoc


class _MediaRouteChooserState implements MediaRouteChooserState {
   _MediaRouteChooserState({required final  List<MediaRoute> routes, required this.requestedId, required this.isConnected}): _routes = routes;
  

 final  List<MediaRoute> _routes;
@override List<MediaRoute> get routes {
  if (_routes is EqualUnmodifiableListView) return _routes;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_routes);
}

@override final  String? requestedId;
@override final  bool isConnected;

/// Create a copy of MediaRouteChooserState
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$MediaRouteChooserStateCopyWith<_MediaRouteChooserState> get copyWith => __$MediaRouteChooserStateCopyWithImpl<_MediaRouteChooserState>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _MediaRouteChooserState&&const DeepCollectionEquality().equals(other._routes, _routes)&&(identical(other.requestedId, requestedId) || other.requestedId == requestedId)&&(identical(other.isConnected, isConnected) || other.isConnected == isConnected));
}


@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(_routes),requestedId,isConnected);

@override
String toString() {
  return 'MediaRouteChooserState(routes: $routes, requestedId: $requestedId, isConnected: $isConnected)';
}


}

/// @nodoc
abstract mixin class _$MediaRouteChooserStateCopyWith<$Res> implements $MediaRouteChooserStateCopyWith<$Res> {
  factory _$MediaRouteChooserStateCopyWith(_MediaRouteChooserState value, $Res Function(_MediaRouteChooserState) _then) = __$MediaRouteChooserStateCopyWithImpl;
@override @useResult
$Res call({
 List<MediaRoute> routes, String? requestedId, bool isConnected
});




}
/// @nodoc
class __$MediaRouteChooserStateCopyWithImpl<$Res>
    implements _$MediaRouteChooserStateCopyWith<$Res> {
  __$MediaRouteChooserStateCopyWithImpl(this._self, this._then);

  final _MediaRouteChooserState _self;
  final $Res Function(_MediaRouteChooserState) _then;

/// Create a copy of MediaRouteChooserState
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? routes = null,Object? requestedId = freezed,Object? isConnected = null,}) {
  return _then(_MediaRouteChooserState(
routes: null == routes ? _self._routes : routes // ignore: cast_nullable_to_non_nullable
as List<MediaRoute>,requestedId: freezed == requestedId ? _self.requestedId : requestedId // ignore: cast_nullable_to_non_nullable
as String?,isConnected: null == isConnected ? _self.isConnected : isConnected // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}


}

// dart format on
