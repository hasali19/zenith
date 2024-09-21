import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:flutter/services.dart';
import 'package:flutter/widgets.dart';
import 'package:video_player_platform_interface/video_player_platform_interface.dart';

const _channel = MethodChannel('video_player_ffi');

final class FfiVideoItem extends Struct {
  external Pointer<Utf8> url;
  external Pointer<Utf8> title;
  external Pointer<Utf8> subtitle;
}

final DynamicLibrary _lib = DynamicLibrary.open('video_player_ffi.dll');
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
final void Function(int player, Pointer<Utf8> url) ffiSetSubtitleFile = _lib
    .lookup<NativeFunction<Void Function(IntPtr, Pointer<Utf8>)>>(
        'set_subtitle_file')
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

class VideoPlayerFfi extends VideoPlayerPlatform {
  static registerWith() {
    VideoPlayerPlatform.instance = VideoPlayerFfi();
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
  List<SubtitleTrack> _subtitleTracks = [];

  VideoControllerWindows(this.player, this.surface) {
    _channel.setMethodCallHandler((call) async {
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
        _updateSubtitleTracks();
      }

      if (args['state'] == 'ended') {
        _state = VideoState.ended;
      }

      _positionHandler.update(
        positionMs: (position * 1000).toInt(),
        isPlaying: _playing,
        speed: _playbackSpeed,
      );

      notifyListeners();
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
        pItem.url = item.url.toNativeUtf8(allocator: alloc);
        pItem.title = item.title == null
            ? Pointer<Utf8>.fromAddress(0)
            : item.title!.toNativeUtf8(allocator: alloc);
        pItem.subtitle = item.subtitle == null
            ? Pointer<Utf8>.fromAddress(0)
            : item.subtitle!.toNativeUtf8(allocator: alloc);
      }

      ffiLoad(player, pItems, items.length, startIndex, startPosition);
    });

    _state = VideoState.active;
    _playlist = items;
    currentItemIndex = startIndex;
    _updateSubtitleTracks();
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
      ffiSetSubtitleFile(player, Pointer.fromAddress(0));
    } else {
      final externalTrack = _playlist![currentItemIndex]
          .subtitles
          .firstWhere((t) => trackId == t.id);
      final pUrl = externalTrack.src.toNativeUtf8();
      ffiSetSubtitleFile(player, pUrl);
      calloc.free(pUrl);
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
  List<SubtitleTrack> get currentTextTracks => _subtitleTracks;

  @override
  // TODO: Support embedded subtitles
  bool get supportsEmbeddedSubtitles => false;

  void _updateSubtitleTracks() {
    final playlist = _playlist;
    if (playlist == null) {
      _subtitleTracks = [];
    } else {
      _subtitleTracks = playlist[currentItemIndex]
          .subtitles
          .map((track) => SubtitleTrack(
                id: track.id,
                label: track.title,
                language: track.language,
              ))
          .toList();
    }
  }
}
