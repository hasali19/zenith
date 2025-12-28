import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'login_user_controller.dart';
import 'login_user_state.dart';

class LoginUserView extends ConsumerStatefulWidget {
  final String? username;
  final void Function() onSuccess;

  const LoginUserView({
    super.key,
    required this.username,
    required this.onSuccess,
  });

  @override
  ConsumerState<LoginUserView> createState() => _LoginUserViewState();
}

class _LoginUserViewState extends ConsumerState<LoginUserView> {
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

    ref.listen(loginUserControllerProvider, (previous, next) {
      if (next case LoginUserSuccess()) {
        widget.onSuccess();
      } else if (next case LoginUserFailure()) {
        ScaffoldMessenger.of(
          context,
        ).showSnackBar(const SnackBar(content: Text('Login failed')));
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
                    ref
                        .read(loginUserControllerProvider.notifier)
                        .login(
                          widget.username ?? _username.text,
                          _password.text,
                        );
                  },
                  child: const Text('Login'),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
