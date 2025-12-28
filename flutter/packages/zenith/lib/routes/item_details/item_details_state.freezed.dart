// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'item_details_state.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$ItemDetailsState {

 MediaItem get item; ImageId? get poster; ImageId? get backdrop; List<EpisodeGroupState> get seasons; PlayableState? get playable; bool get isWatched; String? get durationText; DownloadedFile? get downloadedFile;
/// Create a copy of ItemDetailsState
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ItemDetailsStateCopyWith<ItemDetailsState> get copyWith => _$ItemDetailsStateCopyWithImpl<ItemDetailsState>(this as ItemDetailsState, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ItemDetailsState&&(identical(other.item, item) || other.item == item)&&(identical(other.poster, poster) || other.poster == poster)&&(identical(other.backdrop, backdrop) || other.backdrop == backdrop)&&const DeepCollectionEquality().equals(other.seasons, seasons)&&(identical(other.playable, playable) || other.playable == playable)&&(identical(other.isWatched, isWatched) || other.isWatched == isWatched)&&(identical(other.durationText, durationText) || other.durationText == durationText)&&const DeepCollectionEquality().equals(other.downloadedFile, downloadedFile));
}


@override
int get hashCode => Object.hash(runtimeType,item,poster,backdrop,const DeepCollectionEquality().hash(seasons),playable,isWatched,durationText,const DeepCollectionEquality().hash(downloadedFile));

@override
String toString() {
  return 'ItemDetailsState(item: $item, poster: $poster, backdrop: $backdrop, seasons: $seasons, playable: $playable, isWatched: $isWatched, durationText: $durationText, downloadedFile: $downloadedFile)';
}


}

/// @nodoc
abstract mixin class $ItemDetailsStateCopyWith<$Res>  {
  factory $ItemDetailsStateCopyWith(ItemDetailsState value, $Res Function(ItemDetailsState) _then) = _$ItemDetailsStateCopyWithImpl;
@useResult
$Res call({
 MediaItem item, ImageId? poster, ImageId? backdrop, List<EpisodeGroupState> seasons, PlayableState? playable, bool isWatched, String? durationText, DownloadedFile? downloadedFile
});


$PlayableStateCopyWith<$Res>? get playable;

}
/// @nodoc
class _$ItemDetailsStateCopyWithImpl<$Res>
    implements $ItemDetailsStateCopyWith<$Res> {
  _$ItemDetailsStateCopyWithImpl(this._self, this._then);

  final ItemDetailsState _self;
  final $Res Function(ItemDetailsState) _then;

/// Create a copy of ItemDetailsState
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? item = null,Object? poster = freezed,Object? backdrop = freezed,Object? seasons = null,Object? playable = freezed,Object? isWatched = null,Object? durationText = freezed,Object? downloadedFile = freezed,}) {
  return _then(_self.copyWith(
item: null == item ? _self.item : item // ignore: cast_nullable_to_non_nullable
as MediaItem,poster: freezed == poster ? _self.poster : poster // ignore: cast_nullable_to_non_nullable
as ImageId?,backdrop: freezed == backdrop ? _self.backdrop : backdrop // ignore: cast_nullable_to_non_nullable
as ImageId?,seasons: null == seasons ? _self.seasons : seasons // ignore: cast_nullable_to_non_nullable
as List<EpisodeGroupState>,playable: freezed == playable ? _self.playable : playable // ignore: cast_nullable_to_non_nullable
as PlayableState?,isWatched: null == isWatched ? _self.isWatched : isWatched // ignore: cast_nullable_to_non_nullable
as bool,durationText: freezed == durationText ? _self.durationText : durationText // ignore: cast_nullable_to_non_nullable
as String?,downloadedFile: freezed == downloadedFile ? _self.downloadedFile : downloadedFile // ignore: cast_nullable_to_non_nullable
as DownloadedFile?,
  ));
}
/// Create a copy of ItemDetailsState
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$PlayableStateCopyWith<$Res>? get playable {
    if (_self.playable == null) {
    return null;
  }

  return $PlayableStateCopyWith<$Res>(_self.playable!, (value) {
    return _then(_self.copyWith(playable: value));
  });
}
}


