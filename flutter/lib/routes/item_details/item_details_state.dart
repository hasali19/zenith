import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:zenith/api.dart';

part 'item_details_state.freezed.dart';

@freezed
class ItemDetailsState with _$ItemDetailsState {
  factory ItemDetailsState({
    required MediaItem item,
    required String posterUrl,
    required String backdropUrl,
    required List<EpisodeGroupState> seasons,
    required PlayableState? playable,
    required bool isWatched,
    required String? durationText,
    required String? videoDownloadUrl,
  }) = _ItemDetailsState;
}

@freezed
class PlayableState with _$PlayableState {
  factory PlayableState({
    required int id,
    required double? progress,
    required String? caption,
    required bool shouldResume,
    required double playPosition,
  }) = _PlayableState;
}

@freezed
class EpisodeGroupState with _$EpisodeGroupState {
  factory EpisodeGroupState({
    required String name,
    required List<EpisodeState> episodes,
  }) = _EpisodeGroupState;
}

@freezed
class EpisodeState with _$EpisodeState {
  factory EpisodeState({
    required int id,
    required String thumbnailUrl,
    required String? overview,
    required bool isWatched,
    required String title,
  }) = _EpisodeState;
}
