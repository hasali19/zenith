// dart format width=80
// GENERATED CODE, DO NOT EDIT BY HAND.
// ignore_for_file: type=lint
import 'package:drift/drift.dart';

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
    defaultConstraints: GeneratedColumn.constraintIsAlways(
      'PRIMARY KEY AUTOINCREMENT',
    ),
  );
  late final GeneratedColumn<String> uuid = GeneratedColumn<String>(
    'uuid',
    aliasedName,
    false,
    type: DriftSqlType.string,
    requiredDuringInsert: true,
    defaultConstraints: GeneratedColumn.constraintIsAlways('UNIQUE'),
  );
  late final GeneratedColumn<String> url = GeneratedColumn<String>(
    'url',
    aliasedName,
    false,
    type: DriftSqlType.string,
    requiredDuringInsert: true,
    defaultConstraints: GeneratedColumn.constraintIsAlways('UNIQUE'),
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
  );
  late final GeneratedColumn<int> itemId = GeneratedColumn<int>(
    'item_id',
    aliasedName,
    false,
    type: DriftSqlType.int,
    requiredDuringInsert: true,
  );
  late final GeneratedColumn<int> videoFileId = GeneratedColumn<int>(
    'video_file_id',
    aliasedName,
    false,
    type: DriftSqlType.int,
    requiredDuringInsert: true,
  );
  late final GeneratedColumn<String> path = GeneratedColumn<String>(
    'path',
    aliasedName,
    true,
    type: DriftSqlType.string,
    requiredDuringInsert: false,
  );
  late final GeneratedColumn<DateTime> createdAt = GeneratedColumn<DateTime>(
    'created_at',
    aliasedName,
    false,
    type: DriftSqlType.dateTime,
    requiredDuringInsert: true,
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
        DriftSqlType.dateTime,
        data['${effectivePrefix}created_at'],
      )!,
    );
  }

  @override
  DownloadedFiles createAlias(String alias) {
    return DownloadedFiles(attachedDatabase, alias);
  }
}

class DownloadedFilesData extends DataClass
    implements Insertable<DownloadedFilesData> {
  final String id;
  final int itemId;
  final int videoFileId;
  final String? path;
  final DateTime createdAt;
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
    map['created_at'] = Variable<DateTime>(createdAt);
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
      createdAt: serializer.fromJson<DateTime>(json['createdAt']),
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
      'createdAt': serializer.toJson<DateTime>(createdAt),
    };
  }

  DownloadedFilesData copyWith({
    String? id,
    int? itemId,
    int? videoFileId,
    Value<String?> path = const Value.absent(),
    DateTime? createdAt,
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
  final Value<DateTime> createdAt;
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
    required DateTime createdAt,
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
    Expression<DateTime>? createdAt,
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
    Value<DateTime>? createdAt,
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
      map['created_at'] = Variable<DateTime>(createdAt.value);
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

class DatabaseAtV2 extends GeneratedDatabase {
  DatabaseAtV2(QueryExecutor e) : super(e);
  late final Servers servers = Servers(this);
  late final DownloadedFiles downloadedFiles = DownloadedFiles(this);
  @override
  Iterable<TableInfo<Table, Object?>> get allTables =>
      allSchemaEntities.whereType<TableInfo<Table, Object?>>();
  @override
  List<DatabaseSchemaEntity> get allSchemaEntities => [
    servers,
    downloadedFiles,
  ];
  @override
  int get schemaVersion => 2;
  @override
  DriftDatabaseOptions get options =>
      const DriftDatabaseOptions(storeDateTimeAsText: true);
}
