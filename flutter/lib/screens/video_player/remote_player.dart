import 'package:cast_framework/cast_framework.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:video_player/video_player.dart' as video_player;
import 'package:zenith/api.dart' as api;
import 'package:zenith/constants.dart';
import 'package:zenith/image.dart';
import 'package:zenith/screens/video_player/media_title.dart';
import 'package:zenith/screens/video_player/ui.dart';

class RemoteVideoPlayer extends ConsumerStatefulWidget {
  final List<api.MediaItem> items;
  final int startIndex;
  final double startPosition;

  const RemoteVideoPlayer({
    super.key,
    required this.items,
    required this.startIndex,
    required this.startPosition,
  });

  @override
  ConsumerState<ConsumerStatefulWidget> createState() =>
      _RemoteVideoPlayerState();
}

class _RemoteVideoPlayerState extends ConsumerState<RemoteVideoPlayer> {
  late final api.ZenithApiClient _api;

  final _controller = RemoteVideoController();

  api.MediaItem get item => widget.items[_controller.currentItemIndex];

  @override
  void initState() {
    super.initState();
    _api = ref.read(api.apiProvider);
    _controller.init();
    _loadMedia();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  void _loadMedia() async {
    final token = await _api.getAccessToken(api.AccessTokenOwner.system, 'cast',
        create: true);

    String withToken(String url) {
      final uri = Uri.parse(url);
      var params = {...uri.queryParameters, 'token': token.token};
      return uri.replace(queryParameters: params).toString();
    }

    String? buildImageUrl(api.ImageId? id, int width) {
      if (id == null) {
        return null;
      }
      return withToken(_api.getImageUrl(id, width: width));
    }

    final items = widget.items
        .map(
          (item) => video_player.VideoItem(
            url: withToken(_api.getVideoUrl(item.videoFile!.id)),
            subtitles: (item.videoFile?.subtitles ?? [])
                .map(
                  (track) => video_player.ExternalSubtitleTrack(
                    id: track.id.toString(),
                    src: withToken(_api.getSubtitleUrl(track.id,
                        format: api.SubtitleFormat.webvtt)),
                    mimeType: 'text/vtt',
                    title: track.title,
                    language: track.language,
                  ),
                )
                .toList(),
            metadata: video_player.MediaMetadata(
              title: item.name,
              seriesTitle: item.grandparent?.name,
              seasonNumber: item.grandparent?.index,
              episodeNumber: item.parent?.index,
              posterUrl: buildImageUrl(item.poster, mediaPosterImageWidth),
              backdropUrl:
                  buildImageUrl(item.backdrop, mediaBackdropImageWidth),
              type: switch (item.type) {
                api.MediaType.movie => video_player.MediaType.movie,
                api.MediaType.episode => video_player.MediaType.tvShow,
                _ => null,
              },
            ),
          ),
        )
        .toList();

    _controller.load(items, widget.startIndex, widget.startPosition);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Stack(
        children: [
          if (item.backdrop case api.ImageId id)
            Positioned.fill(
              child: ZenithApiImage(id: id, requestWidth: 780),
            ),
          ListenableBuilder(
            listenable: _controller,
            builder: (context, child) {
              return VideoPlayerUi(
                title: MediaTitle(item: item),
                controller: _controller,
                onInteractionStart: () {},
                onInteractionEnd: () {},
              );
            },
          ),
        ],
      ),
    );
  }
}

