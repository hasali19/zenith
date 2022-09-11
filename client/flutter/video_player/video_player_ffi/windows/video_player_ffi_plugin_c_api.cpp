#include "include/video_player_ffi/video_player_ffi_plugin_c_api.h"

#include <flutter/plugin_registrar_windows.h>

#include "video_player_ffi_plugin.h"

void VideoPlayerFfiPluginCApiRegisterWithRegistrar(
    FlutterDesktopPluginRegistrarRef registrar) {
  video_player_ffi::VideoPlayerFfiPlugin::RegisterWithRegistrar(
      flutter::PluginRegistrarManager::GetInstance()
          ->GetRegistrar<flutter::PluginRegistrarWindows>(registrar));
}
