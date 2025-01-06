import 'package:flutter/material.dart';
import 'package:zenith/api.dart';

abstract class BaseDownloader {
  @protected
  final ZenithApiClient api;

  BaseDownloader(this.api);

  void downloadFile(
    BuildContext context, {
    required int itemId,
    required int videoFileId,
    required String fileName,
  });

  void cancelDownload(String id);

  void removeDownloadedFile(String id);

  @protected
  String getUrl(int itemId, int videoFileId) {
    return api.getVideoUrl(videoFileId, attachment: true);
  }
}
