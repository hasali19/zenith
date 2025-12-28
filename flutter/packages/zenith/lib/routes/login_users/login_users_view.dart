import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:gap/gap.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:zenith/api.dart';
import 'package:zenith/router.dart';

part 'login_users_view.g.dart';

@riverpod
Future<List<User>> _users(Ref ref) async {
  final api = ref.watch(apiProvider);
  return await api.fetchUsers();
}

class LoginUsersView extends StatelessWidget {
  const LoginUsersView({super.key});

  @override
  Widget build(BuildContext context) {
    return ScaffoldMessenger(
      child: Scaffold(
        body: Center(
          child: Consumer(
            builder: (context, ref, child) {
              final users = ref.watch(_usersProvider);

              ref.listen(_usersProvider, (previous, next) {
                if ((previous == null || !previous.hasError) && next.hasError) {
                  ScaffoldMessenger.of(context).showSnackBar(SnackBar(
                    content: const Text('Failed to retrieve users list'),
                    behavior: SnackBarBehavior.floating,
                    duration: const Duration(days: 365),
                    action: SnackBarAction(
                      label: 'Retry',
                      onPressed: () {
                        ref.invalidate(_usersProvider);
                      },
                    ),
                  ));
                }

                if (next case AsyncData(value: [])) {
                  context.router.replace(LoginRegisterRoute(initial: true));
                }
              });

              return users.when(
                data: (data) =>
                    _buildSuccess(context, data, users.isRefreshing),
                error: (error, stackTrace) =>
                    _buildSuccess(context, [], users.isRefreshing),
                loading: () => const CircularProgressIndicator(),
              );
            },
          ),
        ),
      ),
    );
  }

  Widget _buildSuccess(
      BuildContext context, List<User> data, bool isRefreshing) {
    final textDisplaySmall = Theme.of(context).textTheme.displaySmall;
    final users = data.map(
      (user) => _UserListCard(
        elevated: true,
        icon: Icons.account_circle,
        text: user.username,
        onTap: () =>
            context.router.push(LoginUserRoute(username: user.username)),
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
            const Gap(32),
            if (isRefreshing) const LinearProgressIndicator(),
            ...users,
            _UserListCard(
              elevated: false,
              icon: Icons.login,
              text: 'Login manually',
              onTap: () => context.router.push(LoginUserRoute(username: null)),
            ),
            _UserListCard(
              elevated: false,
              icon: Icons.add_circle_outline,
              text: 'Add user',
              onTap: () => context.router.push(LoginRegisterRoute()),
            ),
          ],
        ),
      ),
    );
  }
}

class _UserListCard extends StatelessWidget {
  final bool elevated;
  final IconData icon;
  final String text;
  final void Function() onTap;

  const _UserListCard({
    required this.elevated,
    required this.icon,
    required this.text,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      elevation: elevated ? null : 0,
      color: elevated ? null : Colors.transparent,
      clipBehavior: Clip.antiAlias,
      child: ListTile(
        leading: Icon(icon),
        title: Text(text),
        onTap: onTap,
      ),
    );
  }
}
