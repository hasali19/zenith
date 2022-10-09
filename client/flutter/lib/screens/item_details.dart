import 'dart:convert';
import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:zenith_flutter/api.dart';
import 'package:zenith_flutter/responsive.dart';
import 'package:zenith_flutter/screens/video_player.dart';

final transparentImage = base64Decode(
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=");

class ItemDetailsScreen extends StatelessWidget {
  final MediaItem item;
  final Widget? body;

  const ItemDetailsScreen({
    Key? key,
    required this.item,
    this.body,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    final isDesktop = MediaQuery.of(context).isDesktop;
    return Scaffold(
      extendBodyBehindAppBar: true,
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        elevation: 0,
        scrolledUnderElevation: 0,
      ),
      body: Stack(
        fit: StackFit.expand,
        children: [
          Backdrop(url: getMediaImageUrl(item.id, ImageType.backdrop)),
          BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 10, sigmaY: 10),
            child: Container(
              decoration: BoxDecoration(
                  color:
                      (isDark ? Colors.black : Colors.white).withOpacity(0.5)),
              child: CustomScrollView(
                slivers: [
                  SliverToBoxAdapter(
                    child: Padding(
                      padding: isDesktop
                          ? const EdgeInsets.fromLTRB(128, 128, 128, 32)
                          : const EdgeInsets.fromLTRB(16, 96, 16, 16),
                      child: ItemDetails(item: item),
                    ),
                  ),
                  if (body != null) body!,
                ],
              ),
            ),
          )
        ],
      ),
    );
  }
}

class ItemDetails extends StatelessWidget {
  const ItemDetails({Key? key, required this.item}) : super(key: key);

  final MediaItem item;

  @override
  Widget build(BuildContext context) {
    final isDesktop = MediaQuery.of(context).isDesktop;

    final poster = Poster(url: getMediaImageUrl(item.id, ImageType.poster));
    final year = _buildYear(context);
    final overview = _buildOverview();

    final mainContent = [
      if (item.grandparent != null)
        Text(
          item.grandparent!.name,
          style: Theme.of(context).textTheme.headline6,
        )
      else if (item.parent != null)
        Text(item.parent!.name),
      Title(text: item.name),
      if (year != null) year,
      _buildActions(context),
      if (overview != null) overview,
    ];

    if (isDesktop) {
      return Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(width: 300, child: poster),
          const SizedBox(width: 48),
          Flexible(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: mainContent,
            ),
          ),
        ],
      );
    } else {
      return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Center(child: SizedBox(width: 200, child: poster)),
          const SizedBox(height: 48),
          ...mainContent,
        ],
      );
    }
  }

  Widget? _buildYear(BuildContext context) {
    final theme = Theme.of(context);
    final style = theme.textTheme.subtitle1;
    final date = item.startDate;
    final videoInfo = item.videoInfo;
    return Padding(
      padding: const EdgeInsets.only(top: 16.0),
      child: Text.rich(TextSpan(
        style: style,
        children: [
          if (date != null) TextSpan(text: "${date.year}"),
          if (date != null && videoInfo != null)
            const TextSpan(children: [
              WidgetSpan(child: SizedBox(width: 12)),
              TextSpan(text: "•"),
              WidgetSpan(child: SizedBox(width: 12)),
            ]),
          if (videoInfo != null)
            TextSpan(text: _formatDuration(videoInfo.duration)),
        ],
      )),
    );
  }

  Widget? _buildOverview() {
    final overview = item.overview;
    if (overview == null) return null;
    return ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 600),
      child: Overview(text: overview),
    );
  }

  Widget _buildActions(BuildContext context) {
    final actions = _buildActionsItems(context);
    if (actions.isEmpty) {
      return const SizedBox(height: 32);
    }
    return Container(
      margin: const EdgeInsets.symmetric(vertical: 16),
      child: Row(children: actions),
    );
  }

  List<Widget> _buildActionsItems(BuildContext context) {
    final actions = <Widget>[];

    if (item.type == MediaType.movie || item.type == MediaType.episode) {
      final position = item.videoUserData?.position ?? 0;
      final duration = item.videoInfo!.duration;
      final shouldResume =
          position > 0.05 * duration && position < 0.9 * duration;
      actions.add(ElevatedButton.icon(
        icon: const Icon(Icons.play_arrow),
        label: Text(shouldResume ? "Resume" : "Play"),
        onPressed: () {
          Navigator.push(
            context,
            MaterialPageRoute(
              builder: (context) => VideoPlayerScreen(
                id: item.id,
                startPosition: shouldResume ? position : 0,
              ),
            ),
          );
        },
      ));

      if (shouldResume) {
        actions.add(Padding(
          padding: const EdgeInsets.symmetric(horizontal: 16),
          child: Text("${(position / duration * 100).toInt()}%"),
        ));
      }
    }

    return actions;
  }

  String _formatDuration(double duration) {
    if (duration <= 90 * 60) {
      return "${duration ~/ 60}m";
    } else {
      final hours = duration ~/ 3600;
      final minutes = (duration % 3600) ~/ 60;
      return "${hours}h ${minutes}m";
    }
  }
}

class Backdrop extends StatelessWidget {
  final String url;

  const Backdrop({
    Key? key,
    required this.url,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return FadeInImage.memoryNetwork(
      placeholder: transparentImage,
      image: url,
      fit: BoxFit.cover,
    );
  }
}

class Poster extends StatelessWidget {
  final String url;

  const Poster({
    Key? key,
    required this.url,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: AspectRatio(
        aspectRatio: 2.0 / 3.0,
        child: FadeInImage.memoryNetwork(
          placeholder: transparentImage,
          image: url,
          fit: BoxFit.cover,
        ),
      ),
    );
  }
}

class Title extends StatelessWidget {
  final String text;

  const Title({
    Key? key,
    required this.text,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final style = theme.textTheme.headline3!.copyWith(
      color: theme.brightness == Brightness.dark ? Colors.white : Colors.black,
    );
    return Text(text, style: style);
  }
}

class Subtitle extends StatelessWidget {
  final String text;

  const Subtitle({
    Key? key,
    required this.text,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final style = theme.textTheme.headline5;
    return Text(text, style: style);
  }
}

class Overview extends StatelessWidget {
  final String text;

  const Overview({
    Key? key,
    required this.text,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final style = theme.textTheme.bodyLarge!.copyWith(fontSize: 16);
    return Text(text, style: style);
  }
}
