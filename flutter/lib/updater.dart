import 'dart:async';
import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:zenith/github.dart';

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
  Future<void> install();
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

    return const _AndroidUpdate();
  }
}

class _AndroidUpdate implements Update {
  const _AndroidUpdate();

  @override
  Future<void> install() async {
    await _platform.invokeMethod('installWithLuna');
  }
}
