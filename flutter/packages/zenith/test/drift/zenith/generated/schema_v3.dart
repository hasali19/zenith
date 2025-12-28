// dart format width=80
// GENERATED CODE, DO NOT EDIT BY HAND.
// ignore_for_file: type=lint
import 'package:drift/drift.dart';

class DownloadedFiles extends Table
    with TableInfo<DownloadedFiles, DownloadedFilesData> {
  @override
  final GeneratedDatabase attachedDatabase;
  final String? _alias;
  DownloadedFiles(this.attachedDatabase, [this._alias]);
  late final GeneratedColumn<String> id = GeneratedColumn<String>(
    'id',
    aliasedName,
    false,
    type: DriftSqlType.string,
    requiredDuringInsert: true,
    $customConstraints: 'NOT NULL',
  );
  late final GeneratedColumn<int> itemId = GeneratedColumn<int>(
    'item_id',
    aliasedName,
    false,
    type: DriftSqlType.int,
    requiredDuringInsert: true,
    $customConstraints: 'NOT NULL',
  );
  late final GeneratedColumn<int> videoFileId = GeneratedColumn<int>(
    'video_file_id',
    aliasedName,
    false,
    type: DriftSqlType.int,
    requiredDuringInsert: true,
    $customConstraints: 'NOT NULL',
  );
  late final GeneratedColumn<String> path = GeneratedColumn<String>(
    'path',
    aliasedName,
    true,
    type: DriftSqlType.string,
    requiredDuringInsert: false,
    $customConstraints: 'NULL',
  );
  late final GeneratedColumn<String> createdAt = GeneratedColumn<String>(
    'created_at',
    aliasedName,
    false,
    type: DriftSqlType.string,
    requiredDuringInsert: true,
    $customConstraints: 'NOT NULL',
  );
  @override
  List<GeneratedColumn> get $columns => [
    id,
    itemId,
    videoFileId,
    path,
    createdAt,
  ];
  @override
  String get aliasedName => _alias ?? actualTableName;
  @override
  String get actualTableName => $name;
  static const String $name = 'downloaded_files';
  @override
  Set<GeneratedColumn> get $primaryKey => {id};
  @override
  DownloadedFilesData map(Map<String, dynamic> data, {String? tablePrefix}) {
    final effectivePrefix = tablePrefix != null ? '$tablePrefix.' : '';
    return DownloadedFilesData(
      id: attachedDatabase.typeMapping.read(
        DriftSqlType.string,
        data['${effectivePrefix}id'],
      )!,
      itemId: attachedDatabase.typeMapping.read(
        DriftSqlType.int,
        data['${effectivePrefix}item_id'],
      )!,
      videoFileId: attachedDatabase.typeMapping.read(
        DriftSqlType.int,
        data['${effectivePrefix}video_file_id'],
      )!,
      path: attachedDatabase.typeMapping.read(
        DriftSqlType.string,
        data['${effectivePrefix}path'],
      ),
      createdAt: attachedDatabase.typeMapping.read(
        DriftSqlType.string,
        data['${effectivePrefix}created_at'],
      )!,
    );
  }

  @override
  DownloadedFiles createAlias(String alias) {
    return DownloadedFiles(attachedDatabase, alias);
  }

  @override
  List<String> get customConstraints => const ['PRIMARY KEY(id)'];
  @override
  bool get dontWriteConstraints => true;
}

