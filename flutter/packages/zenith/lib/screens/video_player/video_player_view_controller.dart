import 'package:cast_framework/cast_framework.dart' as cast;
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:zenith/api.dart';
import 'package:zenith/repo/media_items_repo.dart';

part 'video_player_view_controller.freezed.dart';
part 'video_player_view_controller.g.dart';

@freezed
abstract class Playlist with _$Playlist {
  factory Playlist({required List<MediaItem> items, required int start}) =
      _Playlist;
}

enum PlaybackLocation { local, remote }

@freezed
abstract class VideoPlayerState with _$VideoPlayerState {
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
    final mediaItemsRepo = ref.watch(mediaItemsRepoProvider);

    MediaItem? requestedItem = await mediaItemsRepo.getById(id);
    if (requestedItem == null) {
      throw Exception('No item found with id $id');
    }

    final playlist = switch (requestedItem.type) {
      .movie => [requestedItem],
      .episode => await Future(() async {
        if (requestedItem.grandparent case MediaItemParent show) {
          try {
            // TODO: Try to load from db
            return await api.fetchShowEpisodes(show.id);
          } catch (e) {
            print('Failed to fetch episodes for show: $e');
          }
        }

        return [requestedItem];
      }),
      .show || .season => throw ArgumentError(
        'Cannot play this media type: ${requestedItem.type}',
      ),
    };

    int startIndex = playlist.indexWhere((item) => item.id == requestedItem.id);

    return VideoPlayerState(
      location: _getPlaybackLocation(),
      playlist: Playlist(items: playlist, start: startIndex),
    );
  }
}
