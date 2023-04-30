import 'package:video_player/video_player.dart' as video_player;
import 'package:zenith/api.dart';
import 'package:zenith/language_codes.dart';
import 'package:zenith/screens/video_player/ui.dart';

String _formatToMimeType(String? format) {
  if (format == null || format == 'webvtt') {
    return 'text/vtt';
  }

  if (format == 'srt') {
    return 'application/x-subrip';
  }

  throw ArgumentError.value(format, 'format', 'unsupported text track format');
}

video_player.SubtitleTrack subtitleFromApi(
    ZenithApiClient api, SubtitleTrack subtitle) {
  return video_player.SubtitleTrack(
    id: subtitle.id.toString(),
    src: api.getSubtitleUrl(subtitle.id),
    mimeType: subtitle.streamIndex == null
        ? _formatToMimeType(subtitle.format)
        : 'text/vtt', // all platforms currently use extracted vtt files for embedded subs, instead of using the stream directly
    title: subtitle.title,
    language: subtitle.language,
    displayLanguage: subtitle.language != null
        ? tryResolveLanguageCode(subtitle.language!)
        : null,
  );
}

AudioTrack audioTrackFromApi(AudioStreamInfo stream) {
  return AudioTrack(
    index: stream.index,
    language: tryResolveLanguageCode(stream.language ?? 'Unknown'),
    codec: stream.codec,
  );
}
