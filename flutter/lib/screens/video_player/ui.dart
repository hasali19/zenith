import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:video_player/video_player.dart';
import 'package:zenith/api.dart' as api;
import 'package:zenith/language_codes.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/theme.dart';

import 'bottom_controls.dart';
import 'video_progress_bar.dart';
import 'utils.dart';

class VideoPlayerUi extends ConsumerStatefulWidget {
  final VideoController controller;
  final api.MediaItem item;
  final Stream<VideoProgressData> progress;

  final void Function() onInteractionStart;
  final void Function() onInteractionEnd;

  const VideoPlayerUi({
    Key? key,
    required this.controller,
    required this.item,
    required this.progress,
    required this.onInteractionStart,
    required this.onInteractionEnd,
  }) : super(key: key);

  @override
  ConsumerState<VideoPlayerUi> createState() => _VideoPlayerUiState();
}

class _VideoPlayerUiState extends ConsumerState<VideoPlayerUi> {
  VideoController get _controller => widget.controller;
  api.ZenithApiClient get _api => ref.watch(api.apiProvider);

  @override
  void initState() {
    super.initState();
    _controller.addListener(_listener);
  }

  @override
  void dispose() {
    super.dispose();
    _controller.removeListener(_listener);
  }

  void _listener() {
    setState(() {});
  }

  Future<void> _showModalBottomSheet(
      Widget Function(BuildContext context) builder) {
    final width = MediaQuery.of(context).size.width;
    return showModalBottomSheet<void>(
      context: context,
      constraints: width > 600
          ? const BoxConstraints.expand(width: 600).copyWith(minHeight: 0)
          : null,
      builder: builder,
    );
  }

  void _showOptionsMenu(BuildContext context) async {
    widget.onInteractionStart();
    await _showModalBottomSheet(
      (context) => Wrap(
        children: [
          ListTile(
            leading: const Icon(Icons.aspect_ratio),
            title: const Text("Fit"),
            onTap: () async {
              await _showBoxFitMenu(context);
              Navigator.pop(context);
            },
          ),
          if (_controller.supportsAudioTrackSelection)
            ListTile(
              leading: const Icon(Icons.audiotrack),
              title: const Text("Audio"),
              onTap: () async {
                await _showAudioMenu(context);
                Navigator.pop(context);
              },
            ),
          ListTile(
            leading: const Icon(Icons.closed_caption),
            title: const Text("Subtitles"),
            onTap: () async {
              await _showSubtitlesMenu(context);
              Navigator.pop(context);
            },
          ),
        ],
      ),
    );
    widget.onInteractionEnd();
  }

  Future<void> _showAudioMenu(BuildContext context) {
    return _showModalBottomSheet(
      (context) => ListView(
        children: _buildAudioMenuItems(context),
      ),
    );
  }

  List<Widget> _buildAudioMenuItems(BuildContext context) {
    final items = <Widget>[];
    final streams = widget.item.videoFile?.streams ?? [];

    for (final stream in streams) {
      if (stream is api.AudioStreamInfo) {
        items.add(ListTile(
          title: Text(stream.language),
          subtitle: Text(stream.codec),
          onTap: () {
            _setAudioTrack(stream.index);
            Navigator.pop(context);
          },
        ));
      }
    }

    return items;
  }

  Future<void> _showSubtitlesMenu(BuildContext context) {
    return _showModalBottomSheet(
      (context) => ListView(
        children: _buildSubtitlesMenuItems(context),
      ),
    );
  }

  List<Widget> _buildSubtitlesMenuItems(BuildContext context) {
    final items = [
      ListTile(
        title: const Text("None"),
        onTap: () {
          _setSubtitleTrack(null);
          Navigator.pop(context);
        },
      )
    ];

    final subtitles = widget.item.videoFile?.subtitles ?? [];
    for (final track in subtitles) {
      var language = track.language;
      if (language != null) {
        language = tryResolveLanguageCode(language);
      }
      items.add(ListTile(
        title: Text(language ?? "Unknown"),
        subtitle: track.title != null ? Text(track.title!) : null,
        onTap: () {
          _setSubtitleTrack(track);
          Navigator.pop(context);
        },
      ));
    }

    return items;
  }

  Future<void> _showBoxFitMenu(BuildContext context) {
    return _showModalBottomSheet(
      (context) => Wrap(
        children: [
          ListTile(
            leading: const Icon(Icons.crop_free),
            title: const Text("Cover"),
            onTap: () => _controller.setFit(BoxFit.cover),
          ),
          ListTile(
            leading: const Icon(Icons.fit_screen),
            title: const Text("Contain"),
            onTap: () => _controller.setFit(BoxFit.contain),
          ),
        ],
      ),
    );
  }

  void _setAudioTrack(int index) {
    _controller.setAudioTrack(index);
  }

  void _setSubtitleTrack(api.SubtitleTrack? track) {
    _controller
        .setTextTrack(track != null ? subtitleFromApi(_api, track) : null);
  }

  Widget _buildAppBar() {
    final Widget title;
    if (widget.item.type == api.MediaType.episode) {
      title = Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            widget.item.getSeasonEpisode()! + ": " + widget.item.name,
            style: context.zenithTheme.titleMedium,
          ),
          Text(
            widget.item.grandparent!.name,
            style: context.zenithTheme.bodyMedium,
          ),
        ],
      );
    } else {
      title = Text(widget.item.name);
    }
    return AppBar(
      title: title,
      backgroundColor: Colors.transparent,
      elevation: 0,
      actions: [
        IconButton(
          icon: const Icon(Icons.more_vert),
          splashRadius: 20,
          onPressed: () => _showOptionsMenu(context),
        ),
      ],
    );
  }

  Widget _buildBottomUi() {
    final desktop = MediaQuery.of(context).isDesktop;
    final playPauseIconSize = desktop ? 64.0 : 48.0;
    final seekIconSize = desktop ? 32.0 : 32.0;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        VideoProgressBar(
          stream: widget.progress,
          onSeek: (position) =>
              _controller.position = position.inSeconds.toDouble(),
          onSeekStart: widget.onInteractionStart,
          onSeekEnd: widget.onInteractionEnd,
        ),
        const SizedBox(height: 8),
        BottomControls(
          seekIconSize: seekIconSize,
          controller: _controller,
          playPauseIconSize: playPauseIconSize,
          onButtonTap: widget.onInteractionEnd,
        ),
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    final desktop = MediaQuery.of(context).isDesktop;
    final appBarPadding = desktop ? 32.0 : 0.0;
    final bottomControlsPadding = desktop
        ? const EdgeInsets.symmetric(horizontal: 300, vertical: 48)
        : const EdgeInsets.symmetric(horizontal: 16, vertical: 8);

    return DecoratedBox(
      decoration: BoxDecoration(
        gradient: LinearGradient(
            begin: const FractionalOffset(0, 0),
            end: const FractionalOffset(0, 1),
            colors: [
              Colors.black.withOpacity(0.7),
              Colors.transparent,
              Colors.black.withOpacity(0.7),
            ]),
      ),
      child: Stack(
        children: [
          Positioned(
            top: 0,
            left: 0,
            right: 0,
            child: Padding(
              padding: EdgeInsets.all(appBarPadding),
              child: _buildAppBar(),
            ),
          ),
          if (_controller.loading)
            const Align(
              alignment: Alignment.center,
              child: CircularProgressIndicator(color: Colors.white),
            ),
          Positioned(
            bottom: 0,
            left: 0,
            right: 0,
            child: SafeArea(
              child: Padding(
                padding: bottomControlsPadding,
                child: _buildBottomUi(),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
