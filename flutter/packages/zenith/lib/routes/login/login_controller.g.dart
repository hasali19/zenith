// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'login_controller.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(loginRedirectPath)
final loginRedirectPathProvider = LoginRedirectPathProvider._();

final class LoginRedirectPathProvider
    extends $FunctionalProvider<String?, String?, String?>
    with $Provider<String?> {
  LoginRedirectPathProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'loginRedirectPathProvider',
        isAutoDispose: true,
        dependencies: <ProviderOrFamily>[],
        $allTransitiveDependencies: <ProviderOrFamily>[],
      );

  @override
  String debugGetCreateSourceHash() => _$loginRedirectPathHash();

  @$internal
  @override
  $ProviderElement<String?> $createElement($ProviderPointer pointer) =>
      $ProviderElement(pointer);

  @override
  String? create(Ref ref) {
    return loginRedirectPath(ref);
  }

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(String? value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<String?>(value),
    );
  }
}

String _$loginRedirectPathHash() => r'6075a313a339b1e09a6608447f52a5138e7987e9';
