import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:flion/flion.dart';
import 'package:flutter/services.dart';
import 'package:flutter/widgets.dart';

import '../video_player_platform_interface.dart';
import 'ffi.dart';

const _channel = MethodChannel('video_player');

class VideoPlayerWindows extends VideoPlayerPlatform {
  static DynamicLibrary? _lib;

  static registerWith() {
    VideoPlayerPlatform.instance = VideoPlayerWindows();
    try {
      _lib = DynamicLibrary.open('video_player.dll');
    } catch (e) {
      print('Could not open video_player.dll');
    }
  }

  @override
  Future<VideoController> createController(
      {Map<String, String>? headers}) async {
    VideoPlayerSymbolLoader loader;
    if (_lib case DynamicLibrary lib) {
      loader = <T extends NativeType>(name) => lib.lookup<T>(name);
    } else {
      final procs = await _channel.invokeMapMethod('getProcs');
      loader =
          <T extends NativeType>(name) => Pointer.fromAddress(procs![name]);
    }

    final procs = VideoPlayerProcs(loader);

    final controller = VideoControllerWindows(procs);
    await controller.init(headers, _lib == null);
    return controller;
  }

  @override
  Widget buildView(VideoController controller) {
    if (controller is VideoControllerWindows) {
      final surface = controller.surface;
      if (surface is _TextureSurface) {
        return Texture(textureId: surface.textureId);
      } else if (surface is _FlionPlatformViewSurface) {
        return FlionPlatformView(controller: surface.controller);
      }
      throw Exception('Unknow surface type: $surface');
    } else {
      throw ArgumentError.value(controller, 'controller');
    }
  }
}

abstract class VideoSurface {
  int get textureId;

  Future<void> preDestroy();
  Future<void> postDestroy();
}

class _TextureSurface implements VideoSurface {
  final int surface;
  final VideoPlayerProcs procs;

  const _TextureSurface(this.surface, this.procs);

  @override
  int get textureId => procs.getTextureId(surface);

  @override
  Future<void> preDestroy() async {
    await _channel.invokeMethod('destroyVideoSurface', {'surface': surface});
  }

  @override
  Future<void> postDestroy() async {}
}

class _FlionPlatformViewSurface implements VideoSurface {
  final FlionPlatformViewController controller;

  const _FlionPlatformViewSurface(this.controller);

  @override
  int get textureId => 0;

  @override
  Future<void> preDestroy() async {}

  @override
  Future<void> postDestroy() async {
    controller.dispose();
  }
}

enum PlayerMsgKind {
  durationChanged,
  pausedChanged,
  idleChanged,
  videoEnded,
}

class VideoControllerWindows extends VideoController with ChangeNotifier {
  final VideoPlayerProcs procs;

  late final VideoSurface surface;
  late final int player;
  late final _SubtitleStyleOptions _subtitleStyle;

  final _positionHandler = MediaPositionHandler();

  List<VideoItem>? _playlist;
  List<AudioTrack> _audioTracks = [];
  List<SubtitleTrack> _subtitleTracks = [];
  String? _activeSubtitleTrack;

  VideoControllerWindows(this.procs) {
    _channel.setMethodCallHandler((call) async {
      bool skipNotify = false;

      final Map<dynamic, dynamic> args = call.arguments;
      final double position = args['position'];

      if (args.containsKey('duration')) {
        _duration = args['duration'];
      }

      if (args.containsKey('paused')) {
        _paused = args['paused'];
      }

      if (args.containsKey('idle')) {
        _playing = !args['idle'];
      }

      if (args.containsKey('speed')) {
        _playbackSpeed = args['speed'];
      }

      if (args.containsKey('playlist-pos')) {
        currentItemIndex = args['playlist-pos'];
        _state = VideoState.active;
      }

      if (args.containsKey('tracks')) {
        _audioTracks = [];
        _subtitleTracks = [];

        List<dynamic> tracks = args['tracks'];

        for (final (index, track) in tracks.indexed) {
          switch (track['type']) {
            case 2:
              _audioTracks.add(AudioTrack(
                index: index,
                language: track['lang'] ?? 'Unknown',
                codec: track['codec'] ?? 'Unknown',
              ));

            case 3:
              _subtitleTracks.add(SubtitleTrack(
                id: track['id'].toString(),
                label: track['title'],
                language: track['lang'],
              ));
          }
        }
      }

      if (args.containsKey('selected-sub-track')) {
        final int? trackId = args['selected-sub-track'];
        _activeSubtitleTrack = trackId?.toString();
      }

      if (args.containsKey('subtitle-style')) {
        final Map<dynamic, dynamic> style = args['subtitle-style'];
        skipNotify = true;
        _subtitleStyle._setSize(style['size']);
      }

      if (args['state'] == 'ended') {
        _state = VideoState.ended;
      }

      _positionHandler.update(
        positionMs: (position * 1000).toInt(),
        isPlaying: _playing,
        speed: _playbackSpeed,
      );

      if (!skipNotify) {
        notifyListeners();
      }
    });
  }

  bool _playing = false;
  double _playbackSpeed = 1.0;

  @override
  bool get supportsAudioTrackSelection => true;

  @override
  int currentItemIndex = 0;

