import 'dart:typed_data';

import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:zenith/api.dart';
import 'package:zenith/database/database.dart';
import 'package:zenith/routes/item_details/item_details_state.dart';

part 'item_details_controller.g.dart';

@riverpod
class ItemDetailsController extends _$ItemDetailsController {
  AsyncValue<MediaItem> _item = AsyncLoading();
  AsyncValue<List<(MediaItem, List<MediaItem>)>> _seasons = AsyncLoading();
  AsyncValue<DownloadedFile?> _download = AsyncLoading();

  late final _api = ref.watch(apiProvider);
  late final _db = ref.watch(databaseProvider);

  @override
  AsyncValue<ItemDetailsState> build(int id) {
    _refreshApi();

    final downloadedFilesSubscription = (_db.select(_db.downloadedFiles)
          ..where((d) => d.itemId.equals(id)))
        .watch()
        .listen((files) {
      _download = AsyncData(files.firstOrNull);
      _updateState();
    }, onError: (e, s) {
      _download = AsyncError(e, s);
      _updateState();
    });

    ref.onDispose(downloadedFilesSubscription.cancel);

    return AsyncLoading();
  }

  Future<void> refresh() {
    return _refreshApi();
  }

  Future<void> _refreshApi() async {
    final MediaItem item;
    try {
      item = await _api.fetchMediaItem(id);
      _item = AsyncData(item);
    } catch (e, s) {
      _item = AsyncError(e, s);
      return _updateState();
    }

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
      _seasons = AsyncError(e, s);
      return _updateState();
    }

    _updateState();
  }

  void _updateState() {
    switch ((_item, _seasons, _download)) {
      case (
          AsyncData(value: final item),
          AsyncData(value: final seasons),
          AsyncData(value: final download)
        ):
        state = AsyncData(ItemDetailsState(
          item: item,
          poster: item.poster,
          backdrop: item.backdrop,
          seasons: seasons.map((seasonAndEpisodes) {
            final (season, episodes) = seasonAndEpisodes;
            return EpisodeGroupState(
              name: season.name,
              episodes: episodes
                  .map((episode) => EpisodeState(
                        id: episode.id,
                        thumbnail: episode.thumbnail,
                        overview: episode.overview,
                        isWatched: episode.videoUserData?.isWatched ?? false,
                        title: '${episode.parent!.index} - ${episode.name}',
                      ))
                  .toList(),
            );
          }).toList(),
          playable: _getPlayableForItem(item, seasons),
          isWatched: switch (item.type) {
            MediaType.movie ||
            MediaType.episode =>
              item.videoUserData?.isWatched ?? false,
            _ => item.collectionUserData?.unwatched == 0,
          },
          durationText: switch (item.videoFile) {
            null => null,
            final videoFile => _formatDuration(videoFile.duration),
          },
          downloadedFile: download,
        ));

      case (AsyncError(:final error, :final stackTrace), _, _) ||
            (_, AsyncError(:final error, :final stackTrace), _) ||
            (_, _, AsyncError(:final error, :final stackTrace)):
        state = AsyncError(error, stackTrace);

      default:
        state = AsyncLoading();
    }
  }

  void setIsWatched(bool isWatched) async {
    final id = state.valueOrNull?.item.id;
    if (id == null) return;

    await _api.updateUserData(id, VideoUserDataPatch(isWatched: isWatched));

    refresh();
  }

  void findMetadataMatch() async {
    final id = state.valueOrNull?.item.id;
    if (id == null) return;

    await _api.findMetadataMatch(id);

    refresh();
  }

  void refreshMetadata() async {
    final id = state.valueOrNull?.item.id;
    if (id == null) return;

    await _api.refreshMetadata(id);

    refresh();
  }

  Future<void> uploadSubtitleFile(String fileName, Uint8List bytes) async {
    await _api.importSubtitleFile(
        state.value!.item.videoFile!.id, fileName, bytes);

    refresh();
  }
}

PlayableState? _getPlayableForItem(
    MediaItem item, List<(MediaItem, List<MediaItem>)> seasons) {
  final playable = _getPlayableMediaForItem(item, seasons);
  if (playable == null) {
    return null;
  }

  final currentProgress = () {
    final position = playable.videoUserData?.position ?? 0;
    final duration = playable.videoFile!.duration;
    final progress = position / duration;
    if (progress > 0.05 && progress < 0.9) {
      return progress;
    }
    return null;
  }();

  final remainingProgress = () {
    final position = playable.videoUserData?.position ?? 0;
    final duration = playable.videoFile!.duration;
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
    playPosition:
        playable.shouldResume ? playable.videoUserData?.position ?? 0 : 0,
  );
}

MediaItem? _getPlayableMediaForItem(
    MediaItem item, List<(MediaItem, List<MediaItem>)> seasons) {
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
                    userData.lastWatchedAt!
                        .isAfter(lastWatched.videoUserData!.lastWatchedAt!))) {
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
