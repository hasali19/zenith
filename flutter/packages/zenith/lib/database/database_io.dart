import 'package:drift/drift.dart';
import 'package:drift_flutter/drift_flutter.dart';

QueryExecutor createExecutor() {
  return driftDatabase(
    name: 'zenith',
    native: DriftNativeOptions(
      shareAcrossIsolates: true,
    ),
  );
}
