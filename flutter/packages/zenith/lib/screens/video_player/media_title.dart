import 'package:flutter/material.dart';
import 'package:zenith/api.dart';
import 'package:zenith/theme.dart';

class MediaTitle extends StatelessWidget {
  final MediaItem item;

  const MediaTitle({super.key, required this.item});

  @override
  Widget build(BuildContext context) {
    if (item.type == MediaType.episode) {
      return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            '${item.getSeasonEpisode()!}: ${item.name}',
            style: context.zenithTheme.titleMedium,
          ),
          Text(item.grandparent!.name, style: context.zenithTheme.bodyMedium),
        ],
      );
    } else {
      return Text(item.name);
    }
  }
}
