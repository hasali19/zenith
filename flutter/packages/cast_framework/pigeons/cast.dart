import 'package:pigeon/pigeon.dart';

class MediaRoute {
  late String id;
  late String name;
  late String? description;
  late bool isSelected;
}

enum RoutesScanningMode {
  none,
  passive,
  active,
}

class MediaLoadRequestData {
  MediaLoadInfo? mediaInfo;
}

class MediaLoadInfo {
  String url;
  List<MediaTrack?>? mediaTracks;
  MediaMetadata? metadata;

  MediaLoadInfo(this.url);
}

enum MediaTrackType {
  text,
}

enum MediaTrackSubtype {
  subtitles,
}

class MediaTrack {
  late int trackId;
  late MediaTrackType type;
  late String contentId;
  MediaTrackSubtype? subtype;
  String? name;
  String? language;
}

class MediaMetadata {
  MediaType mediaType;
  String? title;
  String? seriesTitle;
  int? seasonNumber;
  int? episodeNumber;
  MediaMetadataImage? poster;
  MediaMetadataImage? backdrop;

  MediaMetadata(this.mediaType);
}

enum MediaType {
  movie,
  tvShow,
  unknown,
}

class MediaMetadataImage {
  final String url;
  final int width;
  final int height;

  MediaMetadataImage({required this.url, this.width = 0, this.height = 0});
}

enum ResumeState {
  pause,
  play,
  unchanged,
}

class MediaSeekOptions {
  int position;
  ResumeState resumeState;

  MediaSeekOptions(this.position, this.resumeState);
}

@HostApi()
abstract class CastApi {
  void init(String receiverAppId);

  // MediaRouter

  void registerRoutesListener(RoutesScanningMode mode);
  void unregisterRoutesListener(RoutesScanningMode mode);
  void selectRoute(String? id);

  // RemoteMediaClient

  void load(MediaLoadRequestData loadRequestData);
  void setActiveMediaTracks(List<int> trackIds);
  void play();
  void pause();
  void seek(MediaSeekOptions options);
  void setPlaybackRate(double playbackRate);
  void stop();
}

enum PlayerState {
  idle,
  buffering,
  loading,
  paused,
  playing,
  unknown,
}

class MediaStatus {
  late PlayerState playerState;
  late MediaInfo? mediaInfo;
  late int streamPosition;
  late double playbackRate;
}

class MediaInfo {
  late int streamDuration;
  MediaMetadata? metadata;
}

@FlutterApi()
abstract class CastEventsApi {
  // MediaRouter

  void onRoutesChanged(List<MediaRoute> routes);

  // RemoteMediaClient

  void onStatusUpdated(MediaStatus? status);
}
