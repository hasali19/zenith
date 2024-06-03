import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:shared_preferences_riverpod/shared_preferences_riverpod.dart';

part 'preferences.freezed.dart';
part 'preferences.g.dart';

@freezed
class Server with _$Server {
  const factory Server({
    required String id,
    required String? name,
    required String url,
  }) = _Server;

  factory Server.fromJson(Map<String, dynamic> json) => _$ServerFromJson(json);
}

final preferencesProvider = Provider<SharedPreferences>((ref) {
  throw UnimplementedError();
});

final serversPrefProvider = createMapPrefProvider<List<Server>>(
  prefs: (ref) => ref.watch(preferencesProvider),
  prefKey: 'servers',
  mapFrom: (value) {
    if (value == null) {
      return [];
    }
    final List<dynamic> json = jsonDecode(value);
    return json.map((json) => Server.fromJson(json)).toList();
  },
  mapTo: jsonEncode,
);

final serverPrefProvider = createPrefProvider<String?>(
  prefs: (ref) => ref.watch(preferencesProvider),
  prefKey: 'server',
  defaultValue: null,
);

final activeServerProvider = Provider((ref) {
  if ((kDebugMode || kIsWeb) && const bool.hasEnvironment('DEFAULT_SERVER')) {
    return const Server(
      id: 'default',
      name: 'default',
      url: String.fromEnvironment('DEFAULT_SERVER'),
    );
  }

  final servers = ref.watch(serversPrefProvider);
  final activeServerId = ref.watch(serverPrefProvider);

  if (activeServerId != null) {
    for (final server in servers) {
      if (server.id == activeServerId) {
        return server;
      }
    }
  } else if (servers.isNotEmpty) {
    return servers[0];
  }

  return null;
});

final enableUpdatesCheck = createPrefProvider<bool>(
  prefs: (ref) => ref.watch(preferencesProvider),
  prefKey: 'enableUpdatesCheck',
  defaultValue: true,
);

enum AppThemeMode {
  light,
  dark,
  system,
}

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

enum PlayerSeekPresetDuration {
  d5,
  d10,
  d30,
}

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
