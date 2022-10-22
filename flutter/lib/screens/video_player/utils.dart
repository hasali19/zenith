import 'package:video_player/video_player.dart' as video_player;
import 'package:zenith_flutter/api.dart' as api;

video_player.SubtitleTrack subtitleFromApi(api.SubtitleTrack subtitle) {
  return video_player.SubtitleTrack(
    id: subtitle.id.toString(),
    src: api.getSubtitleUrl(subtitle.id),
    title: subtitle.title,
    language: subtitle.language,
  );
}
