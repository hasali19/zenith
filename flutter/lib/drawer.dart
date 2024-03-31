import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:gap/gap.dart';
import 'package:zenith/main.dart';

class MainNavigationDrawer extends ConsumerWidget {
  final Screen current;
  final void Function(Screen) onDestinationTap;
  final void Function() onLogoutTap;

  const MainNavigationDrawer({
    super.key,
    required this.current,
    required this.onDestinationTap,
    required this.onLogoutTap,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return NavigationDrawer(
      backgroundColor: Theme.of(context).colorScheme.surface,
      selectedIndex: _screenToIndex(current),
      onDestinationSelected: (value) {
        if (value == 4) {
          onLogoutTap();
        } else {
          onDestinationTap(_indexToScreen(value));
        }
      },
      children: const [
        Gap(16),
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
        // NavigationDrawerDestination(
        //   icon: Icon(Icons.video_collection),
        //   label: Text('Collections'),
        // ),
        Padding(
          padding: EdgeInsets.fromLTRB(28, 16, 28, 10),
          child: Divider(),
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

  Screen _indexToScreen(int value) {
    return switch (value) {
      0 => Screen.home,
      1 => Screen.movies,
      2 => Screen.shows,
      3 => Screen.settings,
      _ => throw Exception('invalid destination: $value'),
    };
  }

  int _screenToIndex(Screen screen) {
    return switch (screen) {
      Screen.home => 0,
      Screen.movies => 1,
      Screen.shows => 2,
      Screen.settings => 3,
    };
  }
}
