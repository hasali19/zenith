import 'package:assorted_layout_widgets/assorted_layout_widgets.dart';
import 'package:flutter/material.dart';

class PosterGrid extends StatelessWidget {
  final double spacing;
  final double maxItemWidth;

  final int count;

  final String Function(int) poster;
  final String Function(int) primary;
  final String Function(int) secondary;
  final void Function(int) onItemTap;

  PosterGrid({
    @required this.count,
    @required this.poster,
    @required this.primary,
    @required this.secondary,
    this.spacing = 8,
    this.maxItemWidth = 120,
    this.onItemTap,
  });

  Widget _item(int i) {
    return PosterItem(
      poster: poster(i),
      primary: primary(i),
      secondary: secondary(i),
      onTap: () => onItemTap(i),
    );
  }

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(builder: (context, constraints) {
      const spacing = 8.0;
      const maxWidth = 120.0;

      final columns =
          ((constraints.maxWidth - spacing) / (maxWidth + spacing)).floor();

      final width = (constraints.maxWidth - spacing) / columns - spacing;
      final height = (3 / 2) * width + 50;

      return GridView.count(
        crossAxisCount: columns,
        childAspectRatio: width / height,
        padding: const EdgeInsets.all(spacing),
        mainAxisSpacing: spacing,
        crossAxisSpacing: spacing,
        children: [for (var i = 0; i < count; i++) _item(i)],
      );
    });
  }
}

class PosterItem extends StatelessWidget {
  final String poster;
  final String primary;
  final String secondary;

  final void Function() onTap;

  PosterItem({
    @required this.poster,
    @required this.primary,
    @required this.secondary,
    this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    final poster = Material(
      elevation: 2.0,
      type: MaterialType.card,
      clipBehavior: Clip.hardEdge,
      child: Ink.image(
        fit: BoxFit.cover,
        image: NetworkImage(this.poster),
        child: InkWell(
          child: AspectRatio(aspectRatio: 2 / 3),
          onTap: onTap,
        ),
      ),
    );

    final info = Container(
      margin: EdgeInsets.only(top: 8),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          TextOneLine(
            primary,
            style: Theme.of(context).textTheme.subtitle2,
            overflow: TextOverflow.fade,
          ),
          SizedBox(height: 2),
          Text(
            secondary,
            maxLines: 1,
            style: Theme.of(context).textTheme.caption,
          )
        ],
      ),
    );

    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        poster,
        info,
      ],
    );
  }
}
