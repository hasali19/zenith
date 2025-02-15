import 'dart:convert';

import 'package:drift/drift.dart';
import 'package:drift/internal/versioned_schema.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:zenith/database/database.steps.dart';

import './database_stub.dart'
    if (dart.library.ffi) './database_io.dart'
    if (dart.library.js_interop) './database_web.dart';

part 'database.g.dart';

class Servers extends Table {
  IntColumn get id => integer().autoIncrement()();
  TextColumn get uuid => text().unique()();
  TextColumn get url => text().unique()();
}

class DownloadedFiles extends Table {
  TextColumn get id => text()();
  IntColumn get itemId => integer()();
  IntColumn get videoFileId => integer()();
  TextColumn get path => text().nullable()();
  DateTimeColumn get createdAt => dateTime()();

  @override
  Set<Column<Object>>? get primaryKey => {id};
}

@DriftDatabase(tables: [Servers, DownloadedFiles])
class AppDatabase extends _$AppDatabase {
  AppDatabase([QueryExecutor? executor]) : super(executor ?? createExecutor());

  @override
  int get schemaVersion => 2;

  @override
  MigrationStrategy get migration => MigrationStrategy(
        onUpgrade: (m, from, to) async {
          // Following the advice from https://drift.simonbinder.eu/Migrations/api/#general-tips
          await customStatement('PRAGMA foreign_keys = OFF');

          await transaction(
            () => VersionedSchema.runMigrationSteps(
              migrator: m,
              from: from,
              to: to,
              steps: _upgrade,
            ),
          );

          if (kDebugMode) {
            final wrongForeignKeys =
                await customSelect('PRAGMA foreign_key_check').get();
            assert(wrongForeignKeys.isEmpty,
                '${wrongForeignKeys.map((e) => e.data)}');
          }

          await customStatement('PRAGMA foreign_keys = ON');
        },
        beforeOpen: (details) async {
          if ((details.versionBefore ?? 0) < 2) {
            final prefs = await SharedPreferences.getInstance();
            final json = prefs.getString('servers');

            final List<dynamic> servers = switch (json) {
              null => [],
              final json => jsonDecode(json),
            };

            for (final Map<String, dynamic> server in servers) {
              await into(this.servers).insert(
                ServersCompanion.insert(
                  uuid: server['id'],
                  url: server['url'],
                ),
                mode: InsertMode.insertOrReplace,
              );
            }
          }
        },
      );

  static final _upgrade = migrationSteps(
    from1To2: (m, schema) async {
      await m.createTable(schema.servers);
    },
  );
}

@riverpod
AppDatabase database(Ref ref) {
  throw UnimplementedError();
}
