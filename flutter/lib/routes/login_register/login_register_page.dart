import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/routes/login_register/login_register_view.dart';

class LoginRegisterPage extends ConsumerWidget {
  final bool initial;
  final String? code;

  const LoginRegisterPage({
    super.key,
    this.initial = false,
    this.code,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return LoginRegisterView(initial: initial, code: code);
  }
}
