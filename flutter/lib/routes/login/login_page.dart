import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/routes/login/login_controller.dart';

@RoutePage()
class LoginPage extends StatelessWidget {
  final String? redirect;

  const LoginPage({super.key, @queryParam this.redirect});

  @override
  Widget build(BuildContext context) {
    return ProviderScope(
      overrides: [loginRedirectPathProvider.overrideWithValue(redirect)],
      child: const AutoRouter(),
    );
  }
}
