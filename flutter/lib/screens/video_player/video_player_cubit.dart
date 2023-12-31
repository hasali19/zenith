import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:zenith/api.dart';
import 'package:zenith/remote_playback.dart';

part 'video_player_cubit.freezed.dart';

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

class VideoPlayerCubit extends Cubit<VideoPlayerState> {
  final ZenithApiClient _api;

  VideoPlayerCubit(this._api, final MediaRouter mediaRouter)
      : super(VideoPlayerState(
          location: mediaRouter.selectedRoute.value == null
              ? PlaybackLocation.local
              : PlaybackLocation.remote,
        ));

  void loadPlaylist(int id) async {
    final requestedItem = await _api.fetchMediaItem(id);

    final playlist = switch (requestedItem.type) {
      MediaType.episode =>
        await _api.fetchShowEpisodes(requestedItem.grandparent!.id),
      _ => [requestedItem],
    };

    int startIndex = playlist.indexWhere((item) => item.id == requestedItem.id);

    emit(state.copyWith(
      playlist: Playlist(items: playlist, start: startIndex),
    ));
  }
}
