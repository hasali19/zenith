import 'package:auto_route/auto_route.dart';
import 'package:expandable/expandable.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:zenith/api.dart';
import 'package:zenith/language_codes.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/screens/item_details/add_to_collection.dart';
import 'package:zenith/screens/item_details/header_layout.dart';
import 'package:zenith/screens/item_details/item_details.dart';
import 'package:zenith/screens/item_details/model.dart';
import 'package:zenith/text_one_line.dart';
import 'package:zenith/theme.dart';

import 'package:zenith/download.dart'
    if (dart.library.html) 'package:zenith/download_web.dart';

class HeaderContent extends ConsumerWidget {
  const HeaderContent({
    Key? key,
    required this.model,
    required this.refresh,
    required this.onPlayPressed,
    required this.onViewItemDetails,
  }) : super(key: key);

  final ItemDetailsModel model;
  final void Function() refresh;
  final void Function(MediaItem item) onPlayPressed;
  final void Function(int id) onViewItemDetails;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final isDesktop = MediaQuery.of(context).isDesktop;
    final api = ref.watch(apiProvider);

    final subtitle = _buildSubtitle(context);
    final metaTable = _buildMetaTable();
    final overview = _buildOverview();
    final videoInfo = _buildVideoInfo(context);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        HeaderLayout(
          backdrop: FadeInImage.memoryNetwork(
            placeholder: transparentImage,
            image: api.getMediaImageUrl(model.item.id, ImageType.backdrop),
            height: 300,
            fit: BoxFit.cover,
            alignment: Alignment.topCenter,
          ),
          poster: Poster(
            url: api.getMediaImageUrl(model.item.id, ImageType.poster),
            progress: model.playableProgress,
            caption: model.playableCaption,
          ),
          title: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              if (model.item.grandparent != null)
                InkWell(
                  child: Text(
                    model.item.grandparent!.name,
                    style: Theme.of(context).textTheme.titleMedium,
                  ),
                  onTap: () => onViewItemDetails(model.item.grandparent!.id),
                ),
              Text(model.item.name, style: context.zenithTheme.titleLarge),
              if (subtitle != null) subtitle,
            ],
          ),
          body: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              if (metaTable != null) metaTable,
              if (overview != null) overview,
              if (videoInfo != null) videoInfo,
            ],
          ),
          playButton: _buildPlayButton(),
          actions: Row(children: _buildActionsItems(context, api)),
          posterWidth: isDesktop ? 300 : 150,
          padding: isDesktop ? 128 : 16,
          separation: isDesktop ? 48 : 16,
        ),
      ],
    );
  }

  Widget? _buildSubtitle(BuildContext context) {
    final theme = Theme.of(context);
    final style = theme.textTheme.titleMedium;
    final seasonEpisode = model.item.getSeasonEpisode();
    final date = model.item.startDate;
    final duration = model.formattedDuration;
    final ageRating = model.item.ageRating;

    final items = [
      if (seasonEpisode != null) TextSpan(text: seasonEpisode),
      if (date != null) TextSpan(text: '${date.year}'),
      if (duration != null) TextSpan(text: duration),
      if (ageRating != null) TextSpan(text: ageRating),
    ];

    final separated = <TextSpan>[];
    for (var i = 0; i < items.length; i++) {
      if (i > 0) {
        separated.add(const TextSpan(children: [
          WidgetSpan(child: SizedBox(width: 8)),
          TextSpan(text: '•'),
          WidgetSpan(child: SizedBox(width: 8)),
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

  Widget? _buildMetaTable() {
    final rows = [
      if (model.item.director != null)
        TableRow(
          children: [
            const TableCell(
                child: Text('Director', style: TextStyle(color: Colors.grey))),
            const TableCell(child: SizedBox()),
            TableCell(child: Text(model.item.director!)),
          ],
        ),
      if (model.item.genres.isNotEmpty)
        TableRow(
          children: [
            const TableCell(
                child: Text('Genres', style: TextStyle(color: Colors.grey))),
            const TableCell(child: SizedBox()),
            TableCell(child: Text(model.item.genres.join(', '))),
          ],
        ),
    ];

    if (rows.isEmpty) {
      return null;
    }

    return Padding(
      padding: const EdgeInsets.only(bottom: 16),
      child: Table(
        columnWidths: const {
          0: IntrinsicColumnWidth(),
          1: FixedColumnWidth(16),
          2: FlexColumnWidth(),
        },
        children: rows,
      ),
    );
  }

  Widget? _buildOverview() {
    final overview = model.item.overview;
    final trailer = model.item.trailer;
    return ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 960),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          if (overview != null) Overview(text: overview),
          if (trailer != null)
            Padding(
              padding: const EdgeInsets.only(top: 16),
              child: Material(
                color: Colors.transparent,
                child: InkWell(
                  child: const Padding(
                    padding: EdgeInsets.all(8.0),
                    child: Column(
                      children: [
                        Icon(Icons.play_circle_outline),
                        Text('Trailer'),
                      ],
                    ),
                  ),
                  onTap: () {
                    final trailer = model.item.trailer;
                    if (trailer != null) {
                      launchUrl(Uri.parse(trailer),
                          mode: LaunchMode.externalApplication);
                    }
                  },
                ),
              ),
            ),
        ],
      ),
    );
  }

  Widget _buildPlayButton() {
    final playable = model.playable;
    return ElevatedButton.icon(
      icon: const Icon(Icons.play_arrow),
      label: Text(playable?.shouldResume == true ? 'Resume' : 'Play'),
      onPressed: () {
        if (playable != null) {
          onPlayPressed(playable);
        }
      },
    );
  }

  List<Widget> _buildActionsItems(BuildContext context, ZenithApiClient api) {
    final isDesktop = context.isDesktop;
    final actions = <Widget>[];

    if (model.item.type == MediaType.movie ||
        model.item.type == MediaType.episode) {
      actions.add(WatchedToggleButton(
        isWatched: model.item.videoUserData?.isWatched ?? false,
        onChange: (v) =>
            api.updateUserData(model.item.id, VideoUserDataPatch(isWatched: v)),
      ));

      if (isDesktop) {
        actions.add(const SizedBox(width: 16));
      }

      actions.add(IconButton(
        icon: const Icon(Icons.download),
        onPressed: () {
          downloadFile(api.getVideoUrl(model.item.id, attachment: true));
        },
      ));

      if (isDesktop) {
        actions.add(const SizedBox(width: 16));
      }
    }

    actions.add(IconButton(
      icon: const Icon(Icons.more_vert),
      onPressed: () => _showOptionsMenu(context, api),
    ));

    return actions;
  }

  void _showOptionsMenu(BuildContext context, ZenithApiClient api) {
    final width = MediaQuery.of(context).size.width;
    showModalBottomSheet(
      context: context,
      constraints: width > 600
          ? const BoxConstraints.expand(width: 600).copyWith(minHeight: 0)
          : null,
      builder: (context) => SafeArea(
        child: Wrap(
          children: [
            if (model.item.type == MediaType.episode) ...[
              ListTile(
                leading: const Icon(Icons.tv),
                title: const Text('Go to show'),
                onTap: () {
                  Navigator.pop(context);
                  onViewItemDetails(model.item.grandparent!.id);
                },
              ),
            ],
            ListTile(
              leading: const Icon(Icons.add),
              title: const Text('Add to collection'),
              onTap: () {
                Navigator.pop(context);
                showDialog(
                  context: context,
                  builder: (context) =>
                      AddToCollectionDialog(id: model.item.id),
                );
              },
            ),
            ListTile(
              leading: const Icon(Icons.search),
              title: const Text('Find match'),
              onTap: () async {
                Navigator.pop(context);
                await api.findMetadataMatch(model.item.id);
                refresh();
              },
            ),
            if (model.item.type == MediaType.episode)
              ListTile(
                leading: const Icon(Icons.edit),
                title: const Text('Fix match'),
                onTap: () async {
                  Navigator.pop(context);
                  await showDialog(
                    context: context,
                    builder: (context) =>
                        FixEpisodeMatchDialog(item: model.item),
                  );
                  refresh();
                },
              ),
            ListTile(
              leading: const Icon(Icons.refresh),
              title: const Text('Refresh metadata'),
              onTap: () async {
                Navigator.pop(context);
                await api.refreshMetadata(model.item.id);
                refresh();
              },
            ),
            ListTile(
              iconColor: Colors.red,
              textColor: Colors.red,
              leading: const Icon(Icons.delete),
              title: const Text('Delete'),
              onTap: () async {
                Navigator.pop(context);
                showDialog(
                  context: context,
                  builder: (context) =>
                      _DeleteConfirmationDialog(id: model.item.id),
                );
              },
            ),
          ],
        ),
      ),
    );
  }

  Widget? _buildVideoInfo(BuildContext context) {
    final videoInfo = model.item.videoFile;
    if (videoInfo == null) {
      return null;
    }

    final isDesktop = context.isDesktop;
    final bodySmall = Theme.of(context).textTheme.bodySmall;
    final bodyLarge = Theme.of(context).textTheme.bodyLarge;

    return Padding(
      padding: const EdgeInsets.only(top: 32),
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 600),
        child: Table(
          defaultColumnWidth: const IntrinsicColumnWidth(),
          columnWidths: {
            1: const IntrinsicColumnWidth(),
            2: isDesktop
                ? const IntrinsicColumnWidth()
                : const FlexColumnWidth(),
          },
          children: [
            TableRow(
              children: [
                Text('Video', style: bodyLarge),
                const SizedBox(width: 16),
                StreamDropdownButton<VideoStreamInfo>(
                  items:
                      videoInfo.streams.whereType<VideoStreamInfo>().toList(),
                  itemBuilder: (item) => Text(item.codec),
                ),
              ],
            ),
            TableRow(
              children: [
                Text('Audio', style: bodyLarge),
                const SizedBox(width: 16),
                StreamDropdownButton<AudioStreamInfo>(
                  items:
                      videoInfo.streams.whereType<AudioStreamInfo>().toList(),
                  itemBuilder: (item) => Text(
                      "${tryResolveLanguageCode(item.language ?? "Unknown")} (${item.codec})"),
                ),
              ],
            ),
            if (videoInfo.subtitles.isNotEmpty)
              TableRow(
                children: [
                  Text('Subtitles', style: bodyLarge),
                  const SizedBox(width: 16),
                  StreamDropdownButton<SubtitleTrack>(
                    items: videoInfo.subtitles,
                    itemBuilder: (item) => Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        TextOneLine(
                            tryResolveLanguageCode(item.language ?? 'Unknown')),
                        if (item.title != null)
                          TextOneLine(
                            item.title!,
                            style: bodyLarge!.copyWith(color: bodySmall!.color),
                          ),
                      ],
                    ),
                    selectedItemBuilder: (context, item) => Text(
                      tryResolveLanguageCode(item.language ?? 'Unknown'),
                      style: bodyLarge,
                    ),
                  ),
                ],
              ),
            TableRow(
              children: [
                Text('Format', style: bodyLarge),
                const SizedBox(width: 16),
                Text(videoInfo.format),
              ],
            ),
            TableRow(
              children: [
                Text('Path', style: bodyLarge),
                const SizedBox(width: 16),
                Text(videoInfo.path),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

class FixEpisodeMatchDialog extends ConsumerStatefulWidget {
  final MediaItem item;

  const FixEpisodeMatchDialog({
    Key? key,
    required this.item,
  }) : super(key: key);

  @override
  ConsumerState<FixEpisodeMatchDialog> createState() =>
      _FixEpisodeMatchDialogState();
}

class _FixEpisodeMatchDialogState extends ConsumerState<FixEpisodeMatchDialog> {
  late TextEditingController _season;

  late TextEditingController _episode;

  @override
  void initState() {
    super.initState();
    _season =
        TextEditingController(text: widget.item.grandparent!.index.toString());
    _episode =
        TextEditingController(text: widget.item.parent!.index.toString());
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Fix metadata match'),
      content: Row(
        // mainAxisSize: MainAxisSize.min,
        // crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Flexible(
            child: TextFormField(
              keyboardType: TextInputType.number,
              decoration: const InputDecoration(labelText: 'Season'),
              controller: _season,
            ),
          ),
          const SizedBox(width: 16),
          Flexible(
            child: TextFormField(
              keyboardType: TextInputType.number,
              decoration: const InputDecoration(labelText: 'Episode'),
              controller: _episode,
            ),
          ),
        ],
      ),
      actions: [
        TextButton(
          child: const Text('Cancel'),
          onPressed: () => Navigator.pop(context),
        ),
        TextButton(
          child: const Text('Ok'),
          onPressed: () async {
            final season = int.parse(_season.text);
            final episode = int.parse(_episode.text);

            await ref.read(apiProvider).fixMetadataMatch(
                widget.item.id,
                FixMetadataMatch(
                  season: season,
                  episode: episode,
                ));

            Navigator.pop(context);
          },
        ),
      ],
    );
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
  final double progress;
  final String? caption;

  const Poster({
    Key? key,
    required this.url,
    required this.progress,
    required this.caption,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        ClipRRect(
          borderRadius: caption != null
              ? const BorderRadius.vertical(top: Radius.circular(16))
              : BorderRadius.circular(16),
          child: AspectRatio(
            aspectRatio: 2.0 / 3.0,
            child: FadeInImage.memoryNetwork(
              placeholder: transparentImage,
              image: url,
              fit: BoxFit.cover,
            ),
          ),
        ),
        if (progress > 0)
          LinearProgressIndicator(
            value: progress,
            backgroundColor: Colors.white,
          ),
        if (caption != null)
          Material(
            color: Theme.of(context).colorScheme.surfaceVariant,
            borderRadius:
                const BorderRadius.vertical(bottom: Radius.circular(16)),
            child: Center(
              child: Padding(
                padding: const EdgeInsets.all(8.0),
                child: Text(
                  caption!,
                  style: Theme.of(context).textTheme.bodySmall,
                ),
              ),
            ),
          ),
      ],
    );
  }
}

class Overview extends StatefulWidget {
  final String text;

  const Overview({
    Key? key,
    required this.text,
  }) : super(key: key);

  @override
  State<Overview> createState() => _OverviewState();
}

class _OverviewState extends State<Overview> {
  final _controller = ExpandableController(initialExpanded: false);

  @override
  void dispose() {
    super.dispose();
    _controller.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final style = theme.textTheme.bodyLarge!.copyWith(fontSize: 16);
    return LayoutBuilder(builder: (context, size) {
      final painter = TextPainter(
          text: TextSpan(text: widget.text, style: style),
          maxLines: 5,
          textDirection: Directionality.of(context));
      painter.layout(maxWidth: size.maxWidth);
      if (!painter.didExceedMaxLines) {
        return Text(widget.text, style: style);
      }
      return ExpandableNotifier(
        controller: _controller,
        child: Expandable(
          collapsed: Column(
            children: [
              Text(widget.text,
                  maxLines: 5, overflow: TextOverflow.ellipsis, style: style),
              Padding(
                padding: const EdgeInsets.all(8.0),
                child: Material(
                  child: InkWell(
                    onTap: _controller.toggle,
                    child: Text('More',
                        style: TextStyle(color: theme.colorScheme.primary)),
                  ),
                ),
              ),
            ],
          ),
          expanded: Column(
            children: [
              Text(widget.text, style: style),
              Padding(
                padding: const EdgeInsets.all(8.0),
                child: Material(
                  child: InkWell(
                    onTap: _controller.toggle,
                    child: Text('Less',
                        style: TextStyle(color: theme.colorScheme.primary)),
                  ),
                ),
              ),
            ],
          ),
        ),
      );
    });
  }
}

class StreamDropdownButton<T> extends StatelessWidget {
  final List<T> items;
  final Widget Function(T item) itemBuilder;
  final Widget Function(BuildContext context, T item)? selectedItemBuilder;

  const StreamDropdownButton({
    super.key,
    required this.items,
    required this.itemBuilder,
    this.selectedItemBuilder,
  });

  @override
  Widget build(BuildContext context) {
    final bodyLarge = Theme.of(context).textTheme.bodyLarge;
    return DropdownButton<T>(
      value: items[0],
      items: items
          .map((a) => DropdownMenuItem(
                value: a,
                child: itemBuilder(a),
              ))
          .toList(),
      onChanged: (value) {},
      underline: const SizedBox(),
      isDense: true,
      isExpanded: true,
      style: bodyLarge,
      selectedItemBuilder: selectedItemBuilder != null
          ? ((context) =>
              items.map((e) => selectedItemBuilder!(context, e)).toList())
          : null,
    );
  }
}

class _DeleteConfirmationDialog extends ConsumerStatefulWidget {
  final int id;

  const _DeleteConfirmationDialog({required this.id});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() =>
      _DeleteConfirmationDialogState();
}

class _DeleteConfirmationDialogState
    extends ConsumerState<_DeleteConfirmationDialog> {
  bool _isInProgress = false;

  @override
  Widget build(BuildContext context) {
    return WillPopScope(
      onWillPop: () async => !_isInProgress,
      child: AlertDialog(
        title: const Text('Delete item'),
        content: const Text(
            'Are you sure you want to permanently delete this item. Files will be removed.'),
        actions: [
          TextButton(
            child: const Text('Delete'),
            onPressed: _isInProgress
                ? null
                : () async {
                    setState(() => _isInProgress = true);
                    await ref.read(apiProvider).deleteMediaItem(widget.id);
                    setState(() => _isInProgress = false);
                    Navigator.pop(context);
                    context.router.pop();
                  },
          ),
          TextButton(
            child: const Text('Cancel'),
            onPressed: _isInProgress ? null : () => Navigator.pop(context),
          ),
        ],
      ),
    );
  }
}
