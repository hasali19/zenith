import 'dart:math' as math;

import 'package:flutter/material.dart';
import 'package:zenith_flutter/api.dart';
import 'package:zenith_flutter/text_one_line.dart';

class MediaLibraryScreen extends StatefulWidget {
  final Future<List<MediaItem>> Function() provider;
  final void Function(MediaItem) onItemTap;

  const MediaLibraryScreen({
    Key? key,
    required this.provider,
    required this.onItemTap,
  }) : super(key: key);

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
          return MediaItemGrid(
            items: snapshot.data!,
            onItemTap: widget.onItemTap,
          );
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
  final void Function(MediaItem) onItemTap;

  final ScrollController _scrollController = ScrollController();

  MediaItemGrid({
    Key? key,
    required this.items,
    required this.onItemTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final desktop = MediaQuery.of(context).size.width > 960;
    return LayoutBuilder(builder: ((context, constraints) {
      final maxColWidth = desktop ? 180.0 : 120.0;
      final gridPadding = desktop ? 64.0 : 0.0;
      final itemSpacing = desktop ? 32.0 : 8.0;
      final borderRadius = desktop ? 16.0 : 8.0;

      final gridWidth = constraints.maxWidth - gridPadding * 2;
      final cols = (gridWidth / (maxColWidth + itemSpacing * 2)).ceil();
      final colWidth = gridWidth / cols;
      final infoTopPadding = desktop ? 16.0 : 8.0;

      final theme = Theme.of(context).textTheme;
      final titleStyle = desktop
          ? theme.subtitle1!.copyWith(fontWeight: FontWeight.bold)
          : theme.subtitle2;
      final subtitleStyle = desktop
          ? theme.bodyMedium!.copyWith(color: theme.caption!.color)
          : theme.caption;

      return ListView.builder(
        controller: _scrollController,
        padding: EdgeInsets.all(gridPadding),
        itemCount: (items.length / cols).ceil(),
        itemBuilder: (context, rowIndex) {
          final columns = <Widget>[];
          final maxItemIndex = math.min((rowIndex + 1) * cols, items.length);

          for (var i = rowIndex * cols; i < maxItemIndex; i++) {
            final item = items[i];

            final poster = Material(
              elevation: 2.0,
              type: MaterialType.card,
              clipBehavior: Clip.hardEdge,
              borderRadius: BorderRadius.all(Radius.circular(borderRadius)),
              child: Ink.image(
                fit: BoxFit.cover,
                image: NetworkImage(item.poster!),
                child: InkWell(
                  onTap: () => onItemTap(item),
                  child: const AspectRatio(aspectRatio: 2 / 3),
                ),
              ),
            );

            final info = Padding(
              padding: EdgeInsets.only(top: infoTopPadding, bottom: 8),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  TextOneLine(
                    item.title,
                    style: titleStyle,
                  ),
                  const SizedBox(height: 2),
                  if (item.year != null)
                    TextOneLine(
                      item.year.toString(),
                      style: subtitleStyle,
                    ),
                ],
              ),
            );

            columns.add(Container(
              width: colWidth,
              padding: EdgeInsets.all(itemSpacing / 2),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [poster, info],
              ),
            ));
          }

          return Row(children: columns);
        },
      );
    }));
  }
}