/// Adds pattern-matching-related methods to [ItemDetailsState].
extension ItemDetailsStatePatterns on ItemDetailsState {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _ItemDetailsState value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _ItemDetailsState() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _ItemDetailsState value)  $default,){
final _that = this;
switch (_that) {
case _ItemDetailsState():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _ItemDetailsState value)?  $default,){
final _that = this;
switch (_that) {
case _ItemDetailsState() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( MediaItem item,  ImageId? poster,  ImageId? backdrop,  List<EpisodeGroupState> seasons,  PlayableState? playable,  bool isWatched,  String? durationText,  DownloadedFile? downloadedFile)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _ItemDetailsState() when $default != null:
return $default(_that.item,_that.poster,_that.backdrop,_that.seasons,_that.playable,_that.isWatched,_that.durationText,_that.downloadedFile);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( MediaItem item,  ImageId? poster,  ImageId? backdrop,  List<EpisodeGroupState> seasons,  PlayableState? playable,  bool isWatched,  String? durationText,  DownloadedFile? downloadedFile)  $default,) {final _that = this;
switch (_that) {
case _ItemDetailsState():
return $default(_that.item,_that.poster,_that.backdrop,_that.seasons,_that.playable,_that.isWatched,_that.durationText,_that.downloadedFile);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( MediaItem item,  ImageId? poster,  ImageId? backdrop,  List<EpisodeGroupState> seasons,  PlayableState? playable,  bool isWatched,  String? durationText,  DownloadedFile? downloadedFile)?  $default,) {final _that = this;
switch (_that) {
case _ItemDetailsState() when $default != null:
return $default(_that.item,_that.poster,_that.backdrop,_that.seasons,_that.playable,_that.isWatched,_that.durationText,_that.downloadedFile);case _:
  return null;

}
}

}

/// @nodoc


class _ItemDetailsState implements ItemDetailsState {
   _ItemDetailsState({required this.item, required this.poster, required this.backdrop, required final  List<EpisodeGroupState> seasons, required this.playable, required this.isWatched, required this.durationText, required this.downloadedFile}): _seasons = seasons;
  

@override final  MediaItem item;
@override final  ImageId? poster;
@override final  ImageId? backdrop;
 final  List<EpisodeGroupState> _seasons;
@override List<EpisodeGroupState> get seasons {
  if (_seasons is EqualUnmodifiableListView) return _seasons;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_seasons);
}

@override final  PlayableState? playable;
@override final  bool isWatched;
@override final  String? durationText;
@override final  DownloadedFile? downloadedFile;

/// Create a copy of ItemDetailsState
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$ItemDetailsStateCopyWith<_ItemDetailsState> get copyWith => __$ItemDetailsStateCopyWithImpl<_ItemDetailsState>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _ItemDetailsState&&(identical(other.item, item) || other.item == item)&&(identical(other.poster, poster) || other.poster == poster)&&(identical(other.backdrop, backdrop) || other.backdrop == backdrop)&&const DeepCollectionEquality().equals(other._seasons, _seasons)&&(identical(other.playable, playable) || other.playable == playable)&&(identical(other.isWatched, isWatched) || other.isWatched == isWatched)&&(identical(other.durationText, durationText) || other.durationText == durationText)&&const DeepCollectionEquality().equals(other.downloadedFile, downloadedFile));
}


@override
int get hashCode => Object.hash(runtimeType,item,poster,backdrop,const DeepCollectionEquality().hash(_seasons),playable,isWatched,durationText,const DeepCollectionEquality().hash(downloadedFile));

@override
String toString() {
  return 'ItemDetailsState(item: $item, poster: $poster, backdrop: $backdrop, seasons: $seasons, playable: $playable, isWatched: $isWatched, durationText: $durationText, downloadedFile: $downloadedFile)';
}


}

