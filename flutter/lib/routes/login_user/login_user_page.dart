import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/routes/login_user/login_user_view.dart';

class LoginUserPage extends ConsumerWidget {
  final String? username;
  final void Function() onSuccess;

  const LoginUserPage({
    super.key,
    required this.username,
    required this.onSuccess,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return LoginUserView(
      username: username,
      onSuccess: onSuccess,
    );
  }
}
