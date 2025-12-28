// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'api.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$AccessToken {
  AccessTokenOwner get owner;
  String get name;
  String get token;

  /// Create a copy of AccessToken
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $AccessTokenCopyWith<AccessToken> get copyWith =>
      _$AccessTokenCopyWithImpl<AccessToken>(this as AccessToken, _$identity);

  /// Serializes this AccessToken to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is AccessToken &&
            (identical(other.owner, owner) || other.owner == owner) &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.token, token) || other.token == token));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, owner, name, token);

  @override
  String toString() {
    return 'AccessToken(owner: $owner, name: $name, token: $token)';
  }
}

/// @nodoc
abstract mixin class $AccessTokenCopyWith<$Res> {
  factory $AccessTokenCopyWith(
          AccessToken value, $Res Function(AccessToken) _then) =
      _$AccessTokenCopyWithImpl;
  @useResult
  $Res call({AccessTokenOwner owner, String name, String token});
}

/// @nodoc
class _$AccessTokenCopyWithImpl<$Res> implements $AccessTokenCopyWith<$Res> {
  _$AccessTokenCopyWithImpl(this._self, this._then);

  final AccessToken _self;
  final $Res Function(AccessToken) _then;

  /// Create a copy of AccessToken
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? owner = null,
    Object? name = null,
    Object? token = null,
  }) {
    return _then(_self.copyWith(
      owner: null == owner
          ? _self.owner
          : owner // ignore: cast_nullable_to_non_nullable
              as AccessTokenOwner,
      name: null == name
          ? _self.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      token: null == token
          ? _self.token
          : token // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// Adds pattern-matching-related methods to [AccessToken].
extension AccessTokenPatterns on AccessToken {
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

  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>(
    TResult Function(_AccessToken value)? $default, {
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case _AccessToken() when $default != null:
        return $default(_that);
      case _:
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

  @optionalTypeArgs
  TResult map<TResult extends Object?>(
    TResult Function(_AccessToken value) $default,
  ) {
    final _that = this;
    switch (_that) {
      case _AccessToken():
        return $default(_that);
      case _:
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

  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>(
    TResult? Function(_AccessToken value)? $default,
  ) {
    final _that = this;
    switch (_that) {
      case _AccessToken() when $default != null:
        return $default(_that);
      case _:
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

  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>(
    TResult Function(AccessTokenOwner owner, String name, String token)?
        $default, {
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case _AccessToken() when $default != null:
        return $default(_that.owner, _that.name, _that.token);
      case _:
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

  @optionalTypeArgs
  TResult when<TResult extends Object?>(
    TResult Function(AccessTokenOwner owner, String name, String token)
        $default,
  ) {
    final _that = this;
    switch (_that) {
      case _AccessToken():
        return $default(_that.owner, _that.name, _that.token);
      case _:
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

  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>(
    TResult? Function(AccessTokenOwner owner, String name, String token)?
        $default,
  ) {
    final _that = this;
    switch (_that) {
      case _AccessToken() when $default != null:
        return $default(_that.owner, _that.name, _that.token);
      case _:
        return null;
    }
  }
}

/// @nodoc
@JsonSerializable()
class _AccessToken implements AccessToken {
  _AccessToken({required this.owner, required this.name, required this.token});
  factory _AccessToken.fromJson(Map<String, dynamic> json) =>
      _$AccessTokenFromJson(json);

  @override
  final AccessTokenOwner owner;
  @override
  final String name;
  @override
  final String token;

  /// Create a copy of AccessToken
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$AccessTokenCopyWith<_AccessToken> get copyWith =>
      __$AccessTokenCopyWithImpl<_AccessToken>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$AccessTokenToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _AccessToken &&
            (identical(other.owner, owner) || other.owner == owner) &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.token, token) || other.token == token));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, owner, name, token);

  @override
  String toString() {
    return 'AccessToken(owner: $owner, name: $name, token: $token)';
  }
}

/// @nodoc
abstract mixin class _$AccessTokenCopyWith<$Res>
    implements $AccessTokenCopyWith<$Res> {
  factory _$AccessTokenCopyWith(
          _AccessToken value, $Res Function(_AccessToken) _then) =
      __$AccessTokenCopyWithImpl;
  @override
  @useResult
  $Res call({AccessTokenOwner owner, String name, String token});
}

/// @nodoc
class __$AccessTokenCopyWithImpl<$Res> implements _$AccessTokenCopyWith<$Res> {
  __$AccessTokenCopyWithImpl(this._self, this._then);

  final _AccessToken _self;
  final $Res Function(_AccessToken) _then;

  /// Create a copy of AccessToken
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? owner = null,
    Object? name = null,
    Object? token = null,
  }) {
    return _then(_AccessToken(
      owner: null == owner
          ? _self.owner
          : owner // ignore: cast_nullable_to_non_nullable
              as AccessTokenOwner,
      name: null == name
          ? _self.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      token: null == token
          ? _self.token
          : token // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc
mixin _$CastConfig {
// ignore: invalid_annotation_target
  @JsonKey(name: 'app_id')
  String? get appId;

  /// Create a copy of CastConfig
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $CastConfigCopyWith<CastConfig> get copyWith =>
      _$CastConfigCopyWithImpl<CastConfig>(this as CastConfig, _$identity);

  /// Serializes this CastConfig to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is CastConfig &&
            (identical(other.appId, appId) || other.appId == appId));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, appId);

  @override
  String toString() {
    return 'CastConfig(appId: $appId)';
  }
}

/// @nodoc
abstract mixin class $CastConfigCopyWith<$Res> {
  factory $CastConfigCopyWith(
          CastConfig value, $Res Function(CastConfig) _then) =
      _$CastConfigCopyWithImpl;
  @useResult
  $Res call({@JsonKey(name: 'app_id') String? appId});
}

/// @nodoc
class _$CastConfigCopyWithImpl<$Res> implements $CastConfigCopyWith<$Res> {
  _$CastConfigCopyWithImpl(this._self, this._then);

  final CastConfig _self;
  final $Res Function(CastConfig) _then;

  /// Create a copy of CastConfig
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? appId = freezed,
  }) {
    return _then(_self.copyWith(
      appId: freezed == appId
          ? _self.appId
          : appId // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// Adds pattern-matching-related methods to [CastConfig].
extension CastConfigPatterns on CastConfig {
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

  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>(
    TResult Function(_CastConfig value)? $default, {
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case _CastConfig() when $default != null:
        return $default(_that);
      case _:
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

  @optionalTypeArgs
  TResult map<TResult extends Object?>(
    TResult Function(_CastConfig value) $default,
  ) {
    final _that = this;
    switch (_that) {
      case _CastConfig():
        return $default(_that);
      case _:
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

  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>(
    TResult? Function(_CastConfig value)? $default,
  ) {
    final _that = this;
    switch (_that) {
      case _CastConfig() when $default != null:
        return $default(_that);
      case _:
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

  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>(
    TResult Function(@JsonKey(name: 'app_id') String? appId)? $default, {
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case _CastConfig() when $default != null:
        return $default(_that.appId);
      case _:
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

  @optionalTypeArgs
  TResult when<TResult extends Object?>(
    TResult Function(@JsonKey(name: 'app_id') String? appId) $default,
  ) {
    final _that = this;
    switch (_that) {
      case _CastConfig():
        return $default(_that.appId);
      case _:
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

  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>(
    TResult? Function(@JsonKey(name: 'app_id') String? appId)? $default,
  ) {
    final _that = this;
    switch (_that) {
      case _CastConfig() when $default != null:
        return $default(_that.appId);
      case _:
        return null;
    }
  }
}

/// @nodoc
@JsonSerializable()
class _CastConfig implements CastConfig {
  _CastConfig({@JsonKey(name: 'app_id') required this.appId});
  factory _CastConfig.fromJson(Map<String, dynamic> json) =>
      _$CastConfigFromJson(json);

// ignore: invalid_annotation_target
  @override
  @JsonKey(name: 'app_id')
  final String? appId;

  /// Create a copy of CastConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$CastConfigCopyWith<_CastConfig> get copyWith =>
      __$CastConfigCopyWithImpl<_CastConfig>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$CastConfigToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _CastConfig &&
            (identical(other.appId, appId) || other.appId == appId));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, appId);

  @override
  String toString() {
    return 'CastConfig(appId: $appId)';
  }
}

/// @nodoc
abstract mixin class _$CastConfigCopyWith<$Res>
    implements $CastConfigCopyWith<$Res> {
  factory _$CastConfigCopyWith(
          _CastConfig value, $Res Function(_CastConfig) _then) =
      __$CastConfigCopyWithImpl;
  @override
  @useResult
  $Res call({@JsonKey(name: 'app_id') String? appId});
}

/// @nodoc
class __$CastConfigCopyWithImpl<$Res> implements _$CastConfigCopyWith<$Res> {
  __$CastConfigCopyWithImpl(this._self, this._then);

  final _CastConfig _self;
  final $Res Function(_CastConfig) _then;

  /// Create a copy of CastConfig
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? appId = freezed,
  }) {
    return _then(_CastConfig(
      appId: freezed == appId
          ? _self.appId
          : appId // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc
mixin _$TranscoderState {
  List<TranscoderJob> get queue;

  /// Create a copy of TranscoderState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $TranscoderStateCopyWith<TranscoderState> get copyWith =>
      _$TranscoderStateCopyWithImpl<TranscoderState>(
          this as TranscoderState, _$identity);

  /// Serializes this TranscoderState to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is TranscoderState &&
            const DeepCollectionEquality().equals(other.queue, queue));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(queue));

  @override
  String toString() {
    return 'TranscoderState(queue: $queue)';
  }
}

/// @nodoc
abstract mixin class $TranscoderStateCopyWith<$Res> {
  factory $TranscoderStateCopyWith(
          TranscoderState value, $Res Function(TranscoderState) _then) =
      _$TranscoderStateCopyWithImpl;
  @useResult
  $Res call({List<TranscoderJob> queue});
}

/// @nodoc
class _$TranscoderStateCopyWithImpl<$Res>
    implements $TranscoderStateCopyWith<$Res> {
  _$TranscoderStateCopyWithImpl(this._self, this._then);

  final TranscoderState _self;
  final $Res Function(TranscoderState) _then;

  /// Create a copy of TranscoderState
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? queue = null,
  }) {
    return _then(_self.copyWith(
      queue: null == queue
          ? _self.queue
          : queue // ignore: cast_nullable_to_non_nullable
              as List<TranscoderJob>,
    ));
  }
}

/// Adds pattern-matching-related methods to [TranscoderState].
extension TranscoderStatePatterns on TranscoderState {
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

  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>(
    TResult Function(_TranscoderState value)? $default, {
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case _TranscoderState() when $default != null:
        return $default(_that);
      case _:
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

  @optionalTypeArgs
  TResult map<TResult extends Object?>(
    TResult Function(_TranscoderState value) $default,
  ) {
    final _that = this;
    switch (_that) {
      case _TranscoderState():
        return $default(_that);
      case _:
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

  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>(
    TResult? Function(_TranscoderState value)? $default,
  ) {
    final _that = this;
    switch (_that) {
      case _TranscoderState() when $default != null:
        return $default(_that);
      case _:
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

  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>(
    TResult Function(List<TranscoderJob> queue)? $default, {
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case _TranscoderState() when $default != null:
        return $default(_that.queue);
      case _:
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

  @optionalTypeArgs
  TResult when<TResult extends Object?>(
    TResult Function(List<TranscoderJob> queue) $default,
  ) {
    final _that = this;
    switch (_that) {
      case _TranscoderState():
        return $default(_that.queue);
      case _:
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

  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>(
    TResult? Function(List<TranscoderJob> queue)? $default,
  ) {
    final _that = this;
    switch (_that) {
      case _TranscoderState() when $default != null:
        return $default(_that.queue);
      case _:
        return null;
    }
  }
}

/// @nodoc
@JsonSerializable()
class _TranscoderState implements TranscoderState {
  _TranscoderState({required final List<TranscoderJob> queue}) : _queue = queue;
  factory _TranscoderState.fromJson(Map<String, dynamic> json) =>
      _$TranscoderStateFromJson(json);

  final List<TranscoderJob> _queue;
  @override
  List<TranscoderJob> get queue {
    if (_queue is EqualUnmodifiableListView) return _queue;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_queue);
  }

  /// Create a copy of TranscoderState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  _$TranscoderStateCopyWith<_TranscoderState> get copyWith =>
      __$TranscoderStateCopyWithImpl<_TranscoderState>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$TranscoderStateToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _TranscoderState &&
            const DeepCollectionEquality().equals(other._queue, _queue));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(_queue));

  @override
  String toString() {
    return 'TranscoderState(queue: $queue)';
  }
}

/// @nodoc
abstract mixin class _$TranscoderStateCopyWith<$Res>
    implements $TranscoderStateCopyWith<$Res> {
  factory _$TranscoderStateCopyWith(
          _TranscoderState value, $Res Function(_TranscoderState) _then) =
      __$TranscoderStateCopyWithImpl;
  @override
  @useResult
  $Res call({List<TranscoderJob> queue});
}

/// @nodoc
class __$TranscoderStateCopyWithImpl<$Res>
    implements _$TranscoderStateCopyWith<$Res> {
  __$TranscoderStateCopyWithImpl(this._self, this._then);

  final _TranscoderState _self;
  final $Res Function(_TranscoderState) _then;

  /// Create a copy of TranscoderState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? queue = null,
  }) {
    return _then(_TranscoderState(
      queue: null == queue
          ? _self._queue
          : queue // ignore: cast_nullable_to_non_nullable
              as List<TranscoderJob>,
    ));
  }
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
  int get videoId; // ignore: invalid_annotation_target
  @JsonKey(name: 'item_id')
  int get itemId;

  /// Create a copy of TranscoderJob
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $TranscoderJobCopyWith<TranscoderJob> get copyWith =>
      _$TranscoderJobCopyWithImpl<TranscoderJob>(
          this as TranscoderJob, _$identity);

  /// Serializes this TranscoderJob to a JSON map.
  Map<String, dynamic> toJson();

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is TranscoderJob &&
            (identical(other.videoId, videoId) || other.videoId == videoId) &&
            (identical(other.itemId, itemId) || other.itemId == itemId));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, videoId, itemId);

  @override
  String toString() {
    return 'TranscoderJob(videoId: $videoId, itemId: $itemId)';
  }
}

/// @nodoc
abstract mixin class $TranscoderJobCopyWith<$Res> {
  factory $TranscoderJobCopyWith(
          TranscoderJob value, $Res Function(TranscoderJob) _then) =
      _$TranscoderJobCopyWithImpl;
  @useResult
  $Res call(
      {@JsonKey(name: 'video_id') int videoId,
      @JsonKey(name: 'item_id') int itemId});
}

/// @nodoc
class _$TranscoderJobCopyWithImpl<$Res>
    implements $TranscoderJobCopyWith<$Res> {
  _$TranscoderJobCopyWithImpl(this._self, this._then);

  final TranscoderJob _self;
  final $Res Function(TranscoderJob) _then;

  /// Create a copy of TranscoderJob
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? videoId = null,
    Object? itemId = null,
  }) {
    return _then(_self.copyWith(
      videoId: null == videoId
          ? _self.videoId
          : videoId // ignore: cast_nullable_to_non_nullable
              as int,
      itemId: null == itemId
          ? _self.itemId
          : itemId // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// Adds pattern-matching-related methods to [TranscoderJob].
extension TranscoderJobPatterns on TranscoderJob {
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

  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Queued value)? queued,
    TResult Function(Processing value)? processing,
    required TResult orElse(),
  }) {
    final _that = this;
    switch (_that) {
      case Queued() when queued != null:
        return queued(_that);
      case Processing() when processing != null:
        return processing(_that);
      case _:
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

  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Queued value) queued,
    required TResult Function(Processing value) processing,
  }) {
    final _that = this;
    switch (_that) {
      case Queued():
        return queued(_that);
      case Processing():
        return processing(_that);
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

  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(Queued value)? queued,
    TResult? Function(Processing value)? processing,
  }) {
    final _that = this;
    switch (_that) {
      case Queued() when queued != null:
        return queued(_that);
      case Processing() when processing != null:
        return processing(_that);
      case _:
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
    final _that = this;
    switch (_that) {
      case Queued() when queued != null:
        return queued(_that.videoId, _that.itemId);
      case Processing() when processing != null:
        return processing(_that.videoId, _that.itemId, _that.progress);
      case _:
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

  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(@JsonKey(name: 'video_id') int videoId,
            @JsonKey(name: 'item_id') int itemId)
        queued,
    required TResult Function(@JsonKey(name: 'video_id') int videoId,
            @JsonKey(name: 'item_id') int itemId, double progress)
        processing,
  }) {
    final _that = this;
    switch (_that) {
      case Queued():
        return queued(_that.videoId, _that.itemId);
      case Processing():
        return processing(_that.videoId, _that.itemId, _that.progress);
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

  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(@JsonKey(name: 'video_id') int videoId,
            @JsonKey(name: 'item_id') int itemId)?
        queued,
    TResult? Function(@JsonKey(name: 'video_id') int videoId,
            @JsonKey(name: 'item_id') int itemId, double progress)?
        processing,
  }) {
    final _that = this;
    switch (_that) {
      case Queued() when queued != null:
        return queued(_that.videoId, _that.itemId);
      case Processing() when processing != null:
        return processing(_that.videoId, _that.itemId, _that.progress);
      case _:
        return null;
    }
  }
}

/// @nodoc
@JsonSerializable()
class Queued implements TranscoderJob {
  Queued(@JsonKey(name: 'video_id') this.videoId,
      @JsonKey(name: 'item_id') this.itemId,
      {final String? $type})
      : $type = $type ?? 'queued';
  factory Queued.fromJson(Map<String, dynamic> json) => _$QueuedFromJson(json);

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

  /// Create a copy of TranscoderJob
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $QueuedCopyWith<Queued> get copyWith =>
      _$QueuedCopyWithImpl<Queued>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$QueuedToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is Queued &&
            (identical(other.videoId, videoId) || other.videoId == videoId) &&
            (identical(other.itemId, itemId) || other.itemId == itemId));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, videoId, itemId);

  @override
  String toString() {
    return 'TranscoderJob.queued(videoId: $videoId, itemId: $itemId)';
  }
}

/// @nodoc
abstract mixin class $QueuedCopyWith<$Res>
    implements $TranscoderJobCopyWith<$Res> {
  factory $QueuedCopyWith(Queued value, $Res Function(Queued) _then) =
      _$QueuedCopyWithImpl;
  @override
  @useResult
  $Res call(
      {@JsonKey(name: 'video_id') int videoId,
      @JsonKey(name: 'item_id') int itemId});
}

/// @nodoc
class _$QueuedCopyWithImpl<$Res> implements $QueuedCopyWith<$Res> {
  _$QueuedCopyWithImpl(this._self, this._then);

  final Queued _self;
  final $Res Function(Queued) _then;

  /// Create a copy of TranscoderJob
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? videoId = null,
    Object? itemId = null,
  }) {
    return _then(Queued(
      null == videoId
          ? _self.videoId
          : videoId // ignore: cast_nullable_to_non_nullable
              as int,
      null == itemId
          ? _self.itemId
          : itemId // ignore: cast_nullable_to_non_nullable
              as int,
    ));
  }
}

/// @nodoc
@JsonSerializable()
class Processing implements TranscoderJob {
  Processing(@JsonKey(name: 'video_id') this.videoId,
      @JsonKey(name: 'item_id') this.itemId, this.progress,
      {final String? $type})
      : $type = $type ?? 'processing';
  factory Processing.fromJson(Map<String, dynamic> json) =>
      _$ProcessingFromJson(json);

// ignore: invalid_annotation_target
  @override
  @JsonKey(name: 'video_id')
  final int videoId;
// ignore: invalid_annotation_target
  @override
  @JsonKey(name: 'item_id')
  final int itemId;
  final double progress;

  @JsonKey(name: 'state')
  final String $type;

  /// Create a copy of TranscoderJob
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  @pragma('vm:prefer-inline')
  $ProcessingCopyWith<Processing> get copyWith =>
      _$ProcessingCopyWithImpl<Processing>(this, _$identity);

  @override
  Map<String, dynamic> toJson() {
    return _$ProcessingToJson(
      this,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is Processing &&
            (identical(other.videoId, videoId) || other.videoId == videoId) &&
            (identical(other.itemId, itemId) || other.itemId == itemId) &&
            (identical(other.progress, progress) ||
                other.progress == progress));
  }

  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  int get hashCode => Object.hash(runtimeType, videoId, itemId, progress);

  @override
  String toString() {
    return 'TranscoderJob.processing(videoId: $videoId, itemId: $itemId, progress: $progress)';
  }
}

/// @nodoc
abstract mixin class $ProcessingCopyWith<$Res>
    implements $TranscoderJobCopyWith<$Res> {
  factory $ProcessingCopyWith(
          Processing value, $Res Function(Processing) _then) =
      _$ProcessingCopyWithImpl;
  @override
  @useResult
  $Res call(
      {@JsonKey(name: 'video_id') int videoId,
      @JsonKey(name: 'item_id') int itemId,
      double progress});
}

/// @nodoc
class _$ProcessingCopyWithImpl<$Res> implements $ProcessingCopyWith<$Res> {
  _$ProcessingCopyWithImpl(this._self, this._then);

  final Processing _self;
  final $Res Function(Processing) _then;

  /// Create a copy of TranscoderJob
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $Res call({
    Object? videoId = null,
    Object? itemId = null,
    Object? progress = null,
  }) {
    return _then(Processing(
      null == videoId
          ? _self.videoId
          : videoId // ignore: cast_nullable_to_non_nullable
              as int,
      null == itemId
          ? _self.itemId
          : itemId // ignore: cast_nullable_to_non_nullable
              as int,
      null == progress
          ? _self.progress
          : progress // ignore: cast_nullable_to_non_nullable
              as double,
    ));
  }
}

// dart format on
