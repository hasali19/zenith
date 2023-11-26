import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/api.dart';
import 'package:zenith/routes/login_register/login_register_cubit.dart';
import 'package:zenith/routes/login_register/login_register_view.dart';

@RoutePage()
class LoginRegisterPage extends ConsumerWidget {
  final bool initial;
  final String? code;

  const LoginRegisterPage({
    super.key,
    @queryParam this.initial = false,
    @queryParam this.code,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return BlocProvider(
      create: (context) => LoginRegisterCubit(ref.read(apiProvider)),
      child: LoginRegisterView(initial: initial, code: code),
    );
  }
}
