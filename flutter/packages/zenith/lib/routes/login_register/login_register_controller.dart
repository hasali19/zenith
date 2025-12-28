import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:zenith/api.dart';

import 'login_register_state.dart';

part 'login_register_controller.g.dart';

@riverpod
class LoginRegisterController extends _$LoginRegisterController {
  @override
  LoginRegisterState build() {
    return LoginRegisterInitial();
  }

  void register(String? code, String username, String password) async {
    final api = ref.read(apiProvider);
    try {
      await api.createUser(
        username,
        password,
        code != null && code.isNotEmpty ? code : null,
      );
      state = LoginRegisterSuccess();
    } catch (e) {
      state = LoginRegisterFailure();
    }
  }
}
