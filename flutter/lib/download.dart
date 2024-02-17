import 'dart:io';

import 'package:cookie_jar/cookie_jar.dart';
import 'package:dio_cookie_manager/dio_cookie_manager.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:zenith/api.dart';
import 'package:zenith/cookies.dart';

const _channel = MethodChannel('zenith.hasali.dev/downloader');

class ZenithDownloader {
  final CookieJar _cookies;
  final ZenithApiClient _api;

  ZenithDownloader(this._cookies, this._api);

  void downloadFile(
    BuildContext context, {
    required String url,
    required String filename,
  }) async {
    if (Platform.isAndroid) {
      var status = await Permission.notification.status;
      while (status.isDenied && !status.isPermanentlyDenied) {
        if (await Permission.notification.shouldShowRequestRationale &&
            context.mounted) {
          await showDialog(
            context: context,
            builder: (context) => AlertDialog(
              title: const Text('Notifications'),
              content: const Text(
                  'Permission required to show progress notification for file downloads.'),
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
          ScaffoldMessenger.of(context).showSnackBar(SnackBar(
            content:
                const Text('Missing notification permission for downloads'),
            action: SnackBarAction(
              label: 'Grant',
              onPressed: () => openAppSettings(),
            ),
          ));
        }
        return;
      }

      await _channel.invokeMethod('enqueue', {
        'uri': url,
        'cookies': CookieManager.getCookies(
            await _cookies.loadForRequest(Uri.parse(url))),
        'filename': filename,
      });
    } else {
      // TODO: This is a hack, we need a proper in-app downloader that passes
      // the necessary auth headers.
      final token = await _api.getAccessToken(AccessTokenOwner.system, 'cast',
          create: true);

      String withToken(String url) {
        final uri = Uri.parse(url);
        var params = {...uri.queryParameters, 'token': token.token};
        return uri.replace(queryParameters: params).toString();
      }

      launchUrl(Uri.parse(withToken(url)),
          mode: LaunchMode.externalApplication);
    }
  }
}

final zenithDownloaderProvider = Provider((ref) {
  return ZenithDownloader(
    ref.watch(cookieJarProvider),
    ref.watch(apiProvider),
  );
});
