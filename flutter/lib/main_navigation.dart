import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/router.dart';

class MainNavigationDrawer extends ConsumerWidget {
  final void Function() onLogoutTap;

  const MainNavigationDrawer({
    super.key,
    required this.onLogoutTap,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return NavigationDrawer(
      backgroundColor: Theme.of(context).colorScheme.surface,
      selectedIndex: _activeDestinationIndex(context),
      onDestinationSelected: (value) {
        if (value == 5) {
          onLogoutTap();
          return;
        }

        final route = switch (value) {
          0 => const HomeRoute(),
          1 => const MoviesRoute(),
          2 => const ShowsRoute(),
          3 => const ManageServerRoute(),
          4 => const SettingsRoute(),
          _ => throw Exception('Invalid destination index: $value'),
        };

        context.router.navigate(route);
      },
      children: [
        NavigationDrawerHeadline(text: 'Library'),
        NavigationDrawerDestination(
          icon: Icon(Icons.home),
          label: Text('Home'),
        ),
        NavigationDrawerDestination(
          icon: Icon(Icons.movie),
          label: Text('Movies'),
        ),
        NavigationDrawerDestination(
          icon: Icon(Icons.tv),
          label: Text('Shows'),
        ),
        Padding(
          padding: EdgeInsets.fromLTRB(28, 16, 28, 10),
          child: Divider(),
        ),
        NavigationDrawerDestination(
          icon: Icon(Icons.dns),
          label: Text('Server'),
        ),
        NavigationDrawerDestination(
          icon: Icon(Icons.settings),
          label: Text('Settings'),
        ),
        NavigationDrawerDestination(
          icon: Icon(Icons.logout),
          label: Text('Logout'),
        ),
      ],
    );
  }

  int? _activeDestinationIndex(BuildContext context) {
    final router = context.watchRouter;
    if (router.isRouteActive(HomeRoute.name)) {
      return 0;
    } else if (router.isRouteActive(MoviesRoute.name)) {
      return 1;
    } else if (router.isRouteActive(ShowsRoute.name)) {
      return 2;
    } else if (router.isRouteActive(ManageServerRoute.name)) {
      return 3;
    } else if (router.isRouteActive(SettingsRoute.name)) {
      return 4;
    } else {
      return null;
    }
  }
}

class NavigationDrawerHeadline extends StatelessWidget {
  final String text;

  const NavigationDrawerHeadline({
    super.key,
    required this.text,
  });

  @override
  Widget build(BuildContext context) {
    final style = Theme.of(context).textTheme.titleSmall;
    return Padding(
      padding: const EdgeInsets.fromLTRB(28, 16, 16, 10),
      child: Text(text, style: style),
    );
  }
}

class MainNavigationBar extends StatefulWidget {
  const MainNavigationBar({super.key});

  @override
  State<MainNavigationBar> createState() => _MainNavigationBarState();
}

class _MainNavigationBarState extends State<MainNavigationBar> {
  int _index = 0;

  @override
  void initState() {
    super.initState();
    _index = _activeDestinationIndex(context);
  }

  @override
  Widget build(BuildContext context) {
    return NavigationBar(
      destinations: [
        NavigationDestination(
          icon: Icon(_index == 0 ? Icons.home : Icons.home_outlined),
          label: 'Home',
        ),
        NavigationDestination(
          icon: Icon(_index == 1 ? Icons.movie : Icons.movie_outlined),
          label: 'Movies',
        ),
        NavigationDestination(
          icon: Icon(_index == 2 ? Icons.tv : Icons.tv_outlined),
          label: 'Shows',
        ),
        NavigationDestination(
          icon: Icon(_index == 3 ? Icons.dns : Icons.dns_outlined),
          label: 'Server',
        ),
        NavigationDestination(
          icon: Icon(_index == 4 ? Icons.settings : Icons.settings_outlined),
          label: 'Settings',
        ),
      ],
      selectedIndex: _index,
      onDestinationSelected: (value) {
        final route = switch (value) {
          0 => const HomeRoute(),
          1 => const MoviesRoute(),
          2 => const ShowsRoute(),
          3 => const ManageServerRoute(),
          4 => const SettingsRoute(),
          _ => throw Exception('Invalid destination index: $value'),
        };

        context.router.navigate(route);

        setState(() {
          _index = value;
        });
      },
    );
  }

  int _activeDestinationIndex(BuildContext context) {
    final router = context.router;
    if (router.isRouteActive(HomeRoute.name)) {
      return 0;
    } else if (router.isRouteActive(MoviesRoute.name)) {
      return 1;
    } else if (router.isRouteActive(ShowsRoute.name)) {
      return 2;
    } else if (router.isRouteActive(ManageServerRoute.name)) {
      return 3;
    } else if (router.isRouteActive(SettingsRoute.name)) {
      return 4;
    } else {
      return 0;
    }
  }
}
