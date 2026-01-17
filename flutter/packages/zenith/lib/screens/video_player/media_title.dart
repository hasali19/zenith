import 'package:flutter/material.dart';
import 'package:zenith/api.dart';
import 'package:zenith/theme.dart';

class MediaTitle extends StatelessWidget {
  final MediaItem item;

  const MediaTitle({super.key, required this.item});

  @override
  Widget build(BuildContext context) {
    if ((item.getSeasonEpisode(), item.grandparent?.name) case (
      final name?,
      final showName?,
    )) {
      return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('$name: ${item.name}', style: context.zenithTheme.titleMedium),
          Text(showName, style: context.zenithTheme.bodyMedium),
        ],
      );
    } else {
      return Text(item.name);
    }
  }
}
