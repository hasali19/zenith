import 'dart:math' as math;

import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:sized_context/sized_context.dart';
import 'package:zenith/api.dart';
import 'package:zenith/poster_item.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/router.dart';

class MediaLibraryScreen extends StatefulWidget {
  final Future<List<MediaItem>> Function() provider;

  const MediaLibraryScreen({Key? key, required this.provider})
      : super(key: key);

  @override
  State<MediaLibraryScreen> createState() => _MediaLibraryScreenState();
}

class _MediaLibraryScreenState extends State<MediaLibraryScreen> {
  late Future<List<MediaItem>> _items;

  @override
  void initState() {
    super.initState();
    _items = widget.provider();
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<List<MediaItem>>(
      future: _items,
      builder: ((context, snapshot) {
        if (snapshot.hasData) {
          return MediaItemGrid(items: snapshot.data!);
        } else if (snapshot.hasError) {
          return Text("${snapshot.error}");
        }
        return const Center(child: CircularProgressIndicator());
      }),
    );
  }
}

class MediaItemGrid extends ConsumerWidget {
  final List<MediaItem> items;

  final ScrollController _scrollController = ScrollController();

  MediaItemGrid({Key? key, required this.items}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final desktop = MediaQuery.of(context).isDesktop;
    return LayoutBuilder(builder: ((context, constraints) {
      final api = ref.read(apiProvider);

      final maxColWidth = desktop ? 240.0 : 150.0;
      final gridPadding = desktop ? 64.0 : 4.0;
      final itemSpacing = desktop ? 16.0 : 4.0;

      final gridWidth = constraints.maxWidth - gridPadding * 2;
      final cols = (gridWidth / (maxColWidth + itemSpacing * 2)).ceil();
      final colWidth = gridWidth / cols;
      final infoTopPadding = desktop ? 16.0 : 8.0;

      return ListView.builder(
        controller: _scrollController,
        padding: EdgeInsets.all(gridPadding) + context.mq.padding,
        itemCount: (items.length / cols).ceil(),
        itemBuilder: (context, rowIndex) {
          final columns = <Widget>[];
          final maxItemIndex = math.min((rowIndex + 1) * cols, items.length);

          for (var i = rowIndex * cols; i < maxItemIndex; i++) {
            final item = items[i];
            columns.add(Container(
              width: colWidth,
              padding: EdgeInsets.all(itemSpacing),
              child: PosterItem(
                poster: api.getMediaImageUrl(item.id, ImageType.poster),
                title: item.name,
                subtitle: item.startDate!.year.toString(),
                isWatched: () {
                  if (item.type == MediaType.show) {
                    return item.collectionUserData?.unwatched == 0;
                  } else {
                    return item.videoUserData?.isWatched ?? false;
                  }
                }(),
                infoSeparator: infoTopPadding,
                onTap: () =>
                    context.router.push(ItemDetailsScreenRoute(id: item.id)),
              ),
            ));
          }

          return Row(children: columns);
        },
      );
    }));
  }
}
