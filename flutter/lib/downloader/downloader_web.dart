import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:web/web.dart';
import 'package:zenith/api.dart';
import 'package:zenith/downloader/downloader_base.dart';

class ZenithDownloader extends BaseDownloader {
  ZenithDownloader(super.api);

  @override
  void downloadFile(
    BuildContext context, {
    required int itemId,
    required int videoFileId,
    required String fileName,
  }) {
    window.open(getUrl(itemId, videoFileId), '_self');
  }

  @override
  void cancelDownload(String id) {
    throw UnimplementedError();
  }

  @override
  void removeDownloadedFile(String id) {
    throw UnimplementedError();
  }
}

final zenithDownloaderProvider = Provider((ref) {
  return ZenithDownloader(ref.watch(apiProvider));
});
