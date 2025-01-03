import 'package:flutter/material.dart';
import 'package:zenith/api.dart';
import 'package:zenith/image.dart';
import 'package:zenith/text_one_line.dart';
import 'package:zenith/theme.dart';

class PosterItem extends StatelessWidget {
  final ImageId? imageId;
  final int requestWidth;
  final IconData fallbackIcon;
  final String title;
  final String? subtitle;
  final bool isWatched;
  final double infoSeparator;
  final void Function() onTap;
  final void Function()? onLongPress;

  const PosterItem({
    super.key,
    required this.imageId,
    required this.requestWidth,
    required this.fallbackIcon,
    required this.title,
    required this.subtitle,
    required this.isWatched,
    required this.infoSeparator,
    required this.onTap,
    this.onLongPress,
  });

  @override
  Widget build(BuildContext context) {
    final cardTheme = Theme.of(context).cardTheme;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        AspectRatio(
          aspectRatio: 2 / 3,
          child: Stack(
            children: [
              Positioned.fill(
                child: Card(
                  elevation: cardTheme.elevation ?? 1,
                  margin: EdgeInsets.zero,
                  clipBehavior: Clip.hardEdge,
                  shape: cardTheme.shape,
                  child: switch (imageId) {
                    null => Center(child: Icon(fallbackIcon, size: 40)),
                    final imageId =>
                      ZenithApiImage(id: imageId, requestWidth: requestWidth)
                  },
                ),
              ),
              if (!isWatched)
                const Positioned.fill(
                  child: Align(
                    alignment: Alignment.topRight,
                    child: Padding(
                      padding: EdgeInsets.all(8),
                      child: Icon(Icons.circle, size: 14, color: Colors.amber),
                    ),
                  ),
                ),
              Positioned.fill(
                child: Material(
                  color: Colors.transparent,
                  clipBehavior: Clip.hardEdge,
                  shape: cardTheme.shape,
                  child: InkWell(
                    onTap: onTap,
                    onLongPress: onLongPress,
                  ),
                ),
              ),
            ],
          ),
        ),
        const SizedBox(height: 8),
        TextOneLine(title, style: context.zenithTheme.bodyMedium),
        if (subtitle != null)
          TextOneLine(subtitle!, style: context.zenithTheme.bodySmall),
      ],
    );
  }
}
