import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/api.dart';
import 'package:zenith/main.dart';
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
    page: MainScreen,
    initial: true,
    guards: [ServerSetupGuard, AuthGuard],
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
    guards: [ServerSetupGuard, AuthGuard],
  ),
  AutoRoute(
    path: '/collections/:id',
    page: CollectionDetailsScreen,
    usesPathAsKey: true,
    guards: [ServerSetupGuard, AuthGuard],
  ),
  AutoRoute(
    path: '/player/:id',
    page: VideoPlayerScreen,
    usesPathAsKey: true,
    guards: [ServerSetupGuard, AuthGuard],
  ),
  AutoRoute(
    path: '/login',
    page: LoginScreen,
    guards: [ServerSetupGuard],
    children: [
      AutoRoute(page: LoginUsersScreen, initial: true),
      AutoRoute(path: 'user', page: LoginUserScreen),
      AutoRoute(path: 'register', page: LoginRegisterScreen),
    ],
  ),
  AutoRoute(path: '/setup', page: SetupScreen),
])
class AppRouter extends _$AppRouter {
  AppRouter({
    required ServerSetupGuard serverSetupGuard,
    required AuthGuard authGuard,
  }) : super(
          serverSetupGuard: serverSetupGuard,
          authGuard: authGuard,
        );
}

class ServerSetupGuard extends AutoRouteGuard {
  final Future<bool> Function() isServerSet;

  ServerSetupGuard(this.isServerSet);

  @override
  void onNavigation(NavigationResolver resolver, StackRouter router) async {
    if (await isServerSet()) {
      return resolver.next(true);
    }

    router.replace(const SetupScreenRoute());
  }
}

class AuthGuard extends AutoRouteGuard {
  final Future<bool> Function() isLoggedIn;

  AuthGuard(this.isLoggedIn);

  @override
  void onNavigation(NavigationResolver resolver, StackRouter router) async {
    if (await isLoggedIn()) {
      return resolver.next(true);
    }

    router.replace(const LoginScreenRoute());
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
