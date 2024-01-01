import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:video_player/video_player.dart';
import 'package:zenith/api.dart' as api;
import 'package:zenith/fade_in_image.dart';
import 'package:zenith/remote_playback.dart';
import 'package:zenith/remote_playback_api.g.dart';
import 'package:zenith/screens/video_player/media_title.dart';
import 'package:zenith/screens/video_player/ui.dart';
import 'package:zenith/screens/video_player/video_progress_bar.dart';

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
  late final RemotePlaybackApi _remote;
  late final MediaRouter _mediaRouter;

  final _positionHandler = MediaPositionHandler();

  api.MediaItem get item => widget.items[widget.startIndex];

  @override
  void initState() {
    super.initState();
    _api = ref.read(api.apiProvider);
    _remote = RemotePlaybackApi();
    _mediaRouter = context.read<MediaRouter>();
    _mediaRouter.mediaStatus.addListener(_onMediaStatusUpdated);
    _loadMedia();
  }

  @override
  void dispose() {
    _mediaRouter.mediaStatus.removeListener(_onMediaStatusUpdated);
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

    await _remote.load(MediaLoadRequestData(
      mediaInfo: MediaLoadInfo(
        url: withToken(_api.getVideoUrl(item.videoFile!.id)),
        metadata: MediaMetadata(
          mediaType: switch (item.type) {
            api.MediaType.movie => MediaType.movie,
            api.MediaType.episode => MediaType.tvShow,
            _ => throw Error(),
          },
          title: item.name,
          seriesTitle: item.grandparent?.name,
          seasonNumber: item.grandparent?.index,
          episodeNumber: item.parent?.index,
          poster: MediaMetadataImage(
            url:
                withToken(_api.getMediaImageUrl(item.id, api.ImageType.poster)),
            width: 0,
            height: 0,
          ),
          backdrop: MediaMetadataImage(
            url: withToken(
                _api.getMediaImageUrl(item.id, api.ImageType.backdrop)),
            width: 0,
            height: 0,
          ),
        ),
      ),
    ));
  }

  void _onMediaStatusUpdated() {
    final mediaStatus = _mediaRouter.mediaStatus.value;
    if (mediaStatus == null) return;

    _positionHandler.update(
      positionMs: mediaStatus.streamPosition,
      isPlaying: mediaStatus.playerState == PlayerState.playing,
      speed: mediaStatus.playbackRate,
    );
  }

  VideoProgressData _getProgress() {
    return VideoProgressData(
      total: Duration(
          milliseconds:
              _mediaRouter.mediaStatus.value?.mediaInfo?.streamDuration ??
                  _positionHandler.positionMs.toInt()),
      progress: Duration(milliseconds: _positionHandler.positionMs.toInt()),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Stack(
        children: [
          Positioned.fill(
            child: ZenithFadeInImage.dio(
              url: _api.getMediaImageUrl(item.id, api.ImageType.backdrop),
            ),
          ),
          ValueListenableBuilder(
            valueListenable: _mediaRouter.mediaStatus,
            builder: (context, mediaStatus, child) => VideoPlayerUi(
              title: MediaTitle(item: item),
              audioTracks: const [],
              subtitles: const [],
              progress: _getProgress,
              isAudioTrackSelectionSupported: false,
              fit: BoxFit.cover,
              playbackSpeed: 1.0,
              isLoading: mediaStatus?.playerState == PlayerState.loading ||
                  mediaStatus?.playerState == PlayerState.buffering,
              isPaused: mediaStatus?.playerState == PlayerState.paused,
              onInteractionStart: () {},
              onInteractionEnd: () {},
              onAudioTrackSelected: (index) {},
              onTextTrackSelected: (track) {},
              onFitSelected: (fit) {},
              onPlaybackSpeedSelected: _remote.setPlaybackRate,
              onSeek: (position) {
                _remote.seek(MediaSeekOptions(
                  position: (position * 1000).toInt(),
                  resumeState: ResumeState.unchanged,
                ));
              },
              onSeekDelta: (delta) {
                _remote.seek(MediaSeekOptions(
                  position:
                      (_positionHandler.positionMs + delta * 1000).toInt(),
                  resumeState: ResumeState.unchanged,
                ));
              },
              onSeekToPrevious: () {},
              onSeekToNext: () {},
              onSetPaused: (isPaused) =>
                  isPaused ? _remote.pause() : _remote.play(),
            ),
          ),
        ],
      ),
    );
  }
}
