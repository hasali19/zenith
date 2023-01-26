import 'package:auto_route/auto_route.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:zenith/api.dart';
import 'package:zenith/drawer.dart';
import 'package:zenith/language_codes.dart';
import 'package:zenith/preferences.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/router.dart';
import 'package:zenith/theme.dart';
import 'package:zenith/update_dialog.dart';
import 'package:zenith/updater.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  final prefs = await SharedPreferences.getInstance();
  runApp(Bootstrap(prefs: prefs));
}

class Bootstrap extends StatefulWidget {
  const Bootstrap({super.key, required this.prefs});

  final SharedPreferences prefs;

  @override
  State<Bootstrap> createState() => _BootstrapState();
}

class _BootstrapState extends State<Bootstrap> {
  @override
  Widget build(BuildContext context) {
    return ProviderScope(
      overrides: [
        preferencesProvider.overrideWithValue(widget.prefs),
        apiProvider.overrideWith((ref) {
          final activeServer = ref.watch(activeServerProvider);
          if (activeServer != null) {
            return ZenithApiClient(activeServer.url);
          } else {
            throw UnimplementedError();
          }
        }),
      ],
      child: const ZenithApp(),
    );
  }
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

  ThemeData _buildTheme(Brightness brightness) => ThemeData(
        brightness: brightness,
        primarySwatch: Colors.deepOrange,
        useMaterial3: true,
        fontFamily: "Exo2",
      );

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
          titleLarge: theme.textTheme.titleLarge!
              .copyWith(fontSize: isDesktop ? 36 : 22),
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
  collections,
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
        CollectionsScreenRoute(),
      ],
      builder: (context, child, animation) {
        final screen = _activeScreen(context.tabsRouter.activeIndex);

        // Use a permanent navigation drawer on larger screens

        final drawer = AppDrawer(
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

      case Screen.collections:
        return "Collections";

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
      case 3:
        return Screen.collections;
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
      case Screen.collections:
        context.tabsRouter.setActiveIndex(3);
        break;
      case Screen.settings:
        context.router.push(SettingsScreenRoute());
        break;
    }
  }
}