class DownloadedFilesData extends DataClass
    implements Insertable<DownloadedFilesData> {
  final String id;
  final int itemId;
  final int videoFileId;
  final String? path;
  final String createdAt;
  const DownloadedFilesData({
    required this.id,
    required this.itemId,
    required this.videoFileId,
    this.path,
    required this.createdAt,
  });
  @override
  Map<String, Expression> toColumns(bool nullToAbsent) {
    final map = <String, Expression>{};
    map['id'] = Variable<String>(id);
    map['item_id'] = Variable<int>(itemId);
    map['video_file_id'] = Variable<int>(videoFileId);
    if (!nullToAbsent || path != null) {
      map['path'] = Variable<String>(path);
    }
    map['created_at'] = Variable<String>(createdAt);
    return map;
  }

  DownloadedFilesCompanion toCompanion(bool nullToAbsent) {
    return DownloadedFilesCompanion(
      id: Value(id),
      itemId: Value(itemId),
      videoFileId: Value(videoFileId),
      path: path == null && nullToAbsent ? const Value.absent() : Value(path),
      createdAt: Value(createdAt),
    );
  }

  factory DownloadedFilesData.fromJson(
    Map<String, dynamic> json, {
    ValueSerializer? serializer,
  }) {
    serializer ??= driftRuntimeOptions.defaultSerializer;
    return DownloadedFilesData(
      id: serializer.fromJson<String>(json['id']),
      itemId: serializer.fromJson<int>(json['itemId']),
      videoFileId: serializer.fromJson<int>(json['videoFileId']),
      path: serializer.fromJson<String?>(json['path']),
      createdAt: serializer.fromJson<String>(json['createdAt']),
    );
  }
  @override
  Map<String, dynamic> toJson({ValueSerializer? serializer}) {
    serializer ??= driftRuntimeOptions.defaultSerializer;
    return <String, dynamic>{
      'id': serializer.toJson<String>(id),
      'itemId': serializer.toJson<int>(itemId),
      'videoFileId': serializer.toJson<int>(videoFileId),
      'path': serializer.toJson<String?>(path),
      'createdAt': serializer.toJson<String>(createdAt),
    };
  }

  DownloadedFilesData copyWith({
    String? id,
    int? itemId,
    int? videoFileId,
    Value<String?> path = const Value.absent(),
    String? createdAt,
  }) => DownloadedFilesData(
    id: id ?? this.id,
    itemId: itemId ?? this.itemId,
    videoFileId: videoFileId ?? this.videoFileId,
    path: path.present ? path.value : this.path,
    createdAt: createdAt ?? this.createdAt,
  );
  DownloadedFilesData copyWithCompanion(DownloadedFilesCompanion data) {
    return DownloadedFilesData(
      id: data.id.present ? data.id.value : this.id,
      itemId: data.itemId.present ? data.itemId.value : this.itemId,
      videoFileId: data.videoFileId.present
          ? data.videoFileId.value
          : this.videoFileId,
      path: data.path.present ? data.path.value : this.path,
      createdAt: data.createdAt.present ? data.createdAt.value : this.createdAt,
    );
  }

  @override
  String toString() {
    return (StringBuffer('DownloadedFilesData(')
          ..write('id: $id, ')
          ..write('itemId: $itemId, ')
          ..write('videoFileId: $videoFileId, ')
          ..write('path: $path, ')
          ..write('createdAt: $createdAt')
          ..write(')'))
        .toString();
  }

  @override
  int get hashCode => Object.hash(id, itemId, videoFileId, path, createdAt);
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      (other is DownloadedFilesData &&
          other.id == this.id &&
          other.itemId == this.itemId &&
          other.videoFileId == this.videoFileId &&
          other.path == this.path &&
          other.createdAt == this.createdAt);
}

class DownloadedFilesCompanion extends UpdateCompanion<DownloadedFilesData> {
  final Value<String> id;
  final Value<int> itemId;
  final Value<int> videoFileId;
  final Value<String?> path;
  final Value<String> createdAt;
  final Value<int> rowid;
  const DownloadedFilesCompanion({
    this.id = const Value.absent(),
    this.itemId = const Value.absent(),
    this.videoFileId = const Value.absent(),
    this.path = const Value.absent(),
    this.createdAt = const Value.absent(),
    this.rowid = const Value.absent(),
  });
  DownloadedFilesCompanion.insert({
    required String id,
    required int itemId,
    required int videoFileId,
    this.path = const Value.absent(),
    required String createdAt,
    this.rowid = const Value.absent(),
  }) : id = Value(id),
       itemId = Value(itemId),
       videoFileId = Value(videoFileId),
       createdAt = Value(createdAt);
  static Insertable<DownloadedFilesData> custom({
    Expression<String>? id,
    Expression<int>? itemId,
    Expression<int>? videoFileId,
    Expression<String>? path,
    Expression<String>? createdAt,
    Expression<int>? rowid,
  }) {
    return RawValuesInsertable({
      if (id != null) 'id': id,
      if (itemId != null) 'item_id': itemId,
      if (videoFileId != null) 'video_file_id': videoFileId,
      if (path != null) 'path': path,
      if (createdAt != null) 'created_at': createdAt,
      if (rowid != null) 'rowid': rowid,
    });
  }