/// @nodoc
abstract mixin class _$ItemDetailsStateCopyWith<$Res> implements $ItemDetailsStateCopyWith<$Res> {
  factory _$ItemDetailsStateCopyWith(_ItemDetailsState value, $Res Function(_ItemDetailsState) _then) = __$ItemDetailsStateCopyWithImpl;
@override @useResult
$Res call({
 MediaItem item, ImageId? poster, ImageId? backdrop, List<EpisodeGroupState> seasons, PlayableState? playable, bool isWatched, String? durationText, DownloadedFile? downloadedFile
});


@override $PlayableStateCopyWith<$Res>? get playable;

}
/// @nodoc
class __$ItemDetailsStateCopyWithImpl<$Res>
    implements _$ItemDetailsStateCopyWith<$Res> {
  __$ItemDetailsStateCopyWithImpl(this._self, this._then);

  final _ItemDetailsState _self;
  final $Res Function(_ItemDetailsState) _then;

/// Create a copy of ItemDetailsState
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? item = null,Object? poster = freezed,Object? backdrop = freezed,Object? seasons = null,Object? playable = freezed,Object? isWatched = null,Object? durationText = freezed,Object? downloadedFile = freezed,}) {
  return _then(_ItemDetailsState(
item: null == item ? _self.item : item // ignore: cast_nullable_to_non_nullable
as MediaItem,poster: freezed == poster ? _self.poster : poster // ignore: cast_nullable_to_non_nullable
as ImageId?,backdrop: freezed == backdrop ? _self.backdrop : backdrop // ignore: cast_nullable_to_non_nullable
as ImageId?,seasons: null == seasons ? _self._seasons : seasons // ignore: cast_nullable_to_non_nullable
as List<EpisodeGroupState>,playable: freezed == playable ? _self.playable : playable // ignore: cast_nullable_to_non_nullable
as PlayableState?,isWatched: null == isWatched ? _self.isWatched : isWatched // ignore: cast_nullable_to_non_nullable
as bool,durationText: freezed == durationText ? _self.durationText : durationText // ignore: cast_nullable_to_non_nullable
as String?,downloadedFile: freezed == downloadedFile ? _self.downloadedFile : downloadedFile // ignore: cast_nullable_to_non_nullable
as DownloadedFile?,
  ));
}

/// Create a copy of ItemDetailsState
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$PlayableStateCopyWith<$Res>? get playable {
    if (_self.playable == null) {
    return null;
  }

  return $PlayableStateCopyWith<$Res>(_self.playable!, (value) {
    return _then(_self.copyWith(playable: value));
  });
}
}

/// @nodoc
mixin _$PlayableState {

 int get id; int? get seasonIndex; double? get progress; String? get caption; bool get shouldResume; double get playPosition;
/// Create a copy of PlayableState
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PlayableStateCopyWith<PlayableState> get copyWith => _$PlayableStateCopyWithImpl<PlayableState>(this as PlayableState, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PlayableState&&(identical(other.id, id) || other.id == id)&&(identical(other.seasonIndex, seasonIndex) || other.seasonIndex == seasonIndex)&&(identical(other.progress, progress) || other.progress == progress)&&(identical(other.caption, caption) || other.caption == caption)&&(identical(other.shouldResume, shouldResume) || other.shouldResume == shouldResume)&&(identical(other.playPosition, playPosition) || other.playPosition == playPosition));
}


@override
int get hashCode => Object.hash(runtimeType,id,seasonIndex,progress,caption,shouldResume,playPosition);

@override
String toString() {
  return 'PlayableState(id: $id, seasonIndex: $seasonIndex, progress: $progress, caption: $caption, shouldResume: $shouldResume, playPosition: $playPosition)';
}


}

