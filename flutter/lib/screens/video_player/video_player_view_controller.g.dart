// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'video_player_view_controller.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

String _$videoPlayerViewControllerHash() =>
    r'47441562cc092f4af6200a890e5d1aa9cd0cf871';

/// Copied from Dart SDK
class _SystemHash {
  _SystemHash._();

  static int combine(int hash, int value) {
    // ignore: parameter_assignments
    hash = 0x1fffffff & (hash + value);
    // ignore: parameter_assignments
    hash = 0x1fffffff & (hash + ((0x0007ffff & hash) << 10));
    return hash ^ (hash >> 6);
  }

  static int finish(int hash) {
    // ignore: parameter_assignments
    hash = 0x1fffffff & (hash + ((0x03ffffff & hash) << 3));
    // ignore: parameter_assignments
    hash = hash ^ (hash >> 11);
    return 0x1fffffff & (hash + ((0x00003fff & hash) << 15));
  }
}

abstract class _$VideoPlayerViewController
    extends BuildlessAutoDisposeAsyncNotifier<VideoPlayerState> {
  late final int id;

  FutureOr<VideoPlayerState> build(
    int id,
  );
}

/// See also [VideoPlayerViewController].
@ProviderFor(VideoPlayerViewController)
const videoPlayerViewControllerProvider = VideoPlayerViewControllerFamily();

/// See also [VideoPlayerViewController].
class VideoPlayerViewControllerFamily
    extends Family<AsyncValue<VideoPlayerState>> {
  /// See also [VideoPlayerViewController].
  const VideoPlayerViewControllerFamily();

  /// See also [VideoPlayerViewController].
  VideoPlayerViewControllerProvider call(
    int id,
  ) {
    return VideoPlayerViewControllerProvider(
      id,
    );
  }

  @override
  VideoPlayerViewControllerProvider getProviderOverride(
    covariant VideoPlayerViewControllerProvider provider,
  ) {
    return call(
      provider.id,
    );
  }

  static const Iterable<ProviderOrFamily>? _dependencies = null;

  @override
  Iterable<ProviderOrFamily>? get dependencies => _dependencies;

  static const Iterable<ProviderOrFamily>? _allTransitiveDependencies = null;

  @override
  Iterable<ProviderOrFamily>? get allTransitiveDependencies =>
      _allTransitiveDependencies;

  @override
  String? get name => r'videoPlayerViewControllerProvider';
}

/// See also [VideoPlayerViewController].
class VideoPlayerViewControllerProvider
    extends AutoDisposeAsyncNotifierProviderImpl<VideoPlayerViewController,
        VideoPlayerState> {
  /// See also [VideoPlayerViewController].
  VideoPlayerViewControllerProvider(
    int id,
  ) : this._internal(
          () => VideoPlayerViewController()..id = id,
          from: videoPlayerViewControllerProvider,
          name: r'videoPlayerViewControllerProvider',
          debugGetCreateSourceHash:
              const bool.fromEnvironment('dart.vm.product')
                  ? null
                  : _$videoPlayerViewControllerHash,
          dependencies: VideoPlayerViewControllerFamily._dependencies,
          allTransitiveDependencies:
              VideoPlayerViewControllerFamily._allTransitiveDependencies,
          id: id,
        );

  VideoPlayerViewControllerProvider._internal(
    super._createNotifier, {
    required super.name,
    required super.dependencies,
    required super.allTransitiveDependencies,
    required super.debugGetCreateSourceHash,
    required super.from,
    required this.id,
  }) : super.internal();

  final int id;

  @override
  FutureOr<VideoPlayerState> runNotifierBuild(
    covariant VideoPlayerViewController notifier,
  ) {
    return notifier.build(
      id,
    );
  }

  @override
  Override overrideWith(VideoPlayerViewController Function() create) {
    return ProviderOverride(
      origin: this,
      override: VideoPlayerViewControllerProvider._internal(
        () => create()..id = id,
        from: from,
        name: null,
        dependencies: null,
        allTransitiveDependencies: null,
        debugGetCreateSourceHash: null,
        id: id,
      ),
    );
  }

  @override
  AutoDisposeAsyncNotifierProviderElement<VideoPlayerViewController,
      VideoPlayerState> createElement() {
    return _VideoPlayerViewControllerProviderElement(this);
  }

  @override
  bool operator ==(Object other) {
    return other is VideoPlayerViewControllerProvider && other.id == id;
  }

  @override
  int get hashCode {
    var hash = _SystemHash.combine(0, runtimeType.hashCode);
    hash = _SystemHash.combine(hash, id.hashCode);

    return _SystemHash.finish(hash);
  }
}

mixin VideoPlayerViewControllerRef
    on AutoDisposeAsyncNotifierProviderRef<VideoPlayerState> {
  /// The parameter `id` of this provider.
  int get id;
}

class _VideoPlayerViewControllerProviderElement
    extends AutoDisposeAsyncNotifierProviderElement<VideoPlayerViewController,
        VideoPlayerState> with VideoPlayerViewControllerRef {
  _VideoPlayerViewControllerProviderElement(super.provider);

  @override
  int get id => (origin as VideoPlayerViewControllerProvider).id;
}
// ignore_for_file: type=lint
// ignore_for_file: subtype_of_sealed_class, invalid_use_of_internal_member, invalid_use_of_visible_for_testing_member
