// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'media_route_controller_controller.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(MediaRouteControllerController)
final mediaRouteControllerControllerProvider =
    MediaRouteControllerControllerProvider._();

final class MediaRouteControllerControllerProvider
    extends
        $NotifierProvider<
          MediaRouteControllerController,
          MediaRouteControllerState
        > {
  MediaRouteControllerControllerProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'mediaRouteControllerControllerProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$mediaRouteControllerControllerHash();

  @$internal
  @override
  MediaRouteControllerController create() => MediaRouteControllerController();

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(MediaRouteControllerState value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<MediaRouteControllerState>(value),
    );
  }
}

String _$mediaRouteControllerControllerHash() =>
    r'6f233b770fe62ff598fdfd813d885aa226a61ff7';

abstract class _$MediaRouteControllerController
    extends $Notifier<MediaRouteControllerState> {
  MediaRouteControllerState build();
  @$mustCallSuper
  @override
  void runBuild() {
    final ref =
        this.ref as $Ref<MediaRouteControllerState, MediaRouteControllerState>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<MediaRouteControllerState, MediaRouteControllerState>,
              MediaRouteControllerState,
              Object?,
              Object?
            >;
    element.handleCreate(ref, build);
  }
}
