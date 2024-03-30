// GENERATED CODE - DO NOT MODIFY BY HAND

// **************************************************************************
// AutoRouterGenerator
// **************************************************************************

// ignore_for_file: type=lint
// coverage:ignore-file

part of 'router.dart';

abstract class _$AppRouter extends RootStackRouter {
  // ignore: unused_element
  _$AppRouter({super.navigatorKey});

  @override
  final Map<String, PageFactory> pagesMap = {
    HomeRoute.name: (routeData) {
      return AutoRoutePage<dynamic>(
        routeData: routeData,
        child: const HomeScreen(),
      );
    },
    ItemDetailsRoute.name: (routeData) {
      final pathParams = routeData.inheritedPathParams;
      final args = routeData.argsAs<ItemDetailsRouteArgs>(
          orElse: () => ItemDetailsRouteArgs(id: pathParams.getInt('id')));
      return AutoRoutePage<dynamic>(
        routeData: routeData,
        child: ItemDetailsPage(
          key: args.key,
          id: args.id,
        ),
      );
    },
    LoginRoute.name: (routeData) {
      final queryParams = routeData.queryParams;
      final args = routeData.argsAs<LoginRouteArgs>(
          orElse: () =>
              LoginRouteArgs(redirect: queryParams.optString('redirect')));
      return AutoRoutePage<dynamic>(
        routeData: routeData,
        child: LoginPage(
          key: args.key,
          redirect: args.redirect,
        ),
      );
    },
    LoginRegisterRoute.name: (routeData) {
      final queryParams = routeData.queryParams;
      final args = routeData.argsAs<LoginRegisterRouteArgs>(
          orElse: () => LoginRegisterRouteArgs(
                initial: queryParams.getBool(
                  'initial',
                  false,
                ),
                code: queryParams.optString('code'),
              ));
      return AutoRoutePage<dynamic>(
        routeData: routeData,
        child: LoginRegisterPage(
          key: args.key,
          initial: args.initial,
          code: args.code,
        ),
      );
    },
    LoginUserRoute.name: (routeData) {
      final queryParams = routeData.queryParams;
      final args = routeData.argsAs<LoginUserRouteArgs>(
          orElse: () =>
              LoginUserRouteArgs(username: queryParams.optString('username')));
      return AutoRoutePage<dynamic>(
        routeData: routeData,
        child: LoginUserPage(
          key: args.key,
          username: args.username,
        ),
      );
    },
    LoginUsersRoute.name: (routeData) {
      return AutoRoutePage<dynamic>(
        routeData: routeData,
        child: const LoginUsersPage(),
      );
    },
    MainRoute.name: (routeData) {
      return AutoRoutePage<dynamic>(
        routeData: routeData,
        child: const MainScreen(),
      );
    },
    MoviesRoute.name: (routeData) {
      return AutoRoutePage<dynamic>(
        routeData: routeData,
        child: const MoviesScreen(),
      );
    },
    SettingsRoute.name: (routeData) {
      return AutoRoutePage<dynamic>(
        routeData: routeData,
        child: const SettingsScreen(),
      );
    },
    SetupRoute.name: (routeData) {
      return AutoRoutePage<dynamic>(
        routeData: routeData,
        child: const SetupScreen(),
      );
    },
    ShowsRoute.name: (routeData) {
      return AutoRoutePage<dynamic>(
        routeData: routeData,
        child: const ShowsScreen(),
      );
    },
    VideoPlayerRoute.name: (routeData) {
      final pathParams = routeData.inheritedPathParams;
      final queryParams = routeData.queryParams;
      final args = routeData.argsAs<VideoPlayerRouteArgs>(
          orElse: () => VideoPlayerRouteArgs(
                id: pathParams.getInt('id'),
                startPosition: queryParams.getDouble(
                  'startPosition',
                  0,
                ),
              ));
      return AutoRoutePage<dynamic>(
        routeData: routeData,
        child: VideoPlayerScreen(
          key: args.key,
          id: args.id,
          startPosition: args.startPosition,
        ),
      );
    },
  };
}

/// generated route for
/// [HomeScreen]
class HomeRoute extends PageRouteInfo<void> {
  const HomeRoute({List<PageRouteInfo>? children})
      : super(
          HomeRoute.name,
          initialChildren: children,
        );

  static const String name = 'HomeRoute';

  static const PageInfo<void> page = PageInfo<void>(name);
}

/// generated route for
/// [ItemDetailsPage]
class ItemDetailsRoute extends PageRouteInfo<ItemDetailsRouteArgs> {
  ItemDetailsRoute({
    Key? key,
    required int id,
    List<PageRouteInfo>? children,
  }) : super(
          ItemDetailsRoute.name,
          args: ItemDetailsRouteArgs(
            key: key,
            id: id,
          ),
          rawPathParams: {'id': id},
          initialChildren: children,
        );

  static const String name = 'ItemDetailsRoute';

  static const PageInfo<ItemDetailsRouteArgs> page =
      PageInfo<ItemDetailsRouteArgs>(name);
}

class ItemDetailsRouteArgs {
  const ItemDetailsRouteArgs({
    this.key,
    required this.id,
  });

  final Key? key;

  final int id;

  @override
  String toString() {
    return 'ItemDetailsRouteArgs{key: $key, id: $id}';
  }
}

/// generated route for
/// [LoginPage]
class LoginRoute extends PageRouteInfo<LoginRouteArgs> {
  LoginRoute({
    Key? key,
    String? redirect,
    List<PageRouteInfo>? children,
  }) : super(
          LoginRoute.name,
          args: LoginRouteArgs(
            key: key,
            redirect: redirect,
          ),
          rawQueryParams: {'redirect': redirect},
          initialChildren: children,
        );