  DownloadedFilesCompanion copyWith({
    Value<String>? id,
    Value<int>? itemId,
    Value<int>? videoFileId,
    Value<String?>? path,
    Value<String>? createdAt,
    Value<int>? rowid,
  }) {
    return DownloadedFilesCompanion(
      id: id ?? this.id,
      itemId: itemId ?? this.itemId,
      videoFileId: videoFileId ?? this.videoFileId,
      path: path ?? this.path,
      createdAt: createdAt ?? this.createdAt,
      rowid: rowid ?? this.rowid,
    );
  }

  @override
  Map<String, Expression> toColumns(bool nullToAbsent) {
    final map = <String, Expression>{};
    if (id.present) {
      map['id'] = Variable<String>(id.value);
    }
    if (itemId.present) {
      map['item_id'] = Variable<int>(itemId.value);
    }
    if (videoFileId.present) {
      map['video_file_id'] = Variable<int>(videoFileId.value);
    }
    if (path.present) {
      map['path'] = Variable<String>(path.value);
    }
    if (createdAt.present) {
      map['created_at'] = Variable<String>(createdAt.value);
    }
    if (rowid.present) {
      map['rowid'] = Variable<int>(rowid.value);
    }
    return map;
  }

  @override
  String toString() {
    return (StringBuffer('DownloadedFilesCompanion(')
          ..write('id: $id, ')
          ..write('itemId: $itemId, ')
          ..write('videoFileId: $videoFileId, ')
          ..write('path: $path, ')
          ..write('createdAt: $createdAt, ')
          ..write('rowid: $rowid')
          ..write(')'))
        .toString();
  }
}

