// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'login_users_view.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(_users)
final _usersProvider = _UsersProvider._();

final class _UsersProvider
    extends
        $FunctionalProvider<
          AsyncValue<List<User>>,
          List<User>,
          FutureOr<List<User>>
        >
    with $FutureModifier<List<User>>, $FutureProvider<List<User>> {
  _UsersProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'_usersProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$_usersHash();

  @$internal
  @override
  $FutureProviderElement<List<User>> $createElement($ProviderPointer pointer) =>
      $FutureProviderElement(pointer);

  @override
  FutureOr<List<User>> create(Ref ref) {
    return _users(ref);
  }
}

String _$_usersHash() => r'f553767850a6403963fd7c857faf8bd0a456afb5';
