import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import 'login_user_cubit.dart';
import 'login_user_state.dart';

class LoginUserView extends StatefulWidget {
  final String? username;
  final void Function() onSuccess;

  const LoginUserView({
    super.key,
    required this.username,
    required this.onSuccess,
  });

  @override
  State<LoginUserView> createState() => _LoginUserViewState();
}

class _LoginUserViewState extends State<LoginUserView> {
  final TextEditingController _username = TextEditingController();
  final TextEditingController _password = TextEditingController();

  @override
  void dispose() {
    _username.dispose();
    _password.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final textDisplaySmall = Theme.of(context).textTheme.displaySmall;
    return BlocListener<LoginUserCubit, LoginUserState>(
      listener: (context, state) {
        if (state case LoginUserSuccess()) {
          widget.onSuccess();
        } else if (state case LoginUserFailure()) {
          ScaffoldMessenger.of(context)
              .showSnackBar(const SnackBar(content: Text('Login failed')));
        }
      },
      child: Scaffold(
        appBar: AppBar(),
        body: Center(
          child: Padding(
            padding: const EdgeInsets.all(16),
            child: ConstrainedBox(
              constraints: BoxConstraints.loose(const Size.fromWidth(600)),
              child: ListView(
                shrinkWrap: true,
                children: [
                  if (widget.username == null) ...[
                    Text('Login', style: textDisplaySmall),
                    const SizedBox(height: 16),
                    TextField(
                      controller: _username,
                      decoration: const InputDecoration(labelText: 'Username'),
                      autofocus: true,
                    ),
                  ],
                  if (widget.username != null)
                    Text(widget.username!, style: textDisplaySmall),
                  const SizedBox(height: 8),
                  TextField(
                    controller: _password,
                    decoration: const InputDecoration(labelText: 'Password'),
                    obscureText: true,
                    autofocus: widget.username != null,
                  ),
                  const SizedBox(height: 16),
                  ElevatedButton(
                    onPressed: () {
                      context.read<LoginUserCubit>().login(
                          widget.username ?? _username.text, _password.text);
                    },
                    child: const Text('Login'),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
