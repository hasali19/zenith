// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'item_details_controller.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(ItemDetailsController)
final itemDetailsControllerProvider = ItemDetailsControllerFamily._();

final class ItemDetailsControllerProvider extends $NotifierProvider<
    ItemDetailsController, AsyncValue<ItemDetailsState>> {
  ItemDetailsControllerProvider._(
      {required ItemDetailsControllerFamily super.from,
      required int super.argument})
      : super(
          retry: null,
          name: r'itemDetailsControllerProvider',
          isAutoDispose: true,
          dependencies: null,
          $allTransitiveDependencies: null,
        );

  @override
  String debugGetCreateSourceHash() => _$itemDetailsControllerHash();

  @override
  String toString() {
    return r'itemDetailsControllerProvider'
        ''
        '($argument)';
  }

  @$internal
  @override
  ItemDetailsController create() => ItemDetailsController();

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(AsyncValue<ItemDetailsState> value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<AsyncValue<ItemDetailsState>>(value),
    );
  }

  @override
  bool operator ==(Object other) {
    return other is ItemDetailsControllerProvider && other.argument == argument;
  }

  @override
  int get hashCode {
    return argument.hashCode;
  }
}

String _$itemDetailsControllerHash() =>
    r'dc4eae35eaf9774e83844ade09c1f22dce71664e';

final class ItemDetailsControllerFamily extends $Family
    with
        $ClassFamilyOverride<
            ItemDetailsController,
            AsyncValue<ItemDetailsState>,
            AsyncValue<ItemDetailsState>,
            AsyncValue<ItemDetailsState>,
            int> {
  ItemDetailsControllerFamily._()
      : super(
          retry: null,
          name: r'itemDetailsControllerProvider',
          dependencies: null,
          $allTransitiveDependencies: null,
          isAutoDispose: true,
        );

  ItemDetailsControllerProvider call(
    int id,
  ) =>
      ItemDetailsControllerProvider._(argument: id, from: this);

  @override
  String toString() => r'itemDetailsControllerProvider';
}

abstract class _$ItemDetailsController
    extends $Notifier<AsyncValue<ItemDetailsState>> {
  late final _$args = ref.$arg as int;
  int get id => _$args;

  AsyncValue<ItemDetailsState> build(
    int id,
  );
  @$mustCallSuper
  @override
  void runBuild() {
    final ref = this.ref
        as $Ref<AsyncValue<ItemDetailsState>, AsyncValue<ItemDetailsState>>;
    final element = ref.element as $ClassProviderElement<
        AnyNotifier<AsyncValue<ItemDetailsState>, AsyncValue<ItemDetailsState>>,
        AsyncValue<ItemDetailsState>,
        Object?,
        Object?>;
    element.handleCreate(
        ref,
        () => build(
              _$args,
            ));
  }
}
