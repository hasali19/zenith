import 'package:drift/drift.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:shared_preferences_riverpod/shared_preferences_riverpod.dart';
import 'package:zenith/database/database.dart';

final preferencesProvider = Provider<SharedPreferences>((ref) {
  throw UnimplementedError();
});

final serverPrefProvider = createPrefProvider<String?>(
  prefs: (ref) => ref.watch(preferencesProvider),
  prefKey: 'server',
  defaultValue: null,
);

class ServersList {
  final AppDatabase _db;

  List<Server> _servers;

  ServersList(AppDatabase db, List<Server> servers)
    : _db = db,
      _servers = servers;

  List<Server> get servers => _servers;

  Future<void> addServer(ServersCompanion server, [InsertMode? mode]) async {
    await _db.into(_db.servers).insert(server, mode: mode);
    _servers = await _db.select(_db.servers).get();
  }
}

final serversProvider = Provider<ServersList>(
  (ref) => throw UnimplementedError(),
);

final activeServerProvider = Provider((ref) {
  if ((kDebugMode || kIsWeb) && const bool.hasEnvironment('DEFAULT_SERVER')) {
    return const Server(
      id: 0,
      uuid: 'default',
      url: String.fromEnvironment('DEFAULT_SERVER'),
    );
  }

  final servers = ref.watch(serversProvider).servers;
  final activeServerId = ref.watch(serverPrefProvider);

  if (activeServerId != null) {
    for (final server in servers) {
      if (server.uuid == activeServerId) {
        return server;
      }
    }
  }

  return null;
});

final enableUpdatesCheck = createPrefProvider<bool>(
  prefs: (ref) => ref.watch(preferencesProvider),
  prefKey: 'enableUpdatesCheck',
  defaultValue: true,
);

enum AppThemeMode { light, dark, system }

extension AppThemeModeExt on AppThemeMode {
  String get label => switch (this) {
    AppThemeMode.light => 'Light',
    AppThemeMode.dark => 'Dark',
    AppThemeMode.system => 'System default',
  };
}

final themeMode = createMapPrefProvider<AppThemeMode>(
  prefs: (ref) => ref.watch(preferencesProvider),
  prefKey: 'themeMode',
  mapFrom: (value) =>
      AppThemeMode.values
          .where((element) => element.name == value)
          .firstOrNull ??
      AppThemeMode.system,
  mapTo: (value) => value.name,
);

final enableDynamicColor = createPrefProvider<bool>(
  prefs: (ref) => ref.watch(preferencesProvider),
  prefKey: 'enableDynamicColor',
  defaultValue: true,
);

enum PlayerSeekPresetDuration { d5, d10, d30 }

extension PlayerSeekPresetDurationExt on PlayerSeekPresetDuration {
  int get value => switch (this) {
    PlayerSeekPresetDuration.d5 => 5,
    PlayerSeekPresetDuration.d10 => 10,
    PlayerSeekPresetDuration.d30 => 30,
  };
}

final fastForwardDurationProvider =
    createMapPrefProvider<PlayerSeekPresetDuration>(
      prefs: (ref) => ref.watch(preferencesProvider),
      prefKey: 'fastForwardDuration',
      mapFrom: (v) =>
          PlayerSeekPresetDuration.values
              .where((element) => element.name == v)
              .firstOrNull ??
          PlayerSeekPresetDuration.d30,
      mapTo: (v) => v.name,
    );

final rewindDurationProvider = createMapPrefProvider<PlayerSeekPresetDuration>(
  prefs: (ref) => ref.watch(preferencesProvider),
  prefKey: 'rewindDuration',
  mapFrom: (v) =>
      PlayerSeekPresetDuration.values
          .where((element) => element.name == v)
          .firstOrNull ??
      PlayerSeekPresetDuration.d10,
  mapTo: (v) => v.name,
);

final setWatchedOnSkipProvider = createPrefProvider<bool>(
  prefs: (ref) => ref.watch(preferencesProvider),
  prefKey: 'player.setWatchedOnSkip',
  defaultValue: kReleaseMode,
);

final applyCropRectsProvider = createPrefProvider<bool>(
  prefs: (ref) => ref.watch(preferencesProvider),
  prefKey: 'player.applyCropRects',
  defaultValue: false,
);

final subtitleSizeProvider = createPrefProvider<int?>(
  prefs: (ref) => ref.watch(preferencesProvider),
  prefKey: 'player.subtitleSize',
  defaultValue: null,
);
