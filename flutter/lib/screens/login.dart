import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/api.dart';
import 'package:zenith/router.dart';

class LoginScreen extends ConsumerStatefulWidget {
  const LoginScreen({super.key});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() => _LoginScreenState();
}

class _LoginScreenState extends ConsumerState<LoginScreen> {
  @override
  Widget build(BuildContext context) {
    return const AutoRouter();
  }
}

final _usersProvider =
    FutureProvider.autoDispose((ref) => ref.watch(apiProvider).fetchUsers());

class LoginUsersScreen extends ConsumerWidget {
  const LoginUsersScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      body: Center(
        child: ref.watch(_usersProvider).maybeWhen(
              data: (data) => _buildData(context, data),
              orElse: () => const CircularProgressIndicator(),
            ),
      ),
    );
  }

  Widget _buildData(BuildContext context, List<User> data) {
    if (data.isEmpty) {
      context.router.replace(const LoginRegisterScreenRoute());
      return Container();
    }

    final textDisplaySmall = Theme.of(context).textTheme.displaySmall;

    final users = data.map(
      (user) => Card(
        clipBehavior: Clip.antiAlias,
        child: ListTile(
          leading: const Icon(Icons.account_circle),
          title: Text(user.username),
          onTap: () => context.router
              .push(LoginUserScreenRoute(username: user.username)),
        ),
      ),
    );

    return Padding(
      padding: const EdgeInsets.all(8),
      child: ConstrainedBox(
        constraints: BoxConstraints.loose(const Size.fromWidth(600)),
        child: ListView(
          shrinkWrap: true,
          children: [
            Text('Login', style: textDisplaySmall, textAlign: TextAlign.center),
            const SizedBox(height: 32),
            ...users,
            Card(
              elevation: 0,
              color: Colors.transparent,
              clipBehavior: Clip.antiAlias,
              child: ListTile(
                leading: const Icon(Icons.arrow_forward),
                title: const Text('Login manually'),
                onTap: () =>
                    context.router.push(LoginUserScreenRoute(username: null)),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class LoginUserScreen extends ConsumerStatefulWidget {
  final String? username;

  const LoginUserScreen({super.key, @queryParam required this.username});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() =>
      _LoginUserScreenState();
}

class _LoginUserScreenState extends ConsumerState<LoginUserScreen> {
  final TextEditingController _username = TextEditingController();
  final TextEditingController _password = TextEditingController();

  @override
  void dispose() {
    _username.dispose();
    super.dispose();
  }

  void _login() async {
    final zenith = ref.read(apiProvider);
    final username = widget.username ?? _username.text;
    if (!await zenith.login(username, _password.text)) {
      ScaffoldMessenger.of(context)
          .showSnackBar(const SnackBar(content: Text('Login failed')));
      return;
    }
    context.router.replace(const MainScreenRoute());
  }

  @override
  Widget build(BuildContext context) {
    final textDisplaySmall = Theme.of(context).textTheme.displaySmall;
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
                  child: const Text('Login'),
                  onPressed: _login,
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

class LoginRegisterScreen extends ConsumerStatefulWidget {
  const LoginRegisterScreen({super.key});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() =>
      _LoginRegisterScreenState();
}

class _LoginRegisterScreenState extends ConsumerState<LoginRegisterScreen> {
  final TextEditingController _username = TextEditingController();
  final TextEditingController _password = TextEditingController();

  @override
  void dispose() {
    _username.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final textDisplaySmall = Theme.of(context).textTheme.displaySmall;
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
                TextField(
                  controller: _username,
                  decoration: const InputDecoration(labelText: 'Username'),
                  autofocus: true,
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
                    final api = ref.read(apiProvider);

                    final username = _username.text;
                    final password = _password.text;

                    try {
                      await api.createUser(username, password);
                    } catch (e) {
                      ScaffoldMessenger.of(context)
                        ..clearSnackBars()
                        ..showSnackBar(const SnackBar(
                            content: Text('Failed to create user')));
                    }

                    context.router.replace(const LoginUsersScreenRoute());
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
