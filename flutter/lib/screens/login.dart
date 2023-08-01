import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/api.dart';
import 'package:zenith/router.dart';

final Provider<String?> redirectProvider = Provider((ref) => null);

class LoginScreen extends ConsumerStatefulWidget {
  final String? redirect;

  const LoginScreen({super.key, @queryParam this.redirect});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() => _LoginScreenState();
}

class _LoginScreenState extends ConsumerState<LoginScreen> {
  @override
  Widget build(BuildContext context) {
    return ProviderScope(
      overrides: [redirectProvider.overrideWithValue(widget.redirect)],
      child: const AutoRouter(),
    );
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
        child: ref.watch(_usersProvider).when(
              data: (data) => _buildData(context, data),
              error: (error, stackTrace) => Center(
                  child: Text('failed to load users: ${error.toString()}')),
              loading: () => const CircularProgressIndicator(),
            ),
      ),
    );
  }

  Widget _buildData(BuildContext context, List<User> data) {
    if (data.isEmpty) {
      context.router.replace(LoginRegisterScreenRoute(initial: true));
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
                leading: const Icon(Icons.login),
                title: const Text('Login manually'),
                onTap: () =>
                    context.router.push(LoginUserScreenRoute(username: null)),
              ),
            ),
            Card(
              elevation: 0,
              color: Colors.transparent,
              clipBehavior: Clip.antiAlias,
              child: ListTile(
                leading: const Icon(Icons.add_circle_outline),
                title: const Text('Add user'),
                onTap: () => context.router.push(LoginRegisterScreenRoute()),
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
    _password.dispose();
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

    final redirectPath = ref.read(redirectProvider);
    if (redirectPath != null) {
      context.router.replaceNamed(redirectPath);
    } else {
      context.router.replace(const MainScreenRoute());
    }
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
  final bool initial;
  final String? code;

  const LoginRegisterScreen({
    super.key,
    @queryParam this.initial = false,
    @queryParam this.code,
  });

  @override
  ConsumerState<ConsumerStatefulWidget> createState() =>
      _LoginRegisterScreenState();
}

class _LoginRegisterScreenState extends ConsumerState<LoginRegisterScreen> {
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
                    final api = ref.read(apiProvider);

                    final username = _username.text;
                    final password = _password.text;

                    try {
                      await api.createUser(username, password,
                          _code.text.isNotEmpty ? _code.text : null);
                    } catch (e) {
                      ScaffoldMessenger.of(context)
                        ..clearSnackBars()
                        ..showSnackBar(const SnackBar(
                            content: Text('Failed to create user')));
                      return;
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
