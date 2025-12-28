import 'package:pigeon/pigeon.dart';

@ConfigurePigeon(
  PigeonOptions(
    dartOut: 'lib/src/cast_api.g.dart',
    kotlinOut:
        'android/src/main/kotlin/dev/hasali/zenith/cast_framework/pigeon/CastApi.g.kt',
    kotlinOptions: KotlinOptions(
      package: 'dev.hasali.zenith.cast_framework.pigeon',
    ),
  ),
)
class MediaRoute {
  late String id;
  late String name;
  late String? description;
  late bool isSelected;
}

enum RoutesScanningMode { none, passive, active }

class MediaLoadRequestData {
  MediaInfo? mediaInfo;
  MediaQueueData? queueData;
  String? customDataJson;
}

class MediaInfo {
  String? url;
  List<MediaTrack>? mediaTracks;
  MediaMetadata? metadata;
  int? streamDuration;
  String? customDataJson;
}

class MediaQueueData {
  List<MediaQueueItem>? items;
  int? startIndex;
  MediaQueueType? queueType;
}

class MediaQueueItem {
  MediaInfo? mediaInfo;
  List<int>? activeTrackIds;
  bool? autoPlay;
  double? startTime;
  String? customDataJson;
}

enum MediaQueueType { generic, tvSeries, videoPlaylist, movie }

enum MediaTrackType { video, audio, text }

enum MediaTrackSubtype { subtitles }

class MediaTrack {
  late int trackId;
  late MediaTrackType type;
  late String? contentId;
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

enum MediaType { movie, tvShow, unknown }

class MediaMetadataImage {
  final String url;
  final int width;
  final int height;

  MediaMetadataImage({required this.url, this.width = 0, this.height = 0});
}

enum ResumeState { pause, play, unchanged }

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

  @async
  void load(MediaLoadRequestData loadRequestData);
  void setActiveMediaTracks(List<int> trackIds);
  void play();
  void pause();
  void seek(MediaSeekOptions options);
  void queueNext();
  void queuePrev();
  void setPlaybackRate(double playbackRate);
  @async
  void sendMessage(String namespace, String message);
  void stop();
}

enum IdleReason { canceled, error, finished, interrupted, none }

enum PlayerState { idle, buffering, loading, paused, playing, unknown }

class MediaStatus {
  late PlayerState playerState;
  late IdleReason idleReason;
  late int streamPosition;
  late double playbackRate;
  late int? currentItemIndex;
  late List<int>? activeTrackIds;
}

@FlutterApi()
abstract class CastEventsApi {
  // MediaRouter

  void onRoutesChanged(List<MediaRoute> routes);

  // RemoteMediaClient

  void onStatusUpdated(MediaStatus? status);
  void onMediaInfoUpdated(MediaInfo? info);
}
