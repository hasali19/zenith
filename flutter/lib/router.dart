import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/api.dart';
import 'package:zenith/main.dart';
import 'package:zenith/preferences.dart';
import 'package:zenith/screens/collection_details.dart';
import 'package:zenith/screens/collections.dart';
import 'package:zenith/screens/home.dart';
import 'package:zenith/screens/item_details/item_details.dart';
import 'package:zenith/screens/login.dart';
import 'package:zenith/screens/media_library.dart';
import 'package:zenith/screens/settings.dart';
import 'package:zenith/screens/setup.dart';
import 'package:zenith/screens/video_player.dart';

part 'router.gr.dart';

@MaterialAutoRouter(routes: [
  AutoRoute(
    path: '/',
    initial: true,
    guards: [NavGuard],
    children: [
      AutoRoute(
        path: '/',
        page: MainScreen,
        initial: true,
        children: [
          AutoRoute(page: HomeScreen, initial: true),
          AutoRoute(path: 'library/movies', page: MoviesScreen),
          AutoRoute(path: 'library/shows', page: ShowsScreen),
          AutoRoute(path: 'library/collections', page: CollectionsScreen),
        ],
      ),
      AutoRoute(
        path: '/items/:id',
        page: ItemDetailsScreen,
        usesPathAsKey: true,
      ),
      AutoRoute(
        path: '/collections/:id',
        page: CollectionDetailsScreen,
        usesPathAsKey: true,
      ),
      AutoRoute(
        path: '/player/:id',
        page: VideoPlayerScreen,
        usesPathAsKey: true,
      ),
      AutoRoute(path: '/setup', page: SetupScreen),
      AutoRoute(path: '/settings', page: SettingsScreen),
      AutoRoute(path: '/login', page: LoginScreen),
    ],
  ),
])
class AppRouter extends _$AppRouter {
  AppRouter({required NavGuard navGuard}) : super(navGuard: navGuard);
}

class NavGuard extends AutoRouteGuard {
  Server? Function() getServer;
  Future<bool> Function() isLoggedIn;

  NavGuard({required this.getServer, required this.isLoggedIn});

  @override
  void onNavigation(NavigationResolver resolver, StackRouter router) async {
    final activeServer = getServer();
    if (activeServer == null) {
      if (resolver.route.path == '/setup') {
        resolver.next(true);
      } else {
        router.push(const SetupScreenRoute());
      }
      return;
    }

    if (resolver.route.path != '/login' && !await isLoggedIn()) {
      router.push(LoginScreenRoute(onLogin: () => resolver.next()));
    } else {
      resolver.next(true);
    }
  }
}

final _moviesProvider = FutureProvider((ref) async {
  final api = ref.watch(apiProvider);
  final movies = await api.fetchMovies();
  return movies.map((e) => MediaLibraryItem.fromMediaItem(e, api)).toList();
});

class MoviesScreen extends ConsumerWidget {
  const MoviesScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return MediaLibraryScreen(
      provider: _moviesProvider,
      posterFallback: Icons.movie,
      onRefresh: () => ref.refresh(_moviesProvider.future),
      onItemTap: (item) =>
          context.router.push(ItemDetailsScreenRoute(id: item.id)),
    );
  }
}

final _showsProvider = FutureProvider((ref) async {
  final api = ref.watch(apiProvider);
  final shows = await api.fetchShows();
  return shows.map((e) => MediaLibraryItem.fromMediaItem(e, api)).toList();
});

class ShowsScreen extends ConsumerWidget {
  const ShowsScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return MediaLibraryScreen(
      provider: _showsProvider,
      posterFallback: Icons.tv,
      onRefresh: () => ref.refresh(_showsProvider.future),
      onItemTap: (item) =>
          context.router.push(ItemDetailsScreenRoute(id: item.id)),
    );
  }
}
