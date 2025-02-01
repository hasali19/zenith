// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'item_details_state.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$ItemDetailsState {
  MediaItem get item => throw _privateConstructorUsedError;
  ImageId? get poster => throw _privateConstructorUsedError;
  ImageId? get backdrop => throw _privateConstructorUsedError;
  List<EpisodeGroupState> get seasons => throw _privateConstructorUsedError;
  PlayableState? get playable => throw _privateConstructorUsedError;
  bool get isWatched => throw _privateConstructorUsedError;
  String? get durationText => throw _privateConstructorUsedError;
  DownloadedFile? get downloadedFile => throw _privateConstructorUsedError;

  /// Create a copy of ItemDetailsState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $ItemDetailsStateCopyWith<ItemDetailsState> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ItemDetailsStateCopyWith<$Res> {
  factory $ItemDetailsStateCopyWith(
          ItemDetailsState value, $Res Function(ItemDetailsState) then) =
      _$ItemDetailsStateCopyWithImpl<$Res, ItemDetailsState>;
  @useResult
  $Res call(
      {MediaItem item,
      ImageId? poster,
      ImageId? backdrop,
      List<EpisodeGroupState> seasons,
      PlayableState? playable,
      bool isWatched,
      String? durationText,
      DownloadedFile? downloadedFile});

  $PlayableStateCopyWith<$Res>? get playable;
}

