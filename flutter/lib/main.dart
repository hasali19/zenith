import 'package:cast_framework/cast_framework.dart';
import 'package:dio_image_provider/dio_image_provider.dart';
import 'package:dynamic_color/dynamic_color.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:logger/logger.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:windowing/windowing.dart';
import 'package:zenith/api.dart';
import 'package:zenith/cookies.dart';
import 'package:zenith/dio_client.dart';
import 'package:zenith/drawer.dart';
import 'package:zenith/language_codes.dart';
import 'package:zenith/media_route_button/media_route_button.dart';
import 'package:zenith/preferences.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/routes/routes.dart';
import 'package:zenith/screens/home.dart';
import 'package:zenith/screens/media_library.dart';
import 'package:zenith/screens/settings.dart';
import 'package:zenith/screens/video_player/video_player.dart';
import 'package:zenith/theme.dart';
import 'package:zenith/themes.dart';
import 'package:zenith/update_dialog.dart';
import 'package:zenith/updater.dart';
import 'package:zenith/window.dart';

final _authStateProvider = StateProvider((ref) => false);

class RouteConfig {}

abstract class ZenithRouter {
  PrimaryRoute get currentRoute;

  void push(PrimaryRoute route);
  void pop();
  void replace(PrimaryRoute route);
  void replaceAll(PrimaryRoute route);
}

sealed class PrimaryRoute {
  const PrimaryRoute();
}

class MainRoute extends PrimaryRoute {
  const MainRoute();
}

class ItemDetailsRoute extends PrimaryRoute {
  final int id;

  const ItemDetailsRoute({required this.id});
}

class VideoPlayerRoute extends PrimaryRoute {
  final int id;
  final double startPosition;

  const VideoPlayerRoute({required this.id, required this.startPosition});
}

class LoginRoute extends PrimaryRoute {
  final String? redirect;

  const LoginRoute({required this.redirect});
}

class ZenithRouterDelegate extends RouterDelegate<RouteConfig>
    with ChangeNotifier, PopNavigatorRouterDelegateMixin<RouteConfig>
    implements ZenithRouter {
  final _routes = <PrimaryRoute>[
    const MainRoute(),
  ];

  @override
  final navigatorKey = GlobalKey<NavigatorState>();

  @override
  Widget build(BuildContext context) {
    return Navigator(
      key: navigatorKey,
      pages: _routes.map((route) => _buildPage(route)).toList(),
      onPopPage: (route, result) {
        if (!route.didPop(result)) {
          return false;
        }
        if (route.settings.arguments is PrimaryRoute) {
          _routes.remove(route.settings.arguments);
        }
        route.onPopInvoked(true);
        return true;
      },
    );
  }

  @override
  PrimaryRoute get currentRoute => _routes.last;

  @override
  void push(PrimaryRoute route) {
    _routes.add(route);
    notifyListeners();
  }

  @override
  void pop() {
    _routes.removeLast();
    notifyListeners();
  }

  @override
  void replace(PrimaryRoute route) {
    _routes.removeLast();
    _routes.add(route);
    notifyListeners();
  }

  @override
  void replaceAll(PrimaryRoute route) {
    _routes.clear();
    _routes.add(route);
    notifyListeners();
  }

  Page _buildPage(PrimaryRoute route) {
    return switch (route) {
      MainRoute() => MaterialPage(
          key: ValueKey(route),
          arguments: route,
          child: const MainScreen(),
        ),
      ItemDetailsRoute(:final id) => MaterialPage(
          key: ValueKey(route),
          arguments: route,
          child: ItemDetailsPage(id: id),
        ),
      VideoPlayerRoute(
        :final id,
        :final startPosition,
      ) =>
        MaterialPage(
          key: ValueKey(route),
          arguments: route,
          child: VideoPlayerScreen(
            id: id,
            startPosition: startPosition,
          ),
        ),
      LoginRoute(:final redirect) => MaterialPage(
          key: ValueKey(route),
          arguments: route,
          child: LoginPage(redirect: redirect),
        ),
    };
  }

  @override
  Future<void> setNewRoutePath(RouteConfig configuration) {
    // TODO: implement setNewRoutePath
    throw UnimplementedError();
  }
}

final _routerDelegateProvider =
    ChangeNotifierProvider((ref) => ZenithRouterDelegate());
