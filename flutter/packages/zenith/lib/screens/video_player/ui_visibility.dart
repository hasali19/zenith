import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:video_player/video_player.dart';

class UiVisibilityController extends ChangeNotifier {
  VideoController? _controller;
  Timer? _timer;

  bool _isAutoHideEnabled = false;
  bool _isUiToggled = false;
  bool _isActiveUiInteraction = false;
  bool _isMediaPaused = false;
  bool _isMediaEnded = false;

  bool get isVisible =>
      _isMediaPaused ||
      _isMediaEnded ||
      _isActiveUiInteraction ||
      (_isAutoHideEnabled && _isUiToggled);

  void setVideoController(VideoController controller) {
    _controller?.dispose();
    _controller = controller;
    controller.addListener(_onVideoStateChanged);
  }

  void setAutoHideEnabled(bool isEnabled) {
    _isAutoHideEnabled = isEnabled;
    _resetTimer();
    notifyListeners();
  }

  void startUiInteraction() {
    _isActiveUiInteraction = true;
    _resetTimer();
    notifyListeners();
  }

  void finishUiInteraction() {
    _isActiveUiInteraction = false;
    _isUiToggled = true;
    _resetTimer();
    notifyListeners();
  }

  void toggle() {
    _isUiToggled = !_isUiToggled;
    _resetTimer();
    notifyListeners();
  }

  @override
  void dispose() {
    _timer?.cancel();
    _controller?.removeListener(_onVideoStateChanged);
    super.dispose();
  }

  void _resetTimer() {
    _timer?.cancel();

    if (_isAutoHideEnabled) {
      _timer = Timer(const Duration(seconds: 5), () {
        _isUiToggled = false;
        notifyListeners();
      });
    }
  }

  void _onVideoStateChanged() {
    bool changed = false;

    if (_isMediaPaused != _controller?.paused) {
      _isMediaPaused = _controller?.paused ?? false;
      _isUiToggled = true;
      _resetTimer();
      changed = true;
    }

    final isEnded = _controller?.state == VideoState.ended;
    if (_isMediaEnded != isEnded) {
      _isMediaEnded = isEnded;
      changed = true;
    }

    if (changed) {
      notifyListeners();
    }
  }
}