/// @nodoc
class _$ItemDetailsStateCopyWithImpl<$Res, $Val extends ItemDetailsState>
    implements $ItemDetailsStateCopyWith<$Res> {
  _$ItemDetailsStateCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of ItemDetailsState
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? item = null,
    Object? poster = freezed,
    Object? backdrop = freezed,
    Object? seasons = null,
    Object? playable = freezed,
    Object? isWatched = null,
    Object? durationText = freezed,
    Object? downloadedFile = freezed,
  }) {
    return _then(_value.copyWith(
      item: null == item
          ? _value.item
          : item // ignore: cast_nullable_to_non_nullable
              as MediaItem,
      poster: freezed == poster
          ? _value.poster
          : poster // ignore: cast_nullable_to_non_nullable
              as ImageId?,
      backdrop: freezed == backdrop
          ? _value.backdrop
          : backdrop // ignore: cast_nullable_to_non_nullable
              as ImageId?,
      seasons: null == seasons
          ? _value.seasons
          : seasons // ignore: cast_nullable_to_non_nullable
              as List<EpisodeGroupState>,
      playable: freezed == playable
          ? _value.playable
          : playable // ignore: cast_nullable_to_non_nullable
              as PlayableState?,
      isWatched: null == isWatched
          ? _value.isWatched
          : isWatched // ignore: cast_nullable_to_non_nullable
              as bool,
      durationText: freezed == durationText
          ? _value.durationText
          : durationText // ignore: cast_nullable_to_non_nullable
              as String?,
      downloadedFile: freezed == downloadedFile
          ? _value.downloadedFile
          : downloadedFile // ignore: cast_nullable_to_non_nullable
              as DownloadedFile?,
    ) as $Val);
  }

  /// Create a copy of ItemDetailsState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $PlayableStateCopyWith<$Res>? get playable {
    if (_value.playable == null) {
      return null;
    }

    return $PlayableStateCopyWith<$Res>(_value.playable!, (value) {
      return _then(_value.copyWith(playable: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$ItemDetailsStateImplCopyWith<$Res>
    implements $ItemDetailsStateCopyWith<$Res> {
  factory _$$ItemDetailsStateImplCopyWith(_$ItemDetailsStateImpl value,
          $Res Function(_$ItemDetailsStateImpl) then) =
      __$$ItemDetailsStateImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {MediaItem item,
      ImageId? poster,
      ImageId? backdrop,
      List<EpisodeGroupState> seasons,
      PlayableState? playable,
      bool isWatched,
      String? durationText,
      DownloadedFile? downloadedFile});

  @override
  $PlayableStateCopyWith<$Res>? get playable;
}

/// @nodoc
class __$$ItemDetailsStateImplCopyWithImpl<$Res>
    extends _$ItemDetailsStateCopyWithImpl<$Res, _$ItemDetailsStateImpl>
    implements _$$ItemDetailsStateImplCopyWith<$Res> {
  __$$ItemDetailsStateImplCopyWithImpl(_$ItemDetailsStateImpl _value,
      $Res Function(_$ItemDetailsStateImpl) _then)
      : super(_value, _then);

  /// Create a copy of ItemDetailsState
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? item = null,
    Object? poster = freezed,
    Object? backdrop = freezed,
    Object? seasons = null,
    Object? playable = freezed,
    Object? isWatched = null,
    Object? durationText = freezed,
    Object? downloadedFile = freezed,
  }) {
    return _then(_$ItemDetailsStateImpl(
      item: null == item
          ? _value.item
          : item // ignore: cast_nullable_to_non_nullable
              as MediaItem,
      poster: freezed == poster
          ? _value.poster
          : poster // ignore: cast_nullable_to_non_nullable
              as ImageId?,
      backdrop: freezed == backdrop
          ? _value.backdrop
          : backdrop // ignore: cast_nullable_to_non_nullable
              as ImageId?,
      seasons: null == seasons
          ? _value._seasons
          : seasons // ignore: cast_nullable_to_non_nullable
              as List<EpisodeGroupState>,
      playable: freezed == playable
          ? _value.playable
          : playable // ignore: cast_nullable_to_non_nullable
              as PlayableState?,
      isWatched: null == isWatched
          ? _value.isWatched
          : isWatched // ignore: cast_nullable_to_non_nullable
              as bool,
      durationText: freezed == durationText
          ? _value.durationText
          : durationText // ignore: cast_nullable_to_non_nullable
              as String?,
      downloadedFile: freezed == downloadedFile
          ? _value.downloadedFile
          : downloadedFile // ignore: cast_nullable_to_non_nullable
              as DownloadedFile?,
    ));
  }
}

/// @nodoc

class _$ItemDetailsStateImpl implements _ItemDetailsState {
  _$ItemDetailsStateImpl(
      {required this.item,
      required this.poster,
      required this.backdrop,
      required final List<EpisodeGroupState> seasons,
      required this.playable,
      required this.isWatched,
      required this.durationText,
      required this.downloadedFile})
      : _seasons = seasons;

  @override
  final MediaItem item;
  @override
  final ImageId? poster;
  @override
  final ImageId? backdrop;
  final List<EpisodeGroupState> _seasons;
  @override
  List<EpisodeGroupState> get seasons {
    if (_seasons is EqualUnmodifiableListView) return _seasons;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_seasons);
  }

  @override
  final PlayableState? playable;
  @override
  final bool isWatched;
  @override
  final String? durationText;
  @override
  final DownloadedFile? downloadedFile;

  @override
  String toString() {
    return 'ItemDetailsState(item: $item, poster: $poster, backdrop: $backdrop, seasons: $seasons, playable: $playable, isWatched: $isWatched, durationText: $durationText, downloadedFile: $downloadedFile)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ItemDetailsStateImpl &&
            (identical(other.item, item) || other.item == item) &&
            (identical(other.poster, poster) || other.poster == poster) &&
            (identical(other.backdrop, backdrop) ||
                other.backdrop == backdrop) &&
            const DeepCollectionEquality().equals(other._seasons, _seasons) &&
            (identical(other.playable, playable) ||
                other.playable == playable) &&
            (identical(other.isWatched, isWatched) ||
                other.isWatched == isWatched) &&
            (identical(other.durationText, durationText) ||
                other.durationText == durationText) &&
            const DeepCollectionEquality()
                .equals(other.downloadedFile, downloadedFile));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType,
      item,
      poster,
      backdrop,
      const DeepCollectionEquality().hash(_seasons),
      playable,
      isWatched,
      durationText,
      const DeepCollectionEquality().hash(downloadedFile));

  /// Create a copy of ItemDetailsState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ItemDetailsStateImplCopyWith<_$ItemDetailsStateImpl> get copyWith =>
      __$$ItemDetailsStateImplCopyWithImpl<_$ItemDetailsStateImpl>(
          this, _$identity);
}

