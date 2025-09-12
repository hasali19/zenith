// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'api.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

AccessToken _$AccessTokenFromJson(Map<String, dynamic> json) {
  return _AccessToken.fromJson(json);
}

/// @nodoc
mixin _$AccessToken {
  AccessTokenOwner get owner => throw _privateConstructorUsedError;
  String get name => throw _privateConstructorUsedError;
  String get token => throw _privateConstructorUsedError;

  /// Serializes this AccessToken to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of AccessToken
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $AccessTokenCopyWith<AccessToken> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $AccessTokenCopyWith<$Res> {
  factory $AccessTokenCopyWith(
          AccessToken value, $Res Function(AccessToken) then) =
      _$AccessTokenCopyWithImpl<$Res, AccessToken>;
  @useResult
  $Res call({AccessTokenOwner owner, String name, String token});
}

/// @nodoc
class _$AccessTokenCopyWithImpl<$Res, $Val extends AccessToken>
    implements $AccessTokenCopyWith<$Res> {
  _$AccessTokenCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of AccessToken
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? owner = null,
    Object? name = null,
    Object? token = null,
  }) {
    return _then(_value.copyWith(
      owner: null == owner
          ? _value.owner
          : owner // ignore: cast_nullable_to_non_nullable
              as AccessTokenOwner,
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      token: null == token
          ? _value.token
          : token // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$AccessTokenImplCopyWith<$Res>
    implements $AccessTokenCopyWith<$Res> {
  factory _$$AccessTokenImplCopyWith(
          _$AccessTokenImpl value, $Res Function(_$AccessTokenImpl) then) =
      __$$AccessTokenImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({AccessTokenOwner owner, String name, String token});
}

/// @nodoc
class __$$AccessTokenImplCopyWithImpl<$Res>
    extends _$AccessTokenCopyWithImpl<$Res, _$AccessTokenImpl>
    implements _$$AccessTokenImplCopyWith<$Res> {
  __$$AccessTokenImplCopyWithImpl(
      _$AccessTokenImpl _value, $Res Function(_$AccessTokenImpl) _then)
      : super(_value, _then);

  /// Create a copy of AccessToken
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? owner = null,
    Object? name = null,
    Object? token = null,
  }) {
    return _then(_$AccessTokenImpl(
      owner: null == owner
          ? _value.owner
          : owner // ignore: cast_nullable_to_non_nullable
              as AccessTokenOwner,
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      token: null == token
          ? _value.token
          : token // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$AccessTokenImpl implements _AccessToken {
  _$AccessTokenImpl(
      {required this.owner, required this.name, required this.token});

  factory _$AccessTokenImpl.fromJson(Map<String, dynamic> json) =>
      _$$AccessTokenImplFromJson(json);

  @override
  final AccessTokenOwner owner;
  @override
  final String name;
  @override
  final String token;

  @override
  String toString() {
    return 'AccessToken(owner: $owner, name: $name, token: $token)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AccessTokenImpl &&
            (identical(other.owner, owner) || other.owner == owner) &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.token, token) || other.token == token));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, owner, name, token);

  /// Create a copy of AccessToken
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$AccessTokenImplCopyWith<_$AccessTokenImpl> get copyWith =>
      __$$AccessTokenImplCopyWithImpl<_$AccessTokenImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$AccessTokenImplToJson(
      this,
    );
  }
}

abstract class _AccessToken implements AccessToken {
  factory _AccessToken(
      {required final AccessTokenOwner owner,
      required final String name,
      required final String token}) = _$AccessTokenImpl;

  factory _AccessToken.fromJson(Map<String, dynamic> json) =
      _$AccessTokenImpl.fromJson;

  @override
  AccessTokenOwner get owner;
  @override
  String get name;
  @override
  String get token;

