import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/main.dart';
import 'package:zenith/router/page.dart';
import 'package:zenith/router/stack_router.dart';
import 'package:zenith/routes/login/login_controller.dart';
import 'package:zenith/routes/login/routes.dart';
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
        onSetLocation: (location) => const [LoginUsersRoute()],
        buildLocation: (route) => switch (route) {
          LoginUsersRoute() => '/login',
          LoginUserRoute(:final username) => '/login/$username',
          LoginRegisterRoute(:final initial) =>
            '/login/register?initial=$initial',
        },
        buildPage: (route) {
          return ZenithPage(
            route: route,
            child: switch (route) {
              LoginUsersRoute() => const LoginUsersPage(),
              LoginUserRoute() => LoginUserPage(
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
              LoginRegisterRoute() => LoginRegisterPage(initial: route.initial),
            },
          );
        },
      ),
    );
  }
}
