// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'login_register_controller.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(LoginRegisterController)
final loginRegisterControllerProvider = LoginRegisterControllerProvider._();

final class LoginRegisterControllerProvider
    extends $NotifierProvider<LoginRegisterController, LoginRegisterState> {
  LoginRegisterControllerProvider._()
      : super(
          from: null,
          argument: null,
          retry: null,
          name: r'loginRegisterControllerProvider',
          isAutoDispose: true,
          dependencies: null,
          $allTransitiveDependencies: null,
        );

  @override
  String debugGetCreateSourceHash() => _$loginRegisterControllerHash();

  @$internal
  @override
  LoginRegisterController create() => LoginRegisterController();

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(LoginRegisterState value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<LoginRegisterState>(value),
    );
  }
}

String _$loginRegisterControllerHash() =>
    r'e2bca5efbf6c72b7f4c18eb064b938550be5a6b8';

abstract class _$LoginRegisterController extends $Notifier<LoginRegisterState> {
  LoginRegisterState build();
  @$mustCallSuper
  @override
  void runBuild() {
    final ref = this.ref as $Ref<LoginRegisterState, LoginRegisterState>;
    final element = ref.element as $ClassProviderElement<
        AnyNotifier<LoginRegisterState, LoginRegisterState>,
        LoginRegisterState,
        Object?,
        Object?>;
    element.handleCreate(ref, build);
  }
}
