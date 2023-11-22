import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/api.dart';

class ItemDetailsModel {
  final MediaItem item;
  final List<SeasonModel> seasons;

  late final MediaItem? _playable;
  late final bool isWatched;

  factory ItemDetailsModel(MediaItem item, List<SeasonModel> seasons) {
    return ItemDetailsModel._(item, seasons);
  }

  ItemDetailsModel._(this.item, this.seasons) {
    _playable = _getPlayableItem();

    if (item.type == MediaType.movie || item.type == MediaType.episode) {
      isWatched = item.videoUserData?.isWatched ?? false;
    } else {
      isWatched = item.collectionUserData?.unwatched == 0;
    }
  }

  MediaItem? get playable => _playable;
  double get playableProgress {
    final position = playable?.videoUserData?.position ?? 0;
    final duration = playable?.videoFile!.duration ?? 0;
    final progress = position / duration;
    if (progress > 0.05 && progress < 0.9) {
      return progress;
    }
    return 0;
  }

  double get _playableRemaining {
    final position = playable?.videoUserData?.position ?? 0;
    final duration = playable?.videoFile!.duration ?? 0;
    return duration - position;
  }

  String? get playableCaption {
    String caption = '';

    if (item.type == MediaType.show) {
      final seasonEpisode = playable?.getSeasonEpisode();
      if (seasonEpisode != null) {
        caption += seasonEpisode;
      }
    }

    if (playableProgress > 0) {
      if (caption.isNotEmpty) {
        caption += ' - ';
      }
      caption += '${_formatDuration(_playableRemaining)} left';
    }

    if (caption.isNotEmpty) {
      return caption;
    }

    return null;
  }

  MediaItem? _getPlayableItem() {
    if (item.type == MediaType.show) {
      MediaItem? lastWatched;
      int? lastWatchedS;
      int? lastWatchedE;
      for (var s = 0; s < seasons.length; s++) {
        final season = seasons[s];
        for (var e = 0; e < season.episodes.length; e++) {
          final episode = season.episodes[e];
          final userData = episode.videoUserData!;
          if (lastWatched == null ||
              userData.lastWatchedAt != null &&
                  (lastWatched.videoUserData!.lastWatchedAt == null ||
                      userData.lastWatchedAt!.isAfter(
                          lastWatched.videoUserData!.lastWatchedAt!))) {
            lastWatched = episode;
            lastWatchedS = s;
            lastWatchedE = e;
          }
        }
      }
      if (lastWatched != null &&
          (lastWatched.videoUserData?.position ?? 0) >
              lastWatched.videoFile!.duration * 0.9) {
        if (lastWatchedE! + 1 < seasons[lastWatchedS!].episodes.length) {
          return seasons[lastWatchedS].episodes[lastWatchedE + 1];
        } else if (lastWatchedS + 1 < seasons.length) {
          return seasons[lastWatchedS + 1].episodes[0];
        } else {
          return seasons[0].episodes[0];
        }
      }
      return lastWatched;
    } else {
      return item;
    }
  }

  String? get formattedDuration {
    final video = item.videoFile;
    if (video == null) return null;
    return _formatDuration(video.duration);
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
}

class SeasonModel {
  final MediaItem item;
  final List<MediaItem> episodes;

  SeasonModel(this.item, this.episodes);
}

final itemDetailsModelProvider =
    FutureProvider.autoDispose.family<ItemDetailsModel, int>((ref, id) async {
  final api = ref.watch(apiProvider);
  final item = await api.fetchMediaItem(id);
  if (item.type == MediaType.show) {
    final seasons = <SeasonModel>[];
    for (final season in await api.fetchSeasons(item.id)) {
      final episodes = await api.fetchEpisodes(season.id);
      seasons.add(SeasonModel(season, episodes));
    }
    return ItemDetailsModel(item, seasons);
  }
  return ItemDetailsModel(item, []);
});
