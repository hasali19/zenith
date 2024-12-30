// GENERATED CODE - DO NOT MODIFY BY HAND

// **************************************************************************
// AutoRouterGenerator
// **************************************************************************

// ignore_for_file: type=lint
// coverage:ignore-file

part of 'router.dart';

/// generated route for
/// [HomeScreen]
class HomeRoute extends PageRouteInfo<void> {
  const HomeRoute({List<PageRouteInfo>? children})
      : super(
          HomeRoute.name,
          initialChildren: children,
        );

  static const String name = 'HomeRoute';

  static PageInfo page = PageInfo(
    name,
    builder: (data) {
      return const HomeScreen();
    },
  );
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

  static PageInfo page = PageInfo(
    name,
    builder: (data) {
      final pathParams = data.inheritedPathParams;
      final args = data.argsAs<ItemDetailsRouteArgs>(
          orElse: () => ItemDetailsRouteArgs(id: pathParams.getInt('id')));
      return ItemDetailsPage(
        key: args.key,
        id: args.id,
      );
    },
  );
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
/// [LibraryPage]
class LibraryRoute extends PageRouteInfo<void> {
  const LibraryRoute({List<PageRouteInfo>? children})
      : super(
          LibraryRoute.name,
          initialChildren: children,
        );

  static const String name = 'LibraryRoute';

  static PageInfo page = PageInfo(
    name,
    builder: (data) {
      return const LibraryPage();
    },
  );
}

/// generated route for
/// [LibraryTabsPage]
class LibraryTabsRoute extends PageRouteInfo<void> {
  const LibraryTabsRoute({List<PageRouteInfo>? children})
      : super(
          LibraryTabsRoute.name,
          initialChildren: children,
        );

  static const String name = 'LibraryTabsRoute';

  static PageInfo page = PageInfo(
    name,
    builder: (data) {
      return const LibraryTabsPage();
    },
  );
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

  static PageInfo page = PageInfo(
    name,
    builder: (data) {
      final queryParams = data.queryParams;
      final args = data.argsAs<LoginRouteArgs>(
          orElse: () =>
              LoginRouteArgs(redirect: queryParams.optString('redirect')));
      return LoginPage(
        key: args.key,
        redirect: args.redirect,
      );
    },
  );
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

  static PageInfo page = PageInfo(
    name,
    builder: (data) {
      final queryParams = data.queryParams;
      final args = data.argsAs<LoginRegisterRouteArgs>(
          orElse: () => LoginRegisterRouteArgs(
                initial: queryParams.getBool(
                  'initial',
                  false,
                ),
                code: queryParams.optString('code'),
              ));
      return LoginRegisterPage(
        key: args.key,
        initial: args.initial,
        code: args.code,
      );
    },
  );
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

  static PageInfo page = PageInfo(
    name,
    builder: (data) {
      final queryParams = data.queryParams;
      final args = data.argsAs<LoginUserRouteArgs>(
          orElse: () =>
              LoginUserRouteArgs(username: queryParams.optString('username')));
      return LoginUserPage(
        key: args.key,
        username: args.username,
      );
    },
  );
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

  static PageInfo page = PageInfo(
    name,
    builder: (data) {
      return const LoginUsersPage();
    },
  );
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

  static PageInfo page = PageInfo(
    name,
    builder: (data) {
      return const MainScreen();
    },
  );
}

/// generated route for
/// [ManageServerPage]
class ManageServerRoute extends PageRouteInfo<void> {
  const ManageServerRoute({List<PageRouteInfo>? children})
      : super(
          ManageServerRoute.name,
          initialChildren: children,
        );

  static const String name = 'ManageServerRoute';

  static PageInfo page = PageInfo(
    name,
    builder: (data) {
      return const ManageServerPage();
    },
  );
}

/// generated route for
/// [ManageServerShellPage]
class ManageServerShellRoute extends PageRouteInfo<void> {
  const ManageServerShellRoute({List<PageRouteInfo>? children})
      : super(
          ManageServerShellRoute.name,
          initialChildren: children,
        );

  static const String name = 'ManageServerShellRoute';

  static PageInfo page = PageInfo(
    name,
    builder: (data) {
      return const ManageServerShellPage();
    },
  );
}

/// generated route for
/// [ManageUsersPage]
class ManageUsersRoute extends PageRouteInfo<void> {
  const ManageUsersRoute({List<PageRouteInfo>? children})
      : super(
          ManageUsersRoute.name,
          initialChildren: children,
        );

  static const String name = 'ManageUsersRoute';

  static PageInfo page = PageInfo(
    name,
    builder: (data) {
      return const ManageUsersPage();
    },
  );
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

  static PageInfo page = PageInfo(
    name,
    builder: (data) {
      return const MoviesScreen();
    },
  );
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

  static PageInfo page = PageInfo(
    name,
    builder: (data) {
      return const SettingsScreen();
    },
  );
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

  static PageInfo page = PageInfo(
    name,
    builder: (data) {
      return const SetupScreen();
    },
  );
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

  static PageInfo page = PageInfo(
    name,
    builder: (data) {
      return const ShowsScreen();
    },
  );
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

  static PageInfo page = PageInfo(
    name,
    builder: (data) {
      final pathParams = data.inheritedPathParams;
      final queryParams = data.queryParams;
      final args = data.argsAs<VideoPlayerRouteArgs>(
          orElse: () => VideoPlayerRouteArgs(
                id: pathParams.getInt('id'),
                startPosition: queryParams.getDouble(
                  'startPosition',
                  0,
                ),
              ));
      return VideoPlayerScreen(
        key: args.key,
        id: args.id,
        startPosition: args.startPosition,
      );
    },
  );
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
