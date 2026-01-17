import 'package:cast_framework/cast_framework.dart' as cast;
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:zenith/api.dart';
import 'package:zenith/database/database.dart' as db;

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
    final database = ref.watch(db.databaseProvider);

    MediaItem requestedItem;
    try {
      requestedItem = await api.fetchMediaItem(id);
    } catch (e) {
      final offlineItemQuery = database.select(database.mediaItems)
        ..where((m) => m.id.equals(id));
      final offlineItem = await offlineItemQuery.getSingleOrNull();
      if (offlineItem == null) {
        rethrow;
      }
      requestedItem = MediaItem(
        id: id,
        type: switch (offlineItem.type) {
          .movie => .movie,
          .show => .show,
          .season => .season,
          .episode => .episode,
        },
        name: offlineItem.name,
        overview: offlineItem.overview,
        startDate: DateTime.tryParse(offlineItem.startDate ?? ''),
        endDate: DateTime.tryParse(offlineItem.endDate ?? ''),
        poster: offlineItem.poster as ImageId?,
        backdrop: offlineItem.backdrop as ImageId?,
        thumbnail: offlineItem.thumbnail as ImageId?,
        parent: null,
        grandparent: null,
        videoFile: null,
        videoUserData: null,
        collectionUserData: null,
        genres: [],
        ageRating: null,
        trailer: null,
        director: null,
        cast: [],
      );
    }

    final playlist = switch (requestedItem.type) {
      .movie => [requestedItem],
      .episode => await Future(() async {
        if (requestedItem.grandparent case MediaItemParent show) {
          try {
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
