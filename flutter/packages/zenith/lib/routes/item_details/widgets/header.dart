import 'dart:typed_data';

import 'package:expandable/expandable.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:zenith/api.dart';
import 'package:zenith/constants.dart';
import 'package:zenith/downloader/downloader.dart';
import 'package:zenith/image.dart';
import 'package:zenith/language_codes.dart';
import 'package:zenith/platform/file_picker/file_picker.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/routes/item_details/item_details_controller.dart';
import 'package:zenith/routes/item_details/item_details_state.dart';
import 'package:zenith/routes/item_details/widgets/header_layout.dart';
import 'package:zenith/text_one_line.dart';
import 'package:zenith/theme.dart';

class HeaderContent extends ConsumerWidget {
  const HeaderContent({
    super.key,
    required this.state,
    required this.onPlayPressed,
    required this.onChildItemPressed,
    required this.onFindMetadataMatch,
    required this.onFixEpisodeMatch,
    required this.onRefreshMetadata,
    required this.onDelete,
  });

  final ItemDetailsState state;
  final void Function() onPlayPressed;
  final void Function(int id) onChildItemPressed;
  final void Function() onFindMetadataMatch;
  final void Function() onFixEpisodeMatch;
  final void Function() onRefreshMetadata;
  final void Function() onDelete;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final isDesktop = MediaQuery.of(context).isDesktop;

    final subtitle = _buildSubtitle(context);
    final metaTable = _buildMetaTable();
    final overview = _buildOverview();
    final videoInfo = _buildVideoInfo(context, ref);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        HeaderLayout(
          backdrop: switch (state.backdrop) {
            null => Container(height: 300, color: Colors.grey),
            final backdrop => ZenithApiImage(
              id: backdrop,
              requestWidth: mediaBackdropImageWidth,
              height: 300,
              alignment: Alignment.topCenter,
            ),
          },
          poster: Poster(
            imageId: state.poster,
            requestWidth: mediaPosterImageWidth,
            progress: state.playable?.progress,
            caption: state.playable?.caption,
          ),
          title: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              if (state.item.grandparent != null)
                InkWell(
                  child: Text(
                    state.item.grandparent!.name,
                    style: Theme.of(context).textTheme.titleMedium,
                  ),
                  onTap: () => onChildItemPressed(state.item.grandparent!.id),
                ),
              Text(state.item.name, style: context.zenithTheme.titleLarge),
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
          actions: Row(children: _buildActionsItems(context, ref)),
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
    final seasonEpisode = state.item.getSeasonEpisode();
    final date = state.item.startDate;
    final duration = state.durationText;
    final ageRating = state.item.ageRating;

    final items = [
      if (seasonEpisode != null) TextSpan(text: seasonEpisode),
      if (date != null) TextSpan(text: '${date.year}'),
      if (duration != null) TextSpan(text: duration),
      if (ageRating != null) TextSpan(text: ageRating),
    ];

    final separated = <TextSpan>[];
    for (var i = 0; i < items.length; i++) {
      if (i > 0) {
        separated.add(
          const TextSpan(
            children: [
              WidgetSpan(child: SizedBox(width: 8)),
              TextSpan(text: '•'),
              WidgetSpan(child: SizedBox(width: 8)),
            ],
          ),
        );
      }
      separated.add(items[i]);
    }

    return Padding(
      padding: const EdgeInsets.only(top: 16.0),
      child: Text.rich(TextSpan(style: style, children: separated)),
    );
  }

