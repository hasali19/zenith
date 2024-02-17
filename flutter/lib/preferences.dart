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
