import 'dart:io';

import 'package:drift/drift.dart';
import 'package:drift_flutter/drift_flutter.dart';
import 'package:flutter/foundation.dart';
import 'package:path_provider/path_provider.dart';
import 'package:path/path.dart' as p;

QueryExecutor createExecutor() {
  return driftDatabase(
    name: 'zenith',
    native: DriftNativeOptions(
      shareAcrossIsolates: true,
      databasePath: () async {
        if (kDebugMode && Platform.isWindows) {
          return './zenith.sqlite';
        }

        final dbFolder = await getApplicationDocumentsDirectory();
        return p.join(dbFolder.path, 'zenith.sqlite');
      },
    ),
  );
}
