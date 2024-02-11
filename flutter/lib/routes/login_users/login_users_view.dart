import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:zenith/api.dart';
import 'package:zenith/router.dart';
import 'package:zenith/routes/login_users/login_users_cubit.dart';
import 'package:zenith/routes/login_users/login_users_state.dart';

class LoginUsersView extends StatelessWidget {
  const LoginUsersView({super.key});

  @override
  Widget build(BuildContext context) {
    return ScaffoldMessenger(
      child: Scaffold(
        body: Center(
          child: BlocConsumer<LoginUsersCubit, LoginUsersState>(
            listener: (context, state) {
              if (state case LoginUsersSuccess(users: [])) {
                context.router.replace(LoginRegisterRoute(initial: true));
              } else if (state case LoginUsersFailure()) {
                ScaffoldMessenger.of(context).showSnackBar(SnackBar(
                  content: const Text('Failed to retrieve users list'),
                  behavior: SnackBarBehavior.floating,
                  duration: const Duration(days: 365),
                  action: SnackBarAction(
                    label: 'Retry',
                    onPressed: () {
                      context.read<LoginUsersCubit>().refresh();
                    },
                  ),
                ));
              }
            },
            builder: (context, state) {
              return switch (state) {
                LoginUsersInitial() ||
                LoginUsersLoading() ||
                LoginUsersSuccess(users: []) =>
                  const CircularProgressIndicator(),
                LoginUsersSuccess(:final users) =>
                  _buildSuccess(context, users),
                LoginUsersFailure() => _buildSuccess(context, []),
              };
            },
          ),
        ),
      ),
    );
  }

  Widget _buildSuccess(BuildContext context, List<User> data) {
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
            const SizedBox(height: 32),
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
