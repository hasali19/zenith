import 'package:assorted_layout_widgets/assorted_layout_widgets.dart';
import 'package:flutter/material.dart';
import 'package:zenith_flutter/api.dart';

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
      final padding = desktop ? 32.0 : 0.0;
      final spacing = desktop ? 32.0 : 8.0;
      final borderRadius = desktop ? 16.0 : 8.0;

      final maxGridWidth = constraints.maxWidth - spacing - padding * 2;
      final cols = (maxGridWidth / (maxColWidth + spacing)).floor();
      final width = maxGridWidth / cols - spacing;
      final height = (3 / 2) * width + (desktop ? 64 : 50);

      final theme = Theme.of(context).textTheme;
      final titleStyle = desktop
          ? theme.subtitle1!.copyWith(fontWeight: FontWeight.bold)
          : theme.subtitle2;
      final subtitleStyle = desktop
          ? theme.bodyMedium!.copyWith(color: theme.caption!.color)
          : theme.caption;

      return GridView.count(
        controller: _scrollController,
        crossAxisCount: cols,
        childAspectRatio: width / height,
        padding: EdgeInsets.all(spacing + padding),
        mainAxisSpacing: spacing,
        crossAxisSpacing: spacing,
        children: items.map((item) {
          final poster = Material(
            elevation: 2.0,
            type: MaterialType.card,
            clipBehavior: Clip.hardEdge,
            borderRadius: BorderRadius.all(Radius.circular(borderRadius)),
            child: Ink.image(
              fit: BoxFit.cover,
              image: NetworkImage(item.getPoster()!),
              child: InkWell(
                onTap: () => onItemTap(item),
                child: const AspectRatio(aspectRatio: 2 / 3),
              ),
            ),
          );

          final info = Padding(
            padding: EdgeInsets.only(top: desktop ? 16 : 8, bottom: 8),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                TextOneLine(
                  item.getTitle(),
                  style: titleStyle,
                ),
                const SizedBox(height: 2),
                if (item.getYear() != null)
                  TextOneLine(
                    item.getYear().toString(),
                    style: subtitleStyle,
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
