import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/routes/login/login_controller.dart';
import 'package:zenith/routes/login_user/login_user_view.dart';

class LoginUserPage extends ConsumerWidget {
  final String? username;

  const LoginUserPage({super.key, required this.username});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return LoginUserView(
      username: username,
      onSuccess: () {
        // final redirectPath = ref.read(loginRedirectPathProvider);
        // final redirectRoute = context.router.root.buildPageRoute(redirectPath);
        // if (redirectRoute != null) {
        //   context.router.root.replace(redirectRoute);
        // } else {
        //   context.router.root.replace(const MainRoute());
        // }
      },
    );
  }
}