abstract class _ItemDetailsState implements ItemDetailsState {
  factory _ItemDetailsState(
      {required final MediaItem item,
      required final ImageId? poster,
      required final ImageId? backdrop,
      required final List<EpisodeGroupState> seasons,
      required final PlayableState? playable,
      required final bool isWatched,
      required final String? durationText,
      required final DownloadedFile? downloadedFile}) = _$ItemDetailsStateImpl;

  @override
  MediaItem get item;
  @override
  ImageId? get poster;
  @override
  ImageId? get backdrop;
  @override
  List<EpisodeGroupState> get seasons;
  @override
  PlayableState? get playable;
  @override
  bool get isWatched;
  @override
  String? get durationText;
  @override
  DownloadedFile? get downloadedFile;

  /// Create a copy of ItemDetailsState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ItemDetailsStateImplCopyWith<_$ItemDetailsStateImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$PlayableState {
  int get id => throw _privateConstructorUsedError;
  int? get seasonIndex => throw _privateConstructorUsedError;
  double? get progress => throw _privateConstructorUsedError;
  String? get caption => throw _privateConstructorUsedError;
  bool get shouldResume => throw _privateConstructorUsedError;
  double get playPosition => throw _privateConstructorUsedError;

  /// Create a copy of PlayableState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $PlayableStateCopyWith<PlayableState> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $PlayableStateCopyWith<$Res> {
  factory $PlayableStateCopyWith(
          PlayableState value, $Res Function(PlayableState) then) =
      _$PlayableStateCopyWithImpl<$Res, PlayableState>;
  @useResult
  $Res call(
      {int id,
      int? seasonIndex,
      double? progress,
      String? caption,
      bool shouldResume,
      double playPosition});
}

