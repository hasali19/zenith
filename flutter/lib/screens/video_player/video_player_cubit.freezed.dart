// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'video_player_cubit.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$Playlist {
  List<MediaItem> get items => throw _privateConstructorUsedError;
  int get start => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $PlaylistCopyWith<Playlist> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $PlaylistCopyWith<$Res> {
  factory $PlaylistCopyWith(Playlist value, $Res Function(Playlist) then) =
      _$PlaylistCopyWithImpl<$Res, Playlist>;
  @useResult
  $Res call({List<MediaItem> items, int start});
}

/// @nodoc
class _$PlaylistCopyWithImpl<$Res, $Val extends Playlist>
    implements $PlaylistCopyWith<$Res> {
  _$PlaylistCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? items = null,
    Object? start = null,
  }) {
    return _then(_value.copyWith(
      items: null == items
          ? _value.items
          : items // ignore: cast_nullable_to_non_nullable
              as List<MediaItem>,
      start: null == start
          ? _value.start
          : start // ignore: cast_nullable_to_non_nullable
              as int,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_PlaylistCopyWith<$Res> implements $PlaylistCopyWith<$Res> {
  factory _$$_PlaylistCopyWith(
          _$_Playlist value, $Res Function(_$_Playlist) then) =
      __$$_PlaylistCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({List<MediaItem> items, int start});
}

/// @nodoc
class __$$_PlaylistCopyWithImpl<$Res>
    extends _$PlaylistCopyWithImpl<$Res, _$_Playlist>
    implements _$$_PlaylistCopyWith<$Res> {
  __$$_PlaylistCopyWithImpl(
      _$_Playlist _value, $Res Function(_$_Playlist) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? items = null,
    Object? start = null,
  }) {
    return _then(_$_Playlist(
      items: null == items
          ? _value._items
          : items // ignore: cast_nullable_to_non_nullable
              as List<MediaItem>,
      start: null == start
          ? _value.start
          : start // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc

class _$_Playlist implements _Playlist {
  _$_Playlist({required final List<MediaItem> items, required this.start})
      : _items = items;

  final List<MediaItem> _items;
  @override
  List<MediaItem> get items {
    if (_items is EqualUnmodifiableListView) return _items;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_items);
  }

  @override
  final int start;

  @override
  String toString() {
    return 'Playlist(items: $items, start: $start)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_Playlist &&
            const DeepCollectionEquality().equals(other._items, _items) &&
            (identical(other.start, start) || other.start == start));
  }

  @override
  int get hashCode => Object.hash(
      runtimeType, const DeepCollectionEquality().hash(_items), start);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_PlaylistCopyWith<_$_Playlist> get copyWith =>
      __$$_PlaylistCopyWithImpl<_$_Playlist>(this, _$identity);
}

abstract class _Playlist implements Playlist {
  factory _Playlist(
      {required final List<MediaItem> items,
      required final int start}) = _$_Playlist;

  @override
  List<MediaItem> get items;
  @override
  int get start;
  @override
  @JsonKey(ignore: true)
  _$$_PlaylistCopyWith<_$_Playlist> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$VideoPlayerState {
  PlaybackLocation get location => throw _privateConstructorUsedError;
  Playlist? get playlist => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $VideoPlayerStateCopyWith<VideoPlayerState> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $VideoPlayerStateCopyWith<$Res> {
  factory $VideoPlayerStateCopyWith(
          VideoPlayerState value, $Res Function(VideoPlayerState) then) =
      _$VideoPlayerStateCopyWithImpl<$Res, VideoPlayerState>;
  @useResult
  $Res call({PlaybackLocation location, Playlist? playlist});

  $PlaylistCopyWith<$Res>? get playlist;
}

/// @nodoc
class _$VideoPlayerStateCopyWithImpl<$Res, $Val extends VideoPlayerState>
    implements $VideoPlayerStateCopyWith<$Res> {
  _$VideoPlayerStateCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? location = null,
    Object? playlist = freezed,
  }) {
    return _then(_value.copyWith(
      location: null == location
          ? _value.location
          : location // ignore: cast_nullable_to_non_nullable
              as PlaybackLocation,
      playlist: freezed == playlist
          ? _value.playlist
          : playlist // ignore: cast_nullable_to_non_nullable
              as Playlist?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $PlaylistCopyWith<$Res>? get playlist {
    if (_value.playlist == null) {
      return null;
    }

    return $PlaylistCopyWith<$Res>(_value.playlist!, (value) {
      return _then(_value.copyWith(playlist: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$_VideoPlayerStateCopyWith<$Res>
    implements $VideoPlayerStateCopyWith<$Res> {
  factory _$$_VideoPlayerStateCopyWith(
          _$_VideoPlayerState value, $Res Function(_$_VideoPlayerState) then) =
      __$$_VideoPlayerStateCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({PlaybackLocation location, Playlist? playlist});

  @override
  $PlaylistCopyWith<$Res>? get playlist;
}

/// @nodoc
class __$$_VideoPlayerStateCopyWithImpl<$Res>
    extends _$VideoPlayerStateCopyWithImpl<$Res, _$_VideoPlayerState>
    implements _$$_VideoPlayerStateCopyWith<$Res> {
  __$$_VideoPlayerStateCopyWithImpl(
      _$_VideoPlayerState _value, $Res Function(_$_VideoPlayerState) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? location = null,
    Object? playlist = freezed,
  }) {
    return _then(_$_VideoPlayerState(
      location: null == location
          ? _value.location
          : location // ignore: cast_nullable_to_non_nullable
              as PlaybackLocation,
      playlist: freezed == playlist
          ? _value.playlist
          : playlist // ignore: cast_nullable_to_non_nullable
              as Playlist?,
    ));
  }
}

/// @nodoc

class _$_VideoPlayerState implements _VideoPlayerState {
  _$_VideoPlayerState({required this.location, this.playlist});

  @override
  final PlaybackLocation location;
  @override
  final Playlist? playlist;

  @override
  String toString() {
    return 'VideoPlayerState(location: $location, playlist: $playlist)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_VideoPlayerState &&
            (identical(other.location, location) ||
                other.location == location) &&
            (identical(other.playlist, playlist) ||
                other.playlist == playlist));
  }

  @override
  int get hashCode => Object.hash(runtimeType, location, playlist);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_VideoPlayerStateCopyWith<_$_VideoPlayerState> get copyWith =>
      __$$_VideoPlayerStateCopyWithImpl<_$_VideoPlayerState>(this, _$identity);
}

abstract class _VideoPlayerState implements VideoPlayerState {
  factory _VideoPlayerState(
      {required final PlaybackLocation location,
      final Playlist? playlist}) = _$_VideoPlayerState;

  @override
  PlaybackLocation get location;
  @override
  Playlist? get playlist;
  @override
  @JsonKey(ignore: true)
  _$$_VideoPlayerStateCopyWith<_$_VideoPlayerState> get copyWith =>
      throw _privateConstructorUsedError;
}
