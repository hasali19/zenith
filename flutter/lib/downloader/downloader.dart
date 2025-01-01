import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/api.dart';
import 'package:zenith/cookies.dart';
import 'package:zenith/database/database.dart';
import 'package:zenith/downloader/downloader_base.dart';

import 'downloader_io.dart' if (dart.library.js_interop) 'downloader_web.dart';

final zenithDownloaderProvider = Provider<BaseDownloader>((ref) {
  return ZenithDownloader(
    ref.watch(cookieJarProvider),
    ref.watch(apiProvider),
    ref.watch(databaseProvider),
  );
});