class MediaItems extends Table with TableInfo<MediaItems, MediaItemsData> {
  @override
  final GeneratedDatabase attachedDatabase;
  final String? _alias;
  MediaItems(this.attachedDatabase, [this._alias]);
  late final GeneratedColumn<int> id = GeneratedColumn<int>(
    'id',
    aliasedName,
    false,
    type: DriftSqlType.int,
    requiredDuringInsert: false,
    $customConstraints: 'NOT NULL',
  );
  late final GeneratedColumn<int> type = GeneratedColumn<int>(
    'type',
    aliasedName,
    false,
    type: DriftSqlType.int,
    requiredDuringInsert: true,
    $customConstraints: 'NOT NULL',
  );
  late final GeneratedColumn<String> name = GeneratedColumn<String>(
    'name',
    aliasedName,
    false,
    type: DriftSqlType.string,
    requiredDuringInsert: true,
    $customConstraints: 'NOT NULL',
  );
  late final GeneratedColumn<String> overview = GeneratedColumn<String>(
    'overview',
    aliasedName,
    true,
    type: DriftSqlType.string,
    requiredDuringInsert: false,
    $customConstraints: 'NULL',
  );
  late final GeneratedColumn<String> startDate = GeneratedColumn<String>(
    'start_date',
    aliasedName,
    true,
    type: DriftSqlType.string,
    requiredDuringInsert: false,
    $customConstraints: 'NULL',
  );
  late final GeneratedColumn<String> endDate = GeneratedColumn<String>(
    'end_date',
    aliasedName,
    true,
    type: DriftSqlType.string,
    requiredDuringInsert: false,
    $customConstraints: 'NULL',
  );
  late final GeneratedColumn<String> poster = GeneratedColumn<String>(
    'poster',
    aliasedName,
    true,
    type: DriftSqlType.string,
    requiredDuringInsert: false,
    $customConstraints: 'NULL',
  );
  late final GeneratedColumn<String> backdrop = GeneratedColumn<String>(
    'backdrop',
    aliasedName,
    true,
    type: DriftSqlType.string,
    requiredDuringInsert: false,
    $customConstraints: 'NULL',
  );
  late final GeneratedColumn<String> thumbnail = GeneratedColumn<String>(
    'thumbnail',
    aliasedName,
    true,
    type: DriftSqlType.string,
    requiredDuringInsert: false,
    $customConstraints: 'NULL',
  );
  @override
  List<GeneratedColumn> get $columns => [
    id,
    type,
    name,
    overview,
    startDate,
    endDate,
    poster,
    backdrop,
    thumbnail,
  ];
  @override
  String get aliasedName => _alias ?? actualTableName;
  @override
  String get actualTableName => $name;
  static const String $name = 'media_items';
  @override
  Set<GeneratedColumn> get $primaryKey => {id};
  @override
  MediaItemsData map(Map<String, dynamic> data, {String? tablePrefix}) {
    final effectivePrefix = tablePrefix != null ? '$tablePrefix.' : '';
    return MediaItemsData(
      id: attachedDatabase.typeMapping.read(
        DriftSqlType.int,
        data['${effectivePrefix}id'],
      )!,
      type: attachedDatabase.typeMapping.read(
        DriftSqlType.int,
        data['${effectivePrefix}type'],
      )!,
      name: attachedDatabase.typeMapping.read(
        DriftSqlType.string,
        data['${effectivePrefix}name'],
      )!,
      overview: attachedDatabase.typeMapping.read(
        DriftSqlType.string,
        data['${effectivePrefix}overview'],
      ),
      startDate: attachedDatabase.typeMapping.read(
        DriftSqlType.string,
        data['${effectivePrefix}start_date'],
      ),
      endDate: attachedDatabase.typeMapping.read(
        DriftSqlType.string,
        data['${effectivePrefix}end_date'],
      ),
      poster: attachedDatabase.typeMapping.read(
        DriftSqlType.string,
        data['${effectivePrefix}poster'],
      ),
      backdrop: attachedDatabase.typeMapping.read(
        DriftSqlType.string,
        data['${effectivePrefix}backdrop'],
      ),
      thumbnail: attachedDatabase.typeMapping.read(
        DriftSqlType.string,
        data['${effectivePrefix}thumbnail'],
      ),
    );
  }

  @override
  MediaItems createAlias(String alias) {
    return MediaItems(attachedDatabase, alias);
  }

  @override
  List<String> get customConstraints => const ['PRIMARY KEY(id)'];
  @override
  bool get dontWriteConstraints => true;
}

class MediaItemsData extends DataClass implements Insertable<MediaItemsData> {
  final int id;
  final int type;
  final String name;
  final String? overview;
  final String? startDate;
  final String? endDate;
  final String? poster;
  final String? backdrop;
  final String? thumbnail;
  const MediaItemsData({
    required this.id,
    required this.type,
    required this.name,
    this.overview,
    this.startDate,
    this.endDate,
    this.poster,
    this.backdrop,
    this.thumbnail,
  });
  @override
  Map<String, Expression> toColumns(bool nullToAbsent) {
    final map = <String, Expression>{};
    map['id'] = Variable<int>(id);
    map['type'] = Variable<int>(type);
    map['name'] = Variable<String>(name);
    if (!nullToAbsent || overview != null) {
      map['overview'] = Variable<String>(overview);
    }
    if (!nullToAbsent || startDate != null) {
      map['start_date'] = Variable<String>(startDate);
    }
    if (!nullToAbsent || endDate != null) {
      map['end_date'] = Variable<String>(endDate);
    }
    if (!nullToAbsent || poster != null) {
      map['poster'] = Variable<String>(poster);
    }
    if (!nullToAbsent || backdrop != null) {
      map['backdrop'] = Variable<String>(backdrop);
    }
    if (!nullToAbsent || thumbnail != null) {
      map['thumbnail'] = Variable<String>(thumbnail);
    }
    return map;
  }

