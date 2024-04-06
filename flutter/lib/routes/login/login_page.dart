import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/main.dart';
import 'package:zenith/router/stack_router.dart';
import 'package:zenith/routes/login/login_controller.dart';
import 'package:zenith/routes/login/router.dart';
import 'package:zenith/routes/login_register/login_register_page.dart';
import 'package:zenith/routes/login_user/login_user_page.dart';
import 'package:zenith/routes/login_users/login_users_page.dart';

class LoginPage extends StatelessWidget {
  final String? redirect;

  const LoginPage({super.key, this.redirect});

  @override
  Widget build(BuildContext context) {
    return ProviderScope(
      overrides: [loginRedirectPathProvider.overrideWithValue(redirect)],
      child: StackRouter<LoginSubRoute>(
        initial: const LoginUsersRoute(),
        buildPage: (route) {
          return switch (route) {
            LoginUsersRoute() => MaterialPage(
                key: ValueKey(route),
                arguments: route,
                child: const LoginUsersPage(),
              ),
            LoginUserRoute() => MaterialPage(
                key: ValueKey(route),
                arguments: route,
                child: LoginUserPage(
                  username: route.username,
                  onSuccess: () {
                    // final redirectPath = ref.read(loginRedirectPathProvider);
                    // final redirectRoute = context.router.root.buildPageRoute(redirectPath);
                    // if (redirectRoute != null) {
                    //   context.router.root.replace(redirectRoute);
                    // } else {
                    //   context.router.root.replace(const MainRoute());
                    // }
                    StackRouter.of<PrimaryRoute>(context)
                        .replace(const MainRoute());
                  },
                ),
              ),
            LoginRegisterRoute() => MaterialPage(
                key: ValueKey(route),
                arguments: route,
                child: LoginRegisterPage(initial: route.initial),
              ),
          };
        },
      ),
    );
  }
}
