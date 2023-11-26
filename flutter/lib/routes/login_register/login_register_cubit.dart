import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:zenith/api.dart';

import 'login_register_state.dart';

class LoginRegisterCubit extends Cubit<LoginRegisterState> {
  final ZenithApiClient _api;

  LoginRegisterCubit(this._api) : super(LoginRegisterInitial());

  void register(String? code, String username, String password) async {
    try {
      await _api.createUser(
          username, password, code != null && code.isNotEmpty ? code : null);
      emit(LoginRegisterSuccess());
    } catch (e) {
      emit(LoginRegisterFailure());
    }
  }
}