  static const String name = 'LoginRoute';

  static const PageInfo<LoginRouteArgs> page = PageInfo<LoginRouteArgs>(name);
}

class LoginRouteArgs {
  const LoginRouteArgs({
    this.key,
    this.redirect,
  });

  final Key? key;

  final String? redirect;

  @override
  String toString() {
    return 'LoginRouteArgs{key: $key, redirect: $redirect}';
  }
}

/// generated route for
/// [LoginRegisterPage]
class LoginRegisterRoute extends PageRouteInfo<LoginRegisterRouteArgs> {
  LoginRegisterRoute({
    Key? key,
    bool initial = false,
    String? code,
    List<PageRouteInfo>? children,
  }) : super(
          LoginRegisterRoute.name,
          args: LoginRegisterRouteArgs(
            key: key,
            initial: initial,
            code: code,
          ),
          rawQueryParams: {
            'initial': initial,
            'code': code,
          },
          initialChildren: children,
        );

  static const String name = 'LoginRegisterRoute';

  static const PageInfo<LoginRegisterRouteArgs> page =
      PageInfo<LoginRegisterRouteArgs>(name);
}

class LoginRegisterRouteArgs {
  const LoginRegisterRouteArgs({
    this.key,
    this.initial = false,
    this.code,
  });

  final Key? key;

  final bool initial;

  final String? code;

  @override
  String toString() {
    return 'LoginRegisterRouteArgs{key: $key, initial: $initial, code: $code}';
  }
}

/// generated route for
/// [LoginUserPage]
class LoginUserRoute extends PageRouteInfo<LoginUserRouteArgs> {
  LoginUserRoute({
    Key? key,
    required String? username,
    List<PageRouteInfo>? children,
  }) : super(
          LoginUserRoute.name,
          args: LoginUserRouteArgs(
            key: key,
            username: username,
          ),
          rawQueryParams: {'username': username},
          initialChildren: children,
        );

  static const String name = 'LoginUserRoute';

  static const PageInfo<LoginUserRouteArgs> page =
      PageInfo<LoginUserRouteArgs>(name);
}

class LoginUserRouteArgs {
  const LoginUserRouteArgs({
    this.key,
    required this.username,
  });

  final Key? key;

  final String? username;

  @override
  String toString() {
    return 'LoginUserRouteArgs{key: $key, username: $username}';
  }
}

/// generated route for
/// [LoginUsersPage]
class LoginUsersRoute extends PageRouteInfo<void> {
  const LoginUsersRoute({List<PageRouteInfo>? children})
      : super(
          LoginUsersRoute.name,
          initialChildren: children,
        );

  static const String name = 'LoginUsersRoute';

  static const PageInfo<void> page = PageInfo<void>(name);
}

/// generated route for
/// [MainScreen]
class MainRoute extends PageRouteInfo<void> {
  const MainRoute({List<PageRouteInfo>? children})
      : super(
          MainRoute.name,
          initialChildren: children,
        );

  static const String name = 'MainRoute';

  static const PageInfo<void> page = PageInfo<void>(name);
}

/// generated route for
/// [MoviesScreen]
class MoviesRoute extends PageRouteInfo<void> {
  const MoviesRoute({List<PageRouteInfo>? children})
      : super(
          MoviesRoute.name,
          initialChildren: children,
        );

  static const String name = 'MoviesRoute';

  static const PageInfo<void> page = PageInfo<void>(name);
}

/// generated route for
/// [SettingsScreen]
class SettingsRoute extends PageRouteInfo<void> {
  const SettingsRoute({List<PageRouteInfo>? children})
      : super(
          SettingsRoute.name,
          initialChildren: children,
        );

  static const String name = 'SettingsRoute';

  static const PageInfo<void> page = PageInfo<void>(name);
}

/// generated route for
/// [SetupScreen]
class SetupRoute extends PageRouteInfo<void> {
  const SetupRoute({List<PageRouteInfo>? children})
      : super(
          SetupRoute.name,
          initialChildren: children,
        );

  static const String name = 'SetupRoute';

  static const PageInfo<void> page = PageInfo<void>(name);
}

/// generated route for
/// [ShowsScreen]
class ShowsRoute extends PageRouteInfo<void> {
  const ShowsRoute({List<PageRouteInfo>? children})
      : super(
          ShowsRoute.name,
          initialChildren: children,
        );

  static const String name = 'ShowsRoute';

  static const PageInfo<void> page = PageInfo<void>(name);
}

/// generated route for
/// [VideoPlayerScreen]
class VideoPlayerRoute extends PageRouteInfo<VideoPlayerRouteArgs> {
  VideoPlayerRoute({
    Key? key,
    required int id,
    double startPosition = 0,
    List<PageRouteInfo>? children,
  }) : super(
          VideoPlayerRoute.name,
          args: VideoPlayerRouteArgs(
            key: key,
            id: id,
            startPosition: startPosition,
          ),
          rawPathParams: {'id': id},
          rawQueryParams: {'startPosition': startPosition},
          initialChildren: children,
        );

  static const String name = 'VideoPlayerRoute';

  static const PageInfo<VideoPlayerRouteArgs> page =
      PageInfo<VideoPlayerRouteArgs>(name);
}

class VideoPlayerRouteArgs {
  const VideoPlayerRouteArgs({
    this.key,
    required this.id,
    this.startPosition = 0,
  });

  final Key? key;

  final int id;

  final double startPosition;

  @override
  String toString() {
    return 'VideoPlayerRouteArgs{key: $key, id: $id, startPosition: $startPosition}';
  }
}
