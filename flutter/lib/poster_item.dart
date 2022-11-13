import 'package:flutter/material.dart';
import 'package:zenith_flutter/screens/item_details/item_details.dart';
import 'package:zenith_flutter/text_one_line.dart';
import 'package:zenith_flutter/theme.dart';

class PosterItem extends StatelessWidget {
  final String poster;
  final String title;
  final String subtitle;
  final double infoSeparator;
  final void Function() onTap;

  const PosterItem({
    Key? key,
    required this.poster,
    required this.title,
    required this.subtitle,
    required this.infoSeparator,
    required this.onTap,
  }) : super(key: key);

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
              Material(
                elevation: cardTheme.elevation ?? 1,
                type: MaterialType.card,
                clipBehavior: Clip.hardEdge,
                shape: cardTheme.shape,
                child: FadeInImage.memoryNetwork(
                  placeholder: transparentImage,
                  image: poster,
                ),
              ),
              Positioned.fill(
                child: Material(
                  color: Colors.transparent,
                  clipBehavior: Clip.hardEdge,
                  shape: cardTheme.shape,
                  child: InkWell(onTap: onTap),
                ),
              )
            ],
          ),
        ),
        const SizedBox(height: 8),
        TextOneLine(title, style: context.zenithTheme.bodyMedium),
        TextOneLine(subtitle, style: context.zenithTheme.bodySmall),
      ],
    );
  }
}
