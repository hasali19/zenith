// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'manage_users_page.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(_data)
final _dataProvider = _DataProvider._();

final class _DataProvider
    extends
        $FunctionalProvider<
          AsyncValue<
            ({List<UserRegistration> registrations, List<User> users})
          >,
          ({List<UserRegistration> registrations, List<User> users}),
          FutureOr<({List<UserRegistration> registrations, List<User> users})>
        >
    with
        $FutureModifier<
          ({List<UserRegistration> registrations, List<User> users})
        >,
        $FutureProvider<
          ({List<UserRegistration> registrations, List<User> users})
        > {
  _DataProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'_dataProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$_dataHash();

  @$internal
  @override
  $FutureProviderElement<
    ({List<UserRegistration> registrations, List<User> users})
  >
  $createElement($ProviderPointer pointer) => $FutureProviderElement(pointer);

  @override
  FutureOr<({List<UserRegistration> registrations, List<User> users})> create(
    Ref ref,
  ) {
    return _data(ref);
  }
}

String _$_dataHash() => r'a514927f6317727b0c2dc32e9a8d7badcfc0a6ec';
