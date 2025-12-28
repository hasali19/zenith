// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'media_route_chooser_controller.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(MediaRouteChooserController)
final mediaRouteChooserControllerProvider =
    MediaRouteChooserControllerProvider._();

final class MediaRouteChooserControllerProvider extends $NotifierProvider<
    MediaRouteChooserController, MediaRouteChooserState> {
  MediaRouteChooserControllerProvider._()
      : super(
          from: null,
          argument: null,
          retry: null,
          name: r'mediaRouteChooserControllerProvider',
          isAutoDispose: true,
          dependencies: null,
          $allTransitiveDependencies: null,
        );

  @override
  String debugGetCreateSourceHash() => _$mediaRouteChooserControllerHash();

  @$internal
  @override
  MediaRouteChooserController create() => MediaRouteChooserController();

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(MediaRouteChooserState value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<MediaRouteChooserState>(value),
    );
  }
}

String _$mediaRouteChooserControllerHash() =>
    r'1d59cfc1f50ca7d8a91bc2cd10f995b040ccaee8';

abstract class _$MediaRouteChooserController
    extends $Notifier<MediaRouteChooserState> {
  MediaRouteChooserState build();
  @$mustCallSuper
  @override
  void runBuild() {
    final ref =
        this.ref as $Ref<MediaRouteChooserState, MediaRouteChooserState>;
    final element = ref.element as $ClassProviderElement<
        AnyNotifier<MediaRouteChooserState, MediaRouteChooserState>,
        MediaRouteChooserState,
        Object?,
        Object?>;
    element.handleCreate(ref, build);
  }
}
