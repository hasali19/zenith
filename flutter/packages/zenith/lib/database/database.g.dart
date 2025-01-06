// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'database.dart';

// ignore_for_file: type=lint
class $DownloadedFilesTable extends DownloadedFiles
    with TableInfo<$DownloadedFilesTable, DownloadedFile> {
  @override
  final GeneratedDatabase attachedDatabase;
  final String? _alias;
  $DownloadedFilesTable(this.attachedDatabase, [this._alias]);
  static const VerificationMeta _idMeta = const VerificationMeta('id');
  @override
  late final GeneratedColumn<String> id = GeneratedColumn<String>(
      'id', aliasedName, false,
      type: DriftSqlType.string, requiredDuringInsert: true);
  static const VerificationMeta _itemIdMeta = const VerificationMeta('itemId');
  @override
  late final GeneratedColumn<int> itemId = GeneratedColumn<int>(
      'item_id', aliasedName, false,
      type: DriftSqlType.int, requiredDuringInsert: true);
  static const VerificationMeta _videoFileIdMeta =
      const VerificationMeta('videoFileId');
  @override
  late final GeneratedColumn<int> videoFileId = GeneratedColumn<int>(
      'video_file_id', aliasedName, false,
      type: DriftSqlType.int, requiredDuringInsert: true);
  static const VerificationMeta _pathMeta = const VerificationMeta('path');
  @override
  late final GeneratedColumn<String> path = GeneratedColumn<String>(
      'path', aliasedName, true,
      type: DriftSqlType.string, requiredDuringInsert: false);
  static const VerificationMeta _createdAtMeta =
      const VerificationMeta('createdAt');
  @override
  late final GeneratedColumn<DateTime> createdAt = GeneratedColumn<DateTime>(
      'created_at', aliasedName, false,
      type: DriftSqlType.dateTime, requiredDuringInsert: true);
  @override
  List<GeneratedColumn> get $columns =>
      [id, itemId, videoFileId, path, createdAt];
  @override
  String get aliasedName => _alias ?? actualTableName;
  @override
  String get actualTableName => $name;
  static const String $name = 'downloaded_files';
  @override
  VerificationContext validateIntegrity(Insertable<DownloadedFile> instance,
      {bool isInserting = false}) {
    final context = VerificationContext();
    final data = instance.toColumns(true);
    if (data.containsKey('id')) {
      context.handle(_idMeta, id.isAcceptableOrUnknown(data['id']!, _idMeta));
    } else if (isInserting) {
      context.missing(_idMeta);
    }
    if (data.containsKey('item_id')) {
      context.handle(_itemIdMeta,
          itemId.isAcceptableOrUnknown(data['item_id']!, _itemIdMeta));
    } else if (isInserting) {
      context.missing(_itemIdMeta);
    }
    if (data.containsKey('video_file_id')) {
      context.handle(
          _videoFileIdMeta,
          videoFileId.isAcceptableOrUnknown(
              data['video_file_id']!, _videoFileIdMeta));
    } else if (isInserting) {
      context.missing(_videoFileIdMeta);
    }
    if (data.containsKey('path')) {
      context.handle(
          _pathMeta, path.isAcceptableOrUnknown(data['path']!, _pathMeta));
    }
    if (data.containsKey('created_at')) {
      context.handle(_createdAtMeta,
          createdAt.isAcceptableOrUnknown(data['created_at']!, _createdAtMeta));
    } else if (isInserting) {
      context.missing(_createdAtMeta);
    }
    return context;
  }

  @override
  Set<GeneratedColumn> get $primaryKey => {id};
  @override
  DownloadedFile map(Map<String, dynamic> data, {String? tablePrefix}) {
    final effectivePrefix = tablePrefix != null ? '$tablePrefix.' : '';
    return DownloadedFile(
      id: attachedDatabase.typeMapping
          .read(DriftSqlType.string, data['${effectivePrefix}id'])!,
      itemId: attachedDatabase.typeMapping
          .read(DriftSqlType.int, data['${effectivePrefix}item_id'])!,
      videoFileId: attachedDatabase.typeMapping
          .read(DriftSqlType.int, data['${effectivePrefix}video_file_id'])!,
      path: attachedDatabase.typeMapping
          .read(DriftSqlType.string, data['${effectivePrefix}path']),
      createdAt: attachedDatabase.typeMapping
          .read(DriftSqlType.dateTime, data['${effectivePrefix}created_at'])!,
    );
  }

  @override
  $DownloadedFilesTable createAlias(String alias) {
    return $DownloadedFilesTable(attachedDatabase, alias);
  }
}

class DownloadedFile extends DataClass implements Insertable<DownloadedFile> {
  final String id;
  final int itemId;
  final int videoFileId;
  final String? path;
  final DateTime createdAt;
  const DownloadedFile(
      {required this.id,
      required this.itemId,
      required this.videoFileId,
      this.path,
      required this.createdAt});
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

