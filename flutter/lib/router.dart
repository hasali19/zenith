import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:zenith_flutter/api.dart';
import 'package:zenith_flutter/main.dart';
import 'package:zenith_flutter/screens/home.dart';
import 'package:zenith_flutter/screens/item_details/item_details.dart';
import 'package:zenith_flutter/screens/media_library.dart';
import 'package:zenith_flutter/screens/settings.dart';
import 'package:zenith_flutter/screens/video_player.dart';

part 'router.gr.dart';

@MaterialAutoRouter(routes: [
  AutoRoute(
    path: '/',
    page: MainScreen,
    initial: true,
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
  AutoRoute(path: '/settings', page: SettingsScreen),
])
class AppRouter extends _$AppRouter {}

class MoviesScreen extends StatelessWidget {
  const MoviesScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return const MediaLibraryScreen(provider: fetchMovies);
  }
}

class ShowsScreen extends StatelessWidget {
  const ShowsScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return const MediaLibraryScreen(provider: fetchShows);
  }
}
