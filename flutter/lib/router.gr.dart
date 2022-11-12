// **************************************************************************
// AutoRouteGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND

// **************************************************************************
// AutoRouteGenerator
// **************************************************************************
//
// ignore_for_file: type=lint

part of 'router.dart';

class _$AppRouter extends RootStackRouter {
  _$AppRouter([GlobalKey<NavigatorState>? navigatorKey]) : super(navigatorKey);

  @override
  final Map<String, PageFactory> pagesMap = {
    MainScreenRoute.name: (routeData) {
      return MaterialPageX<dynamic>(
        routeData: routeData,
        child: const MainScreen(),
      );
    },
    ItemDetailsScreenRoute.name: (routeData) {
      final pathParams = routeData.inheritedPathParams;
      final args = routeData.argsAs<ItemDetailsScreenRouteArgs>(
          orElse: () =>
              ItemDetailsScreenRouteArgs(id: pathParams.getInt('id')));
      return MaterialPageX<dynamic>(
        routeData: routeData,
        child: ItemDetailsScreen(
          key: args.key,
          id: args.id,
        ),
      );
    },
    VideoPlayerScreenRoute.name: (routeData) {
      final pathParams = routeData.inheritedPathParams;
      final queryParams = routeData.queryParams;
      final args = routeData.argsAs<VideoPlayerScreenRouteArgs>(
          orElse: () => VideoPlayerScreenRouteArgs(
                id: pathParams.getInt('id'),
                startPosition: queryParams.getDouble(
                  'startPosition',
                  0,
                ),
              ));
      return MaterialPageX<dynamic>(
        routeData: routeData,
        child: VideoPlayerScreen(
          key: args.key,
          id: args.id,
          startPosition: args.startPosition,
        ),
      );
    },
    SettingsScreenRoute.name: (routeData) {
      final args = routeData.argsAs<SettingsScreenRouteArgs>(
          orElse: () => const SettingsScreenRouteArgs());
      return MaterialPageX<dynamic>(
        routeData: routeData,
        child: SettingsScreen(key: args.key),
      );
    },
    HomeScreenRoute.name: (routeData) {
      return MaterialPageX<dynamic>(
        routeData: routeData,
        child: const HomeScreen(),
      );
    },
    MoviesScreenRoute.name: (routeData) {
      return MaterialPageX<dynamic>(
        routeData: routeData,
        child: const MoviesScreen(),
      );
    },
    ShowsScreenRoute.name: (routeData) {
      return MaterialPageX<dynamic>(
        routeData: routeData,
        child: const ShowsScreen(),
      );
    },
  };

  @override
  List<RouteConfig> get routes => [
        RouteConfig(
          MainScreenRoute.name,
          path: '/',
          children: [
            RouteConfig(
              HomeScreenRoute.name,
              path: '',
              parent: MainScreenRoute.name,
            ),
            RouteConfig(
              MoviesScreenRoute.name,
              path: 'library/movies',
              parent: MainScreenRoute.name,
            ),
            RouteConfig(
              ShowsScreenRoute.name,
              path: 'library/shows',
              parent: MainScreenRoute.name,
            ),
          ],
        ),
        RouteConfig(
          ItemDetailsScreenRoute.name,
          path: '/items/:id',
          usesPathAsKey: true,
        ),
        RouteConfig(
          VideoPlayerScreenRoute.name,
          path: '/player/:id',
          usesPathAsKey: true,
        ),
        RouteConfig(
          SettingsScreenRoute.name,
          path: '/settings',
        ),
      ];
}

/// generated route for
/// [MainScreen]
class MainScreenRoute extends PageRouteInfo<void> {
  const MainScreenRoute({List<PageRouteInfo>? children})
      : super(
          MainScreenRoute.name,
          path: '/',
          initialChildren: children,
        );

  static const String name = 'MainScreenRoute';
}

/// generated route for
/// [ItemDetailsScreen]
class ItemDetailsScreenRoute extends PageRouteInfo<ItemDetailsScreenRouteArgs> {
  ItemDetailsScreenRoute({
    Key? key,
    required int id,
  }) : super(
          ItemDetailsScreenRoute.name,
          path: '/items/:id',
          args: ItemDetailsScreenRouteArgs(
            key: key,
            id: id,
          ),
          rawPathParams: {'id': id},
        );

  static const String name = 'ItemDetailsScreenRoute';
}

class ItemDetailsScreenRouteArgs {
  const ItemDetailsScreenRouteArgs({
    this.key,
    required this.id,
  });

  final Key? key;

  final int id;

  @override
  String toString() {
    return 'ItemDetailsScreenRouteArgs{key: $key, id: $id}';
  }
}

/// generated route for
/// [VideoPlayerScreen]
class VideoPlayerScreenRoute extends PageRouteInfo<VideoPlayerScreenRouteArgs> {
  VideoPlayerScreenRoute({
    Key? key,
    required int id,
    double startPosition = 0,
  }) : super(
          VideoPlayerScreenRoute.name,
          path: '/player/:id',
          args: VideoPlayerScreenRouteArgs(
            key: key,
            id: id,
            startPosition: startPosition,
          ),
          rawPathParams: {'id': id},
          rawQueryParams: {'startPosition': startPosition},
        );

  static const String name = 'VideoPlayerScreenRoute';
}

class VideoPlayerScreenRouteArgs {
  const VideoPlayerScreenRouteArgs({
    this.key,
    required this.id,
    this.startPosition = 0,
  });

  final Key? key;

  final int id;

  final double startPosition;

  @override
  String toString() {
    return 'VideoPlayerScreenRouteArgs{key: $key, id: $id, startPosition: $startPosition}';
  }
}

/// generated route for
/// [SettingsScreen]
class SettingsScreenRoute extends PageRouteInfo<SettingsScreenRouteArgs> {
  SettingsScreenRoute({Key? key})
      : super(
          SettingsScreenRoute.name,
          path: '/settings',
          args: SettingsScreenRouteArgs(key: key),
        );

  static const String name = 'SettingsScreenRoute';
}

class SettingsScreenRouteArgs {
  const SettingsScreenRouteArgs({this.key});

  final Key? key;

  @override
  String toString() {
    return 'SettingsScreenRouteArgs{key: $key}';
  }
}

/// generated route for
/// [HomeScreen]
class HomeScreenRoute extends PageRouteInfo<void> {
  const HomeScreenRoute()
      : super(
          HomeScreenRoute.name,
          path: '',
        );

  static const String name = 'HomeScreenRoute';
}

/// generated route for
/// [MoviesScreen]
class MoviesScreenRoute extends PageRouteInfo<void> {
  const MoviesScreenRoute()
      : super(
          MoviesScreenRoute.name,
          path: 'library/movies',
        );

  static const String name = 'MoviesScreenRoute';
}

/// generated route for
/// [ShowsScreen]
class ShowsScreenRoute extends PageRouteInfo<void> {
  const ShowsScreenRoute()
      : super(
          ShowsScreenRoute.name,
          path: 'library/shows',
        );

  static const String name = 'ShowsScreenRoute';
}
