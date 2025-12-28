import 'package:video_player/video_player.dart' as video_player;
import 'package:zenith/api.dart';

String _formatToMimeType(String? format) {
  if (format == null || format == 'webvtt') {
    return 'text/vtt';
  }

  if (format == 'srt') {
    return 'application/x-subrip';
  }

  throw ArgumentError.value(format, 'format', 'unsupported text track format');
}

video_player.ExternalSubtitleTrack subtitleFromApi(
  ZenithApiClient api,
  SubtitleTrack subtitle,
) {
  return video_player.ExternalSubtitleTrack(
    id: subtitle.id.toString(),
    src: api.getSubtitleUrl(subtitle.id),
    mimeType: subtitle.streamIndex == null
        ? _formatToMimeType(subtitle.format)
        : 'text/vtt', // all platforms currently use extracted vtt files for embedded subs, instead of using the stream directly
    title: subtitle.title,
    language: subtitle.language,
  );
}