  @override
  double get position => _positionHandler.positionMs / 1000;

  @override
  set position(value) {
    procs.seekTo(player, value);
  }

  @override
  void dispose() {
    super.dispose();
    Future.microtask(() async {
      await surface.preDestroy();
      await _channel.invokeMethod('destroyPlayer', {'player': player});
      await surface.postDestroy();
    });
  }

  @override
  double get duration => _duration;
  double _duration = 0;

  @override
  BoxFit get fit => BoxFit.contain;

  @override
  double get playbackSpeed => _playbackSpeed;

  Future<void> init(Map<String, String>? headers, bool isFlion) async {
    player = await _channel.invokeMethod('createPlayer');

    if (headers != null) {
      final pHeaders = calloc<Pointer<Utf8>>(headers.length);

      for (final (i, MapEntry(key: name, :value)) in headers.entries.indexed) {
        pHeaders[i] = '$name: $value'.toNativeUtf8();
      }

      procs.setHttpHeaders(player, pHeaders, headers.length);

      for (var i = 0; i < headers.length; i++) {
        calloc.free(pHeaders[i]);
      }

      calloc.free(pHeaders);
    }

    if (isFlion) {
      final viewController = FlionPlatformViewController();
      await viewController.init(type: 'video', args: player);
      surface = _FlionPlatformViewSurface(viewController);
    } else {
      final surfaceId =
          await _channel.invokeMethod('createVideoSurface', {'player': player});
      surface = _TextureSurface(surfaceId, procs);
    }

    _subtitleStyle =
        _SubtitleStyleOptions(procs: procs, player: player, size: 40);
  }

  @override
  void load(List<VideoItem> items, int startIndex, double startPosition) {
    using((alloc) {
      final pItems = alloc<FfiVideoItem>(items.length);

      for (final (i, item) in items.indexed) {
        final pItem = pItems[i];
        final url = switch (item.source) {
          NetworkSource(:final url) => url,
          LocalFileSource() => throw UnimplementedError(),
        };
        pItem.url = url.toNativeUtf8(allocator: alloc);
        pItem.title = item.metadata.title?.toNativeUtf8(allocator: alloc) ??
            Pointer.fromAddress(0);
        pItem.subtitle =
            item.metadata.subtitle?.toNativeUtf8(allocator: alloc) ??
                Pointer.fromAddress(0);
        pItem.externalSubtitlesCount = item.subtitles.length;
        pItem.externalSubtitles =
            alloc<FfiExternalSubtitle>(item.subtitles.length);

        for (final (j, sub) in item.subtitles.indexed) {
          pItem.externalSubtitles[j].url =
              sub.src.toNativeUtf8(allocator: alloc);
          pItem.externalSubtitles[j].title =
              sub.title?.toNativeUtf8(allocator: alloc) ??
                  Pointer.fromAddress(0);
          pItem.externalSubtitles[j].language =
              sub.language?.toNativeUtf8(allocator: alloc) ??
                  Pointer.fromAddress(0);
        }
      }

      procs.load(player, pItems, items.length, startIndex, startPosition);
    });

    _state = VideoState.active;
    _playlist = items;
    currentItemIndex = startIndex;
  }

  @override
  bool get loading => !paused && !_playing && state != VideoState.ended;

  @override
  void pause() {
    procs.pause(player);
    notifyListeners();
  }

  @override
  bool get paused => _paused;
  bool _paused = false;

  @override
  void play() {
    procs.play(player);
    notifyListeners();
  }

  @override
  void seekToNextItem() {
    procs.playlistNext(player);
  }

  @override
  void seekToPreviousItem() {
    procs.playlistPrev(player);
  }

  @override
  void setFit(BoxFit fit) {
    // TODO: implement setFit
  }

  @override
  void setAudioTrack(int index) {
    procs.setAudioTrack(player, index);
  }

  @override
  void setSubtitleTrack(String? trackId) {
    if (_playlist == null) return;
    if (trackId == null) {
      procs.setSubtitleTrack(player, -1);
    } else {
      procs.setSubtitleTrack(player, int.parse(trackId));
    }
  }

  @override
  void setPlaybackSpeed(double speed) {
    procs.setSpeed(player, speed);
  }

  @override
  VideoState get state => _state;
  VideoState _state = VideoState.idle;

  @override
  List<AudioTrack> get availableAudioTracks => _audioTracks;

  @override
  List<SubtitleTrack> get currentSubtitleTracks => _subtitleTracks;

  @override
  String? get activeSubtitleTrackId => _activeSubtitleTrack;

  @override
  bool get supportsEmbeddedSubtitles => true;

  @override
  SubtitleStyleOptions? get subtitleStyle => _subtitleStyle;
}

class _SubtitleStyleOptions extends SubtitleStyleOptions with ChangeNotifier {
  final VideoPlayerProcs _procs;
  final int _player;

  int _size;

  _SubtitleStyleOptions({
    required VideoPlayerProcs procs,
    required int player,
    required int size,
  })  : _procs = procs,
        _player = player,
        _size = size;

  @override
  int get size => _size;

  @override
  set size(int value) {
    _procs.setSubtitleFontSize(_player, value);
  }

  void _setSize(int size) {
    _size = size;
    notifyListeners();
  }
}