/// @nodoc
class _$PlayableStateCopyWithImpl<$Res, $Val extends PlayableState>
    implements $PlayableStateCopyWith<$Res> {
  _$PlayableStateCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of PlayableState
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? seasonIndex = freezed,
    Object? progress = freezed,
    Object? caption = freezed,
    Object? shouldResume = null,
    Object? playPosition = null,
  }) {
    return _then(_value.copyWith(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as int,
      seasonIndex: freezed == seasonIndex
          ? _value.seasonIndex
          : seasonIndex // ignore: cast_nullable_to_non_nullable
              as int?,
      progress: freezed == progress
          ? _value.progress
          : progress // ignore: cast_nullable_to_non_nullable
              as double?,
      caption: freezed == caption
          ? _value.caption
          : caption // ignore: cast_nullable_to_non_nullable
              as String?,
      shouldResume: null == shouldResume
          ? _value.shouldResume
          : shouldResume // ignore: cast_nullable_to_non_nullable
              as bool,
      playPosition: null == playPosition
          ? _value.playPosition
          : playPosition // ignore: cast_nullable_to_non_nullable
              as double,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$PlayableStateImplCopyWith<$Res>
    implements $PlayableStateCopyWith<$Res> {
  factory _$$PlayableStateImplCopyWith(
          _$PlayableStateImpl value, $Res Function(_$PlayableStateImpl) then) =
      __$$PlayableStateImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {int id,
      int? seasonIndex,
      double? progress,
      String? caption,
      bool shouldResume,
      double playPosition});
}

/// @nodoc
class __$$PlayableStateImplCopyWithImpl<$Res>
    extends _$PlayableStateCopyWithImpl<$Res, _$PlayableStateImpl>
    implements _$$PlayableStateImplCopyWith<$Res> {
  __$$PlayableStateImplCopyWithImpl(
      _$PlayableStateImpl _value, $Res Function(_$PlayableStateImpl) _then)
      : super(_value, _then);

  /// Create a copy of PlayableState
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? seasonIndex = freezed,
    Object? progress = freezed,
    Object? caption = freezed,
    Object? shouldResume = null,
    Object? playPosition = null,
  }) {
    return _then(_$PlayableStateImpl(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as int,
      seasonIndex: freezed == seasonIndex
          ? _value.seasonIndex
          : seasonIndex // ignore: cast_nullable_to_non_nullable
              as int?,
      progress: freezed == progress
          ? _value.progress
          : progress // ignore: cast_nullable_to_non_nullable
              as double?,
      caption: freezed == caption
          ? _value.caption
          : caption // ignore: cast_nullable_to_non_nullable
              as String?,
      shouldResume: null == shouldResume
          ? _value.shouldResume
          : shouldResume // ignore: cast_nullable_to_non_nullable
              as bool,
      playPosition: null == playPosition
          ? _value.playPosition
          : playPosition // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc

class _$PlayableStateImpl implements _PlayableState {
  _$PlayableStateImpl(
      {required this.id,
      required this.seasonIndex,
      required this.progress,
      required this.caption,
      required this.shouldResume,
      required this.playPosition});

  @override
  final int id;
  @override
  final int? seasonIndex;
  @override
  final double? progress;
  @override
  final String? caption;
  @override
  final bool shouldResume;
  @override
  final double playPosition;

  @override
  String toString() {
    return 'PlayableState(id: $id, seasonIndex: $seasonIndex, progress: $progress, caption: $caption, shouldResume: $shouldResume, playPosition: $playPosition)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PlayableStateImpl &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.seasonIndex, seasonIndex) ||
                other.seasonIndex == seasonIndex) &&
            (identical(other.progress, progress) ||
                other.progress == progress) &&
            (identical(other.caption, caption) || other.caption == caption) &&
            (identical(other.shouldResume, shouldResume) ||
                other.shouldResume == shouldResume) &&
            (identical(other.playPosition, playPosition) ||
                other.playPosition == playPosition));
  }

  @override
  int get hashCode => Object.hash(runtimeType, id, seasonIndex, progress,
      caption, shouldResume, playPosition);

  /// Create a copy of PlayableState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$PlayableStateImplCopyWith<_$PlayableStateImpl> get copyWith =>
      __$$PlayableStateImplCopyWithImpl<_$PlayableStateImpl>(this, _$identity);
}

abstract class _PlayableState implements PlayableState {
  factory _PlayableState(
      {required final int id,
      required final int? seasonIndex,
      required final double? progress,
      required final String? caption,
      required final bool shouldResume,
      required final double playPosition}) = _$PlayableStateImpl;

  @override
  int get id;
  @override
  int? get seasonIndex;
  @override
  double? get progress;
  @override
  String? get caption;
  @override
  bool get shouldResume;
  @override
  double get playPosition;

  /// Create a copy of PlayableState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$PlayableStateImplCopyWith<_$PlayableStateImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$EpisodeGroupState {
  String get name => throw _privateConstructorUsedError;
  List<EpisodeState> get episodes => throw _privateConstructorUsedError;

  /// Create a copy of EpisodeGroupState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $EpisodeGroupStateCopyWith<EpisodeGroupState> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EpisodeGroupStateCopyWith<$Res> {
  factory $EpisodeGroupStateCopyWith(
          EpisodeGroupState value, $Res Function(EpisodeGroupState) then) =
      _$EpisodeGroupStateCopyWithImpl<$Res, EpisodeGroupState>;
  @useResult
  $Res call({String name, List<EpisodeState> episodes});
}

/// @nodoc
class _$EpisodeGroupStateCopyWithImpl<$Res, $Val extends EpisodeGroupState>
    implements $EpisodeGroupStateCopyWith<$Res> {
  _$EpisodeGroupStateCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of EpisodeGroupState
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? name = null,
    Object? episodes = null,
  }) {
    return _then(_value.copyWith(
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      episodes: null == episodes
          ? _value.episodes
          : episodes // ignore: cast_nullable_to_non_nullable
              as List<EpisodeState>,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$EpisodeGroupStateImplCopyWith<$Res>
    implements $EpisodeGroupStateCopyWith<$Res> {
  factory _$$EpisodeGroupStateImplCopyWith(_$EpisodeGroupStateImpl value,
          $Res Function(_$EpisodeGroupStateImpl) then) =
      __$$EpisodeGroupStateImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String name, List<EpisodeState> episodes});
}

/// @nodoc
class __$$EpisodeGroupStateImplCopyWithImpl<$Res>
    extends _$EpisodeGroupStateCopyWithImpl<$Res, _$EpisodeGroupStateImpl>
    implements _$$EpisodeGroupStateImplCopyWith<$Res> {
  __$$EpisodeGroupStateImplCopyWithImpl(_$EpisodeGroupStateImpl _value,
      $Res Function(_$EpisodeGroupStateImpl) _then)
      : super(_value, _then);

  /// Create a copy of EpisodeGroupState
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? name = null,
    Object? episodes = null,
  }) {
    return _then(_$EpisodeGroupStateImpl(
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      episodes: null == episodes
          ? _value._episodes
          : episodes // ignore: cast_nullable_to_non_nullable
              as List<EpisodeState>,
    ));
  }
}