  MediaItemsCompanion toCompanion(bool nullToAbsent) {
    return MediaItemsCompanion(
      id: Value(id),
      type: Value(type),
      name: Value(name),
      overview: overview == null && nullToAbsent
          ? const Value.absent()
          : Value(overview),
      startDate: startDate == null && nullToAbsent
          ? const Value.absent()
          : Value(startDate),
      endDate: endDate == null && nullToAbsent
          ? const Value.absent()
          : Value(endDate),
      poster: poster == null && nullToAbsent
          ? const Value.absent()
          : Value(poster),
      backdrop: backdrop == null && nullToAbsent
          ? const Value.absent()
          : Value(backdrop),
      thumbnail: thumbnail == null && nullToAbsent
          ? const Value.absent()
          : Value(thumbnail),
    );
  }

  factory MediaItemsData.fromJson(
    Map<String, dynamic> json, {
    ValueSerializer? serializer,
  }) {
    serializer ??= driftRuntimeOptions.defaultSerializer;
    return MediaItemsData(
      id: serializer.fromJson<int>(json['id']),
      type: serializer.fromJson<int>(json['type']),
      name: serializer.fromJson<String>(json['name']),
      overview: serializer.fromJson<String?>(json['overview']),
      startDate: serializer.fromJson<String?>(json['startDate']),
      endDate: serializer.fromJson<String?>(json['endDate']),
      poster: serializer.fromJson<String?>(json['poster']),
      backdrop: serializer.fromJson<String?>(json['backdrop']),
      thumbnail: serializer.fromJson<String?>(json['thumbnail']),
    );
  }
  @override
  Map<String, dynamic> toJson({ValueSerializer? serializer}) {
    serializer ??= driftRuntimeOptions.defaultSerializer;
    return <String, dynamic>{
      'id': serializer.toJson<int>(id),
      'type': serializer.toJson<int>(type),
      'name': serializer.toJson<String>(name),
      'overview': serializer.toJson<String?>(overview),
      'startDate': serializer.toJson<String?>(startDate),
      'endDate': serializer.toJson<String?>(endDate),
      'poster': serializer.toJson<String?>(poster),
      'backdrop': serializer.toJson<String?>(backdrop),
      'thumbnail': serializer.toJson<String?>(thumbnail),
    };
  }

  MediaItemsData copyWith({
    int? id,
    int? type,
    String? name,
    Value<String?> overview = const Value.absent(),
    Value<String?> startDate = const Value.absent(),
    Value<String?> endDate = const Value.absent(),
    Value<String?> poster = const Value.absent(),
    Value<String?> backdrop = const Value.absent(),
    Value<String?> thumbnail = const Value.absent(),
  }) => MediaItemsData(
    id: id ?? this.id,
    type: type ?? this.type,
    name: name ?? this.name,
    overview: overview.present ? overview.value : this.overview,
    startDate: startDate.present ? startDate.value : this.startDate,
    endDate: endDate.present ? endDate.value : this.endDate,
    poster: poster.present ? poster.value : this.poster,
    backdrop: backdrop.present ? backdrop.value : this.backdrop,
    thumbnail: thumbnail.present ? thumbnail.value : this.thumbnail,
  );
  MediaItemsData copyWithCompanion(MediaItemsCompanion data) {
    return MediaItemsData(
      id: data.id.present ? data.id.value : this.id,
      type: data.type.present ? data.type.value : this.type,
      name: data.name.present ? data.name.value : this.name,
      overview: data.overview.present ? data.overview.value : this.overview,
      startDate: data.startDate.present ? data.startDate.value : this.startDate,
      endDate: data.endDate.present ? data.endDate.value : this.endDate,
      poster: data.poster.present ? data.poster.value : this.poster,
      backdrop: data.backdrop.present ? data.backdrop.value : this.backdrop,
      thumbnail: data.thumbnail.present ? data.thumbnail.value : this.thumbnail,
    );
  }

