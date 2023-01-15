import 'dart:async';
import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:zenith/github.dart';

const _gitHash = bool.hasEnvironment("GIT_COMMIT_HASH")
    ? String.fromEnvironment("GIT_COMMIT_HASH")
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
  Future<void> install(ProgressHandler onProgress);
}

class _StubUpdater implements Updater {
  @override
  Future<Update?> checkForUpdates() async {
    return null;
  }
}

class _AndroidUpdater implements Updater {
  @override
  Future<Update?> checkForUpdates() async {
    if (_gitHash == null) {
      return null;
    }

    final github = GitHub();
    final ref = await github.getGitRef("tags/flutter/latest");

    if (_gitHash == ref.object.sha) {
      return null;
    }

    return _AndroidUpdate();
  }
}

class _AndroidUpdate implements Update {
  static const _platform = MethodChannel("zenith.hasali.dev/updater");

  @override
  Future<void> install(ProgressHandler onProgress) async {
    final completer = Completer();

    await _platform.invokeMethod("install", {
      "url":
          "https://github.com/hasali19/zenith/releases/download/flutter/latest/zenith.apk"
    });

    _platform.setMethodCallHandler((call) async {
      if (call.method == "install/onProgress") {
        onProgress(call.arguments);
      }
    });

    await completer.future;
  }
}