/// @nodoc
abstract mixin class $PlayableStateCopyWith<$Res>  {
  factory $PlayableStateCopyWith(PlayableState value, $Res Function(PlayableState) _then) = _$PlayableStateCopyWithImpl;
@useResult
$Res call({
 int id, int? seasonIndex, double? progress, String? caption, bool shouldResume, double playPosition
});




}
/// @nodoc
class _$PlayableStateCopyWithImpl<$Res>
    implements $PlayableStateCopyWith<$Res> {
  _$PlayableStateCopyWithImpl(this._self, this._then);

  final PlayableState _self;
  final $Res Function(PlayableState) _then;

/// Create a copy of PlayableState
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? id = null,Object? seasonIndex = freezed,Object? progress = freezed,Object? caption = freezed,Object? shouldResume = null,Object? playPosition = null,}) {
  return _then(_self.copyWith(
id: null == id ? _self.id : id // ignore: cast_nullable_to_non_nullable
as int,seasonIndex: freezed == seasonIndex ? _self.seasonIndex : seasonIndex // ignore: cast_nullable_to_non_nullable
as int?,progress: freezed == progress ? _self.progress : progress // ignore: cast_nullable_to_non_nullable
as double?,caption: freezed == caption ? _self.caption : caption // ignore: cast_nullable_to_non_nullable
as String?,shouldResume: null == shouldResume ? _self.shouldResume : shouldResume // ignore: cast_nullable_to_non_nullable
as bool,playPosition: null == playPosition ? _self.playPosition : playPosition // ignore: cast_nullable_to_non_nullable
as double,
  ));
}

}


/// Adds pattern-matching-related methods to [PlayableState].
extension PlayableStatePatterns on PlayableState {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _PlayableState value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _PlayableState() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _PlayableState value)  $default,){
final _that = this;
switch (_that) {
case _PlayableState():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _PlayableState value)?  $default,){
final _that = this;
switch (_that) {
case _PlayableState() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( int id,  int? seasonIndex,  double? progress,  String? caption,  bool shouldResume,  double playPosition)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _PlayableState() when $default != null:
return $default(_that.id,_that.seasonIndex,_that.progress,_that.caption,_that.shouldResume,_that.playPosition);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( int id,  int? seasonIndex,  double? progress,  String? caption,  bool shouldResume,  double playPosition)  $default,) {final _that = this;
switch (_that) {
case _PlayableState():
return $default(_that.id,_that.seasonIndex,_that.progress,_that.caption,_that.shouldResume,_that.playPosition);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( int id,  int? seasonIndex,  double? progress,  String? caption,  bool shouldResume,  double playPosition)?  $default,) {final _that = this;
switch (_that) {
case _PlayableState() when $default != null:
return $default(_that.id,_that.seasonIndex,_that.progress,_that.caption,_that.shouldResume,_that.playPosition);case _:
  return null;

}
}

}

/// @nodoc


class _PlayableState implements PlayableState {
   _PlayableState({required this.id, required this.seasonIndex, required this.progress, required this.caption, required this.shouldResume, required this.playPosition});
  

@override final  int id;
@override final  int? seasonIndex;
@override final  double? progress;
@override final  String? caption;
@override final  bool shouldResume;
@override final  double playPosition;

/// Create a copy of PlayableState
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$PlayableStateCopyWith<_PlayableState> get copyWith => __$PlayableStateCopyWithImpl<_PlayableState>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _PlayableState&&(identical(other.id, id) || other.id == id)&&(identical(other.seasonIndex, seasonIndex) || other.seasonIndex == seasonIndex)&&(identical(other.progress, progress) || other.progress == progress)&&(identical(other.caption, caption) || other.caption == caption)&&(identical(other.shouldResume, shouldResume) || other.shouldResume == shouldResume)&&(identical(other.playPosition, playPosition) || other.playPosition == playPosition));
}


@override
int get hashCode => Object.hash(runtimeType,id,seasonIndex,progress,caption,shouldResume,playPosition);

@override
String toString() {
  return 'PlayableState(id: $id, seasonIndex: $seasonIndex, progress: $progress, caption: $caption, shouldResume: $shouldResume, playPosition: $playPosition)';
}


}