class RemoteVideoController extends video_player.VideoController
    with ChangeNotifier {
  final _client = CastFrameworkPlatform.instance.remoteMediaClient;
  final _positionHandler = video_player.MediaPositionHandler();

  MediaStatus? _mediaStatus;
  MediaInfo? _mediaInfo;
  int _currentItemIndex = 0;
  Set<int> _subtitleTrackIds = {};

  @override
  String? activeSubtitleTrackId;

  @override
  double get position => _positionHandler.positionMs.toDouble() / 1000;

  @override
  set position(double value) {
    _client.seek(MediaSeekOptions(
      position: (value * 1000).toInt(),
      resumeState: ResumeState.unchanged,
    ));
  }

  void init() {
    _client.mediaStatus.addListener(_onMediaStatusUpdated);
    _client.mediaInfo.addListener(_onMediaInfoUpdated);
  }

  @override
  int get currentItemIndex => _currentItemIndex;

  @override
  List<video_player.AudioTrack> get availableAudioTracks => [];

  @override
  List<video_player.SubtitleTrack> currentSubtitleTracks = [];

  @override
  void dispose() {
    super.dispose();
    _client.mediaStatus.removeListener(_onMediaStatusUpdated);
    _client.mediaInfo.removeListener(_onMediaInfoUpdated);
  }

  @override
  double get duration {
    final duration = _mediaInfo?.streamDuration?.toDouble();
    return (duration ?? _positionHandler.positionMs) / 1000;
  }

  @override
  BoxFit get fit => BoxFit.contain;

  @override
  void load(
    List<video_player.VideoItem> items,
    int startIndex,
    double startPosition,
  ) {
    _client.load(MediaLoadRequestData(
      queueData: MediaQueueData(
        items: items
            .map(
              (item) => MediaQueueItem(
                mediaInfo: MediaInfo(
                  url: item.url,
                  mediaTracks: item.subtitles
                      .map(
                        (track) => MediaTrack(
                          // TODO: This assumes id is an integer which may not be correct. We should generate our own integer ids and store a mapping instead.
                          trackId: int.parse(track.id),
                          type: MediaTrackType.text,
                          contentId: track.src,
                          subtype: MediaTrackSubtype.subtitles,
                          name: track.title,
                          language: track.language,
                        ),
                      )
                      .toList(),
                  metadata: MediaMetadata(
                    mediaType: switch (item.metadata.type) {
                      video_player.MediaType.movie => MediaType.movie,
                      video_player.MediaType.tvShow => MediaType.tvShow,
                      _ => MediaType.unknown,
                    },
                    title: item.metadata.title,
                    seriesTitle: item.metadata.seriesTitle,
                    seasonNumber: item.metadata.seasonNumber,
                    episodeNumber: item.metadata.episodeNumber,
                    poster: switch (item.metadata.posterUrl) {
                      null => null,
                      final url => MediaMetadataImage(
                          url: url,
                          width: 0,
                          height: 0,
                        ),
                    },
                    backdrop: switch (item.metadata.backdropUrl) {
                      null => null,
                      final url => MediaMetadataImage(
                          url: url,
                          width: 0,
                          height: 0,
                        ),
                    },
                  ),
                ),
                autoPlay: true,
              ),
            )
            .toList(),
        startIndex: startIndex,
      ),
    ));
  }

  @override
  bool get loading =>
      _mediaStatus?.playerState == PlayerState.loading ||
      _mediaStatus?.playerState == PlayerState.buffering;

  @override
  void pause() {
    _client.pause();
  }

  @override
  bool get paused => _mediaStatus?.playerState == PlayerState.paused;

  @override
  void play() {
    _client.play();
  }

  @override
  double get playbackSpeed => 1.0;

  @override
  void seekToNextItem() {
    _client.queueNext();
  }

  @override
  void seekToPreviousItem() {
    _client.queuePrev();
  }

  @override
  void setAudioTrack(int index) {}

  @override
  void setFit(BoxFit fit) {}

  @override
  void setPlaybackSpeed(double speed) {
    _client.setPlaybackRate(speed);
  }

  @override
  void setSubtitleTrack(String? trackId) {
    _client.setActiveMediaTracks([if (trackId != null) int.parse(trackId)]);
  }

  @override
  video_player.VideoState get state => switch (_mediaStatus?.playerState) {
        null => video_player.VideoState.idle,
        PlayerState.idle when _mediaStatus?.idleReason == IdleReason.finished =>
          video_player.VideoState.ended,
        PlayerState.idle => video_player.VideoState.idle,
        PlayerState.buffering => video_player.VideoState.active,
        PlayerState.loading => video_player.VideoState.active,
        PlayerState.paused => video_player.VideoState.active,
        PlayerState.playing => video_player.VideoState.active,
        PlayerState.unknown => video_player.VideoState.idle,
      };

  @override
  bool get supportsAudioTrackSelection => false;

  @override
  bool get supportsEmbeddedSubtitles => false;

  void _onMediaStatusUpdated() {
    _mediaStatus = _client.mediaStatus.value;

    final mediaStatus = _mediaStatus;
    if (mediaStatus == null) return;

    _positionHandler.update(
      positionMs: mediaStatus.streamPosition,
      isPlaying: mediaStatus.playerState == PlayerState.playing,
      speed: mediaStatus.playbackRate,
    );

    final index = mediaStatus.currentItemIndex;
    if (index != null) {
      _currentItemIndex = index;
    }

    _updateActiveTracks();

    notifyListeners();
  }

  void _onMediaInfoUpdated() {
    _mediaInfo = _client.mediaInfo.value;

    final mediaInfo = _mediaInfo;
    if (mediaInfo == null) return;

    _updateMediaTracks(mediaInfo);

    notifyListeners();
  }

  void _updateMediaTracks(MediaInfo mediaInfo) {
    currentSubtitleTracks = [];

    _subtitleTrackIds = {};

    final mediaTracks = mediaInfo.mediaTracks ?? [];
    for (final track in mediaTracks) {
      if (track.type != MediaTrackType.text) continue;

      currentSubtitleTracks.add(video_player.SubtitleTrack(
        id: track.trackId.toString(),
        language: track.language,
        label: track.name,
      ));

      _subtitleTrackIds.add(track.trackId);
    }

    _updateActiveTracks();
  }

  void _updateActiveTracks() {
    activeSubtitleTrackId = null;

    final activeTrackIds = _mediaStatus?.activeTrackIds ?? [];
    for (final trackId in activeTrackIds) {
      if (_subtitleTrackIds.contains(trackId)) {
        activeSubtitleTrackId = trackId.toString();
      }
    }
  }
}
