// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'video_player_view_controller.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(VideoPlayerViewController)
final videoPlayerViewControllerProvider = VideoPlayerViewControllerFamily._();

final class VideoPlayerViewControllerProvider extends $AsyncNotifierProvider<
    VideoPlayerViewController, VideoPlayerState> {
  VideoPlayerViewControllerProvider._(
      {required VideoPlayerViewControllerFamily super.from,
      required int super.argument})
      : super(
          retry: null,
          name: r'videoPlayerViewControllerProvider',
          isAutoDispose: true,
          dependencies: null,
          $allTransitiveDependencies: null,
        );

  @override
  String debugGetCreateSourceHash() => _$videoPlayerViewControllerHash();

  @override
  String toString() {
    return r'videoPlayerViewControllerProvider'
        ''
        '($argument)';
  }

  @$internal
  @override
  VideoPlayerViewController create() => VideoPlayerViewController();

  @override
  bool operator ==(Object other) {
    return other is VideoPlayerViewControllerProvider &&
        other.argument == argument;
  }

  @override
  int get hashCode {
    return argument.hashCode;
  }
}

String _$videoPlayerViewControllerHash() =>
    r'47441562cc092f4af6200a890e5d1aa9cd0cf871';

final class VideoPlayerViewControllerFamily extends $Family
    with
        $ClassFamilyOverride<
            VideoPlayerViewController,
            AsyncValue<VideoPlayerState>,
            VideoPlayerState,
            FutureOr<VideoPlayerState>,
            int> {
  VideoPlayerViewControllerFamily._()
      : super(
          retry: null,
          name: r'videoPlayerViewControllerProvider',
          dependencies: null,
          $allTransitiveDependencies: null,
          isAutoDispose: true,
        );

  VideoPlayerViewControllerProvider call(
    int id,
  ) =>
      VideoPlayerViewControllerProvider._(argument: id, from: this);

  @override
  String toString() => r'videoPlayerViewControllerProvider';
}

abstract class _$VideoPlayerViewController
    extends $AsyncNotifier<VideoPlayerState> {
  late final _$args = ref.$arg as int;
  int get id => _$args;

  FutureOr<VideoPlayerState> build(
    int id,
  );
  @$mustCallSuper
  @override
  void runBuild() {
    final ref =
        this.ref as $Ref<AsyncValue<VideoPlayerState>, VideoPlayerState>;
    final element = ref.element as $ClassProviderElement<
        AnyNotifier<AsyncValue<VideoPlayerState>, VideoPlayerState>,
        AsyncValue<VideoPlayerState>,
        Object?,
        Object?>;
    element.handleCreate(
        ref,
        () => build(
              _$args,
            ));
  }
}
