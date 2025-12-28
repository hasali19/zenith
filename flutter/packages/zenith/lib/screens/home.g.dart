// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'home.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(_state)
final _stateProvider = _StateProvider._();

final class _StateProvider
    extends
        $FunctionalProvider<
          AsyncValue<HomeScreenData>,
          HomeScreenData,
          FutureOr<HomeScreenData>
        >
    with $FutureModifier<HomeScreenData>, $FutureProvider<HomeScreenData> {
  _StateProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'_stateProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$_stateHash();

  @$internal
  @override
  $FutureProviderElement<HomeScreenData> $createElement(
    $ProviderPointer pointer,
  ) => $FutureProviderElement(pointer);

  @override
  FutureOr<HomeScreenData> create(Ref ref) {
    return _state(ref);
  }
}

String _$_stateHash() => r'61296860eadd3fd62f6d152ba071caaf79c538b1';
