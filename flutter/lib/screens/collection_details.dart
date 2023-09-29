import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/api.dart';
import 'package:zenith/router.dart';
import 'package:zenith/screens/media_library.dart';

final _collectionItemsProvider = FutureProvider.family((ref, int id) async {
  final api = ref.watch(apiProvider);
  final items = await api.fetchCollectionItems(id);
  return items.map((e) => MediaLibraryItem.fromMediaItem(e, api)).toList();
});

@RoutePage()
class CollectionDetailsScreen extends ConsumerStatefulWidget {
  final int id;

  const CollectionDetailsScreen({
    super.key,
    @pathParam required this.id,
  });

  @override
  ConsumerState<ConsumerStatefulWidget> createState() {
    return _CollectionDetailsScreenState();
  }
}

class _CollectionDetailsScreenState
    extends ConsumerState<CollectionDetailsScreen> {
  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    ref.invalidate(_collectionItemsProvider(widget.id));
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(),
      body: MediaLibraryScreen(
        provider: _collectionItemsProvider(widget.id),
        posterFallback: Icons.movie,
        onRefresh: () =>
            ref.refresh(_collectionItemsProvider(widget.id).future),
        onItemTap: (item) => context.router.push(ItemDetailsRoute(id: item.id)),
        onItemLongPress: (item) {
          showModalBottomSheet(
            context: context,
            builder: (context) => SafeArea(
              child: Wrap(children: [
                ListTile(
                  title: const Text('Remove from collection'),
                  onTap: () async {
                    final items = await ref
                        .read(apiProvider)
                        .fetchCollectionItems(widget.id);

                    final newIds = items
                        .map((e) => e.id)
                        .where((id) => id != item.id)
                        .toList();

                    await ref
                        .read(apiProvider)
                        .updateCollection(widget.id, newIds);

                    ref.invalidate(_collectionItemsProvider(widget.id));

                    Navigator.pop(context);
                  },
                ),
              ]),
            ),
          );
        },
      ),
    );
  }
}
