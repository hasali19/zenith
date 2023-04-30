import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/main.dart';
import 'package:zenith/preferences.dart';

class AppDrawer extends ConsumerWidget {
  final Screen current;
  final void Function(Screen) onTap;

  const AppDrawer({
    Key? key,
    required this.current,
    required this.onTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final servers = ref.watch(serversPrefProvider);
    return Drawer(
      child: ListView(
        children: [
          const DrawerHeader(
            child: Padding(
              padding: EdgeInsets.all(16),
              child: Image(
                image: AssetImage('assets/zenith_icon.png'),
              ),
            ),
          ),
          if (servers.length > 1)
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              child: DropdownButton<Server>(
                items: servers
                    .map((e) => DropdownMenuItem(
                        value: e, child: Text(e.name ?? e.url)))
                    .toList(),
                value: ref.watch(activeServerProvider),
                onChanged: (value) {},
              ),
            ),
          const DrawerSectionTitle(text: 'General'),
          NavigationDrawerItem(
            title: 'Home',
            icon: Icons.home,
            selected: current == Screen.home,
            onTap: () => onTap(Screen.home),
          ),
          const DrawerSectionTitle(text: 'Libraries'),
          NavigationDrawerItem(
            title: 'Movies',
            icon: Icons.movie,
            selected: current == Screen.movies,
            onTap: () => onTap(Screen.movies),
          ),
          NavigationDrawerItem(
            title: 'Shows',
            icon: Icons.tv,
            selected: current == Screen.shows,
            onTap: () => onTap(Screen.shows),
          ),
          const NavigationDrawerItem(
            title: 'Collections',
            icon: Icons.video_collection,
            // selected: current == Screen.collections,
            // onTap: () => onTap(Screen.collections),
          ),
          const DrawerSectionTitle(text: 'System'),
          NavigationDrawerItem(
            title: 'Settings',
            icon: Icons.settings,
            selected: current == Screen.settings,
            onTap: () => onTap(Screen.settings),
          ),
        ],
      ),
    );
  }
}

class DrawerSectionTitle extends StatelessWidget {
  final String text;

  const DrawerSectionTitle({
    Key? key,
    required this.text,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final textStyle = theme.textTheme.bodyMedium!;
    final color = theme.textTheme.bodySmall!.color;
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 24, 16, 8),
      child: Text(
        text,
        style: textStyle.copyWith(color: color),
      ),
    );
  }
}

class NavigationDrawerItem extends StatelessWidget {
  final String title;
  final IconData icon;
  final bool selected;
  final void Function()? onTap;

  const NavigationDrawerItem({
    Key? key,
    required this.title,
    required this.icon,
    this.selected = false,
    this.onTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(8, 4, 8, 4),
      child: ListTile(
        title: Text(title),
        leading: Icon(icon),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
        dense: true,
        selected: selected,
        selectedColor: Theme.of(context).colorScheme.secondary,
        selectedTileColor: const Color.fromARGB(30, 255, 255, 255),
        onTap: onTap,
      ),
    );
  }
}
