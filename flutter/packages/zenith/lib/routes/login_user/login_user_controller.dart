import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:zenith/api.dart';

import 'login_user_state.dart';

part 'login_user_controller.g.dart';

@riverpod
class LoginUserController extends _$LoginUserController {
  @override
  LoginUserState build() {
    return LoginUserInitial();
  }

  Future<void> login(String username, String password) async {
    final api = ref.read(apiProvider);
    final isLoggedIn = await api.login(username, password);

    if (isLoggedIn) {
      state = LoginUserSuccess();
    } else {
      state = LoginUserFailure();
    }
  }
}
