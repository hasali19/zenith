import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:flutter/services.dart';
import 'package:flutter/widgets.dart';
import 'package:video_player_platform_interface/video_player_platform_interface.dart';

const _channel = MethodChannel('video_player_ffi');

final DynamicLibrary _lib = DynamicLibrary.open("video_player_ffi.dll");
final int Function(int surface) ffiGetTextureId = _lib
    .lookup<NativeFunction<IntPtr Function(IntPtr)>>("get_texture_id")
    .asFunction();
final void Function(int player, Pointer<Utf8> url, Pointer<Utf8> title,
        Pointer<Utf8> subtitle, double startPosition) ffiLoad =
    _lib
        .lookup<
            NativeFunction<
                Void Function(IntPtr, Pointer<Utf8>, Pointer<Utf8>,
                    Pointer<Utf8>, Double)>>("load")
        .asFunction();
final void Function(int player, int index) ffiSetAudioTrack = _lib
    .lookup<NativeFunction<Void Function(IntPtr, Int32)>>("set_audio_track")
    .asFunction();
final void Function(int player, Pointer<Utf8> url) ffiSetSubtitleFile = _lib
    .lookup<NativeFunction<Void Function(IntPtr, Pointer<Utf8>)>>(
        "set_subtitle_file")
    .asFunction();
final void Function(int player) ffiPause =
    _lib.lookup<NativeFunction<Void Function(IntPtr)>>("pause").asFunction();
final void Function(int player) ffiPlay =
    _lib.lookup<NativeFunction<Void Function(IntPtr)>>("play").asFunction();
final void Function(int player, double position) ffiSeekTo = _lib
    .lookup<NativeFunction<Void Function(IntPtr, Double)>>("seek_to")
    .asFunction();

class VideoPlayerFfi extends VideoPlayerPlatform {
  static registerWith() {
    VideoPlayerPlatform.instance = VideoPlayerFfi();
  }

  bool _isFullScreen = false;

  @override
  Future<VideoController> createController() async {
    final int player = await _channel.invokeMethod("createPlayer");
    final int surface =
        await _channel.invokeMethod("createVideoSurface", {"player": player});
    return VideoControllerWindows(player, surface);
  }

  @override
  Widget buildView(VideoController controller) {
    if (controller is VideoControllerWindows) {
      return Texture(textureId: controller.textureId);
    } else {
      throw ArgumentError.value(controller, "controller");
    }
  }

  @override
  bool get isWindowed => true;

  Future<void> _setFullscreen(bool isFullscreen) async {
    _isFullScreen = isFullscreen;
    await _channel
        .invokeMethod('setFullScreen', {'isFullScreen': isFullscreen});
  }

  @override
  Future<void> enterFullscreen() async {
    await _setFullscreen(true);
  }

  @override
  Future<void> exitFullscreen() async {
    await _setFullscreen(false);
  }

  @override
  Future<void> toggleFullscreen() async {
    await _setFullscreen(!_isFullScreen);
  }
}

enum PlayerMsgKind {
  durationChanged,
  pausedChanged,
  idleChanged,
  videoEnded,
}

class VideoControllerWindows extends VideoController {
  final int player;
  final int surface;

  late final int textureId;

  VideoControllerWindows(this.player, this.surface) {
    _channel.setMethodCallHandler((call) async {
      final Map<dynamic, dynamic> args = call.arguments;
      final double position = args['position'];

      _lastKnownPosition = position * 1000;
      _lastKnownPositionTs = DateTime.now().millisecondsSinceEpoch;

      if (args.containsKey("duration")) {
        _duration = args["duration"];
      }

      if (args.containsKey("paused")) {
        _paused = args["paused"];
      }

      if (args.containsKey("idle")) {
        _playing = !args["idle"];
      }

      if (args.containsKey("state")) {
        final String state = args["state"];
        if (state == "ended") {
          _state = VideoState.ended;
        }
      }

      _notifyListeners();
    });

    textureId = ffiGetTextureId(surface);
  }

  double _lastKnownPosition = 0;
  int _lastKnownPositionTs = DateTime.now().millisecondsSinceEpoch;
  bool _playing = false;

  @override
  bool get supportsAudioTrackSelection => true;

  @override
  double get position {
    var position = _lastKnownPosition;
    if (_playing) {
      position += DateTime.now().millisecondsSinceEpoch - _lastKnownPositionTs;
    }
    return position / 1000;
  }

  @override
  set position(value) {
    ffiSeekTo(player, value);
  }

  @override
  void dispose() {
    Future.microtask(() async {
      await _channel.invokeMethod("destroyVideoSurface", {"surface": surface});
      await _channel.invokeMethod("destroyPlayer", {"player": player});
    });
    _channel.setMethodCallHandler(null);
  }

  @override
  double get duration => _duration;
  double _duration = 0;

  @override
  void load(VideoItem item) {
    final pUrl = item.url.toNativeUtf8();
    final pTitle = item.title == null
        ? Pointer<Utf8>.fromAddress(0)
        : item.title!.toNativeUtf8();
    final pSubtitle = item.subtitle == null
        ? Pointer<Utf8>.fromAddress(0)
        : item.subtitle!.toNativeUtf8();
    ffiLoad(player, pUrl, pTitle, pSubtitle, item.startPosition);
    calloc.free(pUrl);
    calloc.free(pTitle);
    calloc.free(pSubtitle);
    _state = VideoState.active;
  }

  @override
  bool get loading => !paused && !_playing && state != VideoState.ended;

  @override
  void pause() {
    ffiPause(player);
    _notifyListeners();
  }

  @override
  bool get paused => _paused;
  bool _paused = false;

  @override
  void play() {
    ffiPlay(player);
    _notifyListeners();
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
  void setTextTrack(SubtitleTrack? track) {
    if (track != null) {
      final pUrl = track.src.toNativeUtf8();
      ffiSetSubtitleFile(player, pUrl);
      calloc.free(pUrl);
    } else {
      ffiSetSubtitleFile(player, Pointer.fromAddress(0));
    }
  }

  @override
  void setPlaybackSpeed(double speed) {
    // TODO: implement setPlaybackSped
  }

  @override
  VideoState get state => _state;
  VideoState _state = VideoState.idle;

  final List<void Function()> _listeners = [];

  @override
  void addListener(void Function() listener) {
    _listeners.add(listener);
  }

  @override
  void removeListener(void Function() listener) {
    _listeners.remove(listener);
  }

  void _notifyListeners() {
    for (final listener in _listeners) {
      listener();
    }
  }
}
