import 'dart:async';
import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:zenith/github.dart';
import 'package:zenith/platform.dart';

const _gitHash = bool.hasEnvironment('GIT_COMMIT_HASH')
    ? String.fromEnvironment('GIT_COMMIT_HASH')
    : null;

abstract class Updater {
  factory Updater() {
    if (kIsWeb) return _StubUpdater();
    if (Platform.isAndroid) {
      return _AndroidUpdater();
    } else {
      return _StubUpdater();
    }
  }

  static String? get revision => _gitHash;

  Future<Update?> checkForUpdates();
}

typedef ProgressHandler = void Function(double progress);

abstract class Update {
  bool get showCustomUpdateUi;
  Future<void> install(ProgressHandler onProgress);
}

class _StubUpdater implements Updater {
  @override
  Future<Update?> checkForUpdates() async {
    return null;
  }
}

const _platform = MethodChannel('zenith.hasali.dev/updater');

class _AndroidUpdater implements Updater {
  @override
  Future<Update?> checkForUpdates() async {
    if (_gitHash == null) {
      return null;
    }

    final github = GitHub();
    final ref = await github.getGitRef('tags/flutter/latest');

    if (_gitHash == ref.object.sha) {
      return null;
    }

    final bool useLunaUpdater = await _platform.invokeMethod('useLunaUpdater');

    return _AndroidUpdate(useLunaUpdater);
  }
}

class _AndroidUpdate implements Update {
  final bool _useLunaUpdater;

  const _AndroidUpdate(this._useLunaUpdater);

  @override
  bool get showCustomUpdateUi => !_useLunaUpdater;

  @override
  Future<void> install(ProgressHandler onProgress) async {
    if (_useLunaUpdater) {
      await _platform.invokeMethod('installWithLuna');
      return;
    }

    final github = GitHub();
    final release = await github.getRelease('flutter/latest');
    final apkAssets =
        release.assets.where((asset) => asset.name.endsWith('.apk'));

    final apkMap = <String, ReleaseAsset>{};
    for (final asset in apkAssets) {
      apkMap[asset.name] = asset;
    }

    var supportedAbis = await getSupportedAbis();
    final abi = supportedAbis
        .firstWhere((abi) => apkMap.containsKey('zenith-$abi-release.apk'));
    final url = apkMap['zenith-$abi-release.apk']?.browserDownloadUrl;

    if (url == null) {
      return;
    }

    final completer = Completer();

    await _platform.invokeMethod('install', {'url': url});

    _platform.setMethodCallHandler((call) async {
      if (call.method == 'install/onProgress') {
        onProgress(call.arguments);
      }
    });

    await completer.future;
  }
}
