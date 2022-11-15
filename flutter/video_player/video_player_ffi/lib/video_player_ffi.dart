import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:flutter/services.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_native_view/flutter_native_view.dart';
import 'package:video_player_platform_interface/video_player_platform_interface.dart';

// const _channel = MethodChannel("video_player_ffi");

final DynamicLibrary _lib = DynamicLibrary.open("video_player_ffi.dll");
final int Function() ffiCreateWindow = _lib
    .lookup<NativeFunction<IntPtr Function()>>("create_window")
    .asFunction();
final int Function(int hwnd) ffiCreatePlayer = _lib
    .lookup<NativeFunction<IntPtr Function(IntPtr)>>("create_player")
    .asFunction();
final int Function(int hwnd, Pointer<Utf8> url) ffiLoad = _lib
    .lookup<NativeFunction<IntPtr Function(IntPtr, Pointer<Utf8>)>>("load")
    .asFunction();
final int Function(int hwnd) ffiPause =
    _lib.lookup<NativeFunction<IntPtr Function(IntPtr)>>("pause").asFunction();
final int Function(int hwnd) ffiPlay =
    _lib.lookup<NativeFunction<IntPtr Function(IntPtr)>>("play").asFunction();

class VideoPlayerFfi extends VideoPlayerPlatform {
  static registerWith() {
    VideoPlayerPlatform.instance = VideoPlayerFfi();
  }

  @override
  Future<VideoController> createController() async {
    FlutterNativeView.ensureInitialized();
    final hwnd = ffiCreateWindow();
    final player = ffiCreatePlayer(hwnd);
    return VideoControllerWindows(hwnd, player);
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

  @override
  Future<void> enterFullscreen() {
    // TODO: implement enterFullscreen
    throw UnimplementedError();
  }

  @override
  Future<void> exitFullscreen() {
    // TODO: implement exitFullscreen
    throw UnimplementedError();
  }

  @override
  Future<void> toggleFullscreen() {
    // TODO: implement toggleFullscreen
    throw UnimplementedError();
  }
}

class VideoControllerWindows extends VideoController {
  final int hwnd;
  final int player;
  final NativeViewController c;

  VideoControllerWindows(this.hwnd, this.player)
      : c = NativeViewController(handle: hwnd);

  @override
  double position = 0;

  @override
  void dispose() {
    // _channel.invokeMethod("DestroyPlayer", player);
  }

  @override
  // TODO: implement duration
  double get duration => 0;

  @override
  void load(String url, List<SubtitleTrack> subtitles, double startPosition) {
    final pUrl = url.toNativeUtf8();
    ffiLoad(player, pUrl);
    calloc.free(pUrl);
  }

  @override
  // TODO: implement loading
  bool get loading => false;

  @override
  void pause() {
    ffiPause(player);
    paused = true;
    _notifyListeners();
  }

  @override
  bool paused = false;

  @override
  void play() {
    ffiPlay(player);
    paused = false;
    _notifyListeners();
  }

  @override
  void setFit(BoxFit fit) {
    // TODO: implement setFit
  }

  @override
  void setTextTrack(SubtitleTrack? track) {
    // TODO: implement setTextTrack
  }

  @override
  // TODO: implement state
  VideoState get state => VideoState.active;

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
