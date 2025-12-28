// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'login_user_controller.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(LoginUserController)
final loginUserControllerProvider = LoginUserControllerProvider._();

final class LoginUserControllerProvider
    extends $NotifierProvider<LoginUserController, LoginUserState> {
  LoginUserControllerProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'loginUserControllerProvider',
        isAutoDispose: true,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$loginUserControllerHash();

  @$internal
  @override
  LoginUserController create() => LoginUserController();

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(LoginUserState value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<LoginUserState>(value),
    );
  }
}

String _$loginUserControllerHash() =>
    r'9c02fe9ae0608a03cf69a220f53b97592a94e7ed';

abstract class _$LoginUserController extends $Notifier<LoginUserState> {
  LoginUserState build();
  @$mustCallSuper
  @override
  void runBuild() {
    final ref = this.ref as $Ref<LoginUserState, LoginUserState>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<LoginUserState, LoginUserState>,
              LoginUserState,
              Object?,
              Object?
            >;
    element.handleCreate(ref, build);
  }
}
