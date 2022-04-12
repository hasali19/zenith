import 'package:assorted_layout_widgets/assorted_layout_widgets.dart';
import 'package:flutter/material.dart';
import 'package:zenith_flutter/api.dart';

class MediaLibraryScreen extends StatefulWidget {
  final Future<List<MediaItem>> Function() provider;

  const MediaLibraryScreen({
    Key? key,
    required this.provider,
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

  const MediaItemGrid({
    Key? key,
    required this.items,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(builder: ((context, constraints) {
      const maxColWidth = 180.0;
      const padding = 8.0;

      final maxGridWidth = constraints.maxWidth - padding;
      final cols = (maxGridWidth / maxColWidth).ceil();
      final width = maxGridWidth / cols;
      final height = (3 / 2) * (width - padding) + 64;

      return GridView.extent(
        padding: const EdgeInsets.all(padding),
        mainAxisSpacing: padding,
        crossAxisSpacing: padding,
        maxCrossAxisExtent: maxColWidth,
        childAspectRatio: width / height,
        children: items.map((item) {
          final theme = Theme.of(context).textTheme;

          final poster = Material(
            elevation: 2.0,
            type: MaterialType.card,
            clipBehavior: Clip.hardEdge,
            borderRadius: const BorderRadius.all(Radius.circular(4)),
            child: Ink.image(
              fit: BoxFit.cover,
              image: NetworkImage(item.getPoster()!),
              child: InkWell(
                onTap: () {},
                child: const AspectRatio(aspectRatio: 2 / 3),
              ),
            ),
          );

          final info = Padding(
            padding: const EdgeInsets.only(top: 8, bottom: 8),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                TextOneLine(
                  item.getTitle(),
                  style: theme.subtitle2,
                ),
                const SizedBox(height: 2),
                if (item.getYear() != null)
                  TextOneLine(
                    item.getYear().toString(),
                    style: theme.caption,
                  ),
              ],
            ),
          );

          return Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [poster, info],
          );
        }).toList(),
      );
    }));
  }
}
