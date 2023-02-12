#include <windows.h>

#include <flutter/plugin_registry.h>
#include <flutter_windows.h>

#include "flutter/generated_plugin_registrant.h"
#include "flutter_native_view/flutter_native_view_plugin.h"
#include "utils.h"

class FlutterPluginRegistry : public flutter::PluginRegistry {
 public:
  FlutterPluginRegistry(FlutterDesktopEngineRef engine) : engine_(engine) {}

  FlutterDesktopPluginRegistrarRef GetRegistrarForPlugin(
      const std::string& plugin_name) {
    return FlutterDesktopEngineGetPluginRegistrar(engine_, plugin_name.c_str());
  }

 private:
  FlutterDesktopEngineRef engine_ = nullptr;
};

extern "C" void rust_main();

extern "C" void register_plugins(FlutterDesktopEngineRef engine) {
  FlutterPluginRegistry registry(engine);
  RegisterPlugins(&registry);
}

int APIENTRY wWinMain(_In_ HINSTANCE instance,
                      _In_opt_ HINSTANCE prev,
                      _In_ wchar_t* command_line,
                      _In_ int show_command) {
  // Attach to console when present (e.g., 'flutter run') or create a
  // new console when running with a debugger.
  if (!::AttachConsole(ATTACH_PARENT_PROCESS) && ::IsDebuggerPresent()) {
    CreateAndAttachConsole();
  }

  // Initialize COM, so that it is available for use in the library and/or
  // plugins.
  ::CoInitializeEx(nullptr, COINIT_APARTMENTTHREADED);

  flutternativeview::NativeViewContainer::GetInstance()->Create();

  rust_main();

  ::CoUninitialize();
  return EXIT_SUCCESS;
}
