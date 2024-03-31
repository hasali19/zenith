import 'package:auto_route/auto_route.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/api.dart';
import 'package:zenith/main.dart';
import 'package:zenith/routes/routes.dart';
import 'package:zenith/screens/home.dart';
import 'package:zenith/screens/media_library.dart';
import 'package:zenith/screens/settings.dart';
import 'package:zenith/screens/setup.dart';
import 'package:zenith/screens/video_player.dart';

part 'router.gr.dart';

@AutoRouterConfig()
class AppRouter extends _$AppRouter {
  final Future<bool> Function() isServerSet;
  final Future<bool?> Function() isLoggedIn;

  AppRouter({required this.isServerSet, required this.isLoggedIn});

  @override
  RouteType get defaultRouteType => const RouteType.material();

  @override
  List<AutoRoute> get routes => [
        AutoRoute(
          path: '/',
          page: MainRoute.page,
          initial: true,
          guards: [ServerSetupGuard(isServerSet), AuthGuard(isLoggedIn)],
          children: [
            AutoRoute(page: HomeRoute.page, initial: true),
            AutoRoute(path: 'library/movies', page: MoviesRoute.page),
            AutoRoute(path: 'library/shows', page: ShowsRoute.page),
            AutoRoute(path: 'settings', page: SettingsRoute.page),
          ],
        ),
        AutoRoute(
          path: '/items/:id',
          page: ItemDetailsRoute.page,
          usesPathAsKey: true,
          guards: [ServerSetupGuard(isServerSet), AuthGuard(isLoggedIn)],
        ),
        AutoRoute(
          path: '/player/:id',
          page: VideoPlayerRoute.page,
          usesPathAsKey: true,
          guards: [ServerSetupGuard(isServerSet), AuthGuard(isLoggedIn)],
        ),
        AutoRoute(
          path: '/login',
          page: LoginRoute.page,
          guards: [ServerSetupGuard(isServerSet)],
          children: [
            AutoRoute(page: LoginUsersRoute.page, initial: true),
            AutoRoute(path: 'user', page: LoginUserRoute.page),
            AutoRoute(path: 'register', page: LoginRegisterRoute.page),
          ],
        ),
        AutoRoute(path: '/setup', page: SetupRoute.page),
      ];

  @override
  RouterConfig<UrlState> config({
    DeepLinkBuilder? deepLinkBuilder,
    String? navRestorationScopeId,
    WidgetBuilder? placeholder,
    NavigatorObserversBuilder navigatorObservers =
        AutoRouterDelegate.defaultNavigatorObserversBuilder,
    bool includePrefixMatches = !kIsWeb,
    bool Function(String? location)? neglectWhen,
    bool rebuildStackOnDeepLink = false,
    Listenable? reevaluateListenable,
  }) {
    return super.config(
      deepLinkBuilder: deepLinkBuilder,
      navRestorationScopeId: navRestorationScopeId,
      placeholder: placeholder,
      navigatorObservers: () => [_routeObserver, ...navigatorObservers()],
      includePrefixMatches: includePrefixMatches,
      neglectWhen: neglectWhen,
      rebuildStackOnDeepLink: rebuildStackOnDeepLink,
      reevaluateListenable: reevaluateListenable,
    );
  }
}

class ServerSetupGuard extends AutoRouteGuard {
  final Future<bool> Function() isServerSet;

  ServerSetupGuard(this.isServerSet);

  @override
  void onNavigation(NavigationResolver resolver, StackRouter router) async {
    if (await isServerSet()) {
      return resolver.next(true);
    }

    router.replace(const SetupRoute());
  }
}

class AuthGuard extends AutoRouteGuard {
  final Future<bool?> Function() isLoggedIn;

  AuthGuard(this.isLoggedIn);

  @override
  void onNavigation(NavigationResolver resolver, StackRouter router) async {
    if (await isLoggedIn() != false) {
      return resolver.next(true);
    }

    router.replace(LoginRoute(redirect: resolver.route.stringMatch));
  }
}

final _moviesProvider = FutureProvider((ref) async {
  final api = ref.watch(apiProvider);
  final movies = await api.fetchMovies();
  return movies.map((e) => MediaLibraryItem.fromMediaItem(e, api)).toList();
});

@RoutePage()
class MoviesScreen extends ConsumerWidget {
  const MoviesScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return MediaLibraryScreen(
      provider: _moviesProvider,
      posterFallback: Icons.movie,
      onRefresh: () => ref.refresh(_moviesProvider.future),
      onItemTap: (item) => context.router.push(ItemDetailsRoute(id: item.id)),
    );
  }
}

final _showsProvider = FutureProvider((ref) async {
  final api = ref.watch(apiProvider);
  final shows = await api.fetchShows();
  return shows.map((e) => MediaLibraryItem.fromMediaItem(e, api)).toList();
});

@RoutePage()
class ShowsScreen extends ConsumerWidget {
  const ShowsScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return MediaLibraryScreen(
      provider: _showsProvider,
      posterFallback: Icons.tv,
      onRefresh: () => ref.refresh(_showsProvider.future),
      onItemTap: (item) => context.router.push(ItemDetailsRoute(id: item.id)),
    );
  }
}

class RouteListener extends StatefulWidget {
  final Widget child;

  final void Function()? didPushNext;
  final void Function()? didPopNext;

  const RouteListener({
    super.key,
    this.didPushNext,
    this.didPopNext,
    required this.child,
  });

  @override
  State<RouteListener> createState() => _RouteListenerState();
}

final _routeObserver = AutoRouteObserver();

class _RouteListenerState extends State<RouteListener> with AutoRouteAware {
  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    _routeObserver.subscribe(this, context.routeData);
  }

  @override
  void dispose() {
    _routeObserver.unsubscribe(this);
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return widget.child;
  }

  @override
  void didPushNext() {
    widget.didPushNext?.call();
  }

  @override
  void didPopNext() {
    widget.didPopNext?.call();
  }
}
