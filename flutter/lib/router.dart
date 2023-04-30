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
import 'package:zenith/screens/media_library.dart';
import 'package:zenith/screens/settings.dart';
import 'package:zenith/screens/setup.dart';
import 'package:zenith/screens/video_player.dart';

part 'router.gr.dart';

@MaterialAutoRouter(routes: [
  AutoRoute(
    path: '/',
    page: MainScreen,
    initial: true,
    guards: [SetupGuard],
    children: [
      AutoRoute(page: HomeScreen, initial: true),
      AutoRoute(path: 'library/movies', page: MoviesScreen),
      AutoRoute(path: 'library/shows', page: ShowsScreen),
      AutoRoute(path: 'library/collections', page: CollectionsScreen),
      AutoRoute(path: 'settings', page: SettingsScreen),
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
  AutoRoute(path: '/setup', page: SetupScreen, guards: [SetupGuard]),
])
class AppRouter extends _$AppRouter {
  AppRouter({required SetupGuard setupGuard}) : super(setupGuard: setupGuard);
}

class SetupGuard extends AutoRouteGuard {
  Server? Function() getServer;

  SetupGuard(this.getServer);

  @override
  void onNavigation(NavigationResolver resolver, StackRouter router) {
    final activeServer = getServer();
    if ((activeServer != null && resolver.route.path != '/setup') ||
        (resolver.route.path == '/setup' && activeServer == null)) {
      return resolver.next(true);
    }

    if (activeServer != null) {
      router.push(const MainScreenRoute());
    } else {
      router.push(const SetupScreenRoute());
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
