import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:zenith/api.dart';
import 'package:zenith/database/database.dart' as db;

part 'item_details_state.freezed.dart';

@freezed
abstract class ItemDetailsState with _$ItemDetailsState {
  factory ItemDetailsState({
    required MediaItem item,
    required ImageId? poster,
    required ImageId? backdrop,
    required List<EpisodeGroupState> seasons,
    required PlayableState? playable,
    required bool isWatched,
    required String? durationText,
    required db.DownloadedFile? downloadedFile,
  }) = _ItemDetailsState;
}

@freezed
abstract class PlayableState with _$PlayableState {
  factory PlayableState({
    required int id,
    required int? seasonIndex,
    required double? progress,
    required String? caption,
    required bool shouldResume,
    required double playPosition,
  }) = _PlayableState;
}

@freezed
abstract class EpisodeGroupState with _$EpisodeGroupState {
  factory EpisodeGroupState({
    required String name,
    required List<EpisodeState> episodes,
  }) = _EpisodeGroupState;
}

@freezed
abstract class EpisodeState with _$EpisodeState {
  factory EpisodeState({
    required int id,
    required ImageId? thumbnail,
    required String? overview,
    required bool isWatched,
    required String title,
  }) = _EpisodeState;
}
