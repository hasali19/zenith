import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/api.dart';
import 'package:zenith/router.dart';
import 'package:zenith/routes/login/login_cubit.dart';
import 'package:zenith/routes/login_user/login_user_cubit.dart';
import 'package:zenith/routes/login_user/login_user_view.dart';

@RoutePage()
class LoginUserPage extends ConsumerWidget {
  final String? username;

  const LoginUserPage({super.key, @queryParam required this.username});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return BlocProvider(
      create: (context) => LoginUserCubit(ref.read(apiProvider)),
      child: LoginUserView(
        username: username,
        onSuccess: () {
          final redirectPath = context.read<LoginCubit>().state.redirectPath;
          final redirectRoute =
              context.router.root.buildPageRoute(redirectPath);
          if (redirectRoute != null) {
            context.router.root.replace(redirectRoute);
          } else {
            context.router.root.replace(const MainRoute());
          }
        },
      ),
    );
  }
}
