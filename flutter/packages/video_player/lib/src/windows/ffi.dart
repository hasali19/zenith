import 'dart:ffi';

import 'package:ffi/ffi.dart';

final class FfiVideoItem extends Struct {
  external Pointer<Utf8> url;
  external Pointer<Utf8> title;
  external Pointer<Utf8> subtitle;
  @IntPtr()
  external int externalSubtitlesCount;
  external Pointer<FfiExternalSubtitle> externalSubtitles;
}

final class FfiExternalSubtitle extends Struct {
  external Pointer<Utf8> url;
  external Pointer<Utf8> title;
  external Pointer<Utf8> language;
}

final class FfiEvent extends Struct {
  @Double()
  external double position;
}

typedef VideoPlayerSymbolLoader = Pointer<T> Function<T extends NativeType>(
    String name);

class VideoPlayerProcs {
  final int Function(int surface) getTextureId;

  final void Function(
          int player, Pointer<Pointer<Utf8>> headers, int headerCount)
      setHttpHeaders;

  final void Function(int player, Pointer<FfiVideoItem> items, int itemCount,
      int startIndex, double startPosition) load;

  final void Function(int player, int index) setAudioTrack;

  final void Function(int player, int id) setSubtitleTrack;

  final void Function(int player) pause;

  final void Function(int player) play;

  final void Function(int player) playlistNext;

  final void Function(int player) playlistPrev;

  final void Function(int player, double position) seekTo;

  final void Function(int player, double speed) setSpeed;

  final void Function(int player, int size) setSubtitleFontSize;

  VideoPlayerProcs(VideoPlayerSymbolLoader lookup)
      : getTextureId =
            lookup<NativeFunction<IntPtr Function(IntPtr)>>('get_texture_id')
                .asFunction(),
        setHttpHeaders = lookup<
                NativeFunction<
                    Void Function(IntPtr, Pointer<Pointer<Utf8>>,
                        UintPtr)>>('set_http_headers')
            .asFunction(),
        load = lookup<
                NativeFunction<
                    Void Function(IntPtr, Pointer<FfiVideoItem>, UintPtr,
                        Uint32, Double)>>('load')
            .asFunction(),
        setAudioTrack = lookup<NativeFunction<Void Function(IntPtr, Int32)>>(
                'set_audio_track')
            .asFunction(),
        setSubtitleTrack = lookup<NativeFunction<Void Function(IntPtr, Int64)>>(
                'set_subtitle_track')
            .asFunction(),
        pause =
            lookup<NativeFunction<Void Function(IntPtr)>>('pause').asFunction(),
        play =
            lookup<NativeFunction<Void Function(IntPtr)>>('play').asFunction(),
        playlistNext =
            lookup<NativeFunction<Void Function(IntPtr)>>('playlist_next')
                .asFunction(),
        playlistPrev =
            lookup<NativeFunction<Void Function(IntPtr)>>('playlist_prev')
                .asFunction(),
        seekTo =
            lookup<NativeFunction<Void Function(IntPtr, Double)>>('seek_to')
                .asFunction(),
        setSpeed =
            lookup<NativeFunction<Void Function(IntPtr, Double)>>('set_speed')
                .asFunction(),
        setSubtitleFontSize =
            lookup<NativeFunction<Void Function(IntPtr, Int64)>>(
                    'set_subtitle_font_size')
                .asFunction();
}
