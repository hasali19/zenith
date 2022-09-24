import 'package:flutter/material.dart';
import 'package:zenith_flutter/main.dart';

class NavigationDrawer extends StatelessWidget {
  final Screen current;
  final void Function(Screen) onTap;

  const NavigationDrawer({
    Key? key,
    required this.current,
    required this.onTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Drawer(
      child: ListView(
        children: [
          const DrawerHeader(
            child: Image(
              image: AssetImage('assets/zenith_icon.png'),
            ),
          ),
          const DrawerSectionTitle(text: "General"),
          NavigationDrawerItem(
            title: "Home",
            icon: Icons.home,
            selected: current == Screen.home,
            onTap: () => onTap(Screen.home),
          ),
          const DrawerSectionTitle(text: "Libraries"),
          NavigationDrawerItem(
            title: "Movies",
            icon: Icons.movie,
            selected: current == Screen.movies,
            onTap: () => onTap(Screen.movies),
          ),
          NavigationDrawerItem(
            title: "Shows",
            icon: Icons.tv,
            selected: current == Screen.shows,
            onTap: () => onTap(Screen.shows),
          ),
          const DrawerSectionTitle(text: "System"),
          NavigationDrawerItem(
            title: "Settings",
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
    final textStyle = theme.textTheme.bodyText2!;
    final color = theme.textTheme.caption!.color;
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