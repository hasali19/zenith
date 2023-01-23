import 'package:video_player/video_player.dart' as video_player;
import 'package:zenith/api.dart';
import 'package:zenith/language_codes.dart';
import 'package:zenith/screens/video_player/ui.dart';

video_player.SubtitleTrack subtitleFromApi(
    ZenithApiClient api, SubtitleTrack subtitle) {
  return video_player.SubtitleTrack(
    id: subtitle.id.toString(),
    src: api.getSubtitleUrl(subtitle.id),
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
    language: tryResolveLanguageCode(stream.language ?? "Unknown"),
    codec: stream.codec,
  );
}
