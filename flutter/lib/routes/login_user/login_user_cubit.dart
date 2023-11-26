import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:zenith/api.dart';

import 'login_user_state.dart';

class LoginUserCubit extends Cubit<LoginUserState> {
  final ZenithApiClient _api;

  LoginUserCubit(this._api) : super(LoginUserInitial());

  void login(String username, String password) async {
    final isLoggedIn = await _api.login(username, password);

    if (isLoggedIn) {
      emit(LoginUserSuccess());
    } else {
      emit(LoginUserFailure());
    }
  }
}