/// @nodoc

class _$EpisodeGroupStateImpl implements _EpisodeGroupState {
  _$EpisodeGroupStateImpl(
      {required this.name, required final List<EpisodeState> episodes})
      : _episodes = episodes;

  @override
  final String name;
  final List<EpisodeState> _episodes;
  @override
  List<EpisodeState> get episodes {
    if (_episodes is EqualUnmodifiableListView) return _episodes;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_episodes);
  }

  @override
  String toString() {
    return 'EpisodeGroupState(name: $name, episodes: $episodes)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EpisodeGroupStateImpl &&
            (identical(other.name, name) || other.name == name) &&
            const DeepCollectionEquality().equals(other._episodes, _episodes));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, name, const DeepCollectionEquality().hash(_episodes));

  /// Create a copy of EpisodeGroupState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$EpisodeGroupStateImplCopyWith<_$EpisodeGroupStateImpl> get copyWith =>
      __$$EpisodeGroupStateImplCopyWithImpl<_$EpisodeGroupStateImpl>(
          this, _$identity);
}

abstract class _EpisodeGroupState implements EpisodeGroupState {
  factory _EpisodeGroupState(
      {required final String name,
      required final List<EpisodeState> episodes}) = _$EpisodeGroupStateImpl;

  @override
  String get name;
  @override
  List<EpisodeState> get episodes;

  /// Create a copy of EpisodeGroupState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$EpisodeGroupStateImplCopyWith<_$EpisodeGroupStateImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$EpisodeState {
  int get id => throw _privateConstructorUsedError;
  ImageId? get thumbnail => throw _privateConstructorUsedError;
  String? get overview => throw _privateConstructorUsedError;
  bool get isWatched => throw _privateConstructorUsedError;
  String get title => throw _privateConstructorUsedError;

  /// Create a copy of EpisodeState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $EpisodeStateCopyWith<EpisodeState> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $EpisodeStateCopyWith<$Res> {
  factory $EpisodeStateCopyWith(
          EpisodeState value, $Res Function(EpisodeState) then) =
      _$EpisodeStateCopyWithImpl<$Res, EpisodeState>;
  @useResult
  $Res call(
      {int id,
      ImageId? thumbnail,
      String? overview,
      bool isWatched,
      String title});
}

