import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/router/stack_router.dart';
import 'package:zenith/routes/login/login_controller.dart';
import 'package:zenith/routes/login/routes.dart';

class LoginPage extends StatelessWidget {
  final String? redirect;

  const LoginPage({super.key, this.redirect});

  @override
  Widget build(BuildContext context) {
    return ProviderScope(
      overrides: [loginRedirectPathProvider.overrideWithValue(redirect)],
      child: StackRouter<LoginChildRoute>(
        buildStack: (location) => const [LoginUsersRoute()],
        buildLocation: (stack) => switch (stack.last) {
          LoginUsersRoute() => '/login',
          LoginUserRoute(:final username) => '/login/$username',
          LoginRegisterRoute(:final initial) =>
            '/login/register?initial=$initial',
        },
      ),
    );
  }
}