  Widget? _buildMetaTable() {
    final rows = [
      if (state.item.director != null)
        TableRow(
          children: [
            TableCell(
              child: Text('Director', style: TextStyle(color: Colors.grey)),
            ),
            TableCell(child: SizedBox()),
            TableCell(child: Text(state.item.director!)),
          ],
        ),
      if (state.item.genres.isNotEmpty)
        TableRow(
          children: [
            TableCell(
              child: Text('Genres', style: TextStyle(color: Colors.grey)),
            ),
            TableCell(child: SizedBox()),
            TableCell(child: Text(state.item.genres.join(', '))),
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
    final overview = state.item.overview;
    final trailer = state.item.trailer;
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
                    final trailer = state.item.trailer;
                    if (trailer != null) {
                      launchUrl(
                        Uri.parse(trailer),
                        mode: LaunchMode.externalApplication,
                      );
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
    final playable = state.playable;
    return ElevatedButton.icon(
      icon: const Icon(Icons.play_arrow),
      label: Text(playable?.shouldResume == true ? 'Resume' : 'Play'),
      onPressed: playable == null ? null : onPlayPressed,
    );
  }

  List<Widget> _buildActionsItems(BuildContext context, WidgetRef ref) {
    final isDesktop = context.isDesktop;
    final actions = <Widget>[];

    actions.add(
      WatchedToggleButton(
        isWatched: state.isWatched,
        onChange: (v) {
          ref
              .read(itemDetailsControllerProvider(state.item.id).notifier)
              .setIsWatched(v);
        },
      ),
    );

    final videoFile = state.item.videoFile;
    if (videoFile != null) {
      if (isDesktop) {
        actions.add(const SizedBox(width: 16));
      }

      actions.add(
        IconButton(
          icon: Icon(
            state.downloadedFile == null
                ? Icons.cloud_download_outlined
                : Icons.cloud_done_outlined,
          ),
          color: switch (state.downloadedFile) {
            null => null,
            final f when f.path == null => Colors.orange,
            _ => Colors.green,
          },
          onPressed: () async {
            final downloadedFile = state.downloadedFile;
            if (downloadedFile == null) {
              final videoFile = state.item.videoFile!;
              final name = videoFile.path.split('/').last;
              ref
                  .read(zenithDownloaderProvider)
                  .downloadFile(
                    context,
                    itemId: state.item.id,
                    videoFileId: videoFile.id,
                    fileName: name,
                  );
            } else if (downloadedFile.path == null) {
              final isCancelConfirmed = await showDialog<bool>(
                context: context,
                builder: (context) {
                  return AlertDialog(
                    title: Text('Cancel download?'),
                    actions: [
                      TextButton(
                        child: Text('No'),
                        onPressed: () => Navigator.pop(context, false),
                      ),
                      TextButton(
                        child: Text('Yes'),
                        onPressed: () => Navigator.pop(context, true),
                      ),
                    ],
                  );
                },
              );

              if (isCancelConfirmed == true) {
                ref
                    .read(zenithDownloaderProvider)
                    .cancelDownload(downloadedFile.id);
              }
            } else {
              final isRemoveConfirmed = await showDialog<bool>(
                context: context,
                builder: (context) {
                  return AlertDialog(
                    title: Text('Delete local file?'),
                    actions: [
                      TextButton(
                        child: Text('Cancel'),
                        onPressed: () => Navigator.pop(context, false),
                      ),
                      TextButton(
                        child: Text('Delete'),
                        onPressed: () => Navigator.pop(context, true),
                      ),
                    ],
                  );
                },
              );

              if (isRemoveConfirmed == true) {
                ref
                    .read(zenithDownloaderProvider)
                    .removeDownloadedFile(downloadedFile.id);
              }
            }
          },
        ),
      );

      if (isDesktop) {
        actions.add(const SizedBox(width: 16));
      }
    }

    actions.add(
      IconButton(
        icon: const Icon(Icons.more_vert),
        onPressed: () => _showOptionsMenu(context),
      ),
    );

    return actions;
  }

  void _showOptionsMenu(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    showModalBottomSheet(
      context: context,
      constraints: width > 600
          ? const BoxConstraints.expand(width: 600).copyWith(minHeight: 0)
          : null,
      builder: (context) => Consumer(
        builder: (context, ref, child) => SafeArea(
          child: Wrap(
            children: [
              if (state.item.type == MediaType.episode) ...[
                ListTile(
                  leading: const Icon(Icons.tv),
                  title: const Text('Go to show'),
                  onTap: () {
                    Navigator.pop(context);
                    onChildItemPressed(state.item.grandparent!.id);
                  },
                ),
              ],
              ListTile(
                leading: const Icon(Icons.search),
                title: const Text('Find match'),
                onTap: () async {
                  Navigator.pop(context);
                  onFindMetadataMatch();
                },
              ),
              if (state.item.type == MediaType.episode)
                ListTile(
                  leading: const Icon(Icons.edit),
                  title: const Text('Fix match'),
                  onTap: () async {
                    Navigator.pop(context);
                    onFixEpisodeMatch();
                  },
                ),
              ListTile(
                leading: const Icon(Icons.refresh),
                title: const Text('Refresh metadata'),
                onTap: () async {
                  Navigator.pop(context);
                  onRefreshMetadata();
                },
              ),
              ListTile(
                iconColor: Colors.red,
                textColor: Colors.red,
                leading: const Icon(Icons.delete),
                title: const Text('Delete'),
                onTap: () async {
                  Navigator.pop(context);
                  onDelete();
                },
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget? _buildVideoInfo(BuildContext context, WidgetRef ref) {
    final videoInfo = state.item.videoFile;
    if (videoInfo == null) {
      return null;
    }

    final isDesktop = context.isDesktop;
    final bodyLarge = Theme.of(context).textTheme.bodyLarge;

    final videoStreams = videoInfo.streams
        .whereType<VideoStreamInfo>()
        .toList();
    final audioStreams = videoInfo.streams
        .whereType<AudioStreamInfo>()
        .toList();

    return Padding(
      padding: const EdgeInsets.only(top: 32),
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 600),
        child: Table(
          defaultColumnWidth: const IntrinsicColumnWidth(),
          defaultVerticalAlignment: TableCellVerticalAlignment.baseline,
          textBaseline: TextBaseline.alphabetic,
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
                _MenuButton<VideoStreamInfo>(
                  items: videoStreams,
                  initialValue: videoStreams.first,
                  itemBuilder: (item) =>
                      _MenuItemEntry(title: Text(item.codec)),
                  selectedItemBuilder: (context, item) => Text(item.codec),
                ),
              ],
            ),
            TableRow(
              children: [
                Text('Audio', style: bodyLarge),
                const SizedBox(width: 16),
                _MenuButton<AudioStreamInfo>(
                  items: audioStreams,
                  initialValue: audioStreams.first,
                  itemBuilder: (item) => _MenuItemEntry(
                    title: Text(
                      "${tryResolveLanguageCode(item.language ?? "Unknown")} (${item.codec})",
                    ),
                  ),
                  selectedItemBuilder: (context, item) => Text(
                    "${tryResolveLanguageCode(item.language ?? "Unknown")} (${item.codec})",
                  ),
                ),
              ],
            ),
            if (videoInfo.subtitles.isNotEmpty)
              TableRow(
                children: [
                  Text('Subtitles', style: bodyLarge),
                  const SizedBox(width: 16),
                  _SubtitlesMenuButton(
                    items: [null, ...videoInfo.subtitles],
                    initialValue: null,
                    itemBuilder: (item) {
                      if (item == null) {
                        return _MenuItemEntry(title: Text('None'));
                      } else {
                        final title = item.title;
                        return _MenuItemEntry(
                          title: Text(
                            tryResolveLanguageCode(item.language ?? 'Unknown'),
                          ),
                          subtitle: title == null
                              ? null
                              : TextOneLine(
                                  title,
                                  style: TextTheme.of(context).bodySmall,
                                ),
                        );
                      }
                    },
                    selectedItemBuilder: (context, item) {
                      if (item == null) {
                        return Text('None');
                      } else {
                        return Text(
                          tryResolveLanguageCode(item.language ?? 'Unknown'),
                        );
                      }
                    },
                    onUploadFile: (fileName, bytes) {
                      ref
                          .read(
                            itemDetailsControllerProvider(
                              state.item.id,
                            ).notifier,
                          )
                          .uploadSubtitleFile(fileName, bytes);
                    },
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

class WatchedToggleButton extends StatefulWidget {
  final bool isWatched;
  final void Function(bool isWatched) onChange;

  const WatchedToggleButton({
    super.key,
    required this.isWatched,
    required this.onChange,
  });

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
  final ImageId? imageId;
  final int requestWidth;
  final double? progress;
  final String? caption;

  const Poster({
    super.key,
    required this.imageId,
    required this.requestWidth,
    required this.progress,
    required this.caption,
  });

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
            child: switch (imageId) {
              null => Material(child: Icon(Icons.tv, size: 48)),
              final imageId => ZenithApiImage(
                id: imageId,
                requestWidth: requestWidth,
              ),
            },
          ),
        ),
        if (progress != null)
          LinearProgressIndicator(
            value: progress,
            backgroundColor: Colors.white,
          ),
        if (caption != null)
          Material(
            color: Theme.of(context).colorScheme.surfaceContainerHighest,
            borderRadius: const BorderRadius.vertical(
              bottom: Radius.circular(16),
            ),
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

  const Overview({super.key, required this.text});

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
    return LayoutBuilder(
      builder: (context, size) {
        final painter = TextPainter(
          text: TextSpan(text: widget.text, style: style),
          maxLines: 5,
          textDirection: Directionality.of(context),
        );
        painter.layout(maxWidth: size.maxWidth);
        if (!painter.didExceedMaxLines) {
          return Text(widget.text, style: style);
        }
        return ExpandableNotifier(
          controller: _controller,
          child: Expandable(
            collapsed: Column(
              children: [
                Text(
                  widget.text,
                  maxLines: 5,
                  overflow: TextOverflow.ellipsis,
                  style: style,
                ),
                Padding(
                  padding: const EdgeInsets.all(8.0),
                  child: Material(
                    type: MaterialType.transparency,
                    child: InkWell(
                      onTap: _controller.toggle,
                      child: Text(
                        'More',
                        style: TextStyle(color: theme.colorScheme.primary),
                      ),
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
                    type: MaterialType.transparency,
                    child: InkWell(
                      onTap: _controller.toggle,
                      child: Text(
                        'Less',
                        style: TextStyle(color: theme.colorScheme.primary),
                      ),
                    ),
                  ),
                ),
              ],
            ),
          ),
        );
      },
    );
  }
}

final class _MenuItemEntry {
  final Widget title;
  final Widget? subtitle;

  const _MenuItemEntry({required this.title, this.subtitle});
}

class _MenuButton<T> extends StatelessWidget {
  final List<T> items;
  final T initialValue;
  final _MenuItemEntry Function(T item) itemBuilder;
  final Widget Function(BuildContext context, T item) selectedItemBuilder;

  const _MenuButton({
    super.key,
    required this.items,
    required this.initialValue,
    required this.itemBuilder,
    required this.selectedItemBuilder,
  });

  @override
  Widget build(BuildContext context) {
    Widget child = selectedItemBuilder.call(context, items.first);

    if (items.length > 1) {
      child = Material(
        type: MaterialType.transparency,
        child: InkWell(
          child: DefaultTextStyle(
            style: TextStyle(color: ColorScheme.of(context).primary),
            child: child,
          ),
          onTapUp: (details) {
            if (context.isDesktop) {
              _showPopupMenu(context);
            } else {
              _showModalSheet(context);
            }
          },
        ),
      );
    }

    return Row(children: [child]);
  }

  void _showPopupMenu(BuildContext context) {
    final RenderBox box = context.findRenderObject()! as RenderBox;
    final RenderBox overlay =
        Navigator.of(context).overlay!.context.findRenderObject()! as RenderBox;
    final offset = box.localToGlobal(
      Offset(0, box.size.height),
      ancestor: overlay,
    );
    showMenu(
      clipBehavior: Clip.antiAlias,
      position: RelativeRect.fromRect(
        Rect.fromPoints(offset, offset),
        Offset.zero & overlay.size,
      ),
      context: context,
      constraints: BoxConstraints.loose(Size(480, 600)),
      items: buildPopupEntries(context, [
        for (final _MenuItemEntry(:title, :subtitle) in items.map(itemBuilder))
          PopupMenuItem(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [title, if (subtitle != null) subtitle],
            ),
          ),
      ]),
    );
  }

  void _showModalSheet(BuildContext context) {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      clipBehavior: Clip.antiAlias,
      builder: (context) {
        return DraggableScrollableSheet(
          expand: false,
          builder: (context, scrollController) => ListView(
            controller: scrollController,
            children: buildBottomSheetItems(context, [
              for (final _MenuItemEntry(:title, :subtitle) in items.map(
                itemBuilder,
              ))
                ListTile(title: title, subtitle: subtitle),
            ]),
          ),
        );
      },
    );
  }

  @protected
  List<Widget> buildBottomSheetItems(BuildContext context, List<Widget> items) {
    return items;
  }

  @protected
  List<PopupMenuEntry> buildPopupEntries(
    BuildContext context,
    List<PopupMenuEntry> entries,
  ) {
    return entries;
  }
}

class _SubtitlesMenuButton extends _MenuButton {
  final void Function(String fileName, Uint8List bytes) onUploadFile;

  const _SubtitlesMenuButton({
    required super.items,
    required super.initialValue,
    required super.itemBuilder,
    required super.selectedItemBuilder,
    required this.onUploadFile,
  });

  @override
  List<Widget> buildBottomSheetItems(BuildContext context, List<Widget> items) {
    return [
      ...items,
      Divider(),
      ListTile(
        leading: const Icon(Icons.upload),
        title: const Text('Upload'),
        onTap: () async {
          await _onUploadTap();
          if (context.mounted) {
            Navigator.pop(context);
          }
        },
      ),
    ];
  }

  @override
  List<PopupMenuEntry> buildPopupEntries(
    BuildContext context,
    List<PopupMenuEntry> entries,
  ) {
    return [
      ...entries,
      PopupMenuDivider(),
      PopupMenuItem(onTap: _onUploadTap, child: Text('Upload')),
    ];
  }

  Future<void> _onUploadTap() async {
    final result = await showFilePicker();

    if (result == null) {
      return;
    }

    onUploadFile(result.name, result.bytes);
  }
}
