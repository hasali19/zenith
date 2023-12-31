import 'package:pigeon/pigeon.dart';

class MediaRoute {
  final String id;
  final String name;
  final String? description;

  MediaRoute({
    required this.id,
    required this.name,
    required this.description,
  });
}

enum RoutesScanningMode {
  none,
  passive,
  active,
}

class MediaRouterState {
  final List<MediaRoute?> routes;
  final MediaRoute? selected;

  MediaRouterState({
    required this.selected,
    required this.routes,
  });
}

class MediaLoadRequestData {
  MediaLoadInfo? mediaInfo;
}

class MediaLoadInfo {
  String url;
  MediaMetadata? metadata;

  MediaLoadInfo(this.url);
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
abstract class RemotePlaybackApi {
  // MediaRouter

  MediaRouterState getRouterState();
  void registerRoutesListener(RoutesScanningMode mode);
  void unregisterRoutesListener(RoutesScanningMode mode);
  void selectRoute(String? id);

  // RemoteMediaClient

  void load(MediaLoadRequestData loadRequestData);
  void play();
  void pause();
  void seek(MediaSeekOptions options);
  void setPlaybackRate(double playbackRate);
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
  late MediaInfo mediaInfo;
  late int streamPosition;
  late double playbackRate;
}

class MediaInfo {
  late int streamDuration;
}

@FlutterApi()
abstract class RemotePlaybackEventsApi {
  // MediaRouter

  void onRoutesChanged(List<MediaRoute> routes);
  void onSelectedMediaRouteChanged(MediaRoute? route);

  // RemoteMediaClient

  void onStatusUpdated(MediaStatus status);
}
