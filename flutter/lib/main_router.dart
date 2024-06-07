import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/main.dart';
import 'package:zenith/preferences.dart';
import 'package:zenith/router/router.dart';
import 'package:zenith/router/stack_router.dart';
import 'package:zenith/routes/routes.dart';
import 'package:zenith/screens/setup.dart';
import 'package:zenith/screens/video_player/video_player.dart';

sealed class PrimaryRoute extends ZenithRoute {
  const PrimaryRoute();
}

class MainRoute extends PrimaryRoute {
  const MainRoute();

  @override
  String get location => '/';

  @override
  Widget build(BuildContext context) {
    return const MainScreen();
  }
}

class ItemDetailsRoute extends PrimaryRoute {
  final int id;

  const ItemDetailsRoute({required this.id});

  @override
  String get location => 'items/$id';

  @override
  Widget build(BuildContext context) {
    return ItemDetailsPage(id: id);
  }
}

class VideoPlayerRoute extends PrimaryRoute {
  final int id;
  final double startPosition;

  const VideoPlayerRoute({required this.id, required this.startPosition});

  @override
  String get location => '/player/$id?startPosition=$startPosition';

  @override
  Widget build(BuildContext context) {
    return VideoPlayerScreen(
      id: id,
      startPosition: startPosition,
    );
  }
}

class LoginRoute extends PrimaryRoute {
  final String? redirect;

  const LoginRoute({required this.redirect});

  @override
  String get location => '/login';

  @override
  Widget build(BuildContext context) {
    return LoginPage(redirect: redirect);
  }
}

class SetupRoute extends PrimaryRoute {
  const SetupRoute();

  @override
  String get location => '/setup';

  @override
  Widget build(BuildContext context) {
    return const SetupScreen();
  }
}

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
            return [
              const MainRoute(),
              if (location.uri.path.startsWith('/items/'))
                ItemDetailsRoute(id: int.parse(location.pathSegments[1])),
            ];
          },
        );
      },
    );
  }
}
