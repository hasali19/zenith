#include "video_player_ffi_plugin.h"

// This must be included before many other Windows headers.
#include <windows.h>

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>
#include <flutter/standard_method_codec.h>

#include <memory>
#include <sstream>

namespace video_player_ffi {

// static
void VideoPlayerFfiPlugin::RegisterWithRegistrar(
    flutter::PluginRegistrarWindows* registrar) {
  auto channel =
      std::make_unique<flutter::MethodChannel<flutter::EncodableValue>>(
          registrar->messenger(), "video_player_ffi",
          &flutter::StandardMethodCodec::GetInstance());

  auto plugin = std::make_unique<VideoPlayerFfiPlugin>(registrar);

  channel->SetMethodCallHandler(
      [plugin_pointer = plugin.get()](const auto& call, auto result) {
        plugin_pointer->HandleMethodCall(call, std::move(result));
      });

  registrar->AddPlugin(std::move(plugin));
}

VideoPlayerFfiPlugin::VideoPlayerFfiPlugin(
    flutter::PluginRegistrarWindows* registrar)
    : registrar_(registrar) {}

VideoPlayerFfiPlugin::~VideoPlayerFfiPlugin() {}

RECT GetMonitorSize(HWND window) {
  MONITORINFO monitor_info;
  monitor_info.cbSize = sizeof(monitor_info);
  GetMonitorInfo(MonitorFromWindow(window, MONITOR_DEFAULTTONEAREST),
                 &monitor_info);
  return std::move(monitor_info.rcMonitor);
}

void SetWindowRect(HWND window, RECT* rect) {
  ::SetWindowPos(window, NULL, rect->left, rect->top, rect->right - rect->left,
                 rect->bottom - rect->top,
                 SWP_NOZORDER | SWP_NOACTIVATE | SWP_FRAMECHANGED);
}

HWND VideoPlayerFfiPlugin::GetWindow() const {
  return GetAncestor(registrar_->GetView()->GetNativeWindow(), GA_ROOT);
}

void VideoPlayerFfiPlugin::HandleMethodCall(
    const flutter::MethodCall<flutter::EncodableValue>& method_call,
    std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result) {
  if (method_call.method_name() == "setFullScreen") {
    const flutter::EncodableMap& args =
        std::get<flutter::EncodableMap>(*method_call.arguments());
    bool is_full_screen =
        std::get<bool>(args.at(flutter::EncodableValue("isFullScreen")));

    auto window = GetWindow();

    if (!is_full_screen_) {
      saved_style_ = GetWindowLongPtr(window, GWL_STYLE);
      GetWindowRect(window, &saved_window_rect_);
    }

    is_full_screen_ = is_full_screen;

    if (is_full_screen_) {
      SetWindowLongPtr(window, GWL_STYLE,
                       saved_style_ & ~(WS_CAPTION | WS_THICKFRAME));
      RECT monitor_size = GetMonitorSize(window);
      SetWindowRect(window, &monitor_size);
    } else {
      SetWindowLongPtr(window, GWL_STYLE, saved_style_);
      SetWindowRect(window, &saved_window_rect_);
    }

    result->Success(flutter::EncodableValue(nullptr));
  } else {
    result->NotImplemented();
  }
}

}  // namespace video_player_ffi
