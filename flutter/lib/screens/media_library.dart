import 'dart:math' as math;

import 'package:flutter/material.dart';
import 'package:zenith_flutter/api.dart';
import 'package:zenith_flutter/poster_item.dart';
import 'package:zenith_flutter/responsive.dart';
import 'package:zenith_flutter/screens/item_details/item_details.dart';

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

class MediaItemGrid extends StatelessWidget {
  final List<MediaItem> items;

  final ScrollController _scrollController = ScrollController();

  MediaItemGrid({Key? key, required this.items}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final desktop = MediaQuery.of(context).isDesktop;
    return LayoutBuilder(builder: ((context, constraints) {
      final maxColWidth = desktop ? 180.0 : 120.0;
      final gridPadding = desktop ? 64.0 : 4.0;
      final itemSpacing = desktop ? 32.0 : 8.0;

      final gridWidth = constraints.maxWidth - gridPadding * 2;
      final cols = (gridWidth / (maxColWidth + itemSpacing * 2)).ceil();
      final colWidth = gridWidth / cols;
      final infoTopPadding = desktop ? 16.0 : 8.0;

      return ListView.builder(
        controller: _scrollController,
        padding: EdgeInsets.all(gridPadding),
        itemCount: (items.length / cols).ceil(),
        itemBuilder: (context, rowIndex) {
          final columns = <Widget>[];
          final maxItemIndex = math.min((rowIndex + 1) * cols, items.length);

          for (var i = rowIndex * cols; i < maxItemIndex; i++) {
            final item = items[i];
            columns.add(Container(
              width: colWidth,
              padding: EdgeInsets.all(itemSpacing / 2),
              child: PosterItem(
                poster: getMediaImageUrl(item.id, ImageType.poster),
                title: item.name,
                subtitle: item.startDate!.year.toString(),
                infoSeparator: infoTopPadding,
                onTap: () => Navigator.push(
                  context,
                  MaterialPageRoute(
                    builder: (context) => ItemDetailsScreen(id: item.id),
                  ),
                ),
              ),
            ));
          }

          return Row(children: columns);
        },
      );
    }));
  }
}
