import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith_flutter/api.dart';
import 'package:zenith_flutter/main.dart';
import 'package:zenith_flutter/preferences.dart';
import 'package:zenith_flutter/screens/home.dart';
import 'package:zenith_flutter/screens/item_details/item_details.dart';
import 'package:zenith_flutter/screens/media_library.dart';
import 'package:zenith_flutter/screens/settings.dart';
import 'package:zenith_flutter/screens/setup.dart';
import 'package:zenith_flutter/screens/video_player.dart';

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
    ],
  ),
  AutoRoute(
    path: '/items/:id',
    page: ItemDetailsScreen,
    usesPathAsKey: true,
  ),
  AutoRoute(
    path: '/player/:id',
    page: VideoPlayerScreen,
    usesPathAsKey: true,
  ),
  AutoRoute(path: '/setup', page: SetupScreen, guards: [SetupGuard]),
  AutoRoute(path: '/settings', page: SettingsScreen),
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
    if ((activeServer != null && resolver.route.path != "/setup") ||
        (resolver.route.path == "/setup" && activeServer == null)) {
      return resolver.next(true);
    }

    if (activeServer != null) {
      router.push(const MainScreenRoute());
    } else {
      router.push(const SetupScreenRoute());
    }
  }
}

class MoviesScreen extends ConsumerWidget {
  const MoviesScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return MediaLibraryScreen(provider: ref.watch(apiProvider).fetchMovies);
  }
}

class ShowsScreen extends ConsumerWidget {
  const ShowsScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return MediaLibraryScreen(provider: ref.watch(apiProvider).fetchShows);
  }
}
