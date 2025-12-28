import 'dart:io';
import 'dart:ui';

import 'package:cookie_jar/cookie_jar.dart';
import 'package:dio_cookie_manager/dio_cookie_manager.dart';
import 'package:drift/drift.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:logger/logger.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:uuid/uuid.dart';
import 'package:zenith/api.dart';
import 'package:zenith/cookies.dart';
import 'package:zenith/database/database.dart';
import 'package:zenith/downloader/downloader_base.dart';

const _channel = MethodChannel('zenith.hasali.dev/downloader');
const _uuid = Uuid();

final _logger = Logger();

class ZenithDownloader extends BaseDownloader {
  final CookieJar _cookies;
  final AppDatabase _db;

  bool _isInit = false;
  Future<void>? _initFuture;

  ZenithDownloader(this._cookies, super.api, this._db);

  Future<void> _ensureInit() async {
    if (!_isInit) {
      _initFuture ??= Future(() async {
        final handle = PluginUtilities.getCallbackHandle(
          _downloaderCallbackDispatcher,
        );

        await _channel.invokeMethod('init', {
          'callbackDispatcherHandle': handle!.toRawHandle(),
        });

        _isInit = true;
      });

      await _initFuture;
    }
  }

  @override
  void downloadFile(
    BuildContext context, {
    required int itemId,
    required int videoFileId,
    required String fileName,
  }) async {
    final url = getUrl(itemId, videoFileId);
    final mediaItem = await this.api.fetchMediaItem(itemId);

    if (Platform.isAndroid) {
      await _ensureInit();

      var status = await Permission.notification.status;
      while (status.isDenied && !status.isPermanentlyDenied) {
        if (await Permission.notification.shouldShowRequestRationale &&
            context.mounted) {
          await showDialog(
            context: context,
            builder: (context) => AlertDialog(
              title: const Text('Notifications'),
              content: const Text(
                'Permission required to show progress notification for file downloads.',
              ),
              actions: [
                TextButton(
                  onPressed: () => Navigator.pop(context),
                  child: const Text('Next'),
                ),
              ],
            ),
          );
        }
        status = await Permission.notification.request();
      }

      if (!status.isGranted) {
        if (context.mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: const Text(
                'Missing notification permission for downloads',
              ),
              action: SnackBarAction(
                label: 'Grant',
                onPressed: () => openAppSettings(),
              ),
            ),
          );
        }
        return;
      }

      final id = _uuid.v4();

      await _db.transaction(() async {
        await _db
            .into(_db.mediaItems)
            .insertOnConflictUpdate(
              MediaItemsCompanion.insert(
                id: Value(mediaItem.id),
                type: switch (mediaItem.type) {
                  .movie => .movie,
                  .show => .show,
                  .season => .season,
                  .episode => .episode,
                },
                name: mediaItem.name,
                overview: Value(mediaItem.overview),
                startDate: Value(mediaItem.startDate?._formatISODate()),
                endDate: Value(mediaItem.endDate?._formatISODate()),
                poster: Value(mediaItem.poster?.value),
                backdrop: Value(mediaItem.backdrop?.value),
                thumbnail: Value(mediaItem.thumbnail?.value),
              ),
            );

        await _db
            .into(_db.downloadedFiles)
            .insert(
              DownloadedFilesCompanion.insert(
                id: id,
                itemId: itemId,
                videoFileId: videoFileId,
                createdAt: DateTime.now(),
              ),
            );
      });

      await _channel.invokeMethod('enqueue', {
        'id': id,
        'uri': url,
        'cookies': CookieManager.getCookies(
          await _cookies.loadForRequest(Uri.parse(url)),
        ),
        'filename': fileName,
      });
    } else {
      // TODO: This is a hack, we need a proper in-app downloader that passes
      // the necessary auth headers.
      final token = await this.api.getAccessToken(
        AccessTokenOwner.system,
        'cast',
        create: true,
      );

      String withToken(String url) {
        final uri = Uri.parse(url);
        var params = {...uri.queryParameters, 'token': token.token};
        return uri.replace(queryParameters: params).toString();
      }

      launchUrl(
        Uri.parse(withToken(url)),
        mode: LaunchMode.externalApplication,
      );
    }
  }

  @override
  void cancelDownload(String id) async {
    assert(Platform.isAndroid);

    await _channel.invokeMethod('cancel', {'id': id});

    await (_db.delete(_db.downloadedFiles)..where((f) => f.id.equals(id))).go();
  }

  @override
  void removeDownloadedFile(String id) async {
    assert(Platform.isAndroid);

    final file = await (_db.select(
      _db.downloadedFiles,
    )..where((f) => f.id.equals(id))).getSingleOrNull();

    if (file == null) {
      return;
    }

    if (file.path != null) {
      try {
        await _channel.invokeMethod('deleteFile', {'uri': file.path});
      } catch (e, s) {
        _logger.w(
          'failed to delete file: ${file.path}',
          error: e,
          stackTrace: s,
        );
      }
    }

    await _db.delete(_db.downloadedFiles).delete(file);
  }
}

final zenithDownloaderProvider = Provider<BaseDownloader>((ref) {
  return ZenithDownloader(
    ref.watch(cookieJarProvider),
    ref.watch(apiProvider),
    ref.watch(databaseProvider),
  );
});

@pragma('vm:entry-point')
Future<void> _downloaderCallbackDispatcher() async {
  WidgetsFlutterBinding.ensureInitialized();

  final db = AppDatabase();

  _channel.setMethodCallHandler((call) async {
    if (call.method == 'onDownloadResult') {
      _logger.i('Received download result: ${call.arguments}');
      final String id = call.arguments['id'];
      final bool success = call.arguments['success'];
      if (success) {
        await (db.update(
          db.downloadedFiles,
        )..where((t) => t.id.equals(id))).write(
          DownloadedFilesCompanion(path: Value(call.arguments['uri'])),
        );
      } else {
        await (db.delete(
          db.downloadedFiles,
        )..where((t) => t.id.equals(id))).go();
      }
    } else {
      throw UnimplementedError();
    }
  });

  await _channel.invokeMethod('ready');
}

extension on DateTime {
  String _formatISODate() {
    return '$year-$month-$day';
  }
}
