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
    final github = GitHub();
    final runs = await github
        .getActionsWorkflowRuns(8229171)
        .then((res) => res.workflowRuns);

    ActionsWorkflowRun? latestRun;
    for (final run in runs) {
      if (run.status == "completed" && run.conclusion == "success") {
        latestRun = run;
        break;
      }
    }

    if (latestRun == null ||
        latestRun.headBranch != "master" ||
        _gitHash == null ||
        _gitHash == latestRun.headSha) {
      return null;
    }

    final artifacts = await github
        .getActionsWorkflowRunArtifacts(latestRun.id)
        .then((res) => res.artifacts);

    ActionsWorkflowRunArtifact? androidArtifact;
    for (final artifact in artifacts) {
      if (artifact.name == "zenith-flutter-android" && !artifact.expired) {
        androidArtifact = artifact;
      }
    }

    if (androidArtifact == null) {
      return null;
    }

    return _AndroidUpdate(androidArtifact);
  }
}

class _AndroidUpdate implements Update {
  static const platform = MethodChannel("zenith.hasali.dev/updater");

  _AndroidUpdate(this.artifact);

  final ActionsWorkflowRunArtifact artifact;

  @override
  Future<void> install(ProgressHandler onProgress) async {
    final completer = Completer();

    await platform.invokeMethod("install", {"artifactId": artifact.id});

    platform.setMethodCallHandler((call) async {
      if (call.method == "install/onProgress") {
        onProgress(call.arguments);
      }
    });

    await completer.future;
  }
}
