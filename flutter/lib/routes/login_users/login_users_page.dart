import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/api.dart';
import 'package:zenith/routes/login_users/login_users_cubit.dart';
import 'package:zenith/routes/login_users/login_users_view.dart';

@RoutePage()
class LoginUsersPage extends ConsumerWidget {
  const LoginUsersPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return BlocProvider(
      create: (context) => LoginUsersCubit(ref.read(apiProvider))..refresh(),
      child: const LoginUsersView(),
    );
  }
}