  /// Create a copy of AccessToken
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$AccessTokenImplCopyWith<_$AccessTokenImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

CastConfig _$CastConfigFromJson(Map<String, dynamic> json) {
  return _CastConfig.fromJson(json);
}

/// @nodoc
mixin _$CastConfig {
// ignore: invalid_annotation_target
  @JsonKey(name: 'app_id')
  String? get appId => throw _privateConstructorUsedError;

  /// Serializes this CastConfig to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of CastConfig
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $CastConfigCopyWith<CastConfig> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CastConfigCopyWith<$Res> {
  factory $CastConfigCopyWith(
          CastConfig value, $Res Function(CastConfig) then) =
      _$CastConfigCopyWithImpl<$Res, CastConfig>;
  @useResult
  $Res call({@JsonKey(name: 'app_id') String? appId});
}

/// @nodoc
class _$CastConfigCopyWithImpl<$Res, $Val extends CastConfig>
    implements $CastConfigCopyWith<$Res> {
  _$CastConfigCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of CastConfig
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? appId = freezed,
  }) {
    return _then(_value.copyWith(
      appId: freezed == appId
          ? _value.appId
          : appId // ignore: cast_nullable_to_non_nullable
              as String?,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$CastConfigImplCopyWith<$Res>
    implements $CastConfigCopyWith<$Res> {
  factory _$$CastConfigImplCopyWith(
          _$CastConfigImpl value, $Res Function(_$CastConfigImpl) then) =
      __$$CastConfigImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({@JsonKey(name: 'app_id') String? appId});
}

/// @nodoc
class __$$CastConfigImplCopyWithImpl<$Res>
    extends _$CastConfigCopyWithImpl<$Res, _$CastConfigImpl>
    implements _$$CastConfigImplCopyWith<$Res> {
  __$$CastConfigImplCopyWithImpl(
      _$CastConfigImpl _value, $Res Function(_$CastConfigImpl) _then)
      : super(_value, _then);

  /// Create a copy of CastConfig
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? appId = freezed,
  }) {
    return _then(_$CastConfigImpl(
      appId: freezed == appId
          ? _value.appId
          : appId // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$CastConfigImpl implements _CastConfig {
  _$CastConfigImpl({@JsonKey(name: 'app_id') required this.appId});

  factory _$CastConfigImpl.fromJson(Map<String, dynamic> json) =>
      _$$CastConfigImplFromJson(json);

// ignore: invalid_annotation_target
  @override
  @JsonKey(name: 'app_id')
  final String? appId;

  @override
  String toString() {
    return 'CastConfig(appId: $appId)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CastConfigImpl &&
            (identical(other.appId, appId) || other.appId == appId));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, appId);

  /// Create a copy of CastConfig
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$CastConfigImplCopyWith<_$CastConfigImpl> get copyWith =>
      __$$CastConfigImplCopyWithImpl<_$CastConfigImpl>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$CastConfigImplToJson(
      this,
    );
  }
}

abstract class _CastConfig implements CastConfig {
  factory _CastConfig({@JsonKey(name: 'app_id') required final String? appId}) =
      _$CastConfigImpl;

  factory _CastConfig.fromJson(Map<String, dynamic> json) =
      _$CastConfigImpl.fromJson;

// ignore: invalid_annotation_target
  @override
  @JsonKey(name: 'app_id')
  String? get appId;

  /// Create a copy of CastConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$CastConfigImplCopyWith<_$CastConfigImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

TranscoderState _$TranscoderStateFromJson(Map<String, dynamic> json) {
  return _TranscoderState.fromJson(json);
}

/// @nodoc
mixin _$TranscoderState {
  List<TranscoderJob> get queue => throw _privateConstructorUsedError;

  /// Serializes this TranscoderState to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of TranscoderState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $TranscoderStateCopyWith<TranscoderState> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $TranscoderStateCopyWith<$Res> {
  factory $TranscoderStateCopyWith(
          TranscoderState value, $Res Function(TranscoderState) then) =
      _$TranscoderStateCopyWithImpl<$Res, TranscoderState>;
  @useResult
  $Res call({List<TranscoderJob> queue});
}

/// @nodoc
class _$TranscoderStateCopyWithImpl<$Res, $Val extends TranscoderState>
    implements $TranscoderStateCopyWith<$Res> {
  _$TranscoderStateCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of TranscoderState
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? queue = null,
  }) {
    return _then(_value.copyWith(
      queue: null == queue
          ? _value.queue
          : queue // ignore: cast_nullable_to_non_nullable
              as List<TranscoderJob>,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$TranscoderStateImplCopyWith<$Res>
    implements $TranscoderStateCopyWith<$Res> {
  factory _$$TranscoderStateImplCopyWith(_$TranscoderStateImpl value,
          $Res Function(_$TranscoderStateImpl) then) =
      __$$TranscoderStateImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({List<TranscoderJob> queue});
}

/// @nodoc
class __$$TranscoderStateImplCopyWithImpl<$Res>
    extends _$TranscoderStateCopyWithImpl<$Res, _$TranscoderStateImpl>
    implements _$$TranscoderStateImplCopyWith<$Res> {
  __$$TranscoderStateImplCopyWithImpl(
      _$TranscoderStateImpl _value, $Res Function(_$TranscoderStateImpl) _then)
      : super(_value, _then);

  /// Create a copy of TranscoderState
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? queue = null,
  }) {
    return _then(_$TranscoderStateImpl(
      queue: null == queue
          ? _value._queue
          : queue // ignore: cast_nullable_to_non_nullable
              as List<TranscoderJob>,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$TranscoderStateImpl implements _TranscoderState {
  _$TranscoderStateImpl({required final List<TranscoderJob> queue})
      : _queue = queue;

  factory _$TranscoderStateImpl.fromJson(Map<String, dynamic> json) =>
      _$$TranscoderStateImplFromJson(json);

  final List<TranscoderJob> _queue;
  @override
  List<TranscoderJob> get queue {
    if (_queue is EqualUnmodifiableListView) return _queue;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_queue);
  }

  @override
  String toString() {
    return 'TranscoderState(queue: $queue)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$TranscoderStateImpl &&
            const DeepCollectionEquality().equals(other._queue, _queue));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(_queue));

  /// Create a copy of TranscoderState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$TranscoderStateImplCopyWith<_$TranscoderStateImpl> get copyWith =>
      __$$TranscoderStateImplCopyWithImpl<_$TranscoderStateImpl>(
          this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$$TranscoderStateImplToJson(
      this,
    );
  }
}

abstract class _TranscoderState implements TranscoderState {
  factory _TranscoderState({required final List<TranscoderJob> queue}) =
      _$TranscoderStateImpl;

  factory _TranscoderState.fromJson(Map<String, dynamic> json) =
      _$TranscoderStateImpl.fromJson;

  @override
  List<TranscoderJob> get queue;

  /// Create a copy of TranscoderState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$TranscoderStateImplCopyWith<_$TranscoderStateImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

TranscoderJob _$TranscoderJobFromJson(Map<String, dynamic> json) {
  switch (json['state']) {
    case 'queued':
      return Queued.fromJson(json);
    case 'processing':
      return Processing.fromJson(json);

    default:
      throw CheckedFromJsonException(json, 'state', 'TranscoderJob',
          'Invalid union type "${json['state']}"!');
  }
}

/// @nodoc
mixin _$TranscoderJob {
// ignore: invalid_annotation_target
  @JsonKey(name: 'video_id')
  int get videoId =>
      throw _privateConstructorUsedError; // ignore: invalid_annotation_target
  @JsonKey(name: 'item_id')
  int get itemId => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(@JsonKey(name: 'video_id') int videoId,
            @JsonKey(name: 'item_id') int itemId)
        queued,
    required TResult Function(@JsonKey(name: 'video_id') int videoId,
            @JsonKey(name: 'item_id') int itemId, double progress)
        processing,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(@JsonKey(name: 'video_id') int videoId,
            @JsonKey(name: 'item_id') int itemId)?
        queued,
    TResult? Function(@JsonKey(name: 'video_id') int videoId,
            @JsonKey(name: 'item_id') int itemId, double progress)?
        processing,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(@JsonKey(name: 'video_id') int videoId,
            @JsonKey(name: 'item_id') int itemId)?
        queued,
    TResult Function(@JsonKey(name: 'video_id') int videoId,
            @JsonKey(name: 'item_id') int itemId, double progress)?
        processing,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Queued value) queued,
    required TResult Function(Processing value) processing,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Queued value)? queued,
    TResult? Function(Processing value)? processing,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Queued value)? queued,
    TResult Function(Processing value)? processing,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;

  /// Serializes this TranscoderJob to a JSON map.
  Map<String, dynamic> toJson() => throw _privateConstructorUsedError;

  /// Create a copy of TranscoderJob
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $TranscoderJobCopyWith<TranscoderJob> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $TranscoderJobCopyWith<$Res> {
  factory $TranscoderJobCopyWith(
          TranscoderJob value, $Res Function(TranscoderJob) then) =
      _$TranscoderJobCopyWithImpl<$Res, TranscoderJob>;
  @useResult
  $Res call(
      {@JsonKey(name: 'video_id') int videoId,
      @JsonKey(name: 'item_id') int itemId});
}

/// @nodoc
class _$TranscoderJobCopyWithImpl<$Res, $Val extends TranscoderJob>
    implements $TranscoderJobCopyWith<$Res> {
  _$TranscoderJobCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of TranscoderJob
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? videoId = null,
    Object? itemId = null,
  }) {
    return _then(_value.copyWith(
      videoId: null == videoId
          ? _value.videoId
          : videoId // ignore: cast_nullable_to_non_nullable
              as int,
      itemId: null == itemId
          ? _value.itemId
          : itemId // ignore: cast_nullable_to_non_nullable
              as int,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$QueuedImplCopyWith<$Res>
    implements $TranscoderJobCopyWith<$Res> {
  factory _$$QueuedImplCopyWith(
          _$QueuedImpl value, $Res Function(_$QueuedImpl) then) =
      __$$QueuedImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {@JsonKey(name: 'video_id') int videoId,
      @JsonKey(name: 'item_id') int itemId});
}

/// @nodoc
class __$$QueuedImplCopyWithImpl<$Res>
    extends _$TranscoderJobCopyWithImpl<$Res, _$QueuedImpl>
    implements _$$QueuedImplCopyWith<$Res> {
  __$$QueuedImplCopyWithImpl(
      _$QueuedImpl _value, $Res Function(_$QueuedImpl) _then)
      : super(_value, _then);

  /// Create a copy of TranscoderJob
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? videoId = null,
    Object? itemId = null,
  }) {
    return _then(_$QueuedImpl(
      null == videoId
          ? _value.videoId
          : videoId // ignore: cast_nullable_to_non_nullable
              as int,
      null == itemId
          ? _value.itemId
          : itemId // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$QueuedImpl implements Queued {
  _$QueuedImpl(@JsonKey(name: 'video_id') this.videoId,
      @JsonKey(name: 'item_id') this.itemId,
      {final String? $type})
      : $type = $type ?? 'queued';

  factory _$QueuedImpl.fromJson(Map<String, dynamic> json) =>
      _$$QueuedImplFromJson(json);

// ignore: invalid_annotation_target
  @override
  @JsonKey(name: 'video_id')
  final int videoId;
// ignore: invalid_annotation_target
  @override
  @JsonKey(name: 'item_id')
  final int itemId;

  @JsonKey(name: 'state')
  final String $type;

  @override
  String toString() {
    return 'TranscoderJob.queued(videoId: $videoId, itemId: $itemId)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$QueuedImpl &&
            (identical(other.videoId, videoId) || other.videoId == videoId) &&
            (identical(other.itemId, itemId) || other.itemId == itemId));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, videoId, itemId);

  /// Create a copy of TranscoderJob
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$QueuedImplCopyWith<_$QueuedImpl> get copyWith =>
      __$$QueuedImplCopyWithImpl<_$QueuedImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(@JsonKey(name: 'video_id') int videoId,
            @JsonKey(name: 'item_id') int itemId)
        queued,
    required TResult Function(@JsonKey(name: 'video_id') int videoId,
            @JsonKey(name: 'item_id') int itemId, double progress)
        processing,
  }) {
    return queued(videoId, itemId);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(@JsonKey(name: 'video_id') int videoId,
            @JsonKey(name: 'item_id') int itemId)?
        queued,
    TResult? Function(@JsonKey(name: 'video_id') int videoId,
            @JsonKey(name: 'item_id') int itemId, double progress)?
        processing,
  }) {
    return queued?.call(videoId, itemId);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(@JsonKey(name: 'video_id') int videoId,
            @JsonKey(name: 'item_id') int itemId)?
        queued,
    TResult Function(@JsonKey(name: 'video_id') int videoId,
            @JsonKey(name: 'item_id') int itemId, double progress)?
        processing,
    required TResult orElse(),
  }) {
    if (queued != null) {
      return queued(videoId, itemId);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Queued value) queued,
    required TResult Function(Processing value) processing,
  }) {
    return queued(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Queued value)? queued,
    TResult? Function(Processing value)? processing,
  }) {
    return queued?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Queued value)? queued,
    TResult Function(Processing value)? processing,
    required TResult orElse(),
  }) {
    if (queued != null) {
      return queued(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$QueuedImplToJson(
      this,
    );
  }
}

abstract class Queued implements TranscoderJob {
  factory Queued(@JsonKey(name: 'video_id') final int videoId,
      @JsonKey(name: 'item_id') final int itemId) = _$QueuedImpl;

  factory Queued.fromJson(Map<String, dynamic> json) = _$QueuedImpl.fromJson;

// ignore: invalid_annotation_target
  @override
  @JsonKey(name: 'video_id')
  int get videoId; // ignore: invalid_annotation_target
  @override
  @JsonKey(name: 'item_id')
  int get itemId;

  /// Create a copy of TranscoderJob
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$QueuedImplCopyWith<_$QueuedImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ProcessingImplCopyWith<$Res>
    implements $TranscoderJobCopyWith<$Res> {
  factory _$$ProcessingImplCopyWith(
          _$ProcessingImpl value, $Res Function(_$ProcessingImpl) then) =
      __$$ProcessingImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {@JsonKey(name: 'video_id') int videoId,
      @JsonKey(name: 'item_id') int itemId,
      double progress});
}

/// @nodoc
class __$$ProcessingImplCopyWithImpl<$Res>
    extends _$TranscoderJobCopyWithImpl<$Res, _$ProcessingImpl>
    implements _$$ProcessingImplCopyWith<$Res> {
  __$$ProcessingImplCopyWithImpl(
      _$ProcessingImpl _value, $Res Function(_$ProcessingImpl) _then)
      : super(_value, _then);

  /// Create a copy of TranscoderJob
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? videoId = null,
    Object? itemId = null,
    Object? progress = null,
  }) {
    return _then(_$ProcessingImpl(
      null == videoId
          ? _value.videoId
          : videoId // ignore: cast_nullable_to_non_nullable
              as int,
      null == itemId
          ? _value.itemId
          : itemId // ignore: cast_nullable_to_non_nullable
              as int,
      null == progress
          ? _value.progress
          : progress // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class _$ProcessingImpl implements Processing {
  _$ProcessingImpl(@JsonKey(name: 'video_id') this.videoId,
      @JsonKey(name: 'item_id') this.itemId, this.progress,
      {final String? $type})
      : $type = $type ?? 'processing';

  factory _$ProcessingImpl.fromJson(Map<String, dynamic> json) =>
      _$$ProcessingImplFromJson(json);

// ignore: invalid_annotation_target
  @override
  @JsonKey(name: 'video_id')
  final int videoId;
// ignore: invalid_annotation_target
  @override
  @JsonKey(name: 'item_id')
  final int itemId;
  @override
  final double progress;

  @JsonKey(name: 'state')
  final String $type;

  @override
  String toString() {
    return 'TranscoderJob.processing(videoId: $videoId, itemId: $itemId, progress: $progress)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ProcessingImpl &&
            (identical(other.videoId, videoId) || other.videoId == videoId) &&
            (identical(other.itemId, itemId) || other.itemId == itemId) &&
            (identical(other.progress, progress) ||
                other.progress == progress));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, videoId, itemId, progress);

  /// Create a copy of TranscoderJob
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ProcessingImplCopyWith<_$ProcessingImpl> get copyWith =>
      __$$ProcessingImplCopyWithImpl<_$ProcessingImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(@JsonKey(name: 'video_id') int videoId,
            @JsonKey(name: 'item_id') int itemId)
        queued,
    required TResult Function(@JsonKey(name: 'video_id') int videoId,
            @JsonKey(name: 'item_id') int itemId, double progress)
        processing,
  }) {
    return processing(videoId, itemId, progress);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(@JsonKey(name: 'video_id') int videoId,
            @JsonKey(name: 'item_id') int itemId)?
        queued,
    TResult? Function(@JsonKey(name: 'video_id') int videoId,
            @JsonKey(name: 'item_id') int itemId, double progress)?
        processing,
  }) {
    return processing?.call(videoId, itemId, progress);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(@JsonKey(name: 'video_id') int videoId,
            @JsonKey(name: 'item_id') int itemId)?
        queued,
    TResult Function(@JsonKey(name: 'video_id') int videoId,
            @JsonKey(name: 'item_id') int itemId, double progress)?
        processing,
    required TResult orElse(),
  }) {
    if (processing != null) {
      return processing(videoId, itemId, progress);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Queued value) queued,
    required TResult Function(Processing value) processing,
  }) {
    return processing(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Queued value)? queued,
    TResult? Function(Processing value)? processing,
  }) {
    return processing?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Queued value)? queued,
    TResult Function(Processing value)? processing,
    required TResult orElse(),
  }) {
    if (processing != null) {
      return processing(this);
    }
    return orElse();
  }

  @override
  Map<String, dynamic> toJson() {
    return _$$ProcessingImplToJson(
      this,
    );
  }
}

abstract class Processing implements TranscoderJob {
  factory Processing(
      @JsonKey(name: 'video_id') final int videoId,
      @JsonKey(name: 'item_id') final int itemId,
      final double progress) = _$ProcessingImpl;

  factory Processing.fromJson(Map<String, dynamic> json) =
      _$ProcessingImpl.fromJson;

// ignore: invalid_annotation_target
  @override
  @JsonKey(name: 'video_id')
  int get videoId; // ignore: invalid_annotation_target
  @override
  @JsonKey(name: 'item_id')
  int get itemId;
  double get progress;

  /// Create a copy of TranscoderJob
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ProcessingImplCopyWith<_$ProcessingImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
