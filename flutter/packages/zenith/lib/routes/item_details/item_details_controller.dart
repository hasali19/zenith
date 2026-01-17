import 'package:drift/drift.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:zenith/api.dart';
import 'package:zenith/database/database.dart' as db;
import 'package:zenith/routes/item_details/item_details_state.dart';

part 'item_details_controller.g.dart';

@riverpod
class ItemDetailsController extends _$ItemDetailsController {
  AsyncValue<MediaItem> _item = AsyncLoading();
  AsyncValue<List<(MediaItem, List<MediaItem>)>> _seasons = AsyncLoading();
  AsyncValue<db.DownloadedFile?> _download = AsyncLoading();

  late final _api = ref.watch(apiProvider);
  late final _db = ref.watch(db.databaseProvider);

  @override
  AsyncValue<ItemDetailsState> build(int id) {
    _refreshApi();

    final downloadedFilesSubscription =
        (_db.select(
          _db.downloadedFiles,
        )..where((d) => d.itemId.equals(id))).watch().listen(
          (files) {
            _download = AsyncData(files.firstOrNull);
            _updateState();
          },
          onError: (e, s) {
            _download = AsyncError(e, s);
            _updateState();
          },
        );

    ref.onDispose(downloadedFilesSubscription.cancel);

    return AsyncLoading();
  }

  Future<void> refresh() {
    return _refreshApi();
  }

