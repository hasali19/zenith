import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:zenith/api.dart';
import 'package:zenith/routes/item_details/item_details_state.dart';

part 'item_details_controller.g.dart';

@riverpod
class ItemDetailsController extends _$ItemDetailsController {
  @override
  Future<ItemDetailsState> build(int id) async {
    final api = ref.watch(apiProvider);
    final item = await api.fetchMediaItem(id);
    final episodeGroups = <List<MediaItem>>[];
    final seasons = <EpisodeGroupState>[];

    if (item.type == MediaType.show) {
      for (final season in await api.fetchSeasons(item.id)) {
        final episodes = await api.fetchEpisodes(season.id);
        episodeGroups.add(episodes);
        seasons.add(EpisodeGroupState(
          name: season.name,
          episodes: episodes
              .map(
                (e) => EpisodeState(
                  id: e.id,
                  thumbnailUrl: api.getMediaImageUrl(e.id, ImageType.thumbnail),
                  overview: e.overview,
                  isWatched: e.videoUserData?.isWatched ?? false,
                  title: '${e.parent!.index} - ${e.name}',
                ),
              )
              .toList(),
        ));
      }
    }

    return ItemDetailsState(
      item: item,
      posterUrl: api.getMediaImageUrl(id, ImageType.poster),
      backdropUrl: api.getMediaImageUrl(id, ImageType.backdrop),
      seasons: seasons,
      playable: _getPlayableForItem(item, episodeGroups),
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
      videoDownloadUrl: switch (item.videoFile) {
        null => null,
        final videoFile => api.getVideoUrl(videoFile.id, attachment: true)
      },
    );
  }

  void setIsWatched(bool isWatched) async {
    final id = state.valueOrNull?.item.id;
    if (id == null) return;

    await ref
        .read(apiProvider)
        .updateUserData(id, VideoUserDataPatch(isWatched: isWatched));

    ref.invalidateSelf();
  }

  void findMetadataMatch() async {
    final id = state.valueOrNull?.item.id;
    if (id == null) return;

    await ref.read(apiProvider).findMetadataMatch(id);

    ref.invalidateSelf();
  }

  void refreshMetadata() async {
    final id = state.valueOrNull?.item.id;
    if (id == null) return;

    await ref.read(apiProvider).refreshMetadata(id);

    ref.invalidateSelf();
  }
}

PlayableState? _getPlayableForItem(
    MediaItem item, List<List<MediaItem>> seasons) {
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

  return PlayableState(
    id: playable.id,
    progress: currentProgress,
    caption: caption,
    shouldResume: playable.shouldResume,
    playPosition:
        playable.shouldResume ? playable.videoUserData?.position ?? 0 : 0,
  );
}

MediaItem? _getPlayableMediaForItem(
    MediaItem item, List<List<MediaItem>> seasons) {
  if (item.type == MediaType.show) {
    MediaItem? lastWatched;
    int? lastWatchedS;
    int? lastWatchedE;
    for (var s = 0; s < seasons.length; s++) {
      final season = seasons[s];
      for (var e = 0; e < season.length; e++) {
        final episode = season[e];
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
      if (lastWatchedE! + 1 < seasons[lastWatchedS!].length) {
        return seasons[lastWatchedS][lastWatchedE + 1];
      } else if (lastWatchedS + 1 < seasons.length) {
        return seasons[lastWatchedS + 1][0];
      } else {
        return seasons[0][0];
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