final routerProvider = Provider<ZenithRouter>((ref) {
  var watch = ref.watch(_routerDelegateProvider);
  print('changed: ${watch._routes}');
  return watch;
});

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  final prefs = await SharedPreferences.getInstance();
  final window = await WindowController.create();

  runApp(ProviderScope(
    observers: [
      if (kDebugMode) _LoggingProviderObserver(),
    ],
    overrides: [
      preferencesProvider.overrideWithValue(prefs),
      windowProvider.overrideWithValue(window),
      apiProvider.overrideWith((ref) {
        final activeServer = ref.watch(activeServerProvider);
        if (activeServer != null) {
          final client =
              createDioClient(activeServer.url, ref.watch(cookieJarProvider));
          DioImage.defaultDio = client;
          return ZenithApiClient(
            client,
            authObserver: AuthenticationObserver(
              onLoggedIn: () =>
                  ref.read(_authStateProvider.notifier).state = true,
              onLoggedOut: () =>
                  ref.read(_authStateProvider.notifier).state = false,
            ),
          );
        } else {
          throw UnimplementedError();
        }
      }),
    ],
    child: const ZenithApp(),
  ));
}

class _LoggingProviderObserver extends ProviderObserver {
  final _logger = Logger(
    printer: PrettyPrinter(
      methodCount: 0,
      noBoxingByDefault: true,
    ),
  );

  @override
  void didAddProvider(ProviderBase<Object?> provider, Object? value,
      ProviderContainer container) {
    // _logger.d('created ${provider.name} : $value');
  }

  @override
  void didUpdateProvider(ProviderBase<Object?> provider, Object? previousValue,
      Object? newValue, ProviderContainer container) {
    // _logger.d('updated ${provider.name} : $newValue');
  }

  @override
  void didDisposeProvider(
      ProviderBase<Object?> provider, ProviderContainer container) {
    // _logger.d('disposed ${provider.name}');
  }
}

class ZenithApp extends ConsumerStatefulWidget {
  const ZenithApp({super.key});

  @override
  ConsumerState<ZenithApp> createState() => _ZenithAppState();
}

class _ZenithAppState extends ConsumerState<ZenithApp> {
  @override
  void initState() {
    super.initState();
    loadLanguageCodes();
  }

  @override
  Widget build(BuildContext context) {
    if (CastFrameworkPlatform.instance.isSupported) {
      ref.listen(_authStateProvider, (previous, next) async {
        if (previous != true && next) {
          final api = ref.read(apiProvider);
          final castConfig = await api.fetchCastConfig();
          final castReceiverAppId = castConfig.appId;
          if (castReceiverAppId != null) {
            await CastFrameworkPlatform.instance.mediaRouter
                .init(receiverAppId: castReceiverAppId);
          }
        }
      });
    }

    final useDynamicColor = ref.watch(enableDynamicColor);
    return DynamicColorBuilder(builder: (light, dark) {
      final lightTheme = _buildTheme(
          context, Brightness.light, useDynamicColor ? light : null);
      final darkTheme =
          _buildTheme(context, Brightness.dark, useDynamicColor ? dark : null);
      return ProviderScope(
        overrides: [
          themesProvider.overrideWithValue(Themes(lightTheme, darkTheme)),
        ],
        child: MaterialApp.router(
          title: 'Zenith',
          theme: lightTheme,
          darkTheme: darkTheme,
          themeMode: switch (ref.watch(themeMode)) {
            AppThemeMode.light => ThemeMode.light,
            AppThemeMode.dark => ThemeMode.dark,
            AppThemeMode.system => ThemeMode.system,
          },
          routerDelegate: ref.watch(_routerDelegateProvider),
        ),
      );
    });
  }

  ThemeData _buildTheme(
      BuildContext context, Brightness brightness, ColorScheme? colorScheme) {
    final isDesktop = context.isDesktop;
    final baseTheme = ThemeData(
      brightness: brightness,
      colorScheme: colorScheme ??
          ColorScheme.fromSeed(
            seedColor: Colors.deepOrange,
            brightness: brightness,
          ),
      fontFamily: 'Exo2',
    );
    return baseTheme.copyWith(
      cardTheme: baseTheme.cardTheme.copyWith(
        shape: const RoundedRectangleBorder(
          borderRadius: BorderRadius.all(Radius.circular(12)),
        ),
      ),
      extensions: [
        ZenithTheme(
          titleLarge: baseTheme.textTheme.titleLarge!
              .copyWith(fontSize: isDesktop ? 36 : 22),
          titleMedium: baseTheme.textTheme.titleMedium!
              .copyWith(fontSize: isDesktop ? 22 : 16),
          bodySmall: baseTheme.textTheme.bodySmall!
              .copyWith(fontSize: isDesktop ? 14 : 12),
          bodyMedium: baseTheme.textTheme.bodyMedium!
              .copyWith(fontSize: isDesktop ? 16 : 14),
        ),
      ],
    );
  }
}

