import 'dart:html';

import 'package:flutter_riverpod/flutter_riverpod.dart';

void downloadFile(String url) {}

class ZenithDownloader {
  void downloadFile(String url) {
    window.open(url, '_self');
  }
}

final zenithDownloaderProvider = Provider((ref) {
  return ZenithDownloader();
});
