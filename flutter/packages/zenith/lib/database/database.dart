import 'package:drift/drift.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

import './database_stub.dart'
    if (dart.library.ffi) './database_io.dart'
    if (dart.library.js_interop) './database_web.dart';

part 'database.g.dart';

class DownloadedFiles extends Table {
  TextColumn get id => text()();
  IntColumn get itemId => integer()();
  IntColumn get videoFileId => integer()();
  TextColumn get path => text().nullable()();
  DateTimeColumn get createdAt => dateTime()();

  @override
  Set<Column<Object>>? get primaryKey => {id};
}

@DriftDatabase(tables: [DownloadedFiles])
class AppDatabase extends _$AppDatabase {
  AppDatabase() : super(createExecutor());

  @override
  int get schemaVersion => 1;
}

@riverpod
AppDatabase database(Ref ref) {
  return AppDatabase();
}
