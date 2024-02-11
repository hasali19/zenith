import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/router.dart';
import 'package:zenith/routes/login_register/login_register_controller.dart';

import 'login_register_state.dart';

class LoginRegisterView extends ConsumerStatefulWidget {
  final bool initial;
  final String? code;

  const LoginRegisterView({
    super.key,
    required this.initial,
    required this.code,
  });

  @override
  ConsumerState<LoginRegisterView> createState() => _LoginRegisterViewState();
}

class _LoginRegisterViewState extends ConsumerState<LoginRegisterView> {
  late final TextEditingController _code;
  final TextEditingController _username = TextEditingController();
  final TextEditingController _password = TextEditingController();

  @override
  void initState() {
    super.initState();
    _code = TextEditingController(text: widget.code);
  }

  @override
  void dispose() {
    _code.dispose();
    _username.dispose();
    _password.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final textDisplaySmall = Theme.of(context).textTheme.displaySmall;

    ref.listen(loginRegisterControllerProvider, (previous, next) {
      if (next case LoginRegisterSuccess()) {
        context.router.replace(const LoginUsersRoute());
      } else if (next case LoginRegisterFailure()) {
        ScaffoldMessenger.of(context)
          ..clearSnackBars()
          ..showSnackBar(
              const SnackBar(content: Text('Failed to create user')));
      }
    });

    return Scaffold(
      appBar: AppBar(),
      body: Center(
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: ConstrainedBox(
            constraints: BoxConstraints.loose(const Size.fromWidth(600)),
            child: ListView(
              shrinkWrap: true,
              children: [
                Text('Create User', style: textDisplaySmall),
                const SizedBox(height: 16),
                if (!widget.initial)
                  Padding(
                    padding: const EdgeInsets.only(bottom: 8),
                    child: TextField(
                      controller: _code,
                      decoration:
                          const InputDecoration(labelText: 'Registration code'),
                      enabled: widget.code == null,
                    ),
                  ),
                TextField(
                  controller: _username,
                  decoration: const InputDecoration(labelText: 'Username'),
                ),
                const SizedBox(height: 8),
                TextField(
                  controller: _password,
                  decoration: const InputDecoration(labelText: 'Password'),
                  obscureText: true,
                ),
                const SizedBox(height: 16),
                ElevatedButton(
                  child: const Text('Register'),
                  onPressed: () async {
                    ref
                        .read(loginRegisterControllerProvider.notifier)
                        .register(_code.text, _username.text, _password.text);
                  },
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
