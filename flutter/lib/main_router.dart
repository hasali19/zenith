import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/main.dart';
import 'package:zenith/preferences.dart';
import 'package:zenith/router/router.dart';
import 'package:zenith/router/stack_router.dart';
import 'package:zenith/routes/routes.dart';
import 'package:zenith/screens/setup.dart';
import 'package:zenith/screens/video_player/video_player.dart';

class MainRouter extends ZenithRouter {
  @override
  Widget build(BuildContext context) {
    return Consumer(
      builder: (context, ref, child) {
        final activeServer = ref.read(activeServerProvider);
        return StackRouter<PrimaryRoute>(
          onSetLocation: (location) {
            if (activeServer == null) {
              return const [SetupRoute()];
            }

            final match =
                RegExp(r'/items/(\d+)').matchAsPrefix(location.location);
            return [
              const MainRoute(),
              if (match != null)
                ItemDetailsRoute(id: int.parse(match.group(1)!)),
            ];
          },
          buildLocation: (route) => switch (route) {
            MainRoute() => '/',
            ItemDetailsRoute(:final id) => '/items/$id',
            VideoPlayerRoute(:final id, :final startPosition) =>
              '/player/$id?startPosition=$startPosition',
            LoginRoute() => '/login',
            SetupRoute() => '/setup',
          },
          buildPage: (route) {
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
              SetupRoute() => MaterialPage(
                  key: ValueKey(route),
                  arguments: route,
                  child: const SetupScreen(),
                ),
            };
          },
        );
      },
    );
  }
}
