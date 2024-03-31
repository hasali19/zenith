import 'package:auto_route/auto_route.dart';
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
import 'package:zenith/router.dart';
import 'package:zenith/theme.dart';
import 'package:zenith/themes.dart';
import 'package:zenith/update_dialog.dart';
import 'package:zenith/updater.dart';
import 'package:zenith/window.dart';

final _authStateProvider = StateProvider((ref) => false);

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
    _logger.d('created ${provider.name} : $value');
  }

  @override
  void didUpdateProvider(ProviderBase<Object?> provider, Object? previousValue,
      Object? newValue, ProviderContainer container) {
    _logger.d('updated ${provider.name} : $newValue');
  }

  @override
  void didDisposeProvider(
      ProviderBase<Object?> provider, ProviderContainer container) {
    _logger.d('disposed ${provider.name}');
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
      isServerSet: () => Future.value(ref.read(activeServerProvider) != null),
      isLoggedIn: () async {
        try {
          return await ref.read(apiProvider).isLoggedIn();
        } catch (e) {
          return null;
        }
      },
    );
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
          routerConfig: _router.config(),
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

@RoutePage()
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
    if (kReleaseMode && ref.read(enableUpdatesCheck)) {
      _checkForUpdates();
    }
  }

  _checkForUpdates() async {
    final update = await _updater.checkForUpdates();
    if (update != null && context.mounted) {
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
      context.router.popUntilRoot();
      context.router.replace(LoginRoute(redirect: null));
    }

    return AutoTabsRouter(
      routes: const [
        HomeRoute(),
        MoviesRoute(),
        ShowsRoute(),
        // CollectionsScreenRoute(),
        SettingsRoute(),
      ],
      transitionBuilder: (context, child, animation) => child,
      builder: (context, child) {
        final screen = _activeScreen(context.tabsRouter.activeIndex);

        // Use a permanent navigation drawer on larger screens

        if (desktop) {
          child = Row(
            children: [
              MainNavigationDrawer(
                current: screen,
                onDestinationTap: (screen) => _navigateTo(context, screen),
                onLogoutTap: onLogout,
              ),
              Expanded(child: child),
            ],
          );
        }

        return Scaffold(
          appBar: AppBar(
            title: Text(_title(screen)),
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
                    icon: Icon(screen == Screen.home
                        ? Icons.home
                        : Icons.home_outlined),
                    label: 'Home',
                  ),
                  NavigationDestination(
                    icon: Icon(screen == Screen.movies
                        ? Icons.movie
                        : Icons.movie_outlined),
                    label: 'Movies',
                  ),
                  NavigationDestination(
                    icon: Icon(
                        screen == Screen.shows ? Icons.tv : Icons.tv_outlined),
                    label: 'Shows',
                  ),
                  NavigationDestination(
                    icon: Icon(screen == Screen.settings
                        ? Icons.settings
                        : Icons.settings_outlined),
                    label: 'Settings',
                  ),
                ],
                selectedIndex: context.tabsRouter.activeIndex,
                onDestinationSelected: (value) {
                  _navigateTo(context, _activeScreen(value));
                },
              ),
          },
        );
      },
    );
  }

  String _title(Screen screen) {
    return switch (screen) {
      Screen.home => 'Zenith',
      Screen.movies => 'Movies',
      Screen.shows => 'Shows',
      // Screen.collections => 'Collections',
      Screen.settings => 'Settings'
    };
  }

  Screen _activeScreen(int index) {
    return switch (index) {
      0 => Screen.home,
      1 => Screen.movies,
      2 => Screen.shows,
      // 3 => Screen.collections,
      3 => Screen.settings,
      _ => throw Exception('invalid tab index: $index')
    };
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
