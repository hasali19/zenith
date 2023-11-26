import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:zenith/routes/login/login_cubit.dart';

@RoutePage()
class LoginPage extends StatelessWidget {
  final String? redirect;

  const LoginPage({super.key, @queryParam this.redirect});

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create: (context) => LoginCubit(redirect),
      child: const AutoRouter(),
    );
  }
}
