import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:zenith/api.dart';

import 'login_users_state.dart';

class LoginUsersCubit extends Cubit<LoginUsersState> {
  final ZenithApiClient _api;

  LoginUsersCubit(this._api) : super(LoginUsersInitial());

  void refresh() async {
    emit(LoginUsersLoading());
    try {
      final users = await _api.fetchUsers();
      emit(LoginUsersSuccess(users));
    } catch (e) {
      emit(LoginUsersFailure());
    }
  }
}
