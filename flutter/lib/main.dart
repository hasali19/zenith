import 'package:auto_route/auto_route.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:windowing/windowing.dart';
import 'package:zenith/api.dart';
import 'package:zenith/drawer.dart';
import 'package:zenith/language_codes.dart';
import 'package:zenith/preferences.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/router.dart';
import 'package:zenith/theme.dart';
import 'package:zenith/update_dialog.dart';
import 'package:zenith/updater.dart';
import 'package:zenith/window.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  final prefs = await SharedPreferences.getInstance();
  final window = await WindowController.create();

  runApp(ProviderScope(
    overrides: [
      preferencesProvider.overrideWithValue(prefs),
      windowProvider.overrideWithValue(window),
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
  ));
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
      authGuard: AuthGuard(() => ref.read(apiProvider).isLoggedIn()),
      serverSetupGuard: ServerSetupGuard(
          () => Future.value(ref.read(activeServerProvider) != null)),
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
        useMaterial3: true,
        colorSchemeSeed: Colors.deepOrange,
        fontFamily: 'Exo2',
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

class MainScreen extends ConsumerStatefulWidget {
  const MainScreen({Key? key}) : super(key: key);
  @override
  ConsumerState<MainScreen> createState() => _MainScreenState();
}

enum Screen {
  home,
  movies,
  shows,
  // collections,
  settings,
}

class _MainScreenState extends ConsumerState<MainScreen> {
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

    void onLogout() {
      ref.read(apiProvider).logout();
      context.router.popUntilRoot();
      context.router.replace(LoginScreenRoute(redirect: null));
    }

    return AutoTabsRouter(
      routes: const [
        HomeScreenRoute(),
        MoviesScreenRoute(),
        ShowsScreenRoute(),
        // CollectionsScreenRoute(),
        SettingsScreenRoute(),
      ],
      builder: (context, child, animation) {
        final screen = _activeScreen(context.tabsRouter.activeIndex);

        // Use a permanent navigation drawer on larger screens

        child = FadeTransition(opacity: animation, child: child);

        if (desktop) {
          return Scaffold(
            body: Row(
              children: [
                MainNavigationDrawer(
                  current: screen,
                  onDestinationTap: (screen) => _navigateTo(context, screen),
                  onLogoutTap: onLogout,
                ),
                Expanded(child: child),
              ],
            ),
          );
        } else {
          return Scaffold(
            appBar: AppBar(
              title: Text(_title(screen)),
              actions: [
                PopupMenuButton(
                  itemBuilder: (context) {
                    return [
                      PopupMenuItem(
                        child: const Text('Logout'),
                        onTap: onLogout,
                      ),
                    ];
                  },
                ),
              ],
            ),
            body: child,
            bottomNavigationBar: NavigationBar(
              destinations: const [
                NavigationDestination(icon: Icon(Icons.home), label: 'Home'),
                NavigationDestination(icon: Icon(Icons.movie), label: 'Movies'),
                NavigationDestination(icon: Icon(Icons.tv), label: 'Shows'),
                NavigationDestination(
                    icon: Icon(Icons.settings), label: 'Settings'),
              ],
              selectedIndex: context.tabsRouter.activeIndex,
              onDestinationSelected: (value) {
                _navigateTo(context, _activeScreen(value));
              },
            ),
          );
        }
      },
    );
  }

  String _title(Screen screen) {
    switch (screen) {
      case Screen.home:
        return 'Zenith';

      case Screen.movies:
        return 'Movies';

      case Screen.shows:
        return 'Shows';

      // case Screen.collections:
      //   return "Collections";

      case Screen.settings:
        return 'Settings';
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
      // case 3:
      //   return Screen.collections;
      case 3:
        return Screen.settings;
      default:
        throw Exception('invalid tab index: $index');
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
      // case Screen.collections:
      //   context.tabsRouter.setActiveIndex(3);
      //   break;
      case Screen.settings:
        context.tabsRouter.setActiveIndex(3);
        break;
    }
  }
}
