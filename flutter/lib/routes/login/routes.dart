import 'package:flutter/material.dart';
import 'package:zenith/main_router.dart';
import 'package:zenith/router/stack_router.dart';
import 'package:zenith/routes/login_register/login_register_page.dart';
import 'package:zenith/routes/login_user/login_user_page.dart';
import 'package:zenith/routes/login_users/login_users_page.dart';

sealed class LoginChildRoute extends ZenithRoute {
  const LoginChildRoute();
}

class LoginUsersRoute extends LoginChildRoute {
  const LoginUsersRoute();

  @override
  Widget build(BuildContext context) {
    return const LoginUsersPage();
  }
}

class LoginUserRoute extends LoginChildRoute {
  final String username;

  const LoginUserRoute({required this.username});

  @override
  Widget build(BuildContext context) {
    return LoginUserPage(
      username: username,
      onSuccess: () {
        // final redirectPath = ref.read(loginRedirectPathProvider);
        // final redirectRoute = context.router.root.buildPageRoute(redirectPath);
        // if (redirectRoute != null) {
        //   context.router.root.replace(redirectRoute);
        // } else {
        //   context.router.root.replace(const MainRoute());
        // }
        StackRouter.of<PrimaryRoute>(context).replace(const MainRoute());
      },
    );
  }
}

class LoginRegisterRoute extends LoginChildRoute {
  final bool initial;

  const LoginRegisterRoute({required this.initial});

  @override
  Widget build(BuildContext context) {
    return LoginRegisterPage(initial: initial);
  }
}