  @override
  String toString() {
    return (StringBuffer('MediaItemsData(')
          ..write('id: $id, ')
          ..write('type: $type, ')
          ..write('name: $name, ')
          ..write('overview: $overview, ')
          ..write('startDate: $startDate, ')
          ..write('endDate: $endDate, ')
          ..write('poster: $poster, ')
          ..write('backdrop: $backdrop, ')
          ..write('thumbnail: $thumbnail')
          ..write(')'))
        .toString();
  }

  @override
  int get hashCode => Object.hash(
    id,
    type,
    name,
    overview,
    startDate,
    endDate,
    poster,
    backdrop,
    thumbnail,
  );
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      (other is MediaItemsData &&
          other.id == this.id &&
          other.type == this.type &&
          other.name == this.name &&
          other.overview == this.overview &&
          other.startDate == this.startDate &&
          other.endDate == this.endDate &&
          other.poster == this.poster &&
          other.backdrop == this.backdrop &&
          other.thumbnail == this.thumbnail);
}

class MediaItemsCompanion extends UpdateCompanion<MediaItemsData> {
  final Value<int> id;
  final Value<int> type;
  final Value<String> name;
  final Value<String?> overview;
  final Value<String?> startDate;
  final Value<String?> endDate;
  final Value<String?> poster;
  final Value<String?> backdrop;
  final Value<String?> thumbnail;
  const MediaItemsCompanion({
    this.id = const Value.absent(),
    this.type = const Value.absent(),
    this.name = const Value.absent(),
    this.overview = const Value.absent(),
    this.startDate = const Value.absent(),
    this.endDate = const Value.absent(),
    this.poster = const Value.absent(),
    this.backdrop = const Value.absent(),
    this.thumbnail = const Value.absent(),
  });
  MediaItemsCompanion.insert({
    this.id = const Value.absent(),
    required int type,
    required String name,
    this.overview = const Value.absent(),
    this.startDate = const Value.absent(),
    this.endDate = const Value.absent(),
    this.poster = const Value.absent(),
    this.backdrop = const Value.absent(),
    this.thumbnail = const Value.absent(),
  }) : type = Value(type),
       name = Value(name);
  static Insertable<MediaItemsData> custom({
    Expression<int>? id,
    Expression<int>? type,
    Expression<String>? name,
    Expression<String>? overview,
    Expression<String>? startDate,
    Expression<String>? endDate,
    Expression<String>? poster,
    Expression<String>? backdrop,
    Expression<String>? thumbnail,
  }) {
    return RawValuesInsertable({
      if (id != null) 'id': id,
      if (type != null) 'type': type,
      if (name != null) 'name': name,
      if (overview != null) 'overview': overview,
      if (startDate != null) 'start_date': startDate,
      if (endDate != null) 'end_date': endDate,
      if (poster != null) 'poster': poster,
      if (backdrop != null) 'backdrop': backdrop,
      if (thumbnail != null) 'thumbnail': thumbnail,
    });
  }

  MediaItemsCompanion copyWith({
    Value<int>? id,
    Value<int>? type,
    Value<String>? name,
    Value<String?>? overview,
    Value<String?>? startDate,
    Value<String?>? endDate,
    Value<String?>? poster,
    Value<String?>? backdrop,
    Value<String?>? thumbnail,
  }) {
    return MediaItemsCompanion(
      id: id ?? this.id,
      type: type ?? this.type,
      name: name ?? this.name,
      overview: overview ?? this.overview,
      startDate: startDate ?? this.startDate,
      endDate: endDate ?? this.endDate,
      poster: poster ?? this.poster,
      backdrop: backdrop ?? this.backdrop,
      thumbnail: thumbnail ?? this.thumbnail,
    );
  }