class MainScreen extends ConsumerStatefulWidget {
  const MainScreen({super.key});

  @override
  ConsumerState<MainScreen> createState() => _MainScreenState();
}

enum Screen {
  home,
  movies,
  shows,
  settings,
}

class _MainScreenState extends ConsumerState<MainScreen> {
  final _updater = Updater();

  Screen _screen = Screen.home;

  @override
  void initState() {
    super.initState();
    if (kReleaseMode && ref.read(enableUpdatesCheck)) {
      _checkForUpdates();
    }
  }

  _checkForUpdates() async {
    final update = await _updater.checkForUpdates();
    if (update != null && mounted) {
      if (update.showCustomUpdateUi) {
        showDialog(
          context: context,
          barrierDismissible: false,
          builder: (context) => UpdateDialog(update: update),
        );
      } else {
        await update.install((progress) {});
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final desktop = MediaQuery.of(context).size.width > 960;

    void onLogout() {
      ref.read(apiProvider).logout();
      ref.read(routerProvider).replaceAll(const LoginRoute(redirect: null));
    }

    final index = switch (_screen) {
      Screen.home => 0,
      Screen.movies => 1,
      Screen.shows => 2,
      Screen.settings => 3,
    };

    Widget child = IndexedStack(
      index: index,
      children: const [
        HomeScreen(),
        MoviesScreen(),
        ShowsScreen(),
        SettingsScreen(),
      ],
    );

    if (desktop) {
      child = Row(
        children: [
          MainNavigationDrawer(
            current: _screen,
            onDestinationTap: (screen) => _navigateTo(context, screen),
            onLogoutTap: onLogout,
          ),
          Expanded(child: child),
        ],
      );
    }

    return Scaffold(
      appBar: AppBar(
        title: Text(_title(_screen)),
        elevation: desktop ? 3 : null,
        backgroundColor:
            desktop ? Theme.of(context).colorScheme.surfaceContainer : null,
        actions: [
          if (CastFrameworkPlatform.instance.isSupported)
            const MediaRouteButton(),
          if (!desktop)
            PopupMenuButton(
              itemBuilder: (context) {
                return [
                  PopupMenuItem(
                    onTap: onLogout,
                    child: const Text('Logout'),
                  ),
                ];
              },
            ),
        ],
      ),
      body: child,
      bottomNavigationBar: switch (desktop) {
        true => null,
        false => NavigationBar(
            destinations: [
              NavigationDestination(
                icon: Icon(
                    _screen == Screen.home ? Icons.home : Icons.home_outlined),
                label: 'Home',
              ),
              NavigationDestination(
                icon: Icon(_screen == Screen.movies
                    ? Icons.movie
                    : Icons.movie_outlined),
                label: 'Movies',
              ),
              NavigationDestination(
                icon: Icon(
                    _screen == Screen.shows ? Icons.tv : Icons.tv_outlined),
                label: 'Shows',
              ),
              NavigationDestination(
                icon: Icon(_screen == Screen.settings
                    ? Icons.settings
                    : Icons.settings_outlined),
                label: 'Settings',
              ),
            ],
            selectedIndex: index,
            onDestinationSelected: (value) {
              _navigateTo(context, _activeScreen(value));
            },
          ),
      },
    );
  }

  String _title(Screen screen) {
    return switch (screen) {
      Screen.home => 'Zenith',
      Screen.movies => 'Movies',
      Screen.shows => 'Shows',
      Screen.settings => 'Settings'
    };
  }

  Screen _activeScreen(int index) {
    return switch (index) {
      0 => Screen.home,
      1 => Screen.movies,
      2 => Screen.shows,
      3 => Screen.settings,
      _ => throw Exception('invalid tab index: $index')
    };
  }

  void _navigateTo(BuildContext context, Screen screen) {
    setState(() {
      _screen = screen;
    });
  }
}
