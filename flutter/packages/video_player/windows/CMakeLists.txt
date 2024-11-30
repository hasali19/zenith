# The Flutter tooling requires that developers have a version of Visual Studio
# installed that includes CMake 3.14 or later. You should not increase this
# version, as doing so will cause the plugin to fail to compile for some
# customers of the plugin.
cmake_minimum_required(VERSION 3.14)

find_package(Corrosion REQUIRED)

# Project-level configuration.
set(PROJECT_NAME "video_player")
project(${PROJECT_NAME} LANGUAGES CXX)

# This value is used when generating builds using this plugin, so it must
# not be changed
set(PLUGIN_NAME "video_player_plugin")

add_library(${PLUGIN_NAME} SHARED IMPORTED GLOBAL)
set_property(TARGET ${PLUGIN_NAME} PROPERTY IMPORTED_LOCATION ${CMAKE_BINARY_DIR}/video_player.dll)
set_property(TARGET ${PLUGIN_NAME} PROPERTY IMPORTED_IMPLIB ${CMAKE_BINARY_DIR}/video_player.dll.lib)
set_property(TARGET ${PLUGIN_NAME} PROPERTY INTERFACE_INCLUDE_DIRECTORIES ${CMAKE_CURRENT_SOURCE_DIR}/include)
add_dependencies(${PLUGIN_NAME} _cargo-build_video_player)

# Link rust lib to flutter engine.
corrosion_add_target_local_rustflags(video_player -Lnative=${FLUTTER_MANAGED_DIR}/ephemeral)
corrosion_add_target_local_rustflags(video_player -lflutter_windows.dll)

# Add explicit dependency on flutter to cargo build.
add_dependencies(_cargo-build_video_player flutter flutter_assemble)

# List of absolute paths to libraries that should be bundled with the plugin.
# This list could contain prebuilt libraries, or libraries created by an
# external build triggered from this build file.
set(video_player_bundled_libraries
  $<TARGET_FILE:video_player-shared>
  $ENV{MPV_DIR}/libmpv-2.dll
  $ENV{ANGLE_BIN_DIR}/libEGL.dll
  $ENV{ANGLE_BIN_DIR}/libGLESv2.dll
  PARENT_SCOPE
)
