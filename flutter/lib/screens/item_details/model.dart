import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/api.dart';

class ItemDetailsModel {
  final MediaItem item;
  final List<SeasonModel> seasons;

  late final MediaItem? _playable;

  factory ItemDetailsModel(MediaItem item, List<SeasonModel> seasons) {
    return ItemDetailsModel._(item, seasons);
  }

  ItemDetailsModel._(this.item, this.seasons) {
    _playable = _getPlayableItem();
  }

  MediaItem? get playable => _playable;

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
