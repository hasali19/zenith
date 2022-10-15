import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:zenith_flutter/api.dart';
import 'package:zenith_flutter/drawer.dart';
import 'package:zenith_flutter/language_codes.dart';
import 'package:zenith_flutter/responsive.dart';
import 'package:zenith_flutter/screens/home.dart';
import 'package:zenith_flutter/screens/media_library.dart';
import 'package:zenith_flutter/screens/show_details.dart';
import 'package:zenith_flutter/screens/video_details_screen.dart';
import 'package:zenith_flutter/update_dialog.dart';
import 'package:zenith_flutter/updater.dart';

void main() {
  runApp(const ZenithApp());
}

class ZenithApp extends StatefulWidget {
  const ZenithApp({Key? key}) : super(key: key);

  @override
  State<ZenithApp> createState() => _ZenithAppState();
}

class _ZenithAppState extends State<ZenithApp> {
  @override
  void initState() {
    super.initState();
    loadLanguageCodes();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Zenith',
      theme: _buildTheme(Brightness.light),
      darkTheme: _buildTheme(Brightness.dark),
      home: const MainScreen(),
      builder: (context, child) => Theme(
        data: _buildThemeOverrides(context),
        child: child!,
      ),
    );
  }

  ThemeData _buildTheme(Brightness brightness) {
    final theme = ThemeData(
      brightness: brightness,
      primarySwatch: brightness == Brightness.light ? Colors.blue : null,
      useMaterial3: true,
      fontFamily: "Exo2",
    );

    switch (brightness) {
      case Brightness.dark:
        return theme.copyWith(
          scaffoldBackgroundColor: const Color.fromARGB(255, 36, 36, 36),
        );

      case Brightness.light:
        return theme;
    }
  }

  ThemeData _buildThemeOverrides(BuildContext context) {
    final theme = Theme.of(context);
    final isDesktop = context.isDesktop;
    return theme.copyWith(
      cardTheme: theme.cardTheme.copyWith(
        shape: const RoundedRectangleBorder(
          borderRadius: BorderRadius.all(Radius.circular(12)),
        ),
      ),
      textTheme: theme.textTheme.copyWith(
        bodySmall:
            theme.textTheme.bodySmall!.copyWith(fontSize: isDesktop ? 14 : 12),
        bodyMedium:
            theme.textTheme.bodyMedium!.copyWith(fontSize: isDesktop ? 16 : 14),
        titleMedium: theme.textTheme.titleMedium!
            .copyWith(fontSize: isDesktop ? 22 : 16),
      ),
    );
  }
}

class MainScreen extends StatefulWidget {
  const MainScreen({Key? key}) : super(key: key);
  @override
  State<MainScreen> createState() => _MainScreenState();
}

enum Screen {
  home,
  movies,
  shows,
  settings,
}

class _MainScreenState extends State<MainScreen> {
  Screen _screen = Screen.home;

  final _updater = Updater();

  @override
  void initState() {
    super.initState();
    if (kReleaseMode) {
      _checkForUpdates();
    }
  }

  _checkForUpdates({bool confirmLatest = false}) async {
    final update = await _updater.checkForUpdates();
    if (update != null) {
      showDialog(
        context: context,
        barrierDismissible: false,
        builder: (context) => UpdateDialog(update: update),
      );
    } else if (confirmLatest) {
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(
        content: const Text("No updates available"),
        action: SnackBarAction(
          label: "OK",
          onPressed: () {
            ScaffoldMessenger.of(context).hideCurrentSnackBar();
          },
        ),
      ));
    }
  }

  @override
  Widget build(BuildContext context) {
    final desktop = MediaQuery.of(context).size.width > 960;

    // Use a permanent navigation drawer on larger screens

    final drawer = NavigationDrawer(
      current: _screen,
      onTap: (screen) {
        if (!desktop) {
          Navigator.pop(context);
        }
        setState(() => _screen = screen);
      },
    );

    if (desktop) {
      return Row(
        children: [
          drawer,
          Expanded(
            child: Scaffold(
              body: _buildScreen(_screen),
            ),
          ),
        ],
      );
    } else {
      return Scaffold(
        appBar: AppBar(
          title: Text(_title(_screen)),
        ),
        drawer: desktop ? null : drawer,
        body: _buildScreen(_screen),
      );
    }
  }

  String _title(Screen screen) {
    switch (screen) {
      case Screen.home:
        return "Zenith";

      case Screen.movies:
        return "Movies";

      case Screen.shows:
        return "Shows";

      case Screen.settings:
        return "Settings";
    }
  }

  Widget _buildScreen(Screen screen) {
    switch (screen) {
      case Screen.home:
        return const HomeScreen();

      case Screen.movies:
        return MediaLibraryScreen(
          key: const ValueKey(Screen.movies),
          provider: fetchMovies,
          onItemTap: (item) {
            Navigator.push(
              context,
              MaterialPageRoute(
                builder: (context) => VideoDetailsScreen(item: item),
              ),
            );
          },
        );

      case Screen.shows:
        return MediaLibraryScreen(
          key: const ValueKey(Screen.shows),
          provider: fetchShows,
          onItemTap: (item) {
            Navigator.push(
              context,
              MaterialPageRoute(
                builder: (context) => ShowDetailsScreen(show: item),
              ),
            );
          },
        );

      case Screen.settings:
        return ListView(children: [
          ListTile(
            title: const Text("Revision"),
            subtitle: Text(Updater.revision ?? "Unknown"),
          ),
          ListTile(
            title: const Text("Check for updates"),
            onTap: () {
              _checkForUpdates(confirmLatest: true);
            },
          ),
        ]);
    }
  }
}
