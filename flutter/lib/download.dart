import 'package:url_launcher/url_launcher.dart';

void downloadFile(String url) {
  launchUrl(Uri.parse(url), mode: LaunchMode.externalApplication);
}