  @override
  Map<String, Expression> toColumns(bool nullToAbsent) {
    final map = <String, Expression>{};
    if (id.present) {
      map['id'] = Variable<int>(id.value);
    }
    if (type.present) {
      map['type'] = Variable<int>(type.value);
    }
    if (name.present) {
      map['name'] = Variable<String>(name.value);
    }
    if (overview.present) {
      map['overview'] = Variable<String>(overview.value);
    }
    if (startDate.present) {
      map['start_date'] = Variable<String>(startDate.value);
    }
    if (endDate.present) {
      map['end_date'] = Variable<String>(endDate.value);
    }
    if (poster.present) {
      map['poster'] = Variable<String>(poster.value);
    }
    if (backdrop.present) {
      map['backdrop'] = Variable<String>(backdrop.value);
    }
    if (thumbnail.present) {
      map['thumbnail'] = Variable<String>(thumbnail.value);
    }
    return map;
  }

  @override
  String toString() {
    return (StringBuffer('MediaItemsCompanion(')
          ..write('id: $id, ')
          ..write('type: $type, ')
          ..write('name: $name, ')
          ..write('overview: $overview, ')
          ..write('startDate: $startDate, ')
          ..write('endDate: $endDate, ')
          ..write('poster: $poster, ')
          ..write('backdrop: $backdrop, ')
          ..write('thumbnail: $thumbnail')
          ..write(')'))
        .toString();
  }
}

class Servers extends Table with TableInfo<Servers, ServersData> {
  @override
  final GeneratedDatabase attachedDatabase;
  final String? _alias;
  Servers(this.attachedDatabase, [this._alias]);
  late final GeneratedColumn<int> id = GeneratedColumn<int>(
    'id',
    aliasedName,
    false,
    hasAutoIncrement: true,
    type: DriftSqlType.int,
    requiredDuringInsert: false,
    $customConstraints: 'NOT NULL PRIMARY KEY AUTOINCREMENT',
  );
  late final GeneratedColumn<String> uuid = GeneratedColumn<String>(
    'uuid',
    aliasedName,
    false,
    type: DriftSqlType.string,
    requiredDuringInsert: true,
    $customConstraints: 'NOT NULL UNIQUE',
  );
  late final GeneratedColumn<String> url = GeneratedColumn<String>(
    'url',
    aliasedName,
    false,
    type: DriftSqlType.string,
    requiredDuringInsert: true,
    $customConstraints: 'NOT NULL UNIQUE',
  );
  @override
  List<GeneratedColumn> get $columns => [id, uuid, url];
  @override
  String get aliasedName => _alias ?? actualTableName;
  @override
  String get actualTableName => $name;
  static const String $name = 'servers';
  @override
  Set<GeneratedColumn> get $primaryKey => {id};
  @override
  ServersData map(Map<String, dynamic> data, {String? tablePrefix}) {
    final effectivePrefix = tablePrefix != null ? '$tablePrefix.' : '';
    return ServersData(
      id: attachedDatabase.typeMapping.read(
        DriftSqlType.int,
        data['${effectivePrefix}id'],
      )!,
      uuid: attachedDatabase.typeMapping.read(
        DriftSqlType.string,
        data['${effectivePrefix}uuid'],
      )!,
      url: attachedDatabase.typeMapping.read(
        DriftSqlType.string,
        data['${effectivePrefix}url'],
      )!,
    );
  }

  @override
  Servers createAlias(String alias) {
    return Servers(attachedDatabase, alias);
  }

  @override
  bool get dontWriteConstraints => true;
}

class ServersData extends DataClass implements Insertable<ServersData> {
  final int id;
  final String uuid;
  final String url;
  const ServersData({required this.id, required this.uuid, required this.url});
  @override
  Map<String, Expression> toColumns(bool nullToAbsent) {
    final map = <String, Expression>{};
    map['id'] = Variable<int>(id);
    map['uuid'] = Variable<String>(uuid);
    map['url'] = Variable<String>(url);
    return map;
  }

  ServersCompanion toCompanion(bool nullToAbsent) {
    return ServersCompanion(id: Value(id), uuid: Value(uuid), url: Value(url));
  }