  factory DownloadedFile.fromJson(Map<String, dynamic> json,
      {ValueSerializer? serializer}) {
    serializer ??= driftRuntimeOptions.defaultSerializer;
    return DownloadedFile(
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

  DownloadedFile copyWith(
          {String? id,
          int? itemId,
          int? videoFileId,
          Value<String?> path = const Value.absent(),
          DateTime? createdAt}) =>
      DownloadedFile(
        id: id ?? this.id,
        itemId: itemId ?? this.itemId,
        videoFileId: videoFileId ?? this.videoFileId,
        path: path.present ? path.value : this.path,
        createdAt: createdAt ?? this.createdAt,
      );
  DownloadedFile copyWithCompanion(DownloadedFilesCompanion data) {
    return DownloadedFile(
      id: data.id.present ? data.id.value : this.id,
      itemId: data.itemId.present ? data.itemId.value : this.itemId,
      videoFileId:
          data.videoFileId.present ? data.videoFileId.value : this.videoFileId,
      path: data.path.present ? data.path.value : this.path,
      createdAt: data.createdAt.present ? data.createdAt.value : this.createdAt,
    );
  }

  @override
  String toString() {
    return (StringBuffer('DownloadedFile(')
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
      (other is DownloadedFile &&
          other.id == this.id &&
          other.itemId == this.itemId &&
          other.videoFileId == this.videoFileId &&
          other.path == this.path &&
          other.createdAt == this.createdAt);
}

class DownloadedFilesCompanion extends UpdateCompanion<DownloadedFile> {
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
  })  : id = Value(id),
        itemId = Value(itemId),
        videoFileId = Value(videoFileId),
        createdAt = Value(createdAt);
  static Insertable<DownloadedFile> custom({
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

  DownloadedFilesCompanion copyWith(
      {Value<String>? id,
      Value<int>? itemId,
      Value<int>? videoFileId,
      Value<String?>? path,
      Value<DateTime>? createdAt,
      Value<int>? rowid}) {
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

abstract class _$AppDatabase extends GeneratedDatabase {
  _$AppDatabase(QueryExecutor e) : super(e);
  $AppDatabaseManager get managers => $AppDatabaseManager(this);
  late final $DownloadedFilesTable downloadedFiles =
      $DownloadedFilesTable(this);
  @override
  Iterable<TableInfo<Table, Object?>> get allTables =>
      allSchemaEntities.whereType<TableInfo<Table, Object?>>();
  @override
  List<DatabaseSchemaEntity> get allSchemaEntities => [downloadedFiles];
  @override
  DriftDatabaseOptions get options =>
      const DriftDatabaseOptions(storeDateTimeAsText: true);
}

typedef $$DownloadedFilesTableCreateCompanionBuilder = DownloadedFilesCompanion
    Function({
  required String id,
  required int itemId,
  required int videoFileId,
  Value<String?> path,
  required DateTime createdAt,
  Value<int> rowid,
});
typedef $$DownloadedFilesTableUpdateCompanionBuilder = DownloadedFilesCompanion
    Function({
  Value<String> id,
  Value<int> itemId,
  Value<int> videoFileId,
  Value<String?> path,
  Value<DateTime> createdAt,
  Value<int> rowid,
});

class $$DownloadedFilesTableFilterComposer
    extends Composer<_$AppDatabase, $DownloadedFilesTable> {
  $$DownloadedFilesTableFilterComposer({
    required super.$db,
    required super.$table,
    super.joinBuilder,
    super.$addJoinBuilderToRootComposer,
    super.$removeJoinBuilderFromRootComposer,
  });
  ColumnFilters<String> get id => $composableBuilder(
      column: $table.id, builder: (column) => ColumnFilters(column));

  ColumnFilters<int> get itemId => $composableBuilder(
      column: $table.itemId, builder: (column) => ColumnFilters(column));

  ColumnFilters<int> get videoFileId => $composableBuilder(
      column: $table.videoFileId, builder: (column) => ColumnFilters(column));

  ColumnFilters<String> get path => $composableBuilder(
      column: $table.path, builder: (column) => ColumnFilters(column));

  ColumnFilters<DateTime> get createdAt => $composableBuilder(
      column: $table.createdAt, builder: (column) => ColumnFilters(column));
}

class $$DownloadedFilesTableOrderingComposer
    extends Composer<_$AppDatabase, $DownloadedFilesTable> {
  $$DownloadedFilesTableOrderingComposer({
    required super.$db,
    required super.$table,
    super.joinBuilder,
    super.$addJoinBuilderToRootComposer,
    super.$removeJoinBuilderFromRootComposer,
  });
  ColumnOrderings<String> get id => $composableBuilder(
      column: $table.id, builder: (column) => ColumnOrderings(column));

  ColumnOrderings<int> get itemId => $composableBuilder(
      column: $table.itemId, builder: (column) => ColumnOrderings(column));

  ColumnOrderings<int> get videoFileId => $composableBuilder(
      column: $table.videoFileId, builder: (column) => ColumnOrderings(column));

  ColumnOrderings<String> get path => $composableBuilder(
      column: $table.path, builder: (column) => ColumnOrderings(column));

  ColumnOrderings<DateTime> get createdAt => $composableBuilder(
      column: $table.createdAt, builder: (column) => ColumnOrderings(column));
}

class $$DownloadedFilesTableAnnotationComposer
    extends Composer<_$AppDatabase, $DownloadedFilesTable> {
  $$DownloadedFilesTableAnnotationComposer({
    required super.$db,
    required super.$table,
    super.joinBuilder,
    super.$addJoinBuilderToRootComposer,
    super.$removeJoinBuilderFromRootComposer,
  });
  GeneratedColumn<String> get id =>
      $composableBuilder(column: $table.id, builder: (column) => column);

  GeneratedColumn<int> get itemId =>
      $composableBuilder(column: $table.itemId, builder: (column) => column);

  GeneratedColumn<int> get videoFileId => $composableBuilder(
      column: $table.videoFileId, builder: (column) => column);

  GeneratedColumn<String> get path =>
      $composableBuilder(column: $table.path, builder: (column) => column);

  GeneratedColumn<DateTime> get createdAt =>
      $composableBuilder(column: $table.createdAt, builder: (column) => column);
}

class $$DownloadedFilesTableTableManager extends RootTableManager<
    _$AppDatabase,
    $DownloadedFilesTable,
    DownloadedFile,
    $$DownloadedFilesTableFilterComposer,
    $$DownloadedFilesTableOrderingComposer,
    $$DownloadedFilesTableAnnotationComposer,
    $$DownloadedFilesTableCreateCompanionBuilder,
    $$DownloadedFilesTableUpdateCompanionBuilder,
    (
      DownloadedFile,
      BaseReferences<_$AppDatabase, $DownloadedFilesTable, DownloadedFile>
    ),
    DownloadedFile,
    PrefetchHooks Function()> {
  $$DownloadedFilesTableTableManager(
      _$AppDatabase db, $DownloadedFilesTable table)
      : super(TableManagerState(
          db: db,
          table: table,
          createFilteringComposer: () =>
              $$DownloadedFilesTableFilterComposer($db: db, $table: table),
          createOrderingComposer: () =>
              $$DownloadedFilesTableOrderingComposer($db: db, $table: table),
          createComputedFieldComposer: () =>
              $$DownloadedFilesTableAnnotationComposer($db: db, $table: table),
          updateCompanionCallback: ({
            Value<String> id = const Value.absent(),
            Value<int> itemId = const Value.absent(),
            Value<int> videoFileId = const Value.absent(),
            Value<String?> path = const Value.absent(),
            Value<DateTime> createdAt = const Value.absent(),
            Value<int> rowid = const Value.absent(),
          }) =>
              DownloadedFilesCompanion(
            id: id,
            itemId: itemId,
            videoFileId: videoFileId,
            path: path,
            createdAt: createdAt,
            rowid: rowid,
          ),
          createCompanionCallback: ({
            required String id,
            required int itemId,
            required int videoFileId,
            Value<String?> path = const Value.absent(),
            required DateTime createdAt,
            Value<int> rowid = const Value.absent(),
          }) =>
              DownloadedFilesCompanion.insert(
            id: id,
            itemId: itemId,
            videoFileId: videoFileId,
            path: path,
            createdAt: createdAt,
            rowid: rowid,
          ),
          withReferenceMapper: (p0) => p0
              .map((e) => (e.readTable(table), BaseReferences(db, table, e)))
              .toList(),
          prefetchHooksCallback: null,
        ));
}

typedef $$DownloadedFilesTableProcessedTableManager = ProcessedTableManager<
    _$AppDatabase,
    $DownloadedFilesTable,
    DownloadedFile,
    $$DownloadedFilesTableFilterComposer,
    $$DownloadedFilesTableOrderingComposer,
    $$DownloadedFilesTableAnnotationComposer,
    $$DownloadedFilesTableCreateCompanionBuilder,
    $$DownloadedFilesTableUpdateCompanionBuilder,
    (
      DownloadedFile,
      BaseReferences<_$AppDatabase, $DownloadedFilesTable, DownloadedFile>
    ),
    DownloadedFile,
    PrefetchHooks Function()>;

class $AppDatabaseManager {
  final _$AppDatabase _db;
  $AppDatabaseManager(this._db);
  $$DownloadedFilesTableTableManager get downloadedFiles =>
      $$DownloadedFilesTableTableManager(_db, _db.downloadedFiles);
}

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

String _$databaseHash() => r'd7a35a83ad0e3d2e5c3a40b438bc9be186d6af93';

/// See also [database].
@ProviderFor(database)
final databaseProvider = AutoDisposeProvider<AppDatabase>.internal(
  database,
  name: r'databaseProvider',
  debugGetCreateSourceHash:
      const bool.fromEnvironment('dart.vm.product') ? null : _$databaseHash,
  dependencies: null,
  allTransitiveDependencies: null,
);

@Deprecated('Will be removed in 3.0. Use Ref instead')
// ignore: unused_element
typedef DatabaseRef = AutoDisposeProviderRef<AppDatabase>;
// ignore_for_file: type=lint
// ignore_for_file: subtype_of_sealed_class, invalid_use_of_internal_member, invalid_use_of_visible_for_testing_member, deprecated_member_use_from_same_package
