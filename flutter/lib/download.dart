import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:zenith/api.dart';

class ZenithDownloader {
  final ZenithApiClient _api;

  ZenithDownloader(this._api);

  void downloadFile(String url) async {
    // TODO: This is a hack, we need a proper in-app downloader that passes
    // the necessary auth headers.
    final token = await _api.getAccessToken(AccessTokenOwner.system, 'cast',
        create: true);

    String withToken(String url) {
      final uri = Uri.parse(url);
      var params = {...uri.queryParameters, 'token': token.token};
      return uri.replace(queryParameters: params).toString();
    }

    launchUrl(Uri.parse(withToken(url)), mode: LaunchMode.externalApplication);
  }
}

final zenithDownloaderProvider = Provider((ref) {
  return ZenithDownloader(ref.watch(apiProvider));
});
