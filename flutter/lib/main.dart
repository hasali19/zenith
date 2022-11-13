import 'package:auto_route/auto_route.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:zenith_flutter/api.dart';
import 'package:zenith_flutter/drawer.dart';
import 'package:zenith_flutter/language_codes.dart';
import 'package:zenith_flutter/preferences.dart';
import 'package:zenith_flutter/responsive.dart';
import 'package:zenith_flutter/router.dart';
import 'package:zenith_flutter/theme.dart';
import 'package:zenith_flutter/update_dialog.dart';
import 'package:zenith_flutter/updater.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  final prefs = await SharedPreferences.getInstance();
  runApp(
    ProviderScope(
      overrides: [
        preferencesProvider.overrideWithValue(prefs),
      ],
      child: Consumer(builder: (context, ref, child) {
        Widget app = const ZenithApp();
        final activeServer = ref.watch(activeServerProvider);
        if (activeServer != null) {
          app = ProviderScope(
            overrides: [
              apiProvider.overrideWithValue(ZenithApiClient(activeServer.url)),
            ],
            child: app,
          );
        }
        return app;
      }),
    ),
  );
}

class ZenithApp extends ConsumerStatefulWidget {
  const ZenithApp({Key? key}) : super(key: key);

  @override
  ConsumerState<ZenithApp> createState() => _ZenithAppState();
}

class _ZenithAppState extends ConsumerState<ZenithApp> {
  late final AppRouter _router;

  @override
  void initState() {
    super.initState();
    loadLanguageCodes();
    _router = AppRouter(
      setupGuard: SetupGuard(() => ref.watch(activeServerProvider)),
    );
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp.router(
      title: 'Zenith',
      theme: _buildTheme(Brightness.light),
      darkTheme: _buildTheme(Brightness.dark),
      routerDelegate: _router.delegate(),
      routeInformationParser: _router.defaultRouteParser(),
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
      extensions: [
        ZenithTheme(
          titleMedium: theme.textTheme.titleMedium!
              .copyWith(fontSize: isDesktop ? 22 : 16),
          bodySmall: theme.textTheme.bodySmall!
              .copyWith(fontSize: isDesktop ? 14 : 12),
          bodyMedium: theme.textTheme.bodyMedium!
              .copyWith(fontSize: isDesktop ? 16 : 14),
        ),
      ],
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
  final _updater = Updater();

  @override
  void initState() {
    super.initState();
    if (kReleaseMode) {
      _checkForUpdates();
    }
  }

  _checkForUpdates() async {
    final update = await _updater.checkForUpdates();
    if (update != null) {
      showDialog(
        context: context,
        barrierDismissible: false,
        builder: (context) => UpdateDialog(update: update),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    final desktop = MediaQuery.of(context).size.width > 960;
    return AutoTabsRouter(
      routes: const [
        HomeScreenRoute(),
        MoviesScreenRoute(),
        ShowsScreenRoute(),
      ],
      builder: (context, child, animation) {
        final screen = _activeScreen(context.tabsRouter.activeIndex);

        // Use a permanent navigation drawer on larger screens

        final drawer = NavigationDrawer(
          current: screen,
          onTap: (screen) {
            if (!desktop) {
              // Close drawer
              Navigator.pop(context);
            }
            _navigateTo(context, screen);
          },
        );

        child = FadeTransition(opacity: animation, child: child);

        if (desktop) {
          return Row(
            children: [
              drawer,
              Expanded(
                child: Scaffold(
                  body: child,
                ),
              ),
            ],
          );
        } else {
          return Scaffold(
            appBar: AppBar(
              title: Text(_title(screen)),
            ),
            drawer: desktop ? null : drawer,
            body: child,
          );
        }
      },
    );
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

  Screen _activeScreen(int index) {
    switch (index) {
      case 0:
        return Screen.home;
      case 1:
        return Screen.movies;
      case 2:
        return Screen.shows;
      default:
        throw Exception("invalid tab index: $index");
    }
  }

  void _navigateTo(BuildContext context, Screen screen) {
    switch (screen) {
      case Screen.home:
        context.tabsRouter.setActiveIndex(0);
        break;
      case Screen.movies:
        context.tabsRouter.setActiveIndex(1);
        break;
      case Screen.shows:
        context.tabsRouter.setActiveIndex(2);
        break;
      case Screen.settings:
        context.router.push(SettingsScreenRoute());
        break;
    }
  }
}
