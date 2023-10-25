import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:video_player/video_player.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/screens/video_player/play_pause_button.dart';

import 'bottom_controls.dart';
import 'video_progress_bar.dart';

class AudioTrack {
  final int index;
  final String language;
  final String codec;

  AudioTrack({
    required this.index,
    required this.language,
    required this.codec,
  });
}

class VideoPlayerUi extends ConsumerStatefulWidget {
  final VideoController controller;
  final Widget title;
  final List<AudioTrack> audioTracks;
  final List<SubtitleTrack> subtitles;
  final Stream<VideoProgressData> progress;

  final void Function() onInteractionStart;
  final void Function() onInteractionEnd;

  const VideoPlayerUi({
    Key? key,
    required this.controller,
    required this.title,
    required this.audioTracks,
    required this.subtitles,
    required this.progress,
    required this.onInteractionStart,
    required this.onInteractionEnd,
  }) : super(key: key);

  @override
  ConsumerState<VideoPlayerUi> createState() => _VideoPlayerUiState();
}

class _VideoPlayerUiState extends ConsumerState<VideoPlayerUi> {
  VideoController get _controller => widget.controller;

  late final List<SubtitleTrack> _subtitles;
  late final List<AudioTrack> _audioTracks;

  @override
  void initState() {
    super.initState();
    _subtitles = [...widget.subtitles];
    _subtitles.sort((a, b) =>
        (a.displayLanguage ?? '').compareTo((b.displayLanguage ?? '')));

    _audioTracks = [...widget.audioTracks];
    _audioTracks.sort((a, b) => a.language.compareTo(b.language));

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
      builder: (context) => SafeArea(child: builder(context)),
    );
  }

  void _showOptionsMenu(BuildContext context) async {
    widget.onInteractionStart();
    await _showModalBottomSheet(
      (context) => Wrap(
        children: [
          ListTile(
            leading: const Icon(Icons.aspect_ratio),
            title: const Text('Fit'),
            onTap: () {
              Navigator.pop(context);
              _showBoxFitMenu(context);
            },
          ),
          if (_controller.supportsAudioTrackSelection &&
              _audioTracks.length > 1)
            ListTile(
              leading: const Icon(Icons.audiotrack),
              title: const Text('Audio'),
              onTap: () {
                Navigator.pop(context);
                _showAudioMenu(context);
              },
            ),
          ListTile(
            leading: const Icon(Icons.speed),
            title: const Text('Playback speed'),
            onTap: () {
              Navigator.pop(context);
              _showPlaybackSpeedMenu(context);
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

    for (final track in _audioTracks) {
      items.add(ListTile(
        title: Text(track.language),
        subtitle: Text(track.codec),
        onTap: () {
          _controller.setAudioTrack(track.index);
          Navigator.pop(context);
        },
      ));
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
        title: const Text('None'),
        onTap: () {
          _controller.setTextTrack(null);
          Navigator.pop(context);
        },
      )
    ];

    for (final track in _subtitles) {
      items.add(ListTile(
        title: Text(track.displayLanguage ?? 'Unknown'),
        subtitle: track.title != null ? Text(track.title!) : null,
        onTap: () {
          _controller.setTextTrack(track);
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
            title: const Text('Cover'),
            onTap: () {
              _controller.setFit(BoxFit.cover);
              Navigator.pop(context);
            },
          ),
          ListTile(
            leading: const Icon(Icons.fit_screen),
            title: const Text('Contain'),
            onTap: () {
              _controller.setFit(BoxFit.contain);
              Navigator.pop(context);
            },
          ),
        ],
      ),
    );
  }

  Future<void> _showPlaybackSpeedMenu(BuildContext context) {
    const speeds = [1.0, 1.25, 1.5, 1.75, 2.0];
    return _showModalBottomSheet(
      (context) => Wrap(
        children: speeds
            .map((speed) => ListTile(
                  title: Text('${speed}x'),
                  onTap: () {
                    _controller.setPlaybackSpeed(speed);
                    Navigator.pop(context);
                  },
                ))
            .toList(),
      ),
    );
  }

  Widget _buildAppBar() {
    return AppBar(
      title: widget.title,
      backgroundColor: Colors.transparent,
      elevation: 0,
    );
  }

  Widget _buildBottomUi() {
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
          onShowCaptionsMenu: () {
            widget.onInteractionEnd();
            _showSubtitlesMenu(context);
          },
          onShowOptionsMenu: () {
            widget.onInteractionEnd();
            _showOptionsMenu(context);
          },
        ),
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    final desktop = MediaQuery.of(context).isDesktop;
    final appBarPadding = desktop ? 32.0 : 0.0;
    final bottomControlsPadding = desktop
        ? const EdgeInsets.symmetric(horizontal: 96, vertical: 48)
        : const EdgeInsets.symmetric(horizontal: 16, vertical: 8);
    final primaryIconSize = desktop ? 128.0 : 64.0;
    final secondaryIconSize = desktop ? 64.0 : 32.0;

    return DecoratedBox(
      decoration: BoxDecoration(
        gradient: LinearGradient(
          begin: const FractionalOffset(0, 0),
          end: const FractionalOffset(0, 1),
          colors: [
            Colors.black.withOpacity(0.7),
            Colors.transparent,
            Colors.black.withOpacity(0.7),
          ],
        ),
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
            Align(
              alignment: Alignment.center,
              child: SizedBox(
                width: primaryIconSize + 16,
                height: primaryIconSize + 16,
                child: const CircularProgressIndicator(color: Colors.white),
              ),
            ),
          Align(
            alignment: Alignment.center,
            child: Row(
              mainAxisAlignment: MainAxisAlignment.center,
              mainAxisSize: MainAxisSize.min,
              children: [
                IconButton(
                  icon: const Icon(Icons.skip_previous),
                  iconSize: secondaryIconSize,
                  onPressed: () {
                    _controller.seekToPreviousItem();
                    widget.onInteractionEnd();
                  },
                ),
                const SizedBox(width: 24),
                IconButton(
                  icon: const Icon(Icons.replay_10),
                  iconSize: secondaryIconSize,
                  onPressed: () {
                    _controller.position -= 10;
                    widget.onInteractionEnd();
                  },
                ),
                const SizedBox(width: 24),
                Container(
                  decoration: BoxDecoration(
                    shape: BoxShape.circle,
                    color: Colors.grey.withAlpha(50),
                  ),
                  child: PlayPauseButton(
                    isPlaying: !_controller.paused,
                    size: primaryIconSize,
                    onSetPlaying: (playing) {
                      playing ? _controller.play() : _controller.pause();
                      widget.onInteractionEnd();
                    },
                  ),
                ),
                const SizedBox(width: 24),
                IconButton(
                  icon: const Icon(Icons.forward_30),
                  iconSize: secondaryIconSize,
                  onPressed: () {
                    _controller.position += 30;
                    widget.onInteractionEnd();
                  },
                ),
                const SizedBox(width: 24),
                IconButton(
                  icon: const Icon(Icons.skip_next),
                  iconSize: secondaryIconSize,
                  onPressed: () {
                    _controller.seekToNextItem();
                    widget.onInteractionEnd();
                  },
                ),
              ],
            ),
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