  Future<void> _refreshApi() async {
    MediaItem item;
    bool isOffline = false;
    try {
      item = await _api.fetchMediaItem(id);
    } catch (e, s) {
      final parentTable = _db.mediaItems.createAlias('parent');
      final grandparentTable = _db.mediaItems.createAlias('grandparent');
      final offlineItemQuery = _db.select(_db.mediaItems).join([
        leftOuterJoin(
          parentTable,
          _db.mediaItems.parentId.equalsExp(parentTable.id),
        ),
        leftOuterJoin(
          grandparentTable,
          _db.mediaItems.grandparentId.equalsExp(grandparentTable.id),
        ),
      ])..where(_db.mediaItems.id.equals(id));

      final offlineItemResult = await offlineItemQuery.getSingleOrNull();

      if (offlineItemResult == null) {
        _item = AsyncError(e, s);
        return _updateState();
      } else {
        isOffline = true;

        final offlineItem = offlineItemResult.readTable(_db.mediaItems);
        final parentItem = offlineItemResult.readTableOrNull(parentTable);
        final grandparentItem = offlineItemResult.readTableOrNull(
          grandparentTable,
        );

        MediaItemParent? parent;
        MediaItemParent? grandparent;

        if ((parentItem, offlineItem.parentIndex) case (
          final parentItem?,
          final index?,
        )) {
          parent = MediaItemParent(parentItem.id, index, parentItem.name);
        }

        if ((grandparentItem, offlineItem.grandparentIndex) case (
          final grandparentItem?,
          final index?,
        )) {
          grandparent = MediaItemParent(
            grandparentItem.id,
            index,
            grandparentItem.name,
          );
        }

        item = MediaItem(
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
          parent: parent,
          grandparent: grandparent,
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
    }

    _item = AsyncData(item);

    try {
      final seasons = <(MediaItem, List<MediaItem>)>[];

      if (item.type == MediaType.show) {
        for (final season in await _api.fetchSeasons(item.id)) {
          final episodes = await _api.fetchEpisodes(season.id);
          seasons.add((season, episodes));
        }
      }

      _seasons = AsyncData(seasons);
    } catch (e, s) {
      if (isOffline) {
        _seasons = AsyncData([]);
      } else {
        _seasons = AsyncError(e, s);
        return _updateState();
      }
    }

    _updateState();
  }

  void _updateState() {
    switch ((_item, _seasons, _download)) {
      case (
        AsyncData(value: final item),
        AsyncData(value: final seasons),
        AsyncData(value: final download),
      ):
        state = AsyncData(
          ItemDetailsState(
            item: item,
            poster: item.poster,
            backdrop: item.backdrop,
            seasons: seasons.map((seasonAndEpisodes) {
              final (season, episodes) = seasonAndEpisodes;
              return EpisodeGroupState(
                name: season.name,
                episodes: episodes
                    .map(
                      (episode) => EpisodeState(
                        id: episode.id,
                        thumbnail: episode.thumbnail,
                        overview: episode.overview,
                        isWatched: episode.videoUserData?.isWatched ?? false,
                        title: '${episode.parent!.index} - ${episode.name}',
                      ),
                    )
                    .toList(),
              );
            }).toList(),
            playable: _getPlayableForItem(item, seasons),
            isWatched: switch (item.type) {
              MediaType.movie ||
              MediaType.episode => item.videoUserData?.isWatched ?? false,
              _ => item.collectionUserData?.unwatched == 0,
            },
            durationText: switch (item.videoFile) {
              null => null,
              final videoFile => _formatDuration(videoFile.duration),
            },
            downloadedFile: download,
          ),
        );

      case (AsyncError(:final error, :final stackTrace), _, _) ||
          (_, AsyncError(:final error, :final stackTrace), _) ||
          (_, _, AsyncError(:final error, :final stackTrace)):
        state = AsyncError(error, stackTrace);

      default:
        state = AsyncLoading();
    }
  }

  void setIsWatched(bool isWatched) async {
    final id = state.value?.item.id;
    if (id == null) return;

    await _api.updateUserData(id, VideoUserDataPatch(isWatched: isWatched));

    refresh();
  }

  void findMetadataMatch() async {
    final id = state.value?.item.id;
    if (id == null) return;

    await _api.findMetadataMatch(id);

    refresh();
  }

  void refreshMetadata() async {
    final id = state.value?.item.id;
    if (id == null) return;

    await _api.refreshMetadata(id);

    refresh();
  }

  Future<void> uploadSubtitleFile(String fileName, Uint8List bytes) async {
    await _api.importSubtitleFile(
      state.value!.item.videoFile!.id,
      fileName,
      bytes,
    );

    refresh();
  }
}

PlayableState? _getPlayableForItem(
  MediaItem item,
  List<(MediaItem, List<MediaItem>)> seasons,
) {
  final playable = _getPlayableMediaForItem(item, seasons);
  if (playable == null) {
    return null;
  }

  final currentProgress = () {
    final position = playable.videoUserData?.position ?? 0;
    final duration = playable.videoFile?.duration ?? 0;
    final progress = position / duration;
    if (progress > 0.05 && progress < 0.9) {
      return progress;
    }
    return null;
  }();

  final remainingProgress = () {
    final position = playable.videoUserData?.position ?? 0;
    final duration = playable.videoFile?.duration ?? 0;
    return duration - position;
  }();

  final caption = () {
    String caption = '';

    if (item.type == MediaType.show) {
      final seasonEpisode = playable.getSeasonEpisode();
      if (seasonEpisode != null) {
        caption += seasonEpisode;
      }
    }

    if ((currentProgress ?? 0) > 0) {
      if (caption.isNotEmpty) {
        caption += ' - ';
      }
      caption += '${_formatDuration(remainingProgress)} left';
    }

    if (caption.isNotEmpty) {
      return caption;
    }

    return null;
  }();

  final seasonIndex = switch (playable.grandparent) {
    null => null,
    final grandparent => grandparent.index - 1,
  };

  return PlayableState(
    id: playable.id,
    seasonIndex: seasonIndex,
    progress: currentProgress,
    caption: caption,
    shouldResume: playable.shouldResume,
    playPosition: playable.shouldResume
        ? playable.videoUserData?.position ?? 0
        : 0,
  );
}

MediaItem? _getPlayableMediaForItem(
  MediaItem item,
  List<(MediaItem, List<MediaItem>)> seasons,
) {
  if (item.type == MediaType.show) {
    MediaItem? lastWatched;
    int? lastWatchedS;
    int? lastWatchedE;
    for (var s = 0; s < seasons.length; s++) {
      final (_, episodes) = seasons[s];
      for (var e = 0; e < episodes.length; e++) {
        final episode = episodes[e];
        final userData = episode.videoUserData!;
        if (lastWatched == null ||
            userData.lastWatchedAt != null &&
                (lastWatched.videoUserData!.lastWatchedAt == null ||
                    userData.lastWatchedAt!.isAfter(
                      lastWatched.videoUserData!.lastWatchedAt!,
                    ))) {
          lastWatched = episode;
          lastWatchedS = s;
          lastWatchedE = e;
        }
      }
    }
    if (lastWatched != null &&
        (lastWatched.videoUserData?.position ?? 0) >
            lastWatched.videoFile!.duration * 0.9) {
      if (lastWatchedE! + 1 < seasons[lastWatchedS!].$2.length) {
        return seasons[lastWatchedS].$2[lastWatchedE + 1];
      } else if (lastWatchedS + 1 < seasons.length) {
        return seasons[lastWatchedS + 1].$2[0];
      } else {
        return seasons[0].$2[0];
      }
    }
    return lastWatched;
  } else {
    return item;
  }
}

String _formatDuration(double duration) {
  if (duration <= 90 * 60) {
    return '${duration ~/ 60}m';
  } else {
    final hours = duration ~/ 3600;
    final minutes = (duration % 3600) ~/ 60;
    return '${hours}h ${minutes}m';
  }
}
