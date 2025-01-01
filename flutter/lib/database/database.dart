import 'package:drift/drift.dart';
import 'package:drift_flutter/drift_flutter.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

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
  AppDatabase() : super(_openConnection());

  @override
  int get schemaVersion => 1;

  static QueryExecutor _openConnection() {
    return driftDatabase(
      name: 'zenith',
      native: DriftNativeOptions(
        shareAcrossIsolates: true,
      ),
    );
  }
}

@riverpod
AppDatabase database(Ref ref) {
  return AppDatabase();
}
