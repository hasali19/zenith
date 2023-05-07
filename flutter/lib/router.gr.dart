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
  _$AppRouter({
    GlobalKey<NavigatorState>? navigatorKey,
    required this.serverSetupGuard,
    required this.authGuard,
  }) : super(navigatorKey);

  final ServerSetupGuard serverSetupGuard;

  final AuthGuard authGuard;

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
    CollectionDetailsScreenRoute.name: (routeData) {
      final pathParams = routeData.inheritedPathParams;
      final args = routeData.argsAs<CollectionDetailsScreenRouteArgs>(
          orElse: () =>
              CollectionDetailsScreenRouteArgs(id: pathParams.getInt('id')));
      return MaterialPageX<dynamic>(
        routeData: routeData,
        child: CollectionDetailsScreen(
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
    LoginScreenRoute.name: (routeData) {
      return MaterialPageX<dynamic>(
        routeData: routeData,
        child: const LoginScreen(),
      );
    },
    SetupScreenRoute.name: (routeData) {
      return MaterialPageX<dynamic>(
        routeData: routeData,
        child: const SetupScreen(),
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
    CollectionsScreenRoute.name: (routeData) {
      return MaterialPageX<dynamic>(
        routeData: routeData,
        child: const CollectionsScreen(),
      );
    },
    SettingsScreenRoute.name: (routeData) {
      return MaterialPageX<dynamic>(
        routeData: routeData,
        child: const SettingsScreen(),
      );
    },
    LoginUsersScreenRoute.name: (routeData) {
      return MaterialPageX<dynamic>(
        routeData: routeData,
        child: const LoginUsersScreen(),
      );
    },
    LoginUserScreenRoute.name: (routeData) {
      final queryParams = routeData.queryParams;
      final args = routeData.argsAs<LoginUserScreenRouteArgs>(
          orElse: () => LoginUserScreenRouteArgs(
              username: queryParams.optString('username')));
      return MaterialPageX<dynamic>(
        routeData: routeData,
        child: LoginUserScreen(
          key: args.key,
          username: args.username,
        ),
      );
    },
    LoginRegisterScreenRoute.name: (routeData) {
      return MaterialPageX<dynamic>(
        routeData: routeData,
        child: const LoginRegisterScreen(),
      );
    },
  };

  @override
  List<RouteConfig> get routes => [
        RouteConfig(
          MainScreenRoute.name,
          path: '/',
          guards: [
            serverSetupGuard,
            authGuard,
          ],
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
            RouteConfig(
              CollectionsScreenRoute.name,
              path: 'library/collections',
              parent: MainScreenRoute.name,
            ),
            RouteConfig(
              SettingsScreenRoute.name,
              path: 'settings',
              parent: MainScreenRoute.name,
            ),
          ],
        ),
        RouteConfig(
          ItemDetailsScreenRoute.name,
          path: '/items/:id',
          usesPathAsKey: true,
          guards: [
            serverSetupGuard,
            authGuard,
          ],
        ),
        RouteConfig(
          CollectionDetailsScreenRoute.name,
          path: '/collections/:id',
          usesPathAsKey: true,
          guards: [
            serverSetupGuard,
            authGuard,
          ],
        ),
        RouteConfig(
          VideoPlayerScreenRoute.name,
          path: '/player/:id',
          usesPathAsKey: true,
          guards: [
            serverSetupGuard,
            authGuard,
          ],
        ),
        RouteConfig(
          LoginScreenRoute.name,
          path: '/login',
          guards: [serverSetupGuard],
          children: [
            RouteConfig(
              LoginUsersScreenRoute.name,
              path: '',
              parent: LoginScreenRoute.name,
            ),
            RouteConfig(
              LoginUserScreenRoute.name,
              path: 'user',
              parent: LoginScreenRoute.name,
            ),
            RouteConfig(
              LoginRegisterScreenRoute.name,
              path: 'register',
              parent: LoginScreenRoute.name,
            ),
          ],
        ),
        RouteConfig(
          SetupScreenRoute.name,
          path: '/setup',
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
/// [CollectionDetailsScreen]
class CollectionDetailsScreenRoute
    extends PageRouteInfo<CollectionDetailsScreenRouteArgs> {
  CollectionDetailsScreenRoute({
    Key? key,
    required int id,
  }) : super(
          CollectionDetailsScreenRoute.name,
          path: '/collections/:id',
          args: CollectionDetailsScreenRouteArgs(
            key: key,
            id: id,
          ),
          rawPathParams: {'id': id},
        );

  static const String name = 'CollectionDetailsScreenRoute';
}

class CollectionDetailsScreenRouteArgs {
  const CollectionDetailsScreenRouteArgs({
    this.key,
    required this.id,
  });

  final Key? key;

  final int id;

  @override
  String toString() {
    return 'CollectionDetailsScreenRouteArgs{key: $key, id: $id}';
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
/// [LoginScreen]
class LoginScreenRoute extends PageRouteInfo<void> {
  const LoginScreenRoute({List<PageRouteInfo>? children})
      : super(
          LoginScreenRoute.name,
          path: '/login',
          initialChildren: children,
        );

  static const String name = 'LoginScreenRoute';
}

/// generated route for
/// [SetupScreen]
class SetupScreenRoute extends PageRouteInfo<void> {
  const SetupScreenRoute()
      : super(
          SetupScreenRoute.name,
          path: '/setup',
        );

  static const String name = 'SetupScreenRoute';
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

/// generated route for
/// [CollectionsScreen]
class CollectionsScreenRoute extends PageRouteInfo<void> {
  const CollectionsScreenRoute()
      : super(
          CollectionsScreenRoute.name,
          path: 'library/collections',
        );

  static const String name = 'CollectionsScreenRoute';
}

/// generated route for
/// [SettingsScreen]
class SettingsScreenRoute extends PageRouteInfo<void> {
  const SettingsScreenRoute()
      : super(
          SettingsScreenRoute.name,
          path: 'settings',
        );

  static const String name = 'SettingsScreenRoute';
}

/// generated route for
/// [LoginUsersScreen]
class LoginUsersScreenRoute extends PageRouteInfo<void> {
  const LoginUsersScreenRoute()
      : super(
          LoginUsersScreenRoute.name,
          path: '',
        );

  static const String name = 'LoginUsersScreenRoute';
}

/// generated route for
/// [LoginUserScreen]
class LoginUserScreenRoute extends PageRouteInfo<LoginUserScreenRouteArgs> {
  LoginUserScreenRoute({
    Key? key,
    required String? username,
  }) : super(
          LoginUserScreenRoute.name,
          path: 'user',
          args: LoginUserScreenRouteArgs(
            key: key,
            username: username,
          ),
          rawQueryParams: {'username': username},
        );

  static const String name = 'LoginUserScreenRoute';
}

class LoginUserScreenRouteArgs {
  const LoginUserScreenRouteArgs({
    this.key,
    required this.username,
  });

  final Key? key;

  final String? username;

  @override
  String toString() {
    return 'LoginUserScreenRouteArgs{key: $key, username: $username}';
  }
}

/// generated route for
/// [LoginRegisterScreen]
class LoginRegisterScreenRoute extends PageRouteInfo<void> {
  const LoginRegisterScreenRoute()
      : super(
          LoginRegisterScreenRoute.name,
          path: 'register',
        );

  static const String name = 'LoginRegisterScreenRoute';
}
