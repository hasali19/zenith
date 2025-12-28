// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'media_route_button_controller.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(MediaRouteButtonController)
final mediaRouteButtonControllerProvider =
    MediaRouteButtonControllerProvider._();

final class MediaRouteButtonControllerProvider
    extends
        $NotifierProvider<MediaRouteButtonController, MediaRouteButtonState> {
  MediaRouteButtonControllerProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'mediaRouteButtonControllerProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$mediaRouteButtonControllerHash();

  @$internal
  @override
  MediaRouteButtonController create() => MediaRouteButtonController();

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(MediaRouteButtonState value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<MediaRouteButtonState>(value),
    );
  }
}

String _$mediaRouteButtonControllerHash() =>
    r'3520f0eab8cc91b5b23929412d0d12e4962947e2';

abstract class _$MediaRouteButtonController
    extends $Notifier<MediaRouteButtonState> {
  MediaRouteButtonState build();
  @$mustCallSuper
  @override
  void runBuild() {
    final ref = this.ref as $Ref<MediaRouteButtonState, MediaRouteButtonState>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<MediaRouteButtonState, MediaRouteButtonState>,
              MediaRouteButtonState,
              Object?,
              Object?
            >;
    element.handleCreate(ref, build);
  }
}