/// @nodoc
class _$EpisodeStateCopyWithImpl<$Res, $Val extends EpisodeState>
    implements $EpisodeStateCopyWith<$Res> {
  _$EpisodeStateCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of EpisodeState
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? thumbnail = freezed,
    Object? overview = freezed,
    Object? isWatched = null,
    Object? title = null,
  }) {
    return _then(_value.copyWith(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as int,
      thumbnail: freezed == thumbnail
          ? _value.thumbnail
          : thumbnail // ignore: cast_nullable_to_non_nullable
              as ImageId?,
      overview: freezed == overview
          ? _value.overview
          : overview // ignore: cast_nullable_to_non_nullable
              as String?,
      isWatched: null == isWatched
          ? _value.isWatched
          : isWatched // ignore: cast_nullable_to_non_nullable
              as bool,
      title: null == title
          ? _value.title
          : title // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$EpisodeStateImplCopyWith<$Res>
    implements $EpisodeStateCopyWith<$Res> {
  factory _$$EpisodeStateImplCopyWith(
          _$EpisodeStateImpl value, $Res Function(_$EpisodeStateImpl) then) =
      __$$EpisodeStateImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {int id,
      ImageId? thumbnail,
      String? overview,
      bool isWatched,
      String title});
}

/// @nodoc
class __$$EpisodeStateImplCopyWithImpl<$Res>
    extends _$EpisodeStateCopyWithImpl<$Res, _$EpisodeStateImpl>
    implements _$$EpisodeStateImplCopyWith<$Res> {
  __$$EpisodeStateImplCopyWithImpl(
      _$EpisodeStateImpl _value, $Res Function(_$EpisodeStateImpl) _then)
      : super(_value, _then);

  /// Create a copy of EpisodeState
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? thumbnail = freezed,
    Object? overview = freezed,
    Object? isWatched = null,
    Object? title = null,
  }) {
    return _then(_$EpisodeStateImpl(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as int,
      thumbnail: freezed == thumbnail
          ? _value.thumbnail
          : thumbnail // ignore: cast_nullable_to_non_nullable
              as ImageId?,
      overview: freezed == overview
          ? _value.overview
          : overview // ignore: cast_nullable_to_non_nullable
              as String?,
      isWatched: null == isWatched
          ? _value.isWatched
          : isWatched // ignore: cast_nullable_to_non_nullable
              as bool,
      title: null == title
          ? _value.title
          : title // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$EpisodeStateImpl implements _EpisodeState {
  _$EpisodeStateImpl(
      {required this.id,
      required this.thumbnail,
      required this.overview,
      required this.isWatched,
      required this.title});

  @override
  final int id;
  @override
  final ImageId? thumbnail;
  @override
  final String? overview;
  @override
  final bool isWatched;
  @override
  final String title;

  @override
  String toString() {
    return 'EpisodeState(id: $id, thumbnail: $thumbnail, overview: $overview, isWatched: $isWatched, title: $title)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$EpisodeStateImpl &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.thumbnail, thumbnail) ||
                other.thumbnail == thumbnail) &&
            (identical(other.overview, overview) ||
                other.overview == overview) &&
            (identical(other.isWatched, isWatched) ||
                other.isWatched == isWatched) &&
            (identical(other.title, title) || other.title == title));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, id, thumbnail, overview, isWatched, title);

  /// Create a copy of EpisodeState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$EpisodeStateImplCopyWith<_$EpisodeStateImpl> get copyWith =>
      __$$EpisodeStateImplCopyWithImpl<_$EpisodeStateImpl>(this, _$identity);
}

abstract class _EpisodeState implements EpisodeState {
  factory _EpisodeState(
      {required final int id,
      required final ImageId? thumbnail,
      required final String? overview,
      required final bool isWatched,
      required final String title}) = _$EpisodeStateImpl;

  @override
  int get id;
  @override
  ImageId? get thumbnail;
  @override
  String? get overview;
  @override
  bool get isWatched;
  @override
  String get title;

  /// Create a copy of EpisodeState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$EpisodeStateImplCopyWith<_$EpisodeStateImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
