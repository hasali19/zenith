import 'package:auto_route/auto_route.dart';
import 'package:cast_framework/cast_framework.dart';
import 'package:dio_image_provider/dio_image_provider.dart';
import 'package:dynamic_color/dynamic_color.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:logger/logger.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:windowing/windowing.dart';
import 'package:zenith/api.dart';
import 'package:zenith/cookies.dart';
import 'package:zenith/database/database.dart';
import 'package:zenith/dio_client.dart';
import 'package:zenith/main_navigation.dart';
import 'package:zenith/language_codes.dart';
import 'package:zenith/preferences.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/router.dart';
import 'package:zenith/theme.dart';
import 'package:zenith/themes.dart';
import 'package:zenith/updater.dart';
import 'package:zenith/window.dart';

final _authStateProvider = StateProvider((ref) => false);

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  final prefs = await SharedPreferences.getInstance();
  final window = await WindowController.create();
  final packageInfo = await PackageInfo.fromPlatform();

  final db = AppDatabase();
  final servers = ServersList(
    db,
    await db.select(db.servers).get(),
  );

  runApp(ProviderScope(
    observers: [
      if (kDebugMode) _LoggingProviderObserver(),
    ],
    overrides: [
      preferencesProvider.overrideWithValue(prefs),
      databaseProvider.overrideWithValue(db),
      serversProvider.overrideWithValue(servers),
      windowProvider.overrideWithValue(window),
      apiProvider.overrideWith((ref) {
        final activeServer = ref.watch(activeServerProvider);
        if (activeServer != null) {
          final client = createDioClient(
            activeServer.url,
            ref.watch(cookieJarProvider),
            packageInfo,
          );
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
  const ZenithApp({super.key});

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
      pageTransitionsTheme: PageTransitionsTheme(
        builders: {
          TargetPlatform.android: FadeForwardsPageTransitionsBuilder(),
        },
      ),
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
  const MainScreen({super.key});

  @override
  ConsumerState<MainScreen> createState() => _MainScreenState();
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
      await update.install();
    }
  }

  @override
  Widget build(BuildContext context) {
    final desktop = MediaQuery.of(context).size.width > 960;

    void onLogout() {
      ref.read(apiProvider).logout();
      context.router.replaceAll([LoginRoute(redirect: null)]);
    }

    return AutoTabsRouter(
      routes: const [
        HomeRoute(),
        ManageServerRoute(),
        SettingsRoute(),
      ],
      transitionBuilder: (context, child, animation) => child,
      builder: (context, child) {
        if (desktop) {
          // Use a permanent navigation drawer on larger screens
          child = Row(
            children: [
              MainNavigationDrawer(onLogoutTap: onLogout),
              Expanded(child: child),
            ],
          );
        }

        return Scaffold(
          body: child,
          bottomNavigationBar: switch (desktop) {
            true => null,
            false => MainNavigationBar(),
          },
        );
      },
    );
  }
}