/// @nodoc
abstract mixin class _$PlayableStateCopyWith<$Res> implements $PlayableStateCopyWith<$Res> {
  factory _$PlayableStateCopyWith(_PlayableState value, $Res Function(_PlayableState) _then) = __$PlayableStateCopyWithImpl;
@override @useResult
$Res call({
 int id, int? seasonIndex, double? progress, String? caption, bool shouldResume, double playPosition
});




}
/// @nodoc
class __$PlayableStateCopyWithImpl<$Res>
    implements _$PlayableStateCopyWith<$Res> {
  __$PlayableStateCopyWithImpl(this._self, this._then);

  final _PlayableState _self;
  final $Res Function(_PlayableState) _then;

/// Create a copy of PlayableState
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? id = null,Object? seasonIndex = freezed,Object? progress = freezed,Object? caption = freezed,Object? shouldResume = null,Object? playPosition = null,}) {
  return _then(_PlayableState(
id: null == id ? _self.id : id // ignore: cast_nullable_to_non_nullable
as int,seasonIndex: freezed == seasonIndex ? _self.seasonIndex : seasonIndex // ignore: cast_nullable_to_non_nullable
as int?,progress: freezed == progress ? _self.progress : progress // ignore: cast_nullable_to_non_nullable
as double?,caption: freezed == caption ? _self.caption : caption // ignore: cast_nullable_to_non_nullable
as String?,shouldResume: null == shouldResume ? _self.shouldResume : shouldResume // ignore: cast_nullable_to_non_nullable
as bool,playPosition: null == playPosition ? _self.playPosition : playPosition // ignore: cast_nullable_to_non_nullable
as double,
  ));
}


}

/// @nodoc
mixin _$EpisodeGroupState {

 String get name; List<EpisodeState> get episodes;
/// Create a copy of EpisodeGroupState
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$EpisodeGroupStateCopyWith<EpisodeGroupState> get copyWith => _$EpisodeGroupStateCopyWithImpl<EpisodeGroupState>(this as EpisodeGroupState, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is EpisodeGroupState&&(identical(other.name, name) || other.name == name)&&const DeepCollectionEquality().equals(other.episodes, episodes));
}


@override
int get hashCode => Object.hash(runtimeType,name,const DeepCollectionEquality().hash(episodes));

@override
String toString() {
  return 'EpisodeGroupState(name: $name, episodes: $episodes)';
}


}

/// @nodoc
abstract mixin class $EpisodeGroupStateCopyWith<$Res>  {
  factory $EpisodeGroupStateCopyWith(EpisodeGroupState value, $Res Function(EpisodeGroupState) _then) = _$EpisodeGroupStateCopyWithImpl;
@useResult
$Res call({
 String name, List<EpisodeState> episodes
});




}
/// @nodoc
class _$EpisodeGroupStateCopyWithImpl<$Res>
    implements $EpisodeGroupStateCopyWith<$Res> {
  _$EpisodeGroupStateCopyWithImpl(this._self, this._then);

  final EpisodeGroupState _self;
  final $Res Function(EpisodeGroupState) _then;

/// Create a copy of EpisodeGroupState
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? name = null,Object? episodes = null,}) {
  return _then(_self.copyWith(
name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,episodes: null == episodes ? _self.episodes : episodes // ignore: cast_nullable_to_non_nullable
as List<EpisodeState>,
  ));
}

}


/// Adds pattern-matching-related methods to [EpisodeGroupState].
extension EpisodeGroupStatePatterns on EpisodeGroupState {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _EpisodeGroupState value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _EpisodeGroupState() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _EpisodeGroupState value)  $default,){
final _that = this;
switch (_that) {
case _EpisodeGroupState():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _EpisodeGroupState value)?  $default,){
final _that = this;
switch (_that) {
case _EpisodeGroupState() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String name,  List<EpisodeState> episodes)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _EpisodeGroupState() when $default != null:
return $default(_that.name,_that.episodes);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String name,  List<EpisodeState> episodes)  $default,) {final _that = this;
switch (_that) {
case _EpisodeGroupState():
return $default(_that.name,_that.episodes);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String name,  List<EpisodeState> episodes)?  $default,) {final _that = this;
switch (_that) {
case _EpisodeGroupState() when $default != null:
return $default(_that.name,_that.episodes);case _:
  return null;

}
}

}

/// @nodoc