  factory ServersData.fromJson(
    Map<String, dynamic> json, {
    ValueSerializer? serializer,
  }) {
    serializer ??= driftRuntimeOptions.defaultSerializer;
    return ServersData(
      id: serializer.fromJson<int>(json['id']),
      uuid: serializer.fromJson<String>(json['uuid']),
      url: serializer.fromJson<String>(json['url']),
    );
  }
  @override
  Map<String, dynamic> toJson({ValueSerializer? serializer}) {
    serializer ??= driftRuntimeOptions.defaultSerializer;
    return <String, dynamic>{
      'id': serializer.toJson<int>(id),
      'uuid': serializer.toJson<String>(uuid),
      'url': serializer.toJson<String>(url),
    };
  }

  ServersData copyWith({int? id, String? uuid, String? url}) => ServersData(
    id: id ?? this.id,
    uuid: uuid ?? this.uuid,
    url: url ?? this.url,
  );
  ServersData copyWithCompanion(ServersCompanion data) {
    return ServersData(
      id: data.id.present ? data.id.value : this.id,
      uuid: data.uuid.present ? data.uuid.value : this.uuid,
      url: data.url.present ? data.url.value : this.url,
    );
  }

  @override
  String toString() {
    return (StringBuffer('ServersData(')
          ..write('id: $id, ')
          ..write('uuid: $uuid, ')
          ..write('url: $url')
          ..write(')'))
        .toString();
  }

  @override
  int get hashCode => Object.hash(id, uuid, url);
  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      (other is ServersData &&
          other.id == this.id &&
          other.uuid == this.uuid &&
          other.url == this.url);
}

class ServersCompanion extends UpdateCompanion<ServersData> {
  final Value<int> id;
  final Value<String> uuid;
  final Value<String> url;
  const ServersCompanion({
    this.id = const Value.absent(),
    this.uuid = const Value.absent(),
    this.url = const Value.absent(),
  });
  ServersCompanion.insert({
    this.id = const Value.absent(),
    required String uuid,
    required String url,
  }) : uuid = Value(uuid),
       url = Value(url);
  static Insertable<ServersData> custom({
    Expression<int>? id,
    Expression<String>? uuid,
    Expression<String>? url,
  }) {
    return RawValuesInsertable({
      if (id != null) 'id': id,
      if (uuid != null) 'uuid': uuid,
      if (url != null) 'url': url,
    });
  }

  ServersCompanion copyWith({
    Value<int>? id,
    Value<String>? uuid,
    Value<String>? url,
  }) {
    return ServersCompanion(
      id: id ?? this.id,
      uuid: uuid ?? this.uuid,
      url: url ?? this.url,
    );
  }

  @override
  Map<String, Expression> toColumns(bool nullToAbsent) {
    final map = <String, Expression>{};
    if (id.present) {
      map['id'] = Variable<int>(id.value);
    }
    if (uuid.present) {
      map['uuid'] = Variable<String>(uuid.value);
    }
    if (url.present) {
      map['url'] = Variable<String>(url.value);
    }
    return map;
  }

  @override
  String toString() {
    return (StringBuffer('ServersCompanion(')
          ..write('id: $id, ')
          ..write('uuid: $uuid, ')
          ..write('url: $url')
          ..write(')'))
        .toString();
  }
}

class DatabaseAtV3 extends GeneratedDatabase {
  DatabaseAtV3(QueryExecutor e) : super(e);
  late final DownloadedFiles downloadedFiles = DownloadedFiles(this);
  late final MediaItems mediaItems = MediaItems(this);
  late final Servers servers = Servers(this);
  @override
  Iterable<TableInfo<Table, Object?>> get allTables =>
      allSchemaEntities.whereType<TableInfo<Table, Object?>>();
  @override
  List<DatabaseSchemaEntity> get allSchemaEntities => [
    downloadedFiles,
    mediaItems,
    servers,
  ];
  @override
  int get schemaVersion => 3;
  @override
  DriftDatabaseOptions get options =>
      const DriftDatabaseOptions(storeDateTimeAsText: true);
}
