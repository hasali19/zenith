import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/api.dart';
import 'package:zenith/router.dart';
import 'package:zenith/screens/media_library.dart';

final collectionsProvider = FutureProvider.autoDispose((ref) {
  final zenith = ref.watch(apiProvider);
  return zenith.fetchCollections();
});

final _collectionsLibraryItemsProvider = Provider.autoDispose((ref) {
  final collections = ref.watch(collectionsProvider);
  return collections.whenData((collections) => collections
      .map((e) => MediaLibraryItem(
            id: e.id,
            title: e.name,
            subtitle: null,
            poster: e.poster,
            isWatched: true,
          ))
      .toList());
});

class CollectionsScreen extends ConsumerStatefulWidget {
  const CollectionsScreen({super.key});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() =>
      _CollectionsScreenState();
}

class _CollectionsScreenState extends ConsumerState<CollectionsScreen> {
  @override
  Widget build(BuildContext context) {
    return MediaLibraryScreen(
      provider: _collectionsLibraryItemsProvider,
      posterFallback: Icons.video_collection,
      onRefresh: () => ref.refresh(collectionsProvider.future),
      onItemTap: (item) async {
        await context.router.push(CollectionDetailsScreenRoute(id: item.id));
        ref.invalidate(collectionsProvider);
      },
      onItemLongPress: (item) {
        showModalBottomSheet(
          context: context,
          builder: (context) => SafeArea(
            child: Wrap(children: [
              ListTile(
                iconColor: Colors.red,
                textColor: Colors.red,
                leading: const Icon(Icons.delete),
                title: const Text('Delete'),
                onTap: () async {
                  await ref.read(apiProvider).deleteCollection(item.id);
                  ref.invalidate(collectionsProvider);
                  Navigator.pop(context);
                },
              ),
            ]),
          ),
        );
      },
    );
  }
}
