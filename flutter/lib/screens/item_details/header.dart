import 'package:auto_route/auto_route.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:zenith_flutter/api.dart';
import 'package:zenith_flutter/responsive.dart';
import 'package:zenith_flutter/router.dart';
import 'package:zenith_flutter/screens/item_details/item_details.dart';

import 'package:zenith_flutter/download.dart'
    if (dart.library.html) 'package:zenith_flutter/download_web.dart';

class HeaderContent extends StatelessWidget {
  const HeaderContent({Key? key, required this.item}) : super(key: key);

  final MediaItem item;

  @override
  Widget build(BuildContext context) {
    final isDesktop = MediaQuery.of(context).isDesktop;

    final poster = Poster(url: getMediaImageUrl(item.id, ImageType.poster));
    final subtitle = _buildSubtitle(context);
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
      if (subtitle != null) subtitle,
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

  Widget? _buildSubtitle(BuildContext context) {
    final theme = Theme.of(context);
    final style = theme.textTheme.subtitle1;
    final date = item.startDate;
    final videoInfo = item.videoInfo;
    final seasonEpisode = item.getSeasonEpisode();

    final items = [
      if (seasonEpisode != null) TextSpan(text: seasonEpisode),
      if (date != null) TextSpan(text: "${date.year}"),
      if (videoInfo != null)
        TextSpan(text: _formatDuration(videoInfo.duration)),
    ];

    final separated = <TextSpan>[];
    for (var i = 0; i < items.length; i++) {
      if (i > 0) {
        separated.add(const TextSpan(children: [
          WidgetSpan(child: SizedBox(width: 12)),
          TextSpan(text: "•"),
          WidgetSpan(child: SizedBox(width: 12)),
        ]));
      }
      separated.add(items[i]);
    }

    return Padding(
      padding: const EdgeInsets.only(top: 16.0),
      child: Text.rich(TextSpan(
        style: style,
        children: separated,
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
          context.router.push(VideoPlayerScreenRoute(
            id: item.id,
            startPosition: shouldResume ? position : 0,
          ));
        },
      ));

      if (shouldResume) {
        actions.add(const SizedBox(width: 16));
        actions.add(Text("${(position / duration * 100).toInt()}%"));
      }

      actions.add(const SizedBox(width: 16));
      actions.add(WatchedToggleButton(
        isWatched: item.videoUserData?.isWatched ?? false,
        onChange: _onIsWatchedToggled,
      ));

      if (kIsWeb) {
        actions.add(const SizedBox(width: 16));
        actions.add(IconButton(
          icon: const Icon(Icons.download),
          onPressed: () {
            downloadFile(getVideoUrl(item.id, attachment: true));
          },
        ));
      }
    }

    return actions;
  }

  void _onIsWatchedToggled(isWatched) {
    updateUserData(item.id, VideoUserDataPatch(isWatched: isWatched));
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

class WatchedToggleButton extends StatefulWidget {
  final bool isWatched;
  final void Function(bool isWatched) onChange;

  const WatchedToggleButton({
    Key? key,
    required this.isWatched,
    required this.onChange,
  }) : super(key: key);

  @override
  State<WatchedToggleButton> createState() => _WatchedToggleButtonState();
}

class _WatchedToggleButtonState extends State<WatchedToggleButton> {
  late bool _isSelected;

  @override
  void initState() {
    super.initState();
    _isSelected = widget.isWatched;
  }

  @override
  Widget build(BuildContext context) {
    return IconButton(
      onPressed: () {
        setState(() {
          _isSelected = !_isSelected;
          widget.onChange(_isSelected);
        });
      },
      icon: const Icon(Icons.check_circle_outline),
      isSelected: _isSelected,
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
