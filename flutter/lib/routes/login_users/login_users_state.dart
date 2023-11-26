import 'package:zenith/api.dart';

sealed class LoginUsersState {}

final class LoginUsersInitial extends LoginUsersState {}

final class LoginUsersLoading extends LoginUsersState {}

final class LoginUsersSuccess extends LoginUsersState {
  final List<User> users;

  LoginUsersSuccess(this.users);
}
