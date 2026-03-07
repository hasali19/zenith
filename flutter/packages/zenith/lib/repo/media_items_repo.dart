import 'package:drift/drift.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../api.dart';
import '../database/database.dart' as db;

export '../api.dart' show MediaItem;

class MediaItemsRepo {
  final ZenithApiClient _api;
  final db.AppDatabase _db;

  MediaItemsRepo(ZenithApiClient api, db.AppDatabase db) : _api = api, _db = db;

  Future<MediaItem?> getById(int id) async {
    MediaItem item;
    try {
      item = await _api.fetchMediaItem(id);
    } catch (e) {
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
        rethrow;
      } else {
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
    return item;
  }
}

final mediaItemsRepoProvider = Provider((ref) {
  return MediaItemsRepo(ref.watch(apiProvider), ref.watch(db.databaseProvider));
});
