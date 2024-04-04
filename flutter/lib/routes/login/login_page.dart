import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/routes/login/login_controller.dart';

class LoginPage extends StatelessWidget {
  final String? redirect;

  const LoginPage({super.key, this.redirect});

  @override
  Widget build(BuildContext context) {
    return ProviderScope(
      overrides: [loginRedirectPathProvider.overrideWithValue(redirect)],
      child: const Navigator(),
    );
  }
}