class _EpisodeGroupState implements EpisodeGroupState {
   _EpisodeGroupState({required this.name, required final  List<EpisodeState> episodes}): _episodes = episodes;
  

@override final  String name;
 final  List<EpisodeState> _episodes;
@override List<EpisodeState> get episodes {
  if (_episodes is EqualUnmodifiableListView) return _episodes;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_episodes);
}


/// Create a copy of EpisodeGroupState
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$EpisodeGroupStateCopyWith<_EpisodeGroupState> get copyWith => __$EpisodeGroupStateCopyWithImpl<_EpisodeGroupState>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _EpisodeGroupState&&(identical(other.name, name) || other.name == name)&&const DeepCollectionEquality().equals(other._episodes, _episodes));
}


@override
int get hashCode => Object.hash(runtimeType,name,const DeepCollectionEquality().hash(_episodes));

@override
String toString() {
  return 'EpisodeGroupState(name: $name, episodes: $episodes)';
}


}

/// @nodoc
abstract mixin class _$EpisodeGroupStateCopyWith<$Res> implements $EpisodeGroupStateCopyWith<$Res> {
  factory _$EpisodeGroupStateCopyWith(_EpisodeGroupState value, $Res Function(_EpisodeGroupState) _then) = __$EpisodeGroupStateCopyWithImpl;
@override @useResult
$Res call({
 String name, List<EpisodeState> episodes
});




}
/// @nodoc
class __$EpisodeGroupStateCopyWithImpl<$Res>
    implements _$EpisodeGroupStateCopyWith<$Res> {
  __$EpisodeGroupStateCopyWithImpl(this._self, this._then);

  final _EpisodeGroupState _self;
  final $Res Function(_EpisodeGroupState) _then;

/// Create a copy of EpisodeGroupState
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? name = null,Object? episodes = null,}) {
  return _then(_EpisodeGroupState(
name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,episodes: null == episodes ? _self._episodes : episodes // ignore: cast_nullable_to_non_nullable
as List<EpisodeState>,
  ));
}


}

/// @nodoc
mixin _$EpisodeState {

 int get id; ImageId? get thumbnail; String? get overview; bool get isWatched; String get title;
/// Create a copy of EpisodeState
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$EpisodeStateCopyWith<EpisodeState> get copyWith => _$EpisodeStateCopyWithImpl<EpisodeState>(this as EpisodeState, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is EpisodeState&&(identical(other.id, id) || other.id == id)&&(identical(other.thumbnail, thumbnail) || other.thumbnail == thumbnail)&&(identical(other.overview, overview) || other.overview == overview)&&(identical(other.isWatched, isWatched) || other.isWatched == isWatched)&&(identical(other.title, title) || other.title == title));
}


@override
int get hashCode => Object.hash(runtimeType,id,thumbnail,overview,isWatched,title);

@override
String toString() {
  return 'EpisodeState(id: $id, thumbnail: $thumbnail, overview: $overview, isWatched: $isWatched, title: $title)';
}


}

/// @nodoc
abstract mixin class $EpisodeStateCopyWith<$Res>  {
  factory $EpisodeStateCopyWith(EpisodeState value, $Res Function(EpisodeState) _then) = _$EpisodeStateCopyWithImpl;
@useResult
$Res call({
 int id, ImageId? thumbnail, String? overview, bool isWatched, String title
});




}
/// @nodoc
class _$EpisodeStateCopyWithImpl<$Res>
    implements $EpisodeStateCopyWith<$Res> {
  _$EpisodeStateCopyWithImpl(this._self, this._then);

  final EpisodeState _self;
  final $Res Function(EpisodeState) _then;

/// Create a copy of EpisodeState
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? id = null,Object? thumbnail = freezed,Object? overview = freezed,Object? isWatched = null,Object? title = null,}) {
  return _then(_self.copyWith(
id: null == id ? _self.id : id // ignore: cast_nullable_to_non_nullable
as int,thumbnail: freezed == thumbnail ? _self.thumbnail : thumbnail // ignore: cast_nullable_to_non_nullable
as ImageId?,overview: freezed == overview ? _self.overview : overview // ignore: cast_nullable_to_non_nullable
as String?,isWatched: null == isWatched ? _self.isWatched : isWatched // ignore: cast_nullable_to_non_nullable
as bool,title: null == title ? _self.title : title // ignore: cast_nullable_to_non_nullable
as String,
  ));
}

}


