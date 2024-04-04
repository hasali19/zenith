import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/routes/login_users/login_users_view.dart';

class LoginUsersPage extends ConsumerWidget {
  const LoginUsersPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return const LoginUsersView();
  }
}
