import 'package:cast_framework/cast_framework.dart' as cast;
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:zenith/api.dart';

part 'video_player_view_controller.freezed.dart';
part 'video_player_view_controller.g.dart';

@freezed
class Playlist with _$Playlist {
  factory Playlist({
    required List<MediaItem> items,
    required int start,
  }) = _Playlist;
}

enum PlaybackLocation {
  local,
  remote,
}

@freezed
class VideoPlayerState with _$VideoPlayerState {
  factory VideoPlayerState({
    required PlaybackLocation location,
    Playlist? playlist,
  }) = _VideoPlayerState;
}

PlaybackLocation _getPlaybackLocation() {
  final castPlugin = cast.CastFrameworkPlatform.instance;
  if (castPlugin.isSupported &&
      castPlugin.mediaRouter.selectedRoute.value != null) {
    return PlaybackLocation.remote;
  } else {
    return PlaybackLocation.local;
  }
}

@riverpod
class VideoPlayerViewController extends _$VideoPlayerViewController {
  @override
  Future<VideoPlayerState> build(int id) async {
    final api = ref.watch(apiProvider);

    final requestedItem = await api.fetchMediaItem(id);
    final playlist = switch (requestedItem.type) {
      MediaType.episode =>
        await api.fetchShowEpisodes(requestedItem.grandparent!.id),
      _ => [requestedItem],
    };

    int startIndex = playlist.indexWhere((item) => item.id == requestedItem.id);

    return VideoPlayerState(
      location: _getPlaybackLocation(),
      playlist: Playlist(items: playlist, start: startIndex),
    );
  }
}