/// Adds pattern-matching-related methods to [EpisodeState].
extension EpisodeStatePatterns on EpisodeState {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _EpisodeState value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _EpisodeState() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _EpisodeState value)  $default,){
final _that = this;
switch (_that) {
case _EpisodeState():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _EpisodeState value)?  $default,){
final _that = this;
switch (_that) {
case _EpisodeState() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( int id,  ImageId? thumbnail,  String? overview,  bool isWatched,  String title)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _EpisodeState() when $default != null:
return $default(_that.id,_that.thumbnail,_that.overview,_that.isWatched,_that.title);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( int id,  ImageId? thumbnail,  String? overview,  bool isWatched,  String title)  $default,) {final _that = this;
switch (_that) {
case _EpisodeState():
return $default(_that.id,_that.thumbnail,_that.overview,_that.isWatched,_that.title);case _:
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( int id,  ImageId? thumbnail,  String? overview,  bool isWatched,  String title)?  $default,) {final _that = this;
switch (_that) {
case _EpisodeState() when $default != null:
return $default(_that.id,_that.thumbnail,_that.overview,_that.isWatched,_that.title);case _:
  return null;

}
}

}

/// @nodoc


class _EpisodeState implements EpisodeState {
   _EpisodeState({required this.id, required this.thumbnail, required this.overview, required this.isWatched, required this.title});
  

@override final  int id;
@override final  ImageId? thumbnail;
@override final  String? overview;
@override final  bool isWatched;
@override final  String title;

/// Create a copy of EpisodeState
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$EpisodeStateCopyWith<_EpisodeState> get copyWith => __$EpisodeStateCopyWithImpl<_EpisodeState>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _EpisodeState&&(identical(other.id, id) || other.id == id)&&(identical(other.thumbnail, thumbnail) || other.thumbnail == thumbnail)&&(identical(other.overview, overview) || other.overview == overview)&&(identical(other.isWatched, isWatched) || other.isWatched == isWatched)&&(identical(other.title, title) || other.title == title));
}


@override
int get hashCode => Object.hash(runtimeType,id,thumbnail,overview,isWatched,title);

@override
String toString() {
  return 'EpisodeState(id: $id, thumbnail: $thumbnail, overview: $overview, isWatched: $isWatched, title: $title)';
}


}

/// @nodoc
abstract mixin class _$EpisodeStateCopyWith<$Res> implements $EpisodeStateCopyWith<$Res> {
  factory _$EpisodeStateCopyWith(_EpisodeState value, $Res Function(_EpisodeState) _then) = __$EpisodeStateCopyWithImpl;
@override @useResult
$Res call({
 int id, ImageId? thumbnail, String? overview, bool isWatched, String title
});




}
/// @nodoc
class __$EpisodeStateCopyWithImpl<$Res>
    implements _$EpisodeStateCopyWith<$Res> {
  __$EpisodeStateCopyWithImpl(this._self, this._then);

  final _EpisodeState _self;
  final $Res Function(_EpisodeState) _then;

/// Create a copy of EpisodeState
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? id = null,Object? thumbnail = freezed,Object? overview = freezed,Object? isWatched = null,Object? title = null,}) {
  return _then(_EpisodeState(
id: null == id ? _self.id : id // ignore: cast_nullable_to_non_nullable
as int,thumbnail: freezed == thumbnail ? _self.thumbnail : thumbnail // ignore: cast_nullable_to_non_nullable
as ImageId?,overview: freezed == overview ? _self.overview : overview // ignore: cast_nullable_to_non_nullable
as String?,isWatched: null == isWatched ? _self.isWatched : isWatched // ignore: cast_nullable_to_non_nullable
as bool,title: null == title ? _self.title : title // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

// dart format on
