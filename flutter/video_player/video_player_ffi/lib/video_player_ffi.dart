import 'dart:ffi';
import 'dart:isolate';

import 'package:ffi/ffi.dart';
import 'package:flutter/services.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_native_view/flutter_native_view.dart';
import 'package:video_player_platform_interface/video_player_platform_interface.dart';

const _channel = MethodChannel('video_player_ffi');

final DynamicLibrary _lib = DynamicLibrary.open("video_player_ffi.dll");
final int Function(int nativePort, Pointer<Void> params) ffiCreatePlayer = _lib
    .lookup<NativeFunction<IntPtr Function(Int64, Pointer<Void>)>>(
        "create_player")
    .asFunction();
final int Function(int player) ffiGetWindowHandle = _lib
    .lookup<NativeFunction<IntPtr Function(IntPtr)>>("get_window_handle")
    .asFunction();
final void Function(int player, Pointer<Utf8> url, double startPosition)
    ffiLoad = _lib
        .lookup<NativeFunction<Void Function(IntPtr, Pointer<Utf8>, Double)>>(
            "load")
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
final void Function(int player) ffiDestroyPlayer = _lib
    .lookup<NativeFunction<Void Function(IntPtr)>>("destroy_player")
    .asFunction();

class VideoPlayerFfi extends VideoPlayerPlatform {
  static registerWith() {
    VideoPlayerPlatform.instance = VideoPlayerFfi();
  }

  bool _flutterNativeViewInitialized = false;
  bool _isFullScreen = false;

  @override
  Future<VideoController> createController() async {
    if (!_flutterNativeViewInitialized) {
      FlutterNativeView.ensureInitialized();
      _flutterNativeViewInitialized = true;
    }
    return VideoControllerWindows();
  }

  @override
  Widget buildView(VideoController controller) {
    if (controller is VideoControllerWindows) {
      return NativeView(controller: controller.c, width: 100, height: 100);
    } else {
      throw ArgumentError.value(controller, "controller");
    }
  }

  @override
  bool get isWindowed => true;

  Future<void> _setFullscreen(bool isFullscreen) async {
    await _channel
        .invokeMethod('setFullScreen', {'isFullScreen': isFullscreen});
    await FlutterNativeView.setFullScreen(isFullscreen);
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
    _isFullScreen = !_isFullScreen;
    await _setFullscreen(_isFullScreen);
  }
}

enum PlayerMsgKind {
  durationChanged,
  pausedChanged,
  idleChanged,
  videoEnded,
}

class VideoControllerWindows extends VideoController {
  late final NativeViewController c;
  late final int player;

  final port = ReceivePort();

  VideoControllerWindows() {
    port.listen((message) {
      List<dynamic> values = message;

      final double position = values[0];
      final kind = PlayerMsgKind.values[values[1] - 1];
      final dynamic data = values[2];

      _lastKnownPosition = position * 1000;
      _lastKnownPositionTs = DateTime.now().millisecondsSinceEpoch;

      switch (kind) {
        case PlayerMsgKind.durationChanged:
          _duration = data;
          break;
        case PlayerMsgKind.pausedChanged:
          _paused = data;
          break;
        case PlayerMsgKind.idleChanged:
          _playing = !data;
          break;
        case PlayerMsgKind.videoEnded:
          _state = VideoState.ended;
          break;
      }

      _notifyListeners();
    });

    player = ffiCreatePlayer(
        port.sendPort.nativePort, NativeApi.initializeApiDLData);

    c = NativeViewController(handle: ffiGetWindowHandle(player));
  }

  double _lastKnownPosition = 0;
  int _lastKnownPositionTs = DateTime.now().millisecondsSinceEpoch;
  bool _playing = false;

  @override
  // TODO: implement supportsAudioTrackSelection
  bool get supportsAudioTrackSelection => false;

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
    ffiDestroyPlayer(player);
    port.close();
  }

  @override
  double get duration => _duration;
  double _duration = 0;

  @override
  void load(String url, List<SubtitleTrack> subtitles, double startPosition) {
    final pUrl = url.toNativeUtf8();
    ffiLoad(player, pUrl, startPosition);
    calloc.free(pUrl);
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
    // TODO: implement setAudioTrack
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
