import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:flutter/services.dart';
import 'package:flutter/widgets.dart';

import '../video_player_platform_interface.dart';

const _channel = MethodChannel('video_player');

final class FfiVideoItem extends Struct {
  external Pointer<Utf8> url;
  external Pointer<Utf8> title;
  external Pointer<Utf8> subtitle;
  @IntPtr()
  external int externalSubtitlesCount;
  external Pointer<FfiExternalSubtitle> externalSubtitles;
}

final class FfiExternalSubtitle extends Struct {
  external Pointer<Utf8> url;
  external Pointer<Utf8> title;
  external Pointer<Utf8> language;
}

final DynamicLibrary _lib = DynamicLibrary.open('video_player.dll');
final int Function(int surface) ffiGetTextureId = _lib
    .lookup<NativeFunction<IntPtr Function(IntPtr)>>('get_texture_id')
    .asFunction();
final void Function(int player, Pointer<Pointer<Utf8>> headers, int headerCount)
    ffiSetHttpHeaders = _lib
        .lookup<
            NativeFunction<
                Void Function(IntPtr, Pointer<Pointer<Utf8>>,
                    UintPtr)>>('set_http_headers')
        .asFunction();
final void Function(int player, Pointer<FfiVideoItem> items, int itemCount,
        int startIndex, double startPosition) ffiLoad =
    _lib
        .lookup<
            NativeFunction<
                Void Function(IntPtr, Pointer<FfiVideoItem>, UintPtr, Uint32,
                    Double)>>('load')
        .asFunction();
final void Function(int player, int index) ffiSetAudioTrack = _lib
    .lookup<NativeFunction<Void Function(IntPtr, Int32)>>('set_audio_track')
    .asFunction();
final void Function(int player, int id) ffiSetSubtitleTrack = _lib
    .lookup<NativeFunction<Void Function(IntPtr, Int64)>>('set_subtitle_track')
    .asFunction();
final void Function(int player) ffiPause =
    _lib.lookup<NativeFunction<Void Function(IntPtr)>>('pause').asFunction();
final void Function(int player) ffiPlay =
    _lib.lookup<NativeFunction<Void Function(IntPtr)>>('play').asFunction();
final void Function(int player) ffiPlaylistNext = _lib
    .lookup<NativeFunction<Void Function(IntPtr)>>('playlist_next')
    .asFunction();
final void Function(int player) ffiPlaylistPrev = _lib
    .lookup<NativeFunction<Void Function(IntPtr)>>('playlist_prev')
    .asFunction();
final void Function(int player, double position) ffiSeekTo = _lib
    .lookup<NativeFunction<Void Function(IntPtr, Double)>>('seek_to')
    .asFunction();
final void Function(int player, double speed) ffiSetSpeed = _lib
    .lookup<NativeFunction<Void Function(IntPtr, Double)>>('set_speed')
    .asFunction();
final void Function(int player, int size) ffiSetSubtitleFontSize = _lib
    .lookup<NativeFunction<Void Function(IntPtr, Int64)>>(
        'set_subtitle_font_size')
    .asFunction();

class VideoPlayerWindows extends VideoPlayerPlatform {
  static registerWith() {
    VideoPlayerPlatform.instance = VideoPlayerWindows();
  }

  @override
  Future<VideoController> createController(
      {Map<String, String>? headers}) async {
    final int player = await _channel.invokeMethod('createPlayer');

    if (headers != null) {
      final pHeaders = calloc<Pointer<Utf8>>(headers.length);

      for (final (i, MapEntry(key: name, :value)) in headers.entries.indexed) {
        pHeaders[i] = '$name: $value'.toNativeUtf8();
      }

      ffiSetHttpHeaders(player, pHeaders, headers.length);

      for (var i = 0; i < headers.length; i++) {
        calloc.free(pHeaders[i]);
      }

      calloc.free(pHeaders);
    }

    final int surface =
        await _channel.invokeMethod('createVideoSurface', {'player': player});

    return VideoControllerWindows(player, surface);
  }

  @override
  Widget buildView(VideoController controller) {
    if (controller is VideoControllerWindows) {
      return Texture(textureId: controller.textureId);
    } else {
      throw ArgumentError.value(controller, 'controller');
    }
  }
}

enum PlayerMsgKind {
  durationChanged,
  pausedChanged,
  idleChanged,
  videoEnded,
}

class VideoControllerWindows extends VideoController with ChangeNotifier {
  final int player;
  final int surface;

  late final int textureId;

  final _positionHandler = MediaPositionHandler();

  List<VideoItem>? _playlist;
  List<AudioTrack> _audioTracks = [];
  List<SubtitleTrack> _subtitleTracks = [];
  String? _activeSubtitleTrack;
  _SubtitleStyleOptions _subtitleStyle;

  VideoControllerWindows(this.player, this.surface)
      : _subtitleStyle = _SubtitleStyleOptions(player: player, size: 40) {
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
                language: track['lang'],
                codec: track['codec'],
              ));
              break;

            case 3:
              _subtitleTracks.add(SubtitleTrack(
                id: track['id'].toString(),
                label: track['title'],
                language: track['lang'],
              ));
              break;
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

    textureId = ffiGetTextureId(surface);
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
    ffiSeekTo(player, value);
  }

  @override
  void dispose() {
    super.dispose();
    Future.microtask(() async {
      await _channel.invokeMethod('destroyVideoSurface', {'surface': surface});
      await _channel.invokeMethod('destroyPlayer', {'player': player});
    });
    _channel.setMethodCallHandler(null);
  }

  @override
  double get duration => _duration;
  double _duration = 0;

  @override
  BoxFit get fit => BoxFit.contain;

  @override
  double get playbackSpeed => _playbackSpeed;

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

      ffiLoad(player, pItems, items.length, startIndex, startPosition);
    });

    _state = VideoState.active;
    _playlist = items;
    currentItemIndex = startIndex;
  }

  @override
  bool get loading => !paused && !_playing && state != VideoState.ended;

  @override
  void pause() {
    ffiPause(player);
    notifyListeners();
  }

  @override
  bool get paused => _paused;
  bool _paused = false;

  @override
  void play() {
    ffiPlay(player);
    notifyListeners();
  }

  @override
  void seekToNextItem() {
    ffiPlaylistNext(player);
  }

  @override
  void seekToPreviousItem() {
    ffiPlaylistPrev(player);
  }

  @override
  void setFit(BoxFit fit) {
    // TODO: implement setFit
  }

  @override
  void setAudioTrack(int index) {
    ffiSetAudioTrack(player, index);
  }

  @override
  void setSubtitleTrack(String? trackId) {
    if (_playlist == null) return;
    if (trackId == null) {
      ffiSetSubtitleTrack(player, -1);
    } else {
      ffiSetSubtitleTrack(player, int.parse(trackId));
    }
  }

  @override
  void setPlaybackSpeed(double speed) {
    ffiSetSpeed(player, speed);
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
  final int _player;

  int _size;

  _SubtitleStyleOptions({
    required int player,
    required int size,
  })  : _player = player,
        _size = size;

  @override
  int get size => _size;

  @override
  set size(int value) {
    ffiSetSubtitleFontSize(_player, value);
  }

  void _setSize(int size) {
    _size = size;
    notifyListeners();
  }
}
