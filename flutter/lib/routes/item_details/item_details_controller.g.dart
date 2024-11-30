// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'item_details_controller.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

String _$itemDetailsControllerHash() =>
    r'cff91e6ddd315c5f0a1026d48c1f60af2c9918b4';

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

abstract class _$ItemDetailsController
    extends BuildlessAutoDisposeAsyncNotifier<ItemDetailsState> {
  late final int id;

  FutureOr<ItemDetailsState> build(
    int id,
  );
}

/// See also [ItemDetailsController].
@ProviderFor(ItemDetailsController)
const itemDetailsControllerProvider = ItemDetailsControllerFamily();

/// See also [ItemDetailsController].
class ItemDetailsControllerFamily extends Family<AsyncValue<ItemDetailsState>> {
  /// See also [ItemDetailsController].
  const ItemDetailsControllerFamily();

  /// See also [ItemDetailsController].
  ItemDetailsControllerProvider call(
    int id,
  ) {
    return ItemDetailsControllerProvider(
      id,
    );
  }

  @override
  ItemDetailsControllerProvider getProviderOverride(
    covariant ItemDetailsControllerProvider provider,
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
  String? get name => r'itemDetailsControllerProvider';
}

/// See also [ItemDetailsController].
class ItemDetailsControllerProvider
    extends AutoDisposeAsyncNotifierProviderImpl<ItemDetailsController,
        ItemDetailsState> {
  /// See also [ItemDetailsController].
  ItemDetailsControllerProvider(
    int id,
  ) : this._internal(
          () => ItemDetailsController()..id = id,
          from: itemDetailsControllerProvider,
          name: r'itemDetailsControllerProvider',
          debugGetCreateSourceHash:
              const bool.fromEnvironment('dart.vm.product')
                  ? null
                  : _$itemDetailsControllerHash,
          dependencies: ItemDetailsControllerFamily._dependencies,
          allTransitiveDependencies:
              ItemDetailsControllerFamily._allTransitiveDependencies,
          id: id,
        );

  ItemDetailsControllerProvider._internal(
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
  FutureOr<ItemDetailsState> runNotifierBuild(
    covariant ItemDetailsController notifier,
  ) {
    return notifier.build(
      id,
    );
  }

  @override
  Override overrideWith(ItemDetailsController Function() create) {
    return ProviderOverride(
      origin: this,
      override: ItemDetailsControllerProvider._internal(
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
  AutoDisposeAsyncNotifierProviderElement<ItemDetailsController,
      ItemDetailsState> createElement() {
    return _ItemDetailsControllerProviderElement(this);
  }

  @override
  bool operator ==(Object other) {
    return other is ItemDetailsControllerProvider && other.id == id;
  }

  @override
  int get hashCode {
    var hash = _SystemHash.combine(0, runtimeType.hashCode);
    hash = _SystemHash.combine(hash, id.hashCode);

    return _SystemHash.finish(hash);
  }
}

@Deprecated('Will be removed in 3.0. Use Ref instead')
// ignore: unused_element
mixin ItemDetailsControllerRef
    on AutoDisposeAsyncNotifierProviderRef<ItemDetailsState> {
  /// The parameter `id` of this provider.
  int get id;
}

class _ItemDetailsControllerProviderElement
    extends AutoDisposeAsyncNotifierProviderElement<ItemDetailsController,
        ItemDetailsState> with ItemDetailsControllerRef {
  _ItemDetailsControllerProviderElement(super.provider);

  @override
  int get id => (origin as ItemDetailsControllerProvider).id;
}
// ignore_for_file: type=lint
// ignore_for_file: subtype_of_sealed_class, invalid_use_of_internal_member, invalid_use_of_visible_for_testing_member, deprecated_member_use_from_same_package
