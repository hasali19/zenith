#ifndef FLUTTER_PLUGIN_VIDEO_PLAYER_FFI_PLUGIN_H_
#define FLUTTER_PLUGIN_VIDEO_PLAYER_FFI_PLUGIN_H_

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>

#include <memory>

namespace video_player_ffi {

class VideoPlayerFfiPlugin : public flutter::Plugin {
 public:
  static void RegisterWithRegistrar(flutter::PluginRegistrarWindows* registrar);

  VideoPlayerFfiPlugin(flutter::PluginRegistrarWindows* registrar);

  virtual ~VideoPlayerFfiPlugin();

  // Disallow copy and assign.
  VideoPlayerFfiPlugin(const VideoPlayerFfiPlugin&) = delete;
  VideoPlayerFfiPlugin& operator=(const VideoPlayerFfiPlugin&) = delete;

 private:
  flutter::PluginRegistrarWindows* registrar_ = nullptr;
  bool is_full_screen_ = false;
  LONG_PTR saved_style_ = 0;
  RECT saved_window_rect_ = {};

  HWND GetWindow() const;

  // Called when a method is called on this plugin's channel from Dart.
  void HandleMethodCall(
      const flutter::MethodCall<flutter::EncodableValue>& method_call,
      std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result);
};

}  // namespace video_player_ffi

#endif  // FLUTTER_PLUGIN_VIDEO_PLAYER_FFI_PLUGIN_H_
