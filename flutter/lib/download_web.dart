import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:web/web.dart';

void downloadFile(String url) {}

class ZenithDownloader {
  void downloadFile(
    BuildContext context, {
    required String url,
    required String filename,
  }) {
    window.open(url, '_self');
  }
}

final zenithDownloaderProvider = Provider((ref) {
  return ZenithDownloader();
});
